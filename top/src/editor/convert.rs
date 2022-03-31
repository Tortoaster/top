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
    T: Clone + FromStr,
{
    type Input = String;
    type Output = T;

    fn component(&mut self, gen: &mut Generator) -> Component {
        let widget = Widget::TextField {
            value: String::new(),
            label: None,
            disabled: false,
        };
        let component = Component::new(gen.next(), widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _ctx: &mut Generator) -> Option<Feedback> {
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

    fn write(&mut self, value: Self::Input) {
        self.value = value.parse().map_err(|_| EditorError::Invalid);
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
