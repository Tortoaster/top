use async_trait::async_trait;
use thiserror::Error;

use crate::html::event::handler::{FeedbackError, FeedbackHandler};
use crate::html::event::Event;
use crate::html::id::Generator;
use crate::html::Html;

pub mod inspect;
pub mod interact;
pub mod parallel;
pub mod sequential;
pub mod tune;

pub type TaskResult<T> = std::result::Result<TaskValue<T>, TaskError>;

// TODO: Merge methods into one executor (requires keeping event queue)
#[async_trait]
pub trait Task: Send {
    type Value;

    async fn start(&mut self, gen: &mut Generator) -> Result<Html, TaskError>;

    async fn on_event(&mut self, event: Event, ctx: &mut Context) -> TaskResult<Self::Value>;
}

/// A context for [`Task`]s to interact with their environment.
#[derive(Debug)]
pub struct Context {
    pub feedback: FeedbackHandler,
    pub gen: Generator,
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
    #[error("task is in invalid state")]
    State,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TaskValue<T> {
    /// The task's value is stable, meaning it cannot be changed by the user anymore.
    Stable(T),
    /// The task's value is unstable, meaning the user can still change it.
    Unstable(T),
    /// The task has no value yet.
    Empty,
}

impl<T> TaskValue<T> {
    pub fn and<U>(self, other: TaskValue<U>) -> TaskValue<(T, U)> {
        match self {
            TaskValue::Stable(a) => match other {
                TaskValue::Stable(b) => TaskValue::Stable((a, b)),
                TaskValue::Unstable(b) => TaskValue::Unstable((a, b)),
                TaskValue::Empty => TaskValue::Empty,
            },
            TaskValue::Unstable(a) => match other {
                TaskValue::Stable(b) | TaskValue::Unstable(b) => TaskValue::Unstable((a, b)),
                TaskValue::Empty => TaskValue::Empty,
            },
            TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn or(self, other: TaskValue<T>) -> TaskValue<T> {
        match self {
            TaskValue::Empty => other,
            _ => self,
        }
    }
}

impl<T> From<TaskValue<T>> for Option<T> {
    fn from(value: TaskValue<T>) -> Self {
        match value {
            TaskValue::Stable(x) | TaskValue::Unstable(x) => Some(x),
            TaskValue::Empty => None,
        }
    }
}

impl<T> Default for TaskValue<T> {
    fn default() -> Self {
        TaskValue::Empty
    }
}
