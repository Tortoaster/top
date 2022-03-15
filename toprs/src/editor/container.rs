use crate::component::{Component, ComponentId, Context, Widget};
use crate::editor::event::{Event, Feedback};
use crate::editor::Editor;
use crate::task::Task;

/// Basic sequential editor. It starts with one editor, adds a continue button, and when the user
/// presses it, it turns into another editor, passing the result of the previous as an argument.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SequentialEditor<E1, F, E2> {
    first: Option<E1>,
    f: F,
    then: Option<E2>,
    done: bool,
    id: ComponentId,
    button_id: ComponentId,
}

impl<E1, F, T2, E2, O> SequentialEditor<E1, F, E2>
where
    E1: Editor<Output = O>,
    F: Fn(O) -> T2,
    T2: Task<Editor = E2>,
    E2: Editor,
{
    /// Creates a new sequential editor.
    pub fn new(first: E1, f: F) -> Self {
        SequentialEditor {
            first: Some(first),
            f,
            then: None,
            done: false,
            id: ComponentId::default(),
            button_id: ComponentId::default(),
        }
    }
}

impl<E1, F, T2, E2, O> Editor for SequentialEditor<E1, F, E2>
where
    E1: Editor<Output = O>,
    F: Fn(O) -> T2,
    T2: Task<Editor = E2>,
    E2: Editor,
{
    type Output = E2::Output;

    fn start(&mut self, ctx: &mut Context) -> Component {
        let form = self.first.as_mut().unwrap().start(ctx);
        let button = ctx.create_component(Widget::Button {
            text: "Next".to_string(),
            disabled: false,
        });
        self.button_id = button.id();
        let component = ctx.create_component(Widget::Group(vec![form, button]));
        self.id = component.id();
        component
    }

    fn respond_to(&mut self, event: Event, ctx: &mut Context) -> Option<Feedback> {
        match event {
            Event::Press { id } if id == self.button_id => {
                let value = self.first.take().unwrap().finish();
                let mut editor = (self.f)(value).editor();
                let component = editor.start(ctx);
                self.then = Some(editor);
                let response = Feedback::Replace {
                    id: self.id,
                    content: component.html(),
                };
                self.id = component.id();
                self.done = true;
                Some(response)
            }
            e => {
                if self.done {
                    self.then.as_mut().unwrap().respond_to(e, ctx)
                } else {
                    self.first.as_mut().unwrap().respond_to(e, ctx)
                }
            }
        }
    }

    fn finish(self) -> Self::Output {
        self.then.unwrap().finish()
    }
}
