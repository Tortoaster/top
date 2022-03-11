//! This module contains functionality for the interaction between the user and the server.

use crate::component::{Component, Context};
use crate::editor::event::{EditorError, Event, Response};

pub mod container;
pub mod event;
pub mod generic;
pub mod primitive;

/// Common [`Editor::Output`] type for editors.
pub type Report<T> = Result<T, EditorError>;

/// Editors describe how tasks should respond to user input, and how data can be retrieved from it.
pub trait Editor {
    /// The type of data this editor can produce, usually [`Report<Self::Input>`]. For example, a
    /// checkbox also produces a boolean value, but retrieving the value might fail.
    type Output;

    /// Create the initial user interface for this editor.
    fn start(&mut self, ctx: &mut Context) -> Component;

    /// React to interaction events from the user, such as when the user checks a checkbox or
    /// presses a button.
    fn respond_to(
        &mut self,
        event: Event,
        ctx: &mut Context,
    ) -> Option<Result<Response, EditorError>>;

    /// Consume the editor, retrieving its value.
    fn finish(self) -> Self::Output;
}
