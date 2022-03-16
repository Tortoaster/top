use async_trait::async_trait;
use thiserror::Error;

use crate::component::event::EventHandler;
use crate::component::Context;
use crate::task::value::TaskValue;

pub mod combinator;
pub mod interaction;
pub mod value;

#[async_trait]
pub trait Task {
    type Value;

    async fn execute(
        self,
        executor: &mut Executor<impl EventHandler + Send>,
    ) -> Result<TaskValue<Self::Value>, TaskError>;
}

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("error during serialization")]
    Serialize(#[from] serde_json::Error),
}

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
