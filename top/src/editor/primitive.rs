//! This module contains basic editors for primitive types.

use std::str::FromStr;

use top_derive::html;

use crate::editor::{Editor, EditorError};
use crate::html::event::{Change, Event, Feedback};
use crate::html::id::{Generator, Id};
use crate::html::{Html, ToHtml};
use crate::task::tune::{InputTuner, Tune};

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

    fn on_event(&mut self, event: Event, _gen: &mut Generator) -> Feedback {
        match event {
            Event::Update { id, value } if id == self.id => match value.parse::<T>() {
                Ok(value) => {
                    self.value = Ok(value);
                    Feedback::from(Change::Valid { id })
                }
                Err(_) => {
                    self.value = Err(EditorError::Invalid);
                    Feedback::from(Change::Invalid { id })
                }
            },
            _ => Feedback::new(),
        }
    }

    fn value(&self) -> Result<Self::Value, EditorError> {
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

impl ToHtml for InputEditor<String> {
    fn to_html(&self) -> Html {
        html! {r#"
            <label for="{self.id}" class="label">{self.tuner.label}</label>
            <input id="{self.id}" class="input" value="{self.value}" onblur="update(this)"/>
        "#}
    }
}

macro_rules! impl_to_html_for_number {
    ($($ty:ty),*) => {
        $(
            impl ToHtml for InputEditor<$ty> {
                fn to_html(&self) -> Html {
                    let value = self.value.as_ref().map(ToString::to_string);
                    html! {r#"
                        <label for="{self.id}" class="label">{self.tuner.label}</label>
                        <input id="{self.id}" type="number" class="input" value="{value}" onblur="update(this)"/>
                    "#}
                }
            }
        )*
    };
}

impl_to_html_for_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl ToHtml for InputEditor<bool> {
    fn to_html(&self) -> Html {
        let checked = self
            .value
            .as_ref()
            .copied()
            .unwrap_or_default()
            .then(|| "checked");
        html! {r#"
            <label class="checkbox">
                <input id="{self.id}" type="checkbox" onclick="update(this, this.checked.toString())" {checked}>
                {self.tuner.label}
            </label>
        "#}
    }
}

impl ToHtml for InputEditor<char> {
    fn to_html(&self) -> Html {
        let value = self
            .value
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();
        html! {r#"
            <label for="{self.id}" class="label">{self.tuner.label}</label>
            <input id="{self.id}" class="input" value="{value}" onblur="update(this)" maxlength="1"/>
        "#}
    }
}
