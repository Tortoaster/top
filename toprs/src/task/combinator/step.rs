use async_trait::async_trait;
use either::Either;

use crate::component::event::{Event, Feedback, FeedbackHandler};
use crate::component::{Id, Widget};
use crate::task::value::TaskValue;
use crate::task::{Context, Error, Task};

/// Continuation of a [`Then`] task. Decides when the current task is consumed, using its value to
/// construct the next task.
pub enum Continuation<A, B> {
    /// Consume the current task as soon as the value satisfies the predicate.
    OnValue(Box<dyn Fn(TaskValue<A>) -> Option<B> + Send>),
    /// Consume the current task as soon as the user performs and action and the value satisfies the
    /// predicate.
    OnAction(Action, Box<dyn Fn(TaskValue<A>) -> Option<B> + Send>),
}

/// Actions that are represented as buttons in the user interface, used in [`Continuation`]s. When
/// the user presses the associated button, and the associated predicate in the continuation is met,
/// the current task is consumed and the next task will be created from the resulting value.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Action(&'static str, Option<Id>);

impl Action {
    pub const OK: Self = Action::new("Ok");
    pub const CANCEL: Self = Action::new("Cancel");
    pub const YES: Self = Action::new("Yes");
    pub const NO: Self = Action::new("No");
    pub const NEXT: Self = Action::new("Next");
    pub const PREVIOUS: Self = Action::new("Previous");
    pub const FINISH: Self = Action::new("Finish");
    pub const CONTINUE: Self = Action::new("Continue");
    pub const NEW: Self = Action::new("New");
    pub const EDIT: Self = Action::new("Edit");
    pub const DELETE: Self = Action::new("Delete");
    pub const REFRESH: Self = Action::new("Refresh");
    pub const CLOSE: Self = Action::new("Close");

    pub const fn new(label: &'static str) -> Self {
        Action(label, None)
    }
}

/// Basic sequential task. Consists of a current task, along with one or more [`Continuation`]s that
/// decide when the current task should finish and what to do with the result.
pub struct Step<T1: Task, T2> {
    current: Either<T1, T2>,
    continuations: Vec<Continuation<T1::Value, T2>>,
}

#[async_trait]
impl<T1, T2> Task for Step<T1, T2>
where
    T1: Task + Send,
    T1::Value: Clone + Send,
    T2: Task + Send,
{
    type Value = T2::Value;

    async fn start<H: FeedbackHandler + Send>(
        &mut self,
        ctx: &mut Context<H>,
    ) -> Result<(), Error<H::Error>> {
        match &mut self.current {
            Either::Left(task) => {
                task.start(ctx).await?;
                for cont in &mut self.continuations {
                    if let Continuation::OnAction(action, _) = cont {
                        let widget = Widget::Button {
                            text: action.0.to_owned(),
                            disabled: false,
                        };
                        let button = ctx.components.create(widget);
                        // TODO: Type-safe way?
                        action.1 = Some(button.id());
                        let feedback = Feedback::Append {
                            id: Id::ROOT,
                            component: button,
                        };
                        ctx.feedback.send(feedback).await?;
                    }
                }
                Ok(())
            }
            Either::Right(task) => task.start(ctx).await,
        }
    }

    async fn on_event<H: FeedbackHandler + Send>(
        &mut self,
        event: Event,
        ctx: &mut Context<H>,
    ) -> Result<TaskValue<Self::Value>, Error<H::Error>> {
        match &mut self.current {
            Either::Left(first) => {
                let value = first.on_event(event.clone(), ctx).await?;
                let next = self.continuations.iter().find_map(|cont| match cont {
                    Continuation::OnValue(f) => f(value.clone()),
                    Continuation::OnAction(action, f) => {
                        if let Event::Press { id } = &event {
                            if action
                                .1
                                .map(|action_id| action_id == *id)
                                .unwrap_or_default()
                            {
                                f(value.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                });
                if let Some(next) = next {
                    self.current = Either::Right(next);
                    self.start(ctx).await?;
                }
                Ok(TaskValue::Empty)
            }
            Either::Right(then) => then.on_event(event, ctx).await,
        }
    }
}

/// Contains functionality that makes it easy to construct sequential tasks.
pub mod dsl {
    use either::Either;

    use crate::prelude::Action;
    use crate::task::combinator::step::{Continuation, Step};
    use crate::task::value::TaskValue;
    use crate::task::Task;

    /// Adds the [`steps`] method to any task, allowing it to become a sequential task through the
    /// [`Steps`] builder.
    pub trait TaskStepExt: Task {
        fn steps<T2>(self) -> Steps<Self, T2>
        where
            Self: Sized,
        {
            Steps {
                current: self,
                continuations: Vec::new(),
            }
        }
    }

    impl<T> TaskStepExt for T where T: Task {}

    /// Builder for the step combinator.
    pub struct Steps<T1: Task, T2> {
        current: T1,
        continuations: Vec<Continuation<T1::Value, T2>>,
    }

    impl<T1, T2> Steps<T1, T2>
    where
        T1: Task,
    {
        /// Consume the current task value and use it to construct the next task as soon as `f`
        /// returns [`Some`] task. Use [`has_value`] or [`if_value`] for `f` to simplify it.
        pub fn on_value(
            mut self,
            f: impl Fn(TaskValue<T1::Value>) -> Option<T2> + Send + 'static,
        ) -> Self {
            self.continuations.push(Continuation::OnValue(Box::new(f)));
            self
        }

        /// Adds a button to the user interface; consumes the current task value and uses it to
        /// construct the next task when the button is pressed, but only if `f` returns [`Some`]
        /// task. Use [`has_value`] or [`if_value`] for `f` to simplify it.
        pub fn on_action(
            mut self,
            action: Action,
            f: impl Fn(TaskValue<T1::Value>) -> Option<T2> + Send + 'static,
        ) -> Self {
            self.continuations
                .push(Continuation::OnAction(action, Box::new(f)));
            self
        }

        /// Turn this builder into a sequential task.
        pub fn confirm(self) -> Step<T1, T2> {
            Step {
                current: Either::Left(self.current),
                continuations: self.continuations,
            }
        }
    }

    /// Utility function to turn a simple mapping function into the type of closure a
    /// [`Continuation`] requires.
    pub fn has_value<A, B>(f: impl Fn(A) -> B) -> impl Fn(TaskValue<A>) -> Option<B> {
        move |value| value.into_option().map(|x| f(x))
    }

    /// Utility function to turn a simple mapping function into the type of closure a
    /// [`Continuation`] requires, but only if its value satisfies the predicate `f`.
    pub fn if_value<A, B>(
        f: impl Fn(&A) -> bool,
        g: impl Fn(A) -> B,
    ) -> impl Fn(TaskValue<A>) -> Option<B> {
        move |value| value.into_option().and_then(|x| f(&x).then(|| g(x)))
    }
}
