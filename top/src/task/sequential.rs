use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use either::Either;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::id::{Generator, Id};
use crate::html::{Html, ToHtml};
use crate::task::{Result, Task, TaskError, TaskValue};

/// Basic sequential task. Consists of a current task, along with one or more [`Continuation`]s that
/// decide when the current task should finish and what to do with the result.
pub struct Sequential<T, B>
where
    T: Task,
{
    id: Id,
    current: Either<T, Box<dyn Task<Value = B> + Send + Sync>>,
    continuations: Vec<Continuation<T::Value, B>>,
}

impl<T, B> Sequential<T, B>
where
    T: Task,
{
    pub fn on_value<T2>(
        mut self,
        trigger: Trigger,
        f: impl Fn(T::Value) -> T2 + Send + Sync + 'static,
    ) -> Self
    where
        T2: Task<Value = B> + Send + Sync + 'static,
    {
        let transform = Box::new(move |value| {
            Option::from(value).map(|x| {
                let result: DynTask<B> = Box::new(f(x));
                result
            })
        });
        self.continuations.push(Continuation { trigger, transform });
        self
    }

    pub fn on_stable<T2>(
        mut self,
        trigger: Trigger,
        f: impl Fn(T::Value) -> T2 + Send + Sync + 'static,
    ) -> Self
    where
        T2: Task<Value = B> + Send + Sync + 'static,
    {
        let transform = Box::new(move |value| match value {
            TaskValue::Stable(x) => {
                let result: DynTask<B> = Box::new(f(x));
                Some(result)
            }
            _ => None,
        });
        self.continuations.push(Continuation { trigger, transform });
        self
    }

    pub fn on_unstable<T2>(
        mut self,
        trigger: Trigger,
        f: impl Fn(T::Value) -> T2 + Send + Sync + 'static,
    ) -> Self
    where
        T2: Task<Value = B> + Send + Sync + 'static,
    {
        let transform = Box::new(move |value| match value {
            TaskValue::Unstable(x) => {
                let result: DynTask<B> = Box::new(f(x));
                Some(result)
            }
            _ => None,
        });
        self.continuations.push(Continuation { trigger, transform });
        self
    }

    pub fn if_value<T2>(
        mut self,
        trigger: Trigger,
        f: impl Fn(&T::Value) -> bool + Send + Sync + 'static,
        g: impl Fn(T::Value) -> T2 + Send + Sync + 'static,
    ) -> Self
    where
        T2: Task<Value = B> + Send + Sync + 'static,
    {
        let transform = Box::new(move |value| {
            Option::from(value).and_then(|x| {
                f(&x).then(|| {
                    let result: DynTask<B> = Box::new(g(x));
                    result
                })
            })
        });
        self.continuations.push(Continuation { trigger, transform });
        self
    }

    pub fn on(
        mut self,
        trigger: Trigger,
        f: impl Fn(TaskValue<T::Value>) -> Option<DynTask<B>> + Send + Sync + 'static,
    ) -> Self {
        self.continuations.push(Continuation {
            trigger,
            transform: Box::new(f),
        });
        self
    }
}

#[async_trait]
impl<T, B> Task for Sequential<T, B>
where
    T: Task + Send + Sync,
    T::Value: Clone + Send,
{
    type Value = B;

    async fn start(&mut self, gen: &mut Generator) -> Result<Html> {
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
                if let Trigger::Button(action) = &mut cont.trigger {
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

    async fn on_event(&mut self, event: Event, gen: &mut Generator) -> Result<Feedback> {
        match &mut self.current {
            Either::Left(task) => {
                let feedback = task.on_event(event.clone(), gen).await?;
                let value = task.value().await?;
                let next = self
                    .continuations
                    .iter()
                    .find_map(|cont| match &cont.trigger {
                        Trigger::Update => (cont.transform)(value.clone()),
                        Trigger::Button(action) => {
                            if let Event::Press { id } = &event {
                                if action.1 == *id {
                                    (cont.transform)(value.clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                    });

                match next {
                    None => Ok(feedback),
                    Some(mut next) => {
                        let html = next.start(gen).await?;

                        self.continuations.clear();
                        self.current = Either::Right(next);

                        Ok(Feedback::from(Change::ReplaceContent { id: self.id, html }))
                    }
                }
            }
            Either::Right(task) => task.on_event(event, gen).await,
        }
    }

    async fn value(&self) -> Result<TaskValue<Self::Value>> {
        match &self.current {
            Either::Left(_) => Ok(TaskValue::Empty),
            Either::Right(t) => t.value().await,
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

pub enum Trigger {
    /// Trigger as soon as possible.
    Update,
    /// Trigger when the user presses a button.
    Button(Button),
}

type DynTask<B> = Box<dyn Task<Value = B> + Send + Sync>;

type Transform<A, B> = Box<dyn Fn(TaskValue<A>) -> Option<DynTask<B>> + Send + Sync>;

/// Continuation of a [`Then`] task. Decides when the current task is consumed, using its value to
/// construct the next task.
struct Continuation<A, B> {
    trigger: Trigger,
    transform: Transform<A, B>,
}

/// Actions that are represented as buttons in the user interface, used in [`Continuation`]s. When
/// the user presses the associated button, and the associated predicate in the continuation is met,
/// the current task is consumed and the next task will be created from the resulting value.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Button(&'static str, Id);

impl Button {
    pub const OK: Self = Button::new("Ok");
    pub const CANCEL: Self = Button::new("Cancel");
    pub const YES: Self = Button::new("Yes");
    pub const NO: Self = Button::new("No");
    pub const NEXT: Self = Button::new("Next");
    pub const PREVIOUS: Self = Button::new("Previous");
    pub const FINISH: Self = Button::new("Finish");
    pub const CONTINUE: Self = Button::new("Continue");
    pub const NEW: Self = Button::new("New");
    pub const EDIT: Self = Button::new("Edit");
    pub const DELETE: Self = Button::new("Delete");
    pub const REFRESH: Self = Button::new("Refresh");
    pub const CLOSE: Self = Button::new("Close");

    pub const fn new(label: &'static str) -> Self {
        Button(label, Id::INVALID)
    }
}

impl ToHtml for Button {
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
