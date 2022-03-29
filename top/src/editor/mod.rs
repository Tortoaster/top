//! This module contains functionality for the interaction between the user and the server.

use thiserror::Error;

use crate::component::event::{Event, Feedback};
use crate::component::{Component, ComponentCreator};

pub mod container;
pub mod from;
pub mod generic;
pub mod primitive;
pub mod tuple;

/// Editors describe how tasks should respond to user input, and how data can be retrieved from it.
pub trait Editor {
    /// The type of data this editor can read. For example, a checkbox can take a boolean value to
    /// represent its checked state.
    type Input;
    /// The type of data this editor can produce, usually [`Self::Input`] for interaction tasks. For
    /// example, a number field produces a number, but the value may not always be valid.
    type Output;

    /// Create the initial user interface for this editor.
    fn component(&mut self, ctx: &mut ComponentCreator) -> Component;

    /// React to interaction events from the user, such as when the user checks a checkbox or
    /// presses a button.
    fn on_event(&mut self, event: Event, ctx: &mut ComponentCreator) -> Option<Feedback>;

    // TODO: Allow borrow and consume
    /// Get the current value of this editor.
    fn read(&self) -> Report<Self::Output>;

    /// Change the value of this editor directly.
    fn write(&mut self, value: Self::Input);
}

/// Common output type for [`Editor`]s.
pub type Report<T> = Result<T, EditorError>;

/// Common error type for [`Editor`]s.
#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum EditorError {
    #[error("failed to parse integer")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("failed to parse float")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("failed to parse boolean")]
    ParseBool(#[from] std::str::ParseBoolError),
    #[error("failed to parse character")]
    ParseChar(#[from] std::char::ParseCharError),
    #[error("something is wrong with the value")]
    Invalid,
}
