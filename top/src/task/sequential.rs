use std::ops::Deref;

use async_trait::async_trait;
use either::Either;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, ToHtml};
use crate::share::{SharedRead, SharedValue};
use crate::task::{Task, TaskValue};

/// Basic sequential task. Consists of a current task, along with one or more [`Continuation`]s that
/// decide when the current task should finish and what to do with the result.
#[derive(Debug)]
pub struct Sequential<T1, T2, C, F> {
    id: Uuid,
    current: Either<T1, T2>,
    continuation: Continuation<C, F>,
}

#[async_trait]
impl<T1, T2, C, F> ToHtml for Sequential<T1, T2, C, F>
where
    T1: ToHtml + Send + Sync,
    T2: ToHtml + Send + Sync,
    C: Send + Sync,
    F: Send + Sync,
{
    async fn to_html(&self) -> Html {
        match &self.current {
            Either::Left(task) => {
                let task = task.to_html().await;

                let buttons = match &self.continuation.trigger {
                    Trigger::Button(button) => button.to_html().await,
                    _ => Html::default(),
                };

                let id = self.id;

                html! {r#"
                    <div id={id}>
                        {task}
                        {buttons}
                    </div>
                "#}
            }
            Either::Right(task) => task.to_html().await,
        }
    }
}

#[async_trait]
impl<T1, T2, C, F> Handler for Sequential<T1, T2, C, F>
where
    T1: Task + Handler + Send + Sync,
    T1::Value: Clone + Send,
    T1::Share: SharedRead<Value = <T1::Share as SharedValue>::Value> + Clone + Send + Sync,
    T2: ToHtml + Handler + Send + Sync,
    C: Fn(&TaskValue<<T1::Share as SharedValue>::Value>) -> bool + Send + Sync,
    F: Fn(TaskValue<<T1::Share as SharedValue>::Value>) -> T2 + Send + Sync,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match &mut self.current {
            Either::Left(task) => {
                let feedback = task.on_event(event.clone()).await;
                let share = task.share().await;

                match &self.continuation.trigger {
                    Trigger::Update => {
                        if (self.continuation.condition)(share.read().await.deref()) {
                            let next = (self.continuation.transform)(share.clone_value().await);
                            let html = next.to_html().await;
                            self.current = Either::Right(next);
                            Feedback::from(Change::ReplaceContent { id: self.id, html })
                        } else {
                            feedback
                        }
                    }
                    Trigger::Button(action) => {
                        if let Event::Press { id } = &event {
                            if action.1 == *id {
                                if (self.continuation.condition)(share.read().await.deref()) {
                                    let next =
                                        (self.continuation.transform)(share.clone_value().await);
                                    let html = next.to_html().await;
                                    self.current = Either::Right(next);
                                    Feedback::from(Change::ReplaceContent { id: self.id, html })
                                } else {
                                    feedback
                                }
                            } else {
                                feedback
                            }
                        } else {
                            feedback
                        }
                    }
                }
            }
            Either::Right(task) => task.on_event(event).await,
        }
    }
}

#[async_trait]
impl<T1, T2, C, F> Task for Sequential<T1, T2, C, F>
where
    T1: Task + Send + Sync,
    T1::Value: Clone + Send,
    T1::Share: SharedRead<Value = <T1::Share as SharedValue>::Value> + Clone + Send + Sync,
    T2: Task + ToHtml + Send + Sync,
    C: Fn(&TaskValue<<T1::Share as SharedValue>::Value>) -> bool + Send + Sync,
    F: Fn(TaskValue<<T1::Share as SharedValue>::Value>) -> T2 + Send + Sync,
{
    type Value = T2::Value;
    type Share = ();

    async fn share(&self) -> Self::Share {
        ()
    }

    async fn value(self) -> TaskValue<Self::Value> {
        match self.current {
            Either::Left(_) => TaskValue::Empty,
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
pub struct Button(&'static str, Uuid);

impl Button {
    pub fn new(label: &'static str) -> Self {
        Button(label, Uuid::new_v4())
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
            id: Uuid::new_v4(),
            current: Either::Left(self),
            continuation,
        }
    }
}

impl<T> TaskSequentialExt for T where T: Task {}
