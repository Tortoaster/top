//! This module contains basic editors for primitive types.

use crate::component::{Context, Widget};
use crate::editor::event::{Event, Response};
use crate::editor::{Component, Editor, EditorError};

/// Basic editor for strings.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TextEditor(String);

impl TextEditor {
    /// Creates a new text editor.
    pub fn new() -> Self {
        TextEditor(String::new())
    }
}

impl Editor for TextEditor {
    type Input = String;
    type Output = String;

    fn start(&self, ctx: &mut Context) -> Component {
        let widget = Widget::TextField {
            value: String::new(),
            label: None,
            disabled: false,
        };
        ctx.create_component(widget)
    }

    fn respond_to(&mut self, event: Event) -> Result<Option<Response>, EditorError> {
        if let Event::Update { value, .. } = event {
            self.0 = value;
        }
        // TODO: Send feedback that the value is synced.
        Ok(None)
    }

    fn finish(self) -> Self::Output {
        self.0
    }
}

/// Basic editor for numbers.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NumberEditor(i32);

impl NumberEditor {
    /// Creates a new number editor.
    pub fn new() -> Self {
        NumberEditor(0)
    }
}

impl Editor for NumberEditor {
    type Input = i32;
    type Output = i32;

    fn start(&self, ctx: &mut Context) -> Component {
        let widget = Widget::NumberField {
            value: 0,
            label: None,
            disabled: false,
        };
        ctx.create_component(widget)
    }

    fn respond_to(&mut self, event: Event) -> Result<Option<Response>, EditorError> {
        if let Event::Update { value, .. } = event {
            self.0 = value.parse().map_err(|_| EditorError::Format(value))?;
        }
        // TODO: Send feedback that the value is synced.
        Ok(None)
    }

    fn finish(self) -> Self::Output {
        self.0
    }
}
