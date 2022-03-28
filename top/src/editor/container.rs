use crate::component::event::{Event, Feedback};
use crate::component::icon::Icon;
use crate::component::{Component, ComponentCreator, Id, Widget};
use crate::editor::{Editor, Report};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VecEditor<E> {
    group_id: Id,
    add_id: Id,
    template: E,
    editors: Vec<Entry<E>>,
}

impl<E> VecEditor<E>
where
    E: Editor,
{
    pub fn new(editor: E) -> Self {
        VecEditor {
            group_id: Id::default(),
            add_id: Id::default(),
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

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        let (editors, components) = initial
            .unwrap_or_default()
            .into_iter()
            .map(|input| Entry::new(self.template.clone(), Some(input), ctx))
            .unzip();

        self.editors = editors;

        let group = ctx.create(Widget::Group {
            children: components,
            horizontal: false,
        });
        self.group_id = group.id();

        let button = ctx.create(Widget::IconButton {
            icon: Icon::Plus,
            disabled: false,
        });
        self.add_id = button.id();

        ctx.create(Widget::Group {
            children: vec![group, button],
            horizontal: false,
        })
    }

    fn on_event(&mut self, event: Event, ctx: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Press { id } if id == self.add_id => {
                let (entry, component) = Entry::new(self.template.clone(), None, ctx);
                self.editors.push(entry);

                Some(Feedback::Append {
                    id: self.group_id,
                    component,
                })
            }
            Event::Press { id } => {
                match self
                    .editors
                    .iter()
                    .enumerate()
                    .find_map(|(index, entry)| (id == entry.remove_id).then(|| index))
                {
                    None => self
                        .editors
                        .iter_mut()
                        .find_map(|entry| entry.editor.on_event(event.clone(), ctx)),
                    Some(index) => {
                        let entry = self.editors.remove(index);
                        Some(Feedback::Remove { id: entry.group_id })
                    }
                }
            }
            Event::Update { .. } => self
                .editors
                .iter_mut()
                .find_map(|entry| entry.editor.on_event(event.clone(), ctx)),
        }
    }

    fn value(&self) -> Report<Self::Output> {
        // TODO: Return all errors
        Ok(self
            .editors
            .iter()
            .map(|entry| entry.editor.value())
            .collect::<Result<Vec<_>, _>>()?)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionEditor<E> {
    id: Id,
    template: Option<E>,
    editor: Option<Entry<E>>,
}

impl<E> OptionEditor<E>
where
    E: Editor,
{
    pub fn new(editor: E) -> Self {
        OptionEditor {
            id: Id::default(),
            template: Some(editor),
            editor: None,
        }
    }

    fn add_button(ctx: &mut ComponentCreator) -> Component {
        ctx.create(Widget::IconButton {
            icon: Icon::Plus,
            disabled: false,
        })
    }
}

impl<E> Editor for OptionEditor<E>
where
    E: Editor,
{
    type Input = Option<E::Input>;
    type Output = Option<E::Output>;

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        // TODO: Use Option::unzip when stable
        let (editor, component) = match initial
            .flatten()
            .map(|input| Entry::new(self.template.take().unwrap(), Some(input), ctx))
        {
            None => (None, None),
            Some((a, b)) => (Some(a), Some(b)),
        };

        self.editor = editor;

        match component {
            None => {
                let button = Self::add_button(ctx);
                self.id = button.id();
                button
            }
            Some(component) => {
                self.id = component.id();
                component
            }
        }
    }

    fn on_event(&mut self, event: Event, ctx: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Press { id } if id == self.id && self.editor.is_none() => {
                let (entry, component) = Entry::new(self.template.take().unwrap(), None, ctx);
                self.editor = Some(entry);

                let old_id = self.id;
                self.id = component.id();

                Some(Feedback::Replace {
                    id: old_id,
                    component,
                })
            }
            Event::Press { id }
                if self
                    .editor
                    .as_ref()
                    .map(|entry| entry.remove_id == id)
                    .unwrap_or_default() =>
            {
                let entry = self.editor.take().unwrap();
                self.template = Some(entry.editor);

                let button = Self::add_button(ctx);

                let old_id = self.id;
                self.id = button.id();

                Some(Feedback::Replace {
                    id: old_id,
                    component: button,
                })
            }
            event => self
                .editor
                .iter_mut()
                .find_map(|entry| entry.editor.on_event(event.clone(), ctx)),
        }
    }

    fn value(&self) -> Report<Self::Output> {
        self.editor
            .as_ref()
            .map(|entry| entry.editor.value())
            .map_or(Ok(None), |value| value.map(Some))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Entry<E> {
    editor: E,
    remove_id: Id,
    group_id: Id,
}

impl<E> Entry<E>
where
    E: Editor,
{
    fn new(
        mut editor: E,
        input: Option<E::Input>,
        ctx: &mut ComponentCreator,
    ) -> (Self, Component) {
        let component = editor.start(input, ctx);
        let remove = ctx.create(Widget::IconButton {
            icon: Icon::Minus,
            disabled: false,
        });
        let remove_id = remove.id();
        let group = ctx.create(Widget::Group {
            children: vec![component, remove],
            horizontal: true,
        });
        let entry = Entry {
            editor,
            remove_id,
            group_id: group.id(),
        };
        (entry, group)
    }
}
