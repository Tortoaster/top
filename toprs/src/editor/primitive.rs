//! This module contains basic editors for primitive types.

use crate::component::event::{Event, Feedback};
use crate::component::{ComponentCreator, Id, Widget};
use crate::editor::{Component, Editor, Report};

/// Basic editor for strings.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TextEditor(Id);

impl Editor for TextEditor {
    type Input = String;
    type Output = Report<String>;

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::TextField {
            value: initial.unwrap_or_default(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.0 = component.id();
        component
    }

    fn on_event(
        &mut self,
        event: Event,
        _: &mut ComponentCreator,
    ) -> Option<(Self::Output, Feedback)> {
        match event {
            Event::Update { id, value } if id == self.0 => {
                Some((Ok(value), Feedback::ValueOk { id }))
            }
            _ => None,
        }
    }
}

/// Basic editor for numbers.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NumberEditor(Id);

impl Editor for NumberEditor {
    type Input = i32;
    type Output = Report<i32>;

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::NumberField {
            value: initial.unwrap_or_default(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.0 = component.id();
        component
    }

    fn on_event(
        &mut self,
        event: Event,
        _: &mut ComponentCreator,
    ) -> Option<(Self::Output, Feedback)> {
        match event {
            Event::Update { id, value } => {
                if id == self.0 {
                    match value.parse::<i32>() {
                        Ok(value) => Some((Ok(value), Feedback::ValueOk { id })),
                        Err(error) => Some((Err(error.into()), Feedback::ValueError { id })),
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
