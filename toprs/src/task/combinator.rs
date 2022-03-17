use async_trait::async_trait;
use either::Either;

use crate::component::event::EventHandler;
use crate::task::value::TaskValue;
use crate::task::{Executor, Task, TaskError};

/// Basic sequential task. Consists of a current task, along with one or more [`Continuation`]s that
/// decide when the current task should finish and what to do with the result.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Then<T1, F, T2> {
    current: Either<T1, T2>,
    continuations: Vec<Continuation<F>>,
}

/// Continuation of a [`Then`] task. Decides when the current task is consumed, using its value to
/// construct the next task.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Continuation<F> {
    /// Consume the current task as soon as the value satisfies the predicate.
    OnValue(F),
    /// Consume the current task as soon as the user performs and action and the value satisfies the
    /// predicate.
    OnAction(Action, F),
}

/// Actions that are represented as buttons in the user interface, used in [`Continuation`]s. When
/// the user presses the associated button, and the associated predicate in the continuation is met,
/// the current task is consumed and the next task will be created from the resulting value.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Action(pub &'static str);

impl Action {
    pub const OK: Self = Action("Ok");
    pub const CANCEL: Self = Action("Cancel");
    pub const YES: Self = Action("Yes");
    pub const NO: Self = Action("No");
    pub const NEXT: Self = Action("Next");
    pub const PREVIOUS: Self = Action("Previous");
    pub const FINISH: Self = Action("Finish");
    pub const CONTINUE: Self = Action("Continue");
    pub const NEW: Self = Action("New");
    pub const EDIT: Self = Action("Edit");
    pub const DELETE: Self = Action("Delete");
    pub const REFRESH: Self = Action("Refresh");
    pub const CLOSE: Self = Action("Close");
}

#[async_trait]
impl<T1, F, T2> Task for Then<T1, F, T2>
where
    T1: Task + Send,
    T1::Value: Clone + Send,
    F: Fn(TaskValue<T1::Value>) -> Option<T2> + Send,
    T2: Task + Send,
{
    type Value = T2::Value;

    async fn start(&mut self, executor: &mut Executor<impl EventHandler + Send>) {
        match &mut self.current {
            Either::Left(task) => task.start(executor).await,
            Either::Right(task) => task.start(executor).await,
        }
    }

    async fn inspect(
        &mut self,
        executor: &mut Executor<impl EventHandler + Send>,
    ) -> Result<TaskValue<Self::Value>, TaskError> {
        match &mut self.current {
            Either::Left(first) => {
                let value = first.inspect(executor).await?;
                let next = self.continuations.iter().find_map(|cont| match cont {
                    Continuation::OnValue(f) | Continuation::OnAction(_, f) => f(value.clone()),
                });
                if let Some(mut next) = next {
                    next.start(executor).await;
                    self.current = Either::Right(next);
                }
                Ok(TaskValue::Empty)
            }
            Either::Right(then) => then.inspect(executor).await,
        }
    }
}

pub trait TaskExt: Task {
    fn then<F, T2>(self, f: F) -> Then<Self, F, T2>
    where
        Self: Task + Sized,
        F: Fn(TaskValue<Self::Value>) -> Option<T2>,
        T2: Task,
    {
        Then {
            current: Either::Left(self),
            continuations: vec![Continuation::OnAction(Action::NEXT, f)],
        }
    }
}

impl<T> TaskExt for T where T: Task {}
