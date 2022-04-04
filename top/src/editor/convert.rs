use std::fmt::Display;
use std::str::FromStr;

use crate::component::{Component, Widget};
use crate::editor::{Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::id::{Generator, Id};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FromStrEditor<T> {
    id: Id,
    value: Result<T, EditorError>,
}

impl<T> FromStrEditor<T>
where
    T: FromStr,
{
    pub fn new() -> Self {
        FromStrEditor {
            id: Id::INVALID,
            value: "".parse().map_err(|_| EditorError::Invalid),
        }
    }
}

impl<T> Editor for FromStrEditor<T>
where
    T: Clone + Display + FromStr,
{
    type Input = T;
    type Output = T;

    fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
        self.id = gen.next();
        if let Some(value) = value {
            self.value = Ok(value);
        }
    }

    fn component(&self) -> Component {
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

    fn read(&self) -> Result<Self::Output, EditorError> {
        self.value.clone()
    }
}

impl<T> Default for FromStrEditor<T>
where
    T: FromStr,
{
    fn default() -> Self {
        FromStrEditor::new()
    }
}
