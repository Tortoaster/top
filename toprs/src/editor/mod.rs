//! This module contains functionality for the interaction between the user and the server.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::component::{Component, Context};
use crate::editor::event::{EditorError, Event, Response};

pub mod container;
pub mod event;
pub mod generic;
pub mod primitive;

/// Common [`Editor::Output`] type for editors.
pub type Report<T> = Result<Option<T>, EditorError>;

/// Editors describe how tasks should respond to user input, and how data can be retrieved from it.
pub trait Editor {
    /// The type of data this editor accepts. For example, a checkbox accepts a boolean to set or
    /// clear its state.
    type Input;
    /// The type of data this editor can produce, usually [`Report<Self::Input>`]. For example, a
    /// checkbox also produces a boolean value, but retrieving the value might fail.
    type Output;

    /// Create the initial user interface for this editor.
    fn start(&mut self, ctx: &mut Context) -> Component;

    /// React to interaction events from the user, such as when the user checks a checkbox or
    /// presses a button.
    fn respond_to(&mut self, event: Event) -> Option<Result<Response, EditorError>>;

    /// Consume the editor, retrieving its value.
    fn finish(self) -> Self::Output;
}

impl Display for EditorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorError::Format { id: s } => {
                write!(f, "{} is not the right format for this field", s)
            }
        }
    }
}

impl Error for EditorError {}
