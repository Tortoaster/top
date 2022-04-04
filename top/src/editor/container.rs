use crate::component::icon::Icon;
use crate::component::{Component, Widget};
use crate::editor::{Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::id::{Generator, Id};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VecEditor<E> {
    /// Represents the list containing all choices.
    group_id: Id,
    /// Represents the plus button.
    add_id: Id,
    /// Represents of each of the choices with their respective identifiers.
    choices: Vec<Row>,
    editors: Vec<E>,
    template: E,
}

impl<E> VecEditor<E> {
    pub fn new(editor: E) -> Self {
        VecEditor {
            group_id: Id::INVALID,
            add_id: Id::INVALID,
            choices: Vec::new(),
            editors: Vec::new(),
            template: editor,
        }
    }
}

impl<E> Editor for VecEditor<E>
where
    E: Editor + Clone,
{
    type Input = Vec<E::Input>;
    type Output = Vec<E::Output>;

    fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
        self.group_id = gen.next();
        self.add_id = gen.next();
        self.editors = value
            .into_iter()
            .flatten()
            .map(|input| {
                let mut editor = self.template.clone();
                editor.start(Some(input), gen);
                editor
            })
            .collect();
        self.choices = self.editors.iter().map(|_| Row::new(gen)).collect();
    }

    fn component(&self) -> Component {
        let choices = self
            .editors
            .iter()
            .zip(&self.choices)
            .map(|(editor, row)| row.component(editor))
            .collect();

        let group = Component::new(self.group_id, Widget::Group(choices));
        let button = Row::add_button(self.add_id);

        Component::new(Id::INVALID, Widget::Group(vec![group, button]))
    }

    fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback> {
        match event {
            Event::Press { id } if id == self.add_id => {
                // Add a new row
                let mut editor = self.template.clone();
                editor.start(None, gen);
                let row = Row::new(gen);
                let component = row.component(&editor);

                self.editors.push(editor);
                self.choices.push(row);

                Some(Feedback::Append {
                    id: self.group_id,
                    component,
                })
            }
            Event::Press { id } if self.choices.iter().any(|row| row.sub_id == id) => {
                // Remove an existing row
                let index = self
                    .choices
                    .iter()
                    .position(|row| row.sub_id == id)
                    .unwrap();
                let Row { id, .. } = self.choices.remove(index);
                self.editors.remove(index);

                Some(Feedback::Remove { id })
            }
            _ => self
                .editors
                .iter_mut()
                .find_map(|editor| editor.on_event(event.clone(), gen)),
        }
    }

    fn read(&self) -> Result<Self::Output, EditorError> {
        // TODO: Return all errors
        self.editors
            .iter()
            .map(|editor| editor.read())
            .collect::<Result<Vec<_>, _>>()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionEditor<E> {
    /// Represents the row containing the editor and the minus button if a value is present.
    row: Row,
    /// Represents the plus button if there is no value present.
    add_id: Id,
    editor: E,
    /// True if this editor contains a value, false otherwise.
    enabled: bool,
}

impl<E> OptionEditor<E>
where
    E: Editor,
{
    pub fn new(editor: E) -> Self {
        OptionEditor {
            row: Row {
                id: Id::INVALID,
                sub_id: Id::INVALID,
            },
            add_id: Id::INVALID,
            editor,
            enabled: false,
        }
    }
}

impl<E> Editor for OptionEditor<E>
where
    E: Editor,
{
    type Input = Option<E::Input>;
    type Output = Option<E::Output>;

    fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
        self.row = Row::new(gen);
        self.add_id = gen.next();
        self.enabled = value.is_some();

        self.editor.start(value.flatten(), gen);
    }

    fn component(&self) -> Component {
        if self.enabled {
            self.row.component(&self.editor)
        } else {
            Row::add_button(self.add_id)
        }
    }

    fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback> {
        match event {
            Event::Press { id } if id == self.add_id && !self.enabled => {
                // Add value
                let id = self.add_id;
                let component = self.row.component(&mut self.editor);
                self.enabled = true;

                Some(Feedback::Replace { id, component })
            }
            Event::Press { id } if id == self.row.sub_id && self.enabled => {
                // Remove value
                let id = self.row.id;
                let component = Row::add_button(self.add_id);
                self.enabled = false;

                Some(Feedback::Replace { id, component })
            }
            _ => self
                .enabled
                .then(|| self.editor.on_event(event, gen))
                .flatten(),
        }
    }

    fn read(&self) -> Result<Self::Output, EditorError> {
        if self.enabled {
            Ok(Some(self.editor.read()?))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    id: Id,
    sub_id: Id,
}

impl Row {
    fn new(gen: &mut Generator) -> Self {
        Row {
            id: gen.next(),
            sub_id: gen.next(),
        }
    }

    /// Creates a row consisting of the editor and a button to remove it.
    fn component<E>(&self, editor: &E) -> Component
    where
        E: Editor,
    {
        let child = editor.component();
        let sub = Component::new(self.sub_id, Widget::IconButton(Icon::Minus));

        Component::new(self.id, Widget::Group(vec![child, sub]))
            .tune()
            .set_direction(true)
            .finish()
    }

    fn add_button(id: Id) -> Component {
        Component::new(id, Widget::IconButton(Icon::Plus))
    }
}
