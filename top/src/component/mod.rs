//! This module contains functionality for generating user interfaces for tasks.

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::component::icon::Icon;

pub mod event;
pub mod html;
pub mod icon;

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
}

// TODO: Allow identifying containing form, and disable any buttons while syncing or invalid
/// Unique component identifier.
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
)]
pub struct Id(u32);

impl Id {
    /// Identity of the wrapper containing the entire application.
    pub const ROOT: Id = Id(0);
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "top-{}", self.0)
    }
}

impl FromStr for Id {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: u32 = s[4..].parse()?;
        Ok(Id(id))
    }
}

/// A creator used to generate components with unique identifiers.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ComponentCreator {
    current_id: Id,
}

impl ComponentCreator {
    /// Construct a new context for generating components with unique identifiers.
    pub fn new() -> Self {
        ComponentCreator { current_id: Id(0) }
    }

    /// Generate a new, uniquely-identifiable component.
    pub fn create(&mut self, widget: Widget) -> Component {
        self.current_id = Id(self.current_id.0 + 1);
        Component {
            id: self.current_id,
            widget,
        }
    }
}
