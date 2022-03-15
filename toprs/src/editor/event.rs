use serde::{Deserialize, Serialize};

use crate::component::ComponentId;

pub type Html = String;

/// Interaction event from the user, such as checking a checkbox or pressing a button.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    Update { id: ComponentId, value: String },
    Press { id: ComponentId },
}

/// Changes to the user interface in response to [`Event`]s, such as confirming a value is valid, or
/// replacing the content after the user presses a button.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Feedback {
    /// Replace the entire UI with the given [`Component`].
    Replace {
        id: ComponentId,
        content: Html,
    },
    ValueOk {
        id: ComponentId,
    },
    ValueError {
        id: ComponentId,
    },
}
