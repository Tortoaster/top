//! This module contains basic editors for primitive types.

use std::str::FromStr;

use crate::editor::{Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::html::{AsHtml, CheckBox, Html, Input, InputType};
use crate::id::{Generator, Id};
use crate::tune::{InputTuner, Tune};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InputEditor<T> {
    pub(in crate::editor) id: Id,
    pub(in crate::editor) value: Result<T, EditorError>,
    pub(in crate::editor) tuner: InputTuner,
}

impl<T> InputEditor<T> {
    pub fn new(value: T) -> Self {
        InputEditor {
            id: Id::INVALID,
            value: Ok(value),
            tuner: InputTuner::default(),
        }
    }

    pub fn empty() -> Self {
        InputEditor {
            id: Id::INVALID,
            value: Err(EditorError::Empty),
            tuner: InputTuner::default(),
        }
    }
}

impl<T> Editor for InputEditor<T>
where
    T: Clone + FromStr,
{
    type Value = T;

    fn start(&mut self, gen: &mut Generator) {
        self.id = gen.next();
    }

    fn on_event(&mut self, event: Event, _gen: &mut Generator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } if id == self.id => match value.parse::<T>() {
                Ok(value) => {
                    self.value = Ok(value);
                    Some(Feedback::Valid { id })
                }
                Err(_) => {
                    self.value = Err(EditorError::Invalid);
                    Some(Feedback::Invalid { id })
                }
            },
            _ => None,
        }
    }

    fn finish(&self) -> Result<Self::Value, EditorError> {
        self.value.clone()
    }
}

impl<T> Tune for InputEditor<T> {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
    }
}

impl<T> Default for InputEditor<T> {
    fn default() -> Self {
        InputEditor::empty()
    }
}

impl AsHtml for InputEditor<String> {
    fn as_html(&self) -> Html {
        Input::new(self.id)
            .with_value(self.value.as_deref().unwrap_or_default())
            .with_label(self.tuner.label.as_deref())
            .as_html()
    }
}

macro_rules! impl_as_html_for_number {
    ($($ty:ty),*) => {
        $(
            impl AsHtml for InputEditor<$ty> {
                fn as_html(&self) -> Html {
                    Input::new(self.id)
                        .with_type(InputType::Number)
                        .with_value(
                            &self
                                .value
                                .as_ref()
                                .map(<$ty>::to_string)
                                .unwrap_or_default(),
                        )
                        .with_label(self.tuner.label.as_deref())
                        .as_html()
                }
            }
        )*
    };
}

impl_as_html_for_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl AsHtml for InputEditor<bool> {
    fn as_html(&self) -> Html {
        CheckBox::new(self.id)
            .with_checked(self.value.as_ref().copied().unwrap_or_default())
            .with_label(self.tuner.label.as_deref())
            .as_html()
    }
}

impl AsHtml for InputEditor<char> {
    fn as_html(&self) -> Html {
        // TODO: Limit length to 1
        Input::new(self.id)
            .with_value(&self.value.as_ref().map(char::to_string).unwrap_or_default())
            .with_label(self.tuner.label.as_deref())
            .as_html()
    }
}
