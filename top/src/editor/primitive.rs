//! This module contains basic editors for primitive types.

use std::str::FromStr;

use async_trait::async_trait;

use top_derive::html;

use crate::editor::{Editor, EditorError};
use crate::html::event::{Change, Event, Feedback};
use crate::html::id::{Generator, Id};
use crate::html::{Html, ToHtml};
use crate::share::Share;
use crate::task::tune::{InputTuner, Tune};

#[derive(Clone, Debug)]
pub struct InputEditor<T> {
    pub(in crate::editor) id: Id,
    pub(in crate::editor) value: Share<Result<T, EditorError>>,
    pub(in crate::editor) tuner: InputTuner,
}

impl<T> InputEditor<T> {
    pub fn new(value: Option<T>) -> Self {
        InputEditor {
            id: Id::INVALID,
            value: Share::new(value.ok_or(EditorError::Empty)),
            tuner: InputTuner::default(),
        }
    }
}

#[async_trait]
impl<T> Editor for InputEditor<T>
where
    T: Clone + FromStr + Send,
{
    type Value = T;
    type Share = Share<Result<Self::Value, EditorError>>;

    fn start(&mut self, gen: &mut Generator) {
        self.id = gen.next();
    }

    async fn on_event(&mut self, event: Event, _gen: &mut Generator) -> Feedback {
        match event {
            Event::Update { id, value } if id == self.id => match value.parse::<T>() {
                Ok(value) => {
                    self.value.write(Ok(value));
                    Feedback::from(Change::Valid { id })
                }
                Err(_) => {
                    self.value.write(Err(EditorError::Invalid));
                    Feedback::from(Change::Invalid { id })
                }
            },
            _ => Feedback::new(),
        }
    }

    fn share(&self) -> Self::Share {
        self.value.clone()
    }

    fn value(self) -> Result<Self::Value, EditorError> {
        self.value.into_inner()
    }
}

impl<T> Tune for InputEditor<T> {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
    }
}

#[async_trait]
impl ToHtml for InputEditor<String> {
    async fn to_html(&self) -> Html {
        html! {r#"
            <label for="{self.id}" class="label">{self.tuner.label}</label>
            <input id="{self.id}" class="input" value="{self.value.read().await}" onblur="update(this)"/>
        "#}
    }
}

macro_rules! impl_to_html_for_number {
    ($($ty:ty),*) => {
        $(
            #[async_trait]
            impl ToHtml for InputEditor<$ty> {
                async fn to_html(&self) -> Html {
                    let value = self
                        .value
                        .read()
                        .await
                        .as_ref()
                        .map(ToString::to_string);
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

#[async_trait]
impl ToHtml for InputEditor<bool> {
    async fn to_html(&self) -> Html {
        let checked = self
            .value
            .read()
            .await
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

#[async_trait]
impl ToHtml for InputEditor<char> {
    async fn to_html(&self) -> Html {
        let value = self
            .value
            .read()
            .await
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();
        html! {r#"
            <label for="{self.id}" class="label">{self.tuner.label}</label>
            <input id="{self.id}" class="input" value="{value}" onblur="update(this)" maxlength="1"/>
        "#}
    }
}
