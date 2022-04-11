use async_trait::async_trait;
use thiserror::Error;

use crate::event::handler::{FeedbackError, FeedbackHandler};
use crate::event::Event;
use crate::id::Generator;

pub mod inspect;
pub mod interact;
pub mod parallel;
pub mod sequential;

pub type TaskResult<T> = std::result::Result<TaskValue<T>, TaskError>;

// TODO: Merge methods into one executor (requires keeping event queue)
#[async_trait]
pub trait Task: Send {
    type Value;

    async fn start(&mut self, ctx: &mut Context) -> Result<(), TaskError>;

    async fn on_event(&mut self, event: Event, ctx: &mut Context) -> TaskResult<Self::Value>;
}

/// A context for [`Task`]s to interact with their environment.
#[derive(Debug)]
pub struct Context {
    feedback: FeedbackHandler,
    gen: Generator,
}

impl Context {
    pub fn new(feedback: FeedbackHandler) -> Self {
        Context {
            feedback,
            gen: Generator::new(),
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

pub trait OptionExt<T> {
    fn into_stable(self) -> TaskValue<T>;

    fn into_unstable(self) -> TaskValue<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn into_stable(self) -> TaskValue<T> {
        match self {
            None => TaskValue::Empty,
            Some(value) => TaskValue::Stable(value),
        }
    }

    fn into_unstable(self) -> TaskValue<T> {
        match self {
            None => TaskValue::Empty,
            Some(value) => TaskValue::Unstable(value),
        }
    }
}
