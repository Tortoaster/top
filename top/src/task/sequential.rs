use std::collections::{BTreeMap, BTreeSet};

use async_trait::async_trait;
use futures::future::Either;
use uuid::Uuid;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::task::{Task, TaskValue, Value};

type Condition<A> = Box<dyn Fn(TaskValue<&A>) -> bool + Send + Sync>;
type Transform<A, B> = Box<dyn FnOnce(TaskValue<A>) -> DynTask<B> + Send + Sync>;

type DynTask<T> = Box<dyn Task<Output = T> + Send + Sync>;

pub struct Sequential<T, U>
where
    T: Value,
{
    container_id: Uuid,
    current: Either<T, DynTask<U>>,
    continuations: BTreeMap<Trigger, Continuation<T::Output, U>>,
}

struct Continuation<A, B> {
    condition: Condition<A>,
    transform: Option<Transform<A, B>>,
}

impl<T, U> Sequential<T, U>
where
    T: Value + Send + Sync,
    T::Output: Send + Sync,
    U: Send + Sync,
{
    async fn transform(&mut self, trigger: Trigger) -> Result<Feedback, TransformError> {
        match &self.current {
            Either::Left(task) => {
                let value = task.value().await;
                for continuation in self
                    .continuations
                    .iter_mut()
                    .filter(|(t, _)| **t == trigger)
                    .map(|(_, c)| c)
                {
                    if (continuation.condition)(value.as_ref()) {
                        let next = (continuation.transform.take().unwrap())(value);
                        let html = next.to_html().await;
                        self.current = Either::Right(next);
                        return Ok(Feedback::from(Change::ReplaceContent {
                            id: self.container_id,
                            html,
                        }));
                    }
                }
                Err(TransformError::FalseConditions)
            }
            Either::Right(_) => Err(TransformError::InvalidState),
        }
    }
}

impl<T, U> Sequential<T, U>
where
    T: Value,
{
    pub fn on<C, F, K>(mut self, trigger: Trigger, condition: C, transform: F) -> Self
    where
        C: Fn(TaskValue<&T::Output>) -> bool + Send + Sync + 'static,
        F: FnOnce(TaskValue<T::Output>) -> K + Send + Sync + 'static,
        K: Task<Output = U> + Sync + Send + 'static,
    {
        let continuation = Continuation {
            condition: Box::new(condition),
            transform: Some(Box::new(move |value| Box::new(transform(value)))),
        };
        self.continuations.insert(trigger, continuation);
        self
    }
}

#[async_trait]
impl<T, U> Value for Sequential<T, U>
where
    T: Value + Send + Sync,
    U: Send + Sync,
{
    type Output = U;

    async fn value(&self) -> TaskValue<Self::Output> {
        match &self.current {
            Either::Left(_) => TaskValue::Empty,
            Either::Right(task) => task.value().await,
        }
    }
}

#[async_trait]
impl<T, U> Handler for Sequential<T, U>
where
    T: Value + Handler + Send + Sync,
    T::Output: Send + Sync + 'static,
    U: Send + Sync + 'static,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match &mut self.current {
            Either::Left(task) => match event {
                Event::Press { id: button_id } => match self
                    .continuations
                    .keys()
                    .find(|trigger| {
                        if let Trigger::Button(Button { id, .. }) = trigger {
                            button_id == *id
                        } else {
                            false
                        }
                    })
                    .cloned()
                {
                    None => Feedback::new(),
                    Some(trigger) => self.transform(trigger).await.unwrap_or_default(),
                },
                _ => {
                    let feedback = task.on_event(event.clone()).await;
                    self.transform(Trigger::Update).await.unwrap_or(feedback)
                }
            },
            Either::Right(task) => task.on_event(event).await,
        }
    }
}

#[async_trait]
impl<T, U> Refresh for Sequential<T, U>
where
    T: Value + Refresh + Send + Sync,
    T::Output: Send + Sync + 'static,
    U: Send + Sync + 'static,
{
    async fn refresh(&mut self, ids: &BTreeSet<Uuid>) -> Feedback {
        match &mut self.current {
            Either::Left(task) => {
                let feedback = task.refresh(ids).await;
                self.transform(Trigger::Update).await.unwrap_or(feedback)
            }
            Either::Right(task) => task.refresh(ids).await,
        }
    }
}

#[async_trait]
impl<T, U> ToHtml for Sequential<T, U>
where
    T: Value + ToHtml + Send + Sync,
    U: Send + Sync,
{
    async fn to_html(&self) -> Html {
        match &self.current {
            Either::Left(task) => {
                let buttons: Html = self
                    .continuations
                    .keys()
                    .filter_map(|trigger| {
                        if let Trigger::Button(button) = trigger {
                            Some(button)
                        } else {
                            None
                        }
                    })
                    .map(|button| {
                        Html(format!(
                            r#"<button id="{}" class="button is-primary" type="button" onclick="press(this)">
                            {}
                        </button>"#,
                            button.id, button.text,
                        ))
                    })
                    .collect();

                Html(format!(
                    r#"<div id="{}">{}<div>{}</div></div>"#,
                    self.container_id,
                    task.to_html().await,
                    buttons
                ))
            }
            Either::Right(task) => task.to_html().await,
        }
    }
}

#[derive(Debug)]
enum TransformError {
    FalseConditions,
    InvalidState,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Trigger {
    Update,
    Button(Button),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Button {
    text: &'static str,
    id: Uuid,
}

impl Button {
    pub fn new(label: &'static str) -> Self {
        Button {
            text: label,
            id: Uuid::new_v4(),
        }
    }
}

pub trait TaskSequentialExt: Value + Sized {
    fn step<U>(self) -> Sequential<Self, U> {
        Sequential {
            container_id: Uuid::new_v4(),
            current: Either::Left(self),
            continuations: BTreeMap::new(),
        }
    }
}

impl<T> TaskSequentialExt for T where T: Value {}

pub fn if_stable<T>(value: TaskValue<&T>) -> bool {
    value.is_stable()
}

pub fn if_unstable<T>(value: TaskValue<&T>) -> bool {
    value.is_unstable()
}

pub fn if_empty<T>(value: TaskValue<&T>) -> bool {
    value.is_empty()
}

pub fn has_value<T>(value: TaskValue<&T>) -> bool {
    !value.is_empty()
}

pub fn always<T>(_: TaskValue<&T>) -> bool {
    true
}
