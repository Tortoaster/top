//! This module contains functionality for generating user interfaces for tasks.

use std::fmt::{Display, Formatter};

use serde_with::SerializeDisplay;

use crate::component::icon::Icon;
use crate::component::id::Id;

pub mod event;
mod html;
pub mod icon;
pub mod id;

/// Assigns a unique identifier to a [`Widget`], allowing the library to synchronize their values
/// with the server.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, SerializeDisplay)]
pub struct Component {
    id: Id,
    widget: Widget,
}

impl Component {
    /// Retrieve this component's unique identifier.
    pub fn id(&self) -> Id {
        self.id
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.html())
    }
}

/// Represents the visual aspect of tasks. In the context of webpages, these are usually translated
/// into (groups of) input elements.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Widget {
    TextField {
        value: String,
        label: Option<String>,
        disabled: bool,
    },
    NumberField {
        value: String,
        label: Option<String>,
        disabled: bool,
    },
    Checkbox {
        checked: bool,
        label: Option<String>,
        disabled: bool,
    },
    Button {
        text: String,
        disabled: bool,
    },
    IconButton {
        icon: Icon,
        disabled: bool,
    },
    Group {
        children: Vec<Component>,
        horizontal: bool,
    },
    RadioGroup {
        options: Vec<Component>,
    },

    Text(String),
}
