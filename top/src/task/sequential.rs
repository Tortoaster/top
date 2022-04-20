use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use either::Either;

use top_derive::html;

use crate::html::event::{Event, Feedback};
use crate::html::id::{Generator, Id};
use crate::html::{Html, ToHtml};
use crate::task::{Context, Task, TaskError, TaskResult, TaskValue};

/// Basic sequential task. Consists of a current task, along with one or more [`Continuation`]s that
/// decide when the current task should finish and what to do with the result.
pub struct Sequential<T, B>
where
    T: Task,
{
    id: Id,
    current: Either<T, Box<dyn Task<Value = B>>>,
    continuations: Vec<Continuation<T::Value, B>>,
}

impl<T, B> Sequential<T, B>
where
    T: Task,
{
    pub fn on_value(
        mut self,
        f: impl Fn(TaskValue<T::Value>) -> Option<Box<dyn Task<Value = B>>> + Send + 'static,
    ) -> Self {
        self.continuations.push(Continuation::OnValue(Box::new(f)));
        self
    }

    pub fn on_action(
        mut self,
        action: Action,
        f: impl Fn(TaskValue<T::Value>) -> Option<Box<dyn Task<Value = B>>> + Send + 'static,
    ) -> Self {
        self.continuations
            .push(Continuation::OnAction(action, Box::new(f)));
        self
    }
}

#[async_trait]
impl<T, B> Task for Sequential<T, B>
where
    T: Task + Debug + Send,
    T::Value: Clone + Send,
    B: Send,
{
    type Value = B;

    async fn start(&mut self, gen: &mut Generator) -> Result<Html, TaskError> {
        self.id = gen.next();

        let task = self
            .current
            .as_mut()
            .left()
            .ok_or(TaskError::State)?
            .start(gen)
            .await?;

        let buttons: Html = self
            .continuations
            .iter_mut()
            .flat_map(|cont| {
                if let Continuation::OnAction(action, _) = cont {
                    action.1 = gen.next();
                    Some(action.to_html())
                } else {
                    None
                }
            })
            .collect();

        let id = self.id;
        let html = html! {r#"
            <div id={id}>
                {task}
                {buttons}
            </div>
        "#};

        Ok(html)
    }

    async fn on_event(&mut self, event: Event, ctx: &mut Context) -> TaskResult<Self::Value> {
        match &mut self.current {
            Either::Left(task) => {
                let value = task.on_event(event.clone(), ctx).await?;

                let next = self.continuations.iter().find_map(|cont| match cont {
                    Continuation::OnValue(f) => f(value.clone()),
                    Continuation::OnAction(action, f) => {
                        if let Event::Press { id } = &event {
                            if action.1 == *id {
                                f(value.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                });

                if let Some(mut next) = next {
                    let task = next.start(&mut ctx.gen).await?;

                    self.continuations.clear();
                    self.current = Either::Right(next);

                    let id = self.id;
                    let html = html! {r#"
                        <div id={id}>
                            {task}
                        </div>
                    "#};

                    ctx.feedback
                        .send(Feedback::Replace { id: self.id, html })
                        .await?;
                }

                Ok(TaskValue::Empty)
            }
            Either::Right(task) => task.on_event(event, ctx).await,
        }
    }
}

impl<T, B> Debug for Sequential<T, B>
where
    T: Task,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sequential {{ id: {:?} }}", self.id)
    }
}

/// Continuation of a [`Then`] task. Decides when the current task is consumed, using its value to
/// construct the next task.
pub enum Continuation<A, B> {
    /// Consume the current task as soon as the value satisfies the predicate.
    OnValue(Box<dyn Fn(TaskValue<A>) -> Option<Box<dyn Task<Value = B>>> + Send>),
    /// Consume the current task as soon as the user performs and action and the value satisfies the
    /// predicate.
    OnAction(
        Action,
        Box<dyn Fn(TaskValue<A>) -> Option<Box<dyn Task<Value = B>>> + Send>,
    ),
}

/// Actions that are represented as buttons in the user interface, used in [`Continuation`]s. When
/// the user presses the associated button, and the associated predicate in the continuation is met,
/// the current task is consumed and the next task will be created from the resulting value.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Action(&'static str, Id);

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
        Action(label, Id::INVALID)
    }
}

impl ToHtml for Action {
    fn to_html(&self) -> Html {
        html! {r#"
            <button id="{self.1}" class="button is-link" type="button" onclick="press(this)">
                {self.0}
            </button>
        "#}
    }
}

/// Adds the [`steps`] method to any task, allowing it to become a sequential task through the
/// [`Steps`] builder.
pub trait TaskSequentialExt: Task {
    fn then<B>(self) -> Sequential<Self, B>
    where
        Self: Sized,
    {
        Sequential {
            id: Id::INVALID,
            current: Either::Left(self),
            continuations: Vec::new(),
        }
    }
}

impl<T> TaskSequentialExt for T where T: Task {}

/// Utility function to turn a simple mapping function into the type of closure a
/// [`Continuation`] requires.
pub fn has_value<A, T>(
    f: impl Fn(A) -> T,
) -> impl Fn(TaskValue<A>) -> Option<Box<dyn Task<Value = T::Value>>>
where
    T: Task + 'static,
{
    move |value| {
        Option::from(value).map(|x| {
            let result: Box<dyn Task<Value = T::Value>> = Box::new(f(x));
            result
        })
    }
}

pub fn if_stable<A, T>(
    f: impl Fn(A) -> T,
) -> impl Fn(TaskValue<A>) -> Option<Box<dyn Task<Value = T::Value>>>
where
    T: Task + 'static,
{
    move |value| match value {
        TaskValue::Stable(x) => {
            let result: Box<dyn Task<Value = T::Value>> = Box::new(f(x));
            Some(result)
        }
        _ => None,
    }
}

pub fn if_unstable<A, T>(
    f: impl Fn(A) -> T,
) -> impl Fn(TaskValue<A>) -> Option<Box<dyn Task<Value = T::Value>>>
where
    T: Task + 'static,
{
    move |value| match value {
        TaskValue::Unstable(x) => {
            let result: Box<dyn Task<Value = T::Value>> = Box::new(f(x));
            Some(result)
        }
        _ => None,
    }
}

/// Utility function to turn a simple mapping function into the type of closure a
/// [`Continuation`] requires, but only if its value satisfies the predicate `f`.
pub fn if_value<A, T>(
    f: impl Fn(&A) -> bool,
    g: impl Fn(A) -> T,
) -> impl Fn(TaskValue<A>) -> Option<Box<dyn Task<Value = T::Value>>>
where
    T: Task + 'static,
{
    move |value| {
        Option::from(value).and_then(|x| {
            f(&x).then(|| {
                let result: Box<dyn Task<Value = T::Value>> = Box::new(g(x));
                result
            })
        })
    }
}
