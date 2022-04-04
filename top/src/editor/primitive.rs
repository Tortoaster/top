//! This module contains basic editors for primitive types.

use std::fmt::Display;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use crate::component::Widget;
use crate::editor::{Component, Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::id::{Generator, Id};

/// Basic editor for strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextEditor {
    id: Id,
    value: String,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            id: Id::INVALID,
            value: String::new(),
        }
    }
}

impl Editor for TextEditor {
    type Input = String;
    type Output = String;

    fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
        self.id = gen.next();
        if let Some(value) = value {
            self.value = value;
        }
    }

    fn component(&self) -> Component {
        let widget = Widget::TextField(self.value.clone());

        Component::new(self.id, widget)
    }

    fn on_event(&mut self, event: Event, _gen: &mut Generator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } if id == self.id => {
                self.value = value;
                Some(Feedback::Valid { id })
            }
            _ => None,
        }
    }

    fn read(&self) -> Result<Self::Output, EditorError> {
        Ok(self.value.clone())
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        TextEditor::new()
    }
}

/// Basic editor for integers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerEditor<N> {
    id: Id,
    value: Result<N, EditorError>,
}

impl<N> IntegerEditor<N>
where
    N: Default,
{
    pub fn new() -> Self {
        IntegerEditor {
            id: Id::INVALID,
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

    fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
        self.id = gen.next();
        if let Some(value) = value {
            self.value = Ok(value);
        }
    }

    fn component(&self) -> Component {
        let widget = Widget::NumberField(
            self.value
                .as_ref()
                .map(|value| value.to_string())
                .unwrap_or_default(),
        );

        Component::new(self.id, widget)
    }

    fn on_event(&mut self, event: Event, _gen: &mut Generator) -> Option<Feedback> {
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

    fn read(&self) -> Result<Self::Output, EditorError> {
        self.value.clone()
    }
}

impl<N> Default for IntegerEditor<N>
where
    N: Default,
{
    fn default() -> Self {
        IntegerEditor::new()
    }
}

/// Basic editor for floats.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloatEditor<N> {
    id: Id,
    value: Result<N, EditorError>,
}

impl<N> FloatEditor<N>
where
    N: Default,
{
    pub fn new() -> Self {
        FloatEditor {
            id: Id::INVALID,
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

    fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
        self.id = gen.next();
        if let Some(value) = value {
            self.value = Ok(value);
        }
    }

    fn component(&self) -> Component {
        let widget = Widget::NumberField(
            self.value
                .as_ref()
                .map(|value| value.to_string())
                .unwrap_or_default(),
        );

        Component::new(self.id, widget)
    }

    fn on_event(&mut self, event: Event, _gen: &mut Generator) -> Option<Feedback> {
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

    fn read(&self) -> Result<Self::Output, EditorError> {
        self.value.clone()
    }
}

impl<N> Default for FloatEditor<N>
where
    N: Default,
{
    fn default() -> Self {
        FloatEditor::new()
    }
}

/// Basic editor for booleans.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BooleanEditor {
    id: Id,
    value: Result<bool, EditorError>,
}

impl BooleanEditor {
    pub fn new() -> Self {
        BooleanEditor {
            id: Id::INVALID,
            value: Ok(false),
        }
    }
}

impl Editor for BooleanEditor {
    type Input = bool;
    type Output = bool;

    fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
        self.id = gen.next();
        if let Some(value) = value {
            self.value = Ok(value);
        }
    }

    fn component(&self) -> Component {
        let widget = Widget::Checkbox(*self.value.as_ref().unwrap_or(&false));

        Component::new(self.id, widget)
    }

    fn on_event(&mut self, event: Event, _gen: &mut Generator) -> Option<Feedback> {
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

    fn read(&self) -> Result<Self::Output, EditorError> {
        self.value.clone()
    }
}

impl Default for BooleanEditor {
    fn default() -> Self {
        BooleanEditor::new()
    }
}

/// Basic editor for characters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharEditor {
    id: Id,
    value: Result<char, EditorError>,
}

impl CharEditor {
    pub fn new() -> Self {
        CharEditor {
            id: Id::INVALID,
            value: Err(EditorError::Invalid),
        }
    }
}

impl Editor for CharEditor {
    type Input = char;
    type Output = char;

    fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
        self.id = gen.next();
        if let Some(value) = value {
            self.value = Ok(value);
        }
    }

    fn component(&self) -> Component {
        // TODO: Limit length to 1
        let widget = Widget::TextField(
            self.value
                .as_ref()
                .map(|value| value.to_string())
                .unwrap_or_default(),
        );

        Component::new(self.id, widget)
    }

    fn on_event(&mut self, event: Event, _gen: &mut Generator) -> Option<Feedback> {
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

    fn read(&self) -> Result<Self::Output, EditorError> {
        self.value.clone()
    }
}

impl Default for CharEditor {
    fn default() -> Self {
        CharEditor::new()
    }
}
