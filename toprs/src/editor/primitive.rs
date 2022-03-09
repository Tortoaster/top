//! This module contains basic editors for primitive types.

use crate::component::{ComponentId, Context, Widget};
use crate::editor::event::{Event, Response};
use crate::editor::{Component, Editor, EditorError};

/// Basic editor for strings.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TextEditor(ComponentId, String);

impl TextEditor {
    /// Creates a new text editor.
    pub fn new() -> Self {
        TextEditor(ComponentId::default(), String::new())
    }
}

impl Editor for TextEditor {
    type Input = String;
    type Output = String;

    fn start(&mut self, ctx: &mut Context) -> Component {
        let widget = Widget::TextField {
            value: String::new(),
            label: None,
            disabled: false,
        };
        let component = ctx.create_component(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.0 = component.id();
        component
    }

    fn respond_to(&mut self, event: Event) -> Option<Result<Response, EditorError>> {
        match event {
            Event::Update { id, value } => {
                if id == self.0 {
                    self.1 = value;
                    Some(Ok(Response::ValueOk { id: id }))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn finish(self) -> Self::Output {
        self.1
    }
}

/// Basic editor for numbers.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NumberEditor(ComponentId, i32);

impl NumberEditor {
    /// Creates a new number editor.
    pub fn new() -> Self {
        NumberEditor(ComponentId::default(), 0)
    }
}

impl Editor for NumberEditor {
    type Input = i32;
    type Output = i32;

    fn start(&mut self, ctx: &mut Context) -> Component {
        let widget = Widget::NumberField {
            value: 0,
            label: None,
            disabled: false,
        };
        let component = ctx.create_component(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.0 = component.id();
        component
    }

    fn respond_to(&mut self, event: Event) -> Option<Result<Response, EditorError>> {
        match event {
            Event::Update { id, value } => {
                if id == self.0 {
                    match value.parse() {
                        Ok(value) => {
                            self.1 = value;
                            Some(Ok(Response::ValueOk { id: id }))
                        }
                        Err(_) => Some(Err(EditorError::Format { id: id })),
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn finish(self) -> Self::Output {
        self.1
    }
}
