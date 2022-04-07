//! This module contains functionality for the interaction between the user and the server.

use thiserror::Error;

use crate::event::{Event, Feedback};
use crate::html::AsHtml;
use crate::id::Generator;

pub mod choice;
pub mod container;
pub mod convert;
pub mod generic;
pub mod primitive;
pub mod tuple;

/// Editors describe how tasks should respond to user input, and how data can be retrieved from it.
pub trait Editor: AsHtml {
    /// The type of data this editor can produce.
    type Output;

    // TODO: Turn into constructor?
    fn start(&mut self, gen: &mut Generator);

    /// React to interaction events from the user, such as when the user checks a checkbox or
    /// presses a button.
    fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback>;

    // TODO: Allow borrow and consume
    /// Get the current value of this editor.
    fn finish(&self) -> Result<Self::Output, EditorError>;
}

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
