use async_trait::async_trait;
use thiserror::Error;

use crate::html::event::{Event, Feedback};
use crate::html::id::Generator;
use crate::html::Html;

pub mod inspect;
pub mod interact;
pub mod parallel;
pub mod sequential;
pub mod tune;

pub type Result<T> = std::result::Result<T, TaskError>;

#[async_trait]
pub trait Task {
    type Value;

    async fn start(&mut self, gen: &mut Generator) -> Result<Html>;

    async fn on_event(&mut self, event: Event, gen: &mut Generator) -> Result<Feedback>;

    async fn value(&self) -> Result<TaskValue<Self::Value>>;
}

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("error during serialization: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to parse integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("task is in invalid state")]
    State,
    #[error("inconsistent feedback")]
    Feedback,
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
