use std::mem;

use async_trait::async_trait;
use thiserror::Error;

use crate::component::event::{Event, FeedbackHandler};
use crate::component::id::ComponentCreator;

pub mod inspect;
pub mod interact;
pub mod sequential;

#[async_trait]
pub trait Task: Send {
    type Value;

    async fn start<H: FeedbackHandler + Send>(
        &mut self,
        executor: &mut Context<H>,
    ) -> Result<(), Error<H::Error>>;

    async fn on_event<H: FeedbackHandler + Send>(
        &mut self,
        event: Event,
        executor: &mut Context<H>,
    ) -> Result<TaskValue<Self::Value>, Error<H::Error>>;

    async fn finish(self) -> TaskValue<Self::Value>;
}

#[derive(Debug, Error)]
pub enum Error<H: HandlerError> {
    #[error("event handler failure: {0}")]
    Handler(#[from] H),
    #[error("error during serialization")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to parse integer")]
    ParseInt(#[from] std::num::ParseIntError),
}

pub trait HandlerError {}

/// A context for [`Task`]s to interact with their environment.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Context<E> {
    feedback: E,
    components: ComponentCreator,
}

impl<E> Context<E>
where
    E: FeedbackHandler,
{
    pub fn new(handler: E) -> Self {
        Context {
            feedback: handler,
            components: ComponentCreator::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TaskValue<T> {
    Stable(T),
    Unstable(T),
    Empty,
}

impl<T> TaskValue<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            TaskValue::Stable(t) => Some(t),
            TaskValue::Unstable(t) => Some(t),
            TaskValue::Empty => None,
        }
    }

    pub const fn as_ref(&self) -> TaskValue<&T> {
        match *self {
            TaskValue::Stable(ref x) => TaskValue::Stable(x),
            TaskValue::Unstable(ref x) => TaskValue::Unstable(x),
            TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> TaskValue<U> {
        match self {
            TaskValue::Stable(x) => TaskValue::Stable(f(x)),
            TaskValue::Unstable(x) => TaskValue::Unstable(f(x)),
            TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn take(&mut self) -> Option<T> {
        mem::take(self).into_option()
    }
}

impl<T: Clone> TaskValue<&T> {
    pub fn cloned(self) -> TaskValue<T> {
        self.map(|t| t.clone())
    }
}

impl<T> Default for TaskValue<T> {
    fn default() -> Self {
        TaskValue::Empty
    }
}

pub trait OptionExt<T>: private::Sealed {
    fn into_stable(self) -> TaskValue<T>;

    fn into_unstable(self) -> TaskValue<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn into_stable(self) -> TaskValue<T> {
        match self {
            Some(t) => TaskValue::Stable(t),
            None => TaskValue::Empty,
        }
    }

    fn into_unstable(self) -> TaskValue<T> {
        match self {
            Some(t) => TaskValue::Unstable(t),
            None => TaskValue::Empty,
        }
    }
}

mod private {
    pub trait Sealed {}

    impl<T> Sealed for Option<T> {}
}
