//! This module contains basic editors for primitive types.

use crate::component::event::{Event, Feedback};
use crate::component::{Context, Id, Widget};
use crate::editor::{Component, Editor, Report};

/// Basic editor for strings.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TextEditor(Id, String);

impl TextEditor {
    /// Creates a new text editor.
    pub fn new() -> Self {
        Self::with_value(Default::default())
    }

    /// Creates a new text editor with a default value.
    pub fn with_value(value: String) -> Self {
        TextEditor(Id::default(), value)
    }
}

impl Editor for TextEditor {
    type Input = String;
    type Output = Report<String>;

    fn start(&mut self, ctx: &mut Context) -> Component {
        let widget = Widget::TextField {
            value: self.1.clone(),
            label: None,
            disabled: false,
        };
        let component = ctx.create_component(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.0 = component.id();
        component
    }

    fn respond_to(&mut self, event: Event, _: &mut Context) -> Option<Feedback> {
        match event {
            Event::Update { id, value } => {
                if id == self.0 {
                    self.1 = value;
                    Some(Feedback::ValueOk { id })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn finish(self) -> Self::Output {
        Ok(self.1)
    }
}

/// Basic editor for numbers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberEditor(Id, Report<i32>);

impl NumberEditor {
    /// Creates a new number editor.
    pub fn new() -> Self {
        Self::with_value(Default::default())
    }

    /// Creates a new number editor with a default value.
    pub fn with_value(value: i32) -> Self {
        NumberEditor(Id::default(), Ok(value))
    }
}

impl Editor for NumberEditor {
    type Input = i32;
    type Output = Report<i32>;

    fn start(&mut self, ctx: &mut Context) -> Component {
        let widget = Widget::NumberField {
            value: *self.1.as_ref().unwrap_or(&0),
            label: None,
            disabled: false,
        };
        let component = ctx.create_component(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.0 = component.id();
        component
    }

    fn respond_to(&mut self, event: Event, _: &mut Context) -> Option<Feedback> {
        match event {
            Event::Update { id, value } => {
                if id == self.0 {
                    match value.parse() {
                        Ok(value) => {
                            self.1 = Ok(value);
                            Some(Feedback::ValueOk { id })
                        }
                        Err(error) => {
                            self.1 = Err(error.into());
                            Some(Feedback::ValueError { id })
                        }
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
