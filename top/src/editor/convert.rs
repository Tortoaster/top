use std::fmt::Display;
use std::str::FromStr;

use crate::editor::{Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::html::{AsHtml, Html, Input};
use crate::id::{Generator, Id};
use crate::tune::{InputTuner, Tune};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FromStrEditor<T> {
    id: Id,
    value: Result<T, EditorError>,
    tuner: InputTuner,
}

impl<T> FromStrEditor<T>
where
    T: FromStr,
{
    pub fn new(value: T) -> Self {
        FromStrEditor {
            id: Id::INVALID,
            value: Ok(value),
            tuner: InputTuner::default(),
        }
    }
}

impl<T> AsHtml for FromStrEditor<T>
where
    T: Display,
{
    fn as_html(&self) -> Html {
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

impl<T> Editor for FromStrEditor<T>
where
    T: Clone + Display + FromStr,
{
    type Output = T;

    fn start(&mut self, gen: &mut Generator) {
        self.id = gen.next();
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
                        Err(_) => {
                            self.value = Err(EditorError::Invalid);
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

impl<T> Tune for FromStrEditor<T> {
    type Tuner = InputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
    }
}
