use crate::component::event::{Event, Feedback};
use crate::component::{Component, ComponentCreator, Id, Widget};
use crate::editor::{Editor, Report};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VecEditor<E> {
    group_id: Id,
    button_id: Id,
    template: E,
    editors: Vec<E>,
}

impl<E> VecEditor<E>
where
    E: Editor,
{
    pub fn new(editor: E) -> Self {
        VecEditor {
            group_id: Id::default(),
            button_id: Id::default(),
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
            .map(|input| {
                let mut editor = self.template.clone();
                let component = editor.start(Some(input), ctx);
                (editor, component)
            })
            .unzip();

        self.editors = editors;

        let group = ctx.create(Widget::Group(components));
        self.group_id = group.id();

        let button = ctx.create(Widget::Button {
            text: "+".to_owned(),
            disabled: false,
        });
        self.button_id = button.id();

        ctx.create(Widget::Group(vec![group, button]))
    }

    fn on_event(&mut self, event: Event, ctx: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Press { id } if id == self.button_id => {
                let mut editor = self.template.clone();
                let component = editor.start(None, ctx);
                self.editors.push(editor);

                Some(Feedback::Append {
                    id: self.group_id,
                    component,
                })
            }
            _ => self
                .editors
                .iter_mut()
                .find_map(|editor| editor.on_event(event.clone(), ctx)),
        }
    }

    fn value(&self) -> Report<Self::Output> {
        Ok(self
            .editors
            .iter()
            .map(Editor::value)
            .collect::<Result<Vec<_>, _>>()?)
    }
}
