//! This module contains basic editors for primitive types.

use std::fmt::Display;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use crate::component::event::{Event, Feedback};
use crate::component::{ComponentCreator, Id, Widget};
use crate::editor::{Component, Editor, EditorError, Report};

/// Basic editor for strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextEditor {
    id: Id,
    value: String,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            id: Id::default(),
            value: String::new(),
        }
    }
}

impl Editor for TextEditor {
    type Input = String;
    type Output = String;

    fn component(&mut self, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::TextField {
            value: self.value.clone(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } if id == self.id => {
                self.value = value;
                Some(Feedback::Valid { id })
            }
            _ => None,
        }
    }

    fn read(&self) -> Report<Self::Output> {
        Ok(self.value.clone())
    }

    fn write(&mut self, value: Self::Input) {
        self.value = value;
    }
}

/// Basic editor for integers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerEditor<N> {
    id: Id,
    value: Report<N>,
}

impl<N> IntegerEditor<N>
where
    N: Default,
{
    pub fn new() -> Self {
        Self {
            id: Id::default(),
            value: Ok(N::default()),
        }
    }
}

impl<N> Editor for IntegerEditor<N>
where
    N: Copy + Display + FromStr<Err = ParseIntError>,
{
    type Input = N;
    type Output = N;

    fn component(&mut self, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::NumberField {
            value: self
                .value
                .as_ref()
                .map(|value| value.to_string())
                .unwrap_or_default(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } => {
                if id == self.id {
                    match value.parse::<N>() {
                        Ok(value) => {
                            self.value = Ok(value);
                            Some(Feedback::Valid { id })
                        }
                        Err(error) => {
                            self.value = Err(error.into());
                            Some(Feedback::Invalid { id })
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn read(&self) -> Report<Self::Output> {
        self.value.clone()
    }

    fn write(&mut self, value: Self::Input) {
        self.value = Ok(value);
    }
}

/// Basic editor for floats.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloatEditor<N> {
    id: Id,
    value: Report<N>,
}

impl<N> FloatEditor<N>
where
    N: Default,
{
    pub fn new() -> Self {
        Self {
            id: Id::default(),
            value: Ok(N::default()),
        }
    }
}

impl<N> Editor for FloatEditor<N>
where
    N: Copy + Display + FromStr<Err = ParseFloatError>,
{
    type Input = N;
    type Output = N;

    fn component(&mut self, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::NumberField {
            value: self
                .value
                .as_ref()
                .map(|value| value.to_string())
                .unwrap_or_default(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } => {
                if id == self.id {
                    match value.parse::<N>() {
                        Ok(value) => {
                            self.value = Ok(value);
                            Some(Feedback::Valid { id })
                        }
                        Err(error) => {
                            self.value = Err(error.into());
                            Some(Feedback::Invalid { id })
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn read(&self) -> Report<Self::Output> {
        self.value.clone()
    }

    fn write(&mut self, value: Self::Input) {
        self.value = Ok(value);
    }
}

/// Basic editor for booleans.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BooleanEditor {
    id: Id,
    value: Report<bool>,
}

impl BooleanEditor {
    pub fn new() -> Self {
        Self {
            id: Id::default(),
            value: Ok(false),
        }
    }
}

impl Editor for BooleanEditor {
    type Input = bool;
    type Output = bool;

    fn component(&mut self, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::Checkbox {
            checked: *self.value.as_ref().unwrap_or(&false),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } => {
                if id == self.id {
                    match value.parse() {
                        Ok(value) => {
                            self.value = Ok(value);
                            Some(Feedback::Valid { id })
                        }
                        Err(error) => {
                            self.value = Err(error.into());
                            Some(Feedback::Invalid { id })
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn read(&self) -> Report<Self::Output> {
        self.value.clone()
    }

    fn write(&mut self, value: Self::Input) {
        self.value = Ok(value);
    }
}

/// Basic editor for characters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharEditor {
    id: Id,
    value: Report<char>,
}

impl CharEditor {
    pub fn new() -> Self {
        Self {
            id: Id::default(),
            value: Err(EditorError::Empty),
        }
    }
}

impl Editor for CharEditor {
    type Input = char;
    type Output = char;

    fn component(&mut self, ctx: &mut ComponentCreator) -> Component {
        // TODO: Limit length to 1
        let widget = Widget::TextField {
            value: self
                .value
                .as_ref()
                .map(|value| value.to_string())
                .unwrap_or_default(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } => {
                if id == self.id {
                    match value.parse() {
                        Ok(value) => {
                            self.value = Ok(value);
                            Some(Feedback::Valid { id })
                        }
                        Err(error) => {
                            self.value = Err(error.into());
                            Some(Feedback::Invalid { id })
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn read(&self) -> Report<Self::Output> {
        self.value.clone()
    }

    fn write(&mut self, value: Self::Input) {
        self.value = Ok(value)
    }
}
