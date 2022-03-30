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
}

impl<T> Default for TaskValue<T> {
    fn default() -> Self {
        TaskValue::Empty
    }
}
