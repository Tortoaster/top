use async_trait::async_trait;
use thiserror::Error;

use crate::component::event::{Event, FeedbackHandler};
use crate::component::ComponentCreator;
use crate::task::value::TaskValue;

pub mod combinator;
pub mod interaction;
pub mod value;

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
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
