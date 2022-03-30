use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::component::{Component, Widget};

// TODO: Allow identifying containing form, and disable any buttons while syncing or invalid
/// Unique component identifier.
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub struct Id(u32);

impl Id {
    /// Identity of the wrapper containing the entire application.
    pub const ROOT: Id = Id(0);
    pub const INVALID: Id = Id(u32::MAX);
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
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

impl Default for ComponentCreator {
    fn default() -> Self {
        ComponentCreator::new()
    }
}
