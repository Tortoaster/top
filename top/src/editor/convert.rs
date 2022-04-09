use std::fmt::Display;
use std::str::FromStr;

use crate::editor::primitive::InputEditor;
use crate::editor::{Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::html::{AsHtml, Html, Input};
use crate::id::Generator;
use crate::tune::{InputTuner, Tune};

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

impl<T> AsHtml for DisplayFromStrEditor<T>
where
    T: Display,
{
    fn as_html(&self) -> Html {
        Input::new(self.editor.id)
            .with_value(
                &self
                    .editor
                    .value
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            )
            .with_label(self.editor.tuner.label.as_deref())
            .as_html()
    }
}

impl<T> Editor for DisplayFromStrEditor<T>
where
    T: Clone + Display + FromStr,
{
    type Output = T;

    fn start(&mut self, gen: &mut Generator) {
        self.editor.start(gen)
    }

    fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback> {
        self.editor.on_event(event, gen)
    }

    fn finish(&self) -> Result<Self::Output, EditorError> {
        self.editor.finish()
    }
}

impl<T> Tune for DisplayFromStrEditor<T> {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.editor.tune(tuner)
    }
}
