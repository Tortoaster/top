use async_trait::async_trait;
use thiserror::Error;

use crate::component::event::EventHandler;
use crate::component::Context;
use crate::task::value::TaskValue;

pub mod combinator;
pub mod interaction;
pub mod value;

#[async_trait]
pub trait Task: Send {
    type Value;

    async fn start<H: EventHandler + Send>(
        &mut self,
        executor: &mut Executor<H>,
    ) -> Result<(), TaskError<H::Error>>;

    async fn inspect<H: EventHandler + Send>(
        &mut self,
        executor: &mut Executor<H>,
    ) -> Result<TaskValue<Self::Value>, TaskError<H::Error>>;
}

#[derive(Debug, Error)]
pub enum TaskError<H: HandlerError> {
    #[error("event handler failure: {0}")]
    Handler(#[from] H),
    #[error("error during serialization")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to parse integer")]
    ParseInt(#[from] std::num::ParseIntError),
}

pub trait HandlerError {}

/// Represents the TopRs runtime.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Executor<E> {
    events: E,
    ctx: Context,
}

impl<E> Executor<E>
where
    E: EventHandler,
{
    pub fn new(handler: E) -> Self {
        Executor {
            events: handler,
            ctx: Context::new(),
        }
    }
}
