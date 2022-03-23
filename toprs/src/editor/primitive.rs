//! This module contains basic editors for primitive types.

use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use num_traits::PrimInt;

use crate::component::event::{Event, Feedback};
use crate::component::{ComponentCreator, Id, Widget};
use crate::editor::{Component, Editor, Report};

/// Basic editor for strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextEditor {
    id: Id,
    value: Report<String>,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            id: Id::default(),
            value: Ok(String::new()),
        }
    }
}

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
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } if id == self.id => {
                self.value = Ok(value);
                Some(Feedback::Valid { id })
            }
            _ => None,
        }
    }

    fn value(&self) -> &Self::Output {
        &self.value
    }

    fn finish(self) -> Self::Output {
        self.value
    }
}

/// Basic editor for numbers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberEditor<N> {
    id: Id,
    value: Report<N>,
}

impl<N> NumberEditor<N>
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

impl<N> Editor for NumberEditor<N>
where
    N: Default + Display + FromStr<Err = ParseIntError> + PrimInt,
{
    type Input = N;
    type Output = Report<N>;

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::NumberField {
            value: initial.unwrap_or_default().to_string(),
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

    fn value(&self) -> &Self::Output {
        &self.value
    }

    fn finish(self) -> Self::Output {
        self.value
    }
}

/// Basic editor for numbers.
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
    type Output = Report<bool>;

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::Checkbox {
            checked: initial.unwrap_or_default(),
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

    fn value(&self) -> &Self::Output {
        &self.value
    }

    fn finish(self) -> Self::Output {
        self.value
    }
}
