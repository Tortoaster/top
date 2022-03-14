//! This module contains functionality for generating user interfaces for tasks.

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde_with::{DeserializeFromStr, SerializeDisplay};

pub mod html;

/// Assigns a unique identifier to a [`Widget`], allowing the library to synchronize their values
/// with the server.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Component {
    id: ComponentId,
    widget: Widget,
}

impl Component {
    /// Retrieve this component's unique identifier.
    pub fn id(&self) -> ComponentId {
        self.id
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
        value: i32,
        label: Option<String>,
        disabled: bool,
    },
    Button {
        text: String,
        disabled: bool,
    },
    Group(Vec<Component>),
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
pub struct ComponentId(u32);

impl Display for ComponentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "top-{}", self.0)
    }
}

impl FromStr for ComponentId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: u32 = s[4..].parse()?;
        Ok(ComponentId(id))
    }
}

/// A context used to generate components with unique identifiers.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Context {
    current_id: ComponentId,
}

impl Context {
    /// Construct a new context for generating components with unique identifiers.
    pub fn new() -> Self {
        Context {
            current_id: ComponentId(0),
        }
    }

    /// Generate a new, uniquely-identifiable component.
    pub fn create_component(&mut self, widget: Widget) -> Component {
        self.current_id = ComponentId(self.current_id.0 + 1);
        Component {
            id: self.current_id,
            widget,
        }
    }
}
