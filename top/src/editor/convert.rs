use std::fmt::Display;
use std::str::FromStr;

use top_derive::html;

use crate::editor::primitive::InputEditor;
use crate::editor::{Editor, EditorError};
use crate::html::event::{Event, Feedback};
use crate::html::id::Generator;
use crate::html::{Html, ToHtml};
use crate::task::tune::{InputTuner, Tune};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayFromStrEditor<T> {
    editor: InputEditor<T>,
}

impl<T> DisplayFromStrEditor<T>
where
    T: FromStr,
{
    pub fn new(value: Option<T>) -> Self {
        let editor = match value {
            None => match "".parse::<T>() {
                Ok(value) => InputEditor::new(value),
                Err(_) => InputEditor::empty(),
            },
            Some(value) => InputEditor::new(value),
        };

        DisplayFromStrEditor { editor }
    }
}

impl<T> ToHtml for DisplayFromStrEditor<T>
where
    T: Display,
{
    fn to_html(&self) -> Html {
        let value = self.editor.value.as_ref().map(ToString::to_string);
        html! {r#"
            <label for="{self.editor.id}" class="label">{self.editor.tuner.label}</label>
            <input id="self.editor.id" class="input" value="{value}" onblur="update(this)"/>
        "#}
    }
}

impl<T> Editor for DisplayFromStrEditor<T>
where
    T: Clone + Display + FromStr,
{
    type Value = T;

    fn start(&mut self, gen: &mut Generator) {
        self.editor.start(gen)
    }

    fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback> {
        self.editor.on_event(event, gen)
    }

    fn finish(&self) -> Result<Self::Value, EditorError> {
        self.editor.finish()
    }
}

impl<T> Tune for DisplayFromStrEditor<T> {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.editor.tune(tuner)
    }
}
