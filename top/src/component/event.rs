use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::component::id::Id;
use crate::component::Component;

/// Interaction event from the user, such as checking a checkbox or pressing a button.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    Update { id: Id, value: String },
    Press { id: Id },
}

#[async_trait]
pub trait EventHandler {
    async fn receive(&mut self) -> Option<Result<Event, EventError>>;
}

#[derive(Debug, Error)]
pub enum EventError {
    #[error("error during deserialization: {0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("failed to receive event")]
    Receive,
}

/// Changes to the user interface in response to [`Event`]s, such as confirming a value is valid, or
/// replacing the content after the user presses a button.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Feedback {
    /// Replace this component with a new component.
    Replace { id: Id, component: Component },
    /// Add a component to this component.
    Append { id: Id, component: Component },
    /// Remove this component.
    Remove { id: Id },
    /// The value of this component is valid.
    Valid { id: Id },
    /// The value of this component is invalid.
    Invalid { id: Id },
}

#[async_trait]
pub trait FeedbackHandler {
    async fn send(&mut self, feedback: Feedback) -> Result<(), FeedbackError>;
}

#[derive(Debug, Error)]
pub enum FeedbackError {
    #[error("error during serialization: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to send feedback")]
    Send,
}
