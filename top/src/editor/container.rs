use either::Either;

use crate::component::event::{Event, Feedback};
use crate::component::icon::Icon;
use crate::component::{Component, ComponentCreator, Id, Widget};
use crate::editor::{Editor, Report};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VecEditor<E> {
    id: Id,
    add_id: Id,
    rows: Vec<Row>,
    template: E,
    editors: Vec<E>,
}

impl<E> VecEditor<E>
where
    E: Editor,
{
    pub fn new(editor: E) -> Self {
        VecEditor {
            id: Id::default(),
            add_id: Id::default(),
            rows: Vec::new(),
            template: editor,
            editors: Vec::new(),
        }
    }
}

impl<E> Editor for VecEditor<E>
where
    E: Editor + Clone,
{
    // TODO: Other input?
    type Input = Vec<E::Input>;
    type Output = Vec<E::Output>;

    fn component(&mut self, ctx: &mut ComponentCreator) -> Component {
        let (children, rows) = self
            .editors
            .iter_mut()
            .map(|editor| row(editor, ctx))
            .unzip();

        self.rows = rows;

        let component = ctx.create(Widget::Group {
            children,
            horizontal: false,
        });
        self.id = component.id();

        let button = add_button(ctx);
        self.add_id = button.id();

        ctx.create(Widget::Group {
            children: vec![component, button],
            horizontal: false,
        })
    }

    fn on_event(&mut self, event: Event, ctx: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Press { id } if id == self.add_id => {
                // Add a new row
                let mut editor = self.template.clone();
                let (component, row) = row(&mut editor, ctx);
                self.editors.push(editor);
                self.rows.push(row);

                Some(Feedback::Append {
                    id: self.id,
                    component,
                })
            }
            Event::Press { id } if self.rows.iter().any(|row| row.sub_id == id) => {
                // Remove an existing row
                let index = self.rows.iter().position(|row| row.sub_id == id).unwrap();
                let row = self.rows.remove(index);
                self.editors.remove(index);

                Some(Feedback::Remove { id: row.id })
            }
            _ => self
                .editors
                .iter_mut()
                .find_map(|editor| editor.on_event(event.clone(), ctx)),
        }
    }

    fn read(&self) -> Report<Self::Output> {
        // TODO: Return all errors
        self.editors
            .iter()
            .map(|editor| editor.read())
            .collect::<Result<Vec<_>, _>>()
    }

    fn write(&mut self, value: Self::Input) {
        self.editors = value
            .into_iter()
            .map(|input| {
                let mut editor = self.template.clone();
                editor.write(input);
                editor
            })
            .collect();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionEditor<E> {
    id: Either<Id, Row>,
    editor: E,
    enabled: bool,
}

impl<E> OptionEditor<E>
where
    E: Editor,
{
    pub fn new(editor: E) -> Self {
        OptionEditor {
            id: Either::Left(Id::default()),
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

    fn component(&mut self, ctx: &mut ComponentCreator) -> Component {
        if self.enabled {
            let (component, row) = row(&mut self.editor, ctx);
            self.id = Either::Right(row);
            component
        } else {
            let component = add_button(ctx);
            self.id = Either::Left(component.id());
            component
        }
    }

    fn on_event(&mut self, event: Event, ctx: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Press { id }
                if self
                    .id
                    .as_ref()
                    .map_left(|add_id| *add_id == id)
                    .left_or_default()
                    && !self.enabled =>
            {
                // Add value
                let id = *self.id.as_ref().unwrap_left();

                let (component, row) = row(&mut self.editor, ctx);
                self.id = Either::Right(row);
                self.enabled = true;

                Some(Feedback::Replace { id, component })
            }
            Event::Press { id }
                if self
                    .id
                    .as_ref()
                    .map_right(|row| row.sub_id == id)
                    .right_or_default()
                    && self.enabled =>
            {
                // Remove value
                let id = self.id.as_ref().unwrap_right().id;

                let component = add_button(ctx);
                self.id = Either::Left(component.id());
                self.enabled = false;

                Some(Feedback::Replace { id, component })
            }
            _ => self
                .enabled
                .then(|| self.editor.on_event(event, ctx))
                .flatten(),
        }
    }

    fn read(&self) -> Report<Self::Output> {
        if self.enabled {
            Ok(Some(self.editor.read()?))
        } else {
            Ok(None)
        }
    }

    fn write(&mut self, value: Self::Input) {
        match value {
            None => self.enabled = false,
            Some(value) => {
                self.enabled = true;
                self.editor.write(value);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    id: Id,
    sub_id: Id,
}

/// Creates a row consisting of the editor and a button to remove it.
fn row<E>(editor: &mut E, ctx: &mut ComponentCreator) -> (Component, Row)
where
    E: Editor,
{
    let child = editor.component(ctx);

    let sub = ctx.create(Widget::IconButton {
        icon: Icon::Minus,
        disabled: false,
    });
    let sub_id = sub.id();

    let component = ctx.create(Widget::Group {
        children: vec![child, sub],
        horizontal: true,
    });
    let id = component.id();

    let row = Row { id, sub_id };

    (component, row)
}

fn add_button(ctx: &mut ComponentCreator) -> Component {
    ctx.create(Widget::IconButton {
        icon: Icon::Plus,
        disabled: false,
    })
}
