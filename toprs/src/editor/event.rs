use std::error::Error;
use std::fmt::{Display, Formatter};

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

/// Responses to [`Event`]s, such as replacing certain parts of the interface after the user presses
/// a button.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Response {
    /// Replace the entire UI with the given [`Component`].
    Replace {
        id: ComponentId,
        content: Html,
    },
    ValueOk {
        id: ComponentId,
    },
}

/// Error type for editors, indicating that an interaction event was invalid.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EditorError {
    Format { id: ComponentId },
    Incomplete,
}

impl Display for EditorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorError::Format { id: s } => {
                write!(f, "{} is not the right format for this field", s)
            }
            EditorError::Incomplete => write!(f, "Incomplete data"),
        }
    }
}

impl Error for EditorError {}
