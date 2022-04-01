//! This module contains functionality for generating user interfaces for tasks.

use std::fmt::{Display, Formatter};

use serde_with::SerializeDisplay;

use crate::component::icon::Icon;
use crate::id::Id;

mod html;
pub mod icon;

/// Assigns a unique identifier to a [`Widget`], allowing the library to synchronize their values
/// with the server.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, SerializeDisplay)]
pub struct Component {
    id: Id,
    widget: Widget,
    attrs: Attributes,
}

impl Component {
    pub fn new(id: Id, widget: Widget) -> Self {
        Component {
            id,
            widget,
            attrs: Attributes::default(),
        }
    }

    /// Retrieve this component's unique identifier.
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn tune(self) -> Tuner {
        Tuner(self)
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.html().unwrap_or("render error".to_string()))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Attributes {
    label: Option<String>,
    disabled: bool,
    horizontal: bool,
}

/// Represents the visual aspect of tasks. In the context of webpages, these are usually translated
/// into (groups of) input elements.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Widget {
    TextField(String),
    NumberField(String),
    Checkbox(bool),
    Button(String),
    IconButton(Icon),
    Group(Vec<Component>),
    RadioGroup(Vec<Component>),

    Text(String),
}

pub struct Tuner(Component);

impl Tuner {
    pub fn add_label(mut self, label: String) -> Self {
        self.0.attrs.label = Some(label);
        self
    }

    pub fn set_direction(mut self, horizontal: bool) -> Self {
        self.0.attrs.horizontal = horizontal;
        self
    }

    pub fn finish(self) -> Component {
        self.0
    }
}
