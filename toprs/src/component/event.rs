use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::component::{Component, Id};
use crate::task::{Error, HandlerError};

#[async_trait]
pub trait FeedbackHandler {
    type Error: HandlerError;

    async fn send(&mut self, feedback: Feedback) -> Result<(), Error<Self::Error>>;
}

/// Interaction event from the user, such as checking a checkbox or pressing a button.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    Update { id: Id, value: String },
    Press { id: Id },
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
    /// The value of this component is valid.
    Valid { id: Id },
    /// The value of this component is invalid.
    Invalid { id: Id },
}
