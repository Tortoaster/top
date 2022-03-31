use async_trait::async_trait;
use thiserror::Error;

use crate::component::event::{Event, FeedbackError, FeedbackHandler};
use crate::component::id::ComponentCreator;

pub mod inspect;
pub mod interact;
pub mod sequential;

pub type TaskResult<T> = std::result::Result<TaskValue<T>, TaskError>;

// TODO: Merge methods into one executor (requires keeping event queue)
#[async_trait]
pub trait Task: Send {
    type Value;

    async fn start<H>(&mut self, ctx: &mut Context<H>) -> Result<(), TaskError>
    where
        H: FeedbackHandler + Send;

    async fn on_event<H>(&mut self, event: Event, ctx: &mut Context<H>) -> TaskResult<Self::Value>
    where
        H: FeedbackHandler + Send;
}

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

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("event handler failure: {0}")]
    Handler(#[from] FeedbackError),
    #[error("error during serialization: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to parse integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
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
