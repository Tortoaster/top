use async_trait::async_trait;
use either::Either;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::id::{Generator, Id};
use crate::html::{Html, ToHtml};
use crate::task::{Result, Task, TaskError, TaskValue};

/// Basic sequential task. Consists of a current task, along with one or more [`Continuation`]s that
/// decide when the current task should finish and what to do with the result.
#[derive(Debug)]
pub struct Sequential<T1, T2, C, F> {
    id: Id,
    current: Either<T1, T2>,
    continuation: Continuation<C, F>,
}

#[async_trait]
impl<T1, T2, C, F> Task for Sequential<T1, T2, C, F>
where
    T1: Task + Send + Sync,
    T1::Value: Clone + Send,
    T2: Task + Send + Sync,
    C: Fn(&TaskValue<T1::Value>) -> bool + Send + Sync,
    F: Fn(TaskValue<T1::Value>) -> T2 + Send + Sync,
{
    type Value = T2::Value;

    async fn start(&mut self, gen: &mut Generator) -> Result<Html> {
        self.id = gen.next();

        let task = self
            .current
            .as_mut()
            .left()
            .ok_or(TaskError::State)?
            .start(gen)
            .await?;

        let buttons = if let Trigger::Button(action) = &mut self.continuation.trigger {
            action.1 = gen.next();
            action.to_html().await
        } else {
            Html::default()
        };

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

                match &self.continuation.trigger {
                    Trigger::Update => {
                        if (self.continuation.condition)(&value) {
                            let mut next = (self.continuation.transform)(value);
                            let html = next.start(gen).await?;
                            self.current = Either::Right(next);
                            Ok(Feedback::from(Change::ReplaceContent { id: self.id, html }))
                        } else {
                            Ok(feedback)
                        }
                    }
                    Trigger::Button(action) => {
                        if let Event::Press { id } = &event {
                            if action.1 == *id {
                                if (self.continuation.condition)(&value) {
                                    let mut next = (self.continuation.transform)(value);
                                    let html = next.start(gen).await?;
                                    self.current = Either::Right(next);
                                    Ok(Feedback::from(Change::ReplaceContent { id: self.id, html }))
                                } else {
                                    Ok(feedback)
                                }
                            } else {
                                Ok(feedback)
                            }
                        } else {
                            Ok(feedback)
                        }
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

#[derive(Debug)]
pub enum Trigger {
    /// Trigger as soon as possible.
    Update,
    /// Trigger when the user presses a button.
    Button(Button),
}

/// Continuation of a [`Then`] task. Decides when the current task is consumed, using its value to
/// construct the next task.
#[derive(Debug)]
struct Continuation<C, F> {
    trigger: Trigger,
    condition: C,
    transform: F,
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

#[async_trait]
impl ToHtml for Button {
    async fn to_html(&self) -> Html {
        html! {r#"
            <button id="{self.1}" class="button is-link" type="button" onclick="press(this)">
                {self.0}
            </button>
        "#}
    }
}

/// Adds the [`steps`] method to any task, allowing it to become a sequential task through the
/// [`Steps`] builder.
pub trait TaskSequentialExt: Task + Sized {
    fn then<T2, C, F>(
        self,
        trigger: Trigger,
        condition: C,
        transform: F,
    ) -> Sequential<Self, T2, C, F>
    where
        C: Fn(&TaskValue<Self::Value>) -> bool,
        F: Fn(TaskValue<Self::Value>) -> T2,
    {
        let continuation = Continuation {
            trigger,
            condition,
            transform,
        };

        Sequential {
            id: Id::INVALID,
            current: Either::Left(self),
            continuation,
        }
    }
}

impl<T> TaskSequentialExt for T where T: Task {}
