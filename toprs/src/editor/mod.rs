//! This module contains functionality for the interaction between the user and the server.

use crate::component::{Component, Context};
use crate::editor::event::{Event, Feedback};

pub mod container;
pub mod event;
pub mod generic;
pub mod primitive;

/// Editors describe how tasks should respond to user input, and how data can be retrieved from it.
pub trait Editor {
    /// The type of data this editor can produce. For example, a checkbox produces a boolean value.
    type Output;

    // TODO: Add optional initial value?
    /// Create the initial user interface for this editor.
    fn start(&mut self, ctx: &mut Context) -> Component;

    /// React to interaction events from the user, such as when the user checks a checkbox or
    /// presses a button.
    fn respond_to(&mut self, event: Event, ctx: &mut Context) -> Option<Feedback>;

    /// Consume the editor, retrieving its value.
    fn finish(self) -> Self::Output;
}
