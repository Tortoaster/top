//! This module contains functionality for generating user interfaces for tasks.

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

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
    Row(Vec<Component>),
    Column(Vec<Component>),
}

/// Unique component identifier.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentId(u32);

impl Display for ComponentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "top-{}", self.0)
    }
}

/// A context used to generate components with unique identifiers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
        let id = self.generate_id();
        Component { id, widget }
    }

    /// Retrieve the last generated identifier.
    pub fn current_id(&self) -> ComponentId {
        self.current_id
    }

    fn generate_id(&mut self) -> ComponentId {
        self.current_id = ComponentId(self.current_id.0 + 1);
        self.current_id
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
