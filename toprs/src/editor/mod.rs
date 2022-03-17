//! This module contains functionality for the interaction between the user and the server.

use thiserror::Error;

use crate::component::event::{Event, Feedback};
use crate::component::{Component, ComponentCreator};

pub mod generic;
pub mod primitive;

/// Editors describe how tasks should respond to user input, and how data can be retrieved from it.
pub trait Editor {
    /// The type of data this editor can read. For example, a checkbox can take a boolean value to
    /// represent its checked state.
    type Input;
    /// The type of data this editor can produce, usually [`Report<Self::Input>`] for interaction
    /// tasks. For example, a number field produces a number, but the value may not always be valid.
    type Output;

    /// Create the initial user interface for this editor.
    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component;

    /// React to interaction events from the user, such as when the user checks a checkbox or
    /// presses a button.
    fn on_event(
        &mut self,
        event: Event,
        ctx: &mut ComponentCreator,
    ) -> Option<(Self::Output, Feedback)>;
}

/// Common output type for [`Editor`]s.
pub type Report<T> = Result<T, EditorError>;

/// Common error type for [`Editor`]s.
#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum EditorError {
    #[error("failed to parse integer")]
    ParseInt(#[from] std::num::ParseIntError),
}
