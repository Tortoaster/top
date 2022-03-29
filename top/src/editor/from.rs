use std::str::FromStr;

use crate::component::event::{Event, Feedback};
use crate::component::{Component, ComponentCreator, Id, Widget};
use crate::editor::{Editor, EditorError, Report};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseEditor<T> {
    id: Id,
    value: Report<T>,
}

impl<T> ParseEditor<T>
where
    T: FromStr,
{
    pub fn new() -> Self {
        ParseEditor {
            id: Id::INVALID,
            value: "".parse().map_err(|_| EditorError::Invalid),
        }
    }
}

impl<T> Editor for ParseEditor<T>
where
    T: Clone + FromStr,
{
    type Input = String;
    type Output = T;

    fn component(&mut self, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::TextField {
            value: String::new(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _ctx: &mut ComponentCreator) -> Option<Feedback> {
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

    fn read(&self) -> Report<Self::Output> {
        self.value.clone()
    }

    fn write(&mut self, value: Self::Input) {
        self.value = value.parse().map_err(|_| EditorError::Invalid);
    }
}
