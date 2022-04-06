//! This module contains basic editors for primitive types.

use std::fmt::Display;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use crate::editor::{Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::html::{AsHtml, CheckBox, Html, Input, InputType};
use crate::id::{Generator, Id};
use crate::tune::{InputTuner, Tune};

/// Basic editor for strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringEditor {
    id: Id,
    value: String,
    tuner: InputTuner,
}

impl StringEditor {
    pub fn new() -> Self {
        StringEditor {
            id: Id::INVALID,
            value: String::new(),
            tuner: InputTuner::default(),
        }
    }
}

impl AsHtml for StringEditor {
    fn as_html(&self) -> Html {
        Input::new(self.id)
            .with_value(&self.value)
            .with_label(self.tuner.label.as_deref())
            .as_html()
    }
}

impl Editor for StringEditor {
    type Input = String;
    type Output = String;

    fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
        self.id = gen.next();
        if let Some(value) = value {
            self.value = value;
        }
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

    fn finish(&self) -> Result<Self::Output, EditorError> {
        Ok(self.value.clone())
    }
}

impl Tune for StringEditor {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
    }
}

impl Default for StringEditor {
    fn default() -> Self {
        StringEditor::new()
    }
}

/// Basic editor for integers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerEditor<N> {
    id: Id,
    value: Result<N, EditorError>,
    tuner: InputTuner,
}

impl<N> IntegerEditor<N>
where
    N: Default,
{
    pub fn new() -> Self {
        IntegerEditor {
            id: Id::INVALID,
            value: Ok(N::default()),
            tuner: InputTuner::default(),
        }
    }
}

impl<N> AsHtml for IntegerEditor<N>
where
    N: Display,
{
    fn as_html(&self) -> Html {
        Input::new(self.id)
            .with_type(InputType::Number)
            .with_value(
                &self
                    .value
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            )
            .with_label(self.tuner.label.as_deref())
            .as_html()
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

    fn finish(&self) -> Result<Self::Output, EditorError> {
        self.value.clone()
    }
}

impl<N> Tune for IntegerEditor<N> {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
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
    tuner: InputTuner,
}

impl<N> FloatEditor<N>
where
    N: Default,
{
    pub fn new() -> Self {
        FloatEditor {
            id: Id::INVALID,
            value: Ok(N::default()),
            tuner: InputTuner::default(),
        }
    }
}

impl<N> AsHtml for FloatEditor<N>
where
    N: Display,
{
    fn as_html(&self) -> Html {
        Input::new(self.id)
            .with_type(InputType::Number)
            .with_value(
                &self
                    .value
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            )
            .with_label(self.tuner.label.as_deref())
            .as_html()
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

    fn finish(&self) -> Result<Self::Output, EditorError> {
        self.value.clone()
    }
}

impl<N> Tune for FloatEditor<N> {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
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
    tuner: InputTuner,
}

impl BooleanEditor {
    pub fn new() -> Self {
        BooleanEditor {
            id: Id::INVALID,
            value: Ok(false),
            tuner: InputTuner::default(),
        }
    }
}

impl AsHtml for BooleanEditor {
    fn as_html(&self) -> Html {
        CheckBox::new(self.id)
            .with_checked(self.value.as_ref().copied().unwrap_or_default())
            .with_label(self.tuner.label.as_deref())
            .as_html()
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

    fn finish(&self) -> Result<Self::Output, EditorError> {
        self.value.clone()
    }
}

impl Tune for BooleanEditor {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
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
    tuner: InputTuner,
}

impl CharEditor {
    pub fn new() -> Self {
        CharEditor {
            id: Id::INVALID,
            value: Err(EditorError::Invalid),
            tuner: InputTuner::default(),
        }
    }
}

impl AsHtml for CharEditor {
    fn as_html(&self) -> Html {
        // TODO: Limit length to 1
        Input::new(self.id)
            .with_value(
                &self
                    .value
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            )
            .with_label(self.tuner.label.as_deref())
            .as_html()
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

    fn finish(&self) -> Result<Self::Output, EditorError> {
        self.value.clone()
    }
}

impl Tune for CharEditor {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
    }
}

impl Default for CharEditor {
    fn default() -> Self {
        CharEditor::new()
    }
}
