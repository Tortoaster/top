use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::editor::primitive::InputEditor;
use crate::editor::{Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::html::{AsHtml, Html};
use crate::id::Generator;
use crate::tune::{InputTuner, Tune};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FromStrEditor<T> {
    editor: InputEditor<String>,
    _parsed: PhantomData<T>,
}

impl<T> FromStrEditor<T>
where
    T: Display,
{
    pub fn new(value: Option<T>) -> Self {
        FromStrEditor {
            editor: InputEditor::new(value.as_ref().map(ToString::to_string)),
            _parsed: PhantomData,
        }
    }
}

impl<T> AsHtml for FromStrEditor<T>
where
    InputEditor<T>: AsHtml,
{
    fn as_html(&self) -> Html {
        self.editor.as_html()
    }
}

impl<T> Editor for FromStrEditor<T>
where
    InputEditor<T>: AsHtml,
    T: FromStr,
{
    type Output = T;

    fn start(&mut self, gen: &mut Generator) {
        self.editor.start(gen)
    }

    fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback> {
        self.editor.on_event(event, gen)
    }

    fn finish(&self) -> Result<Self::Output, EditorError> {
        self.editor
            .finish()
            .and_then(|s| s.parse().map_err(|_| EditorError::Invalid))
    }
}

impl<T> Tune for FromStrEditor<T> {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.editor.tune(tuner)
    }
}
