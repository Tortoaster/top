use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde_with::{DeserializeFromStr, SerializeDisplay};

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
        let id: u32 = s[4.min(s.len())..].parse()?;
        Ok(Id(id))
    }
}

/// A creator used to generate components with unique identifiers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Generator(Id);

impl Generator {
    pub fn new() -> Self {
        Generator(Id(0))
    }

    pub fn next(&mut self) -> Id {
        self.0 = Id(self.0 .0 + 1);
        self.0
    }
}

impl Default for Generator {
    fn default() -> Self {
        Generator::new()
    }
}
