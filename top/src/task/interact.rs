use async_trait::async_trait;

use crate::editor::choice::ChoiceEditor;
use crate::editor::generic::Edit;
use crate::editor::Editor;
use crate::event::{Event, Feedback, FeedbackHandler};
use crate::id::Id;
use crate::task::{Context, Task, TaskError, TaskResult, TaskValue};
use crate::viewer::generic::View;
use crate::viewer::Viewer;

/// Basic interaction task. Supports both reading and writing. Use [`enter`] or [`edit`] to
/// construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Interact<I, E> {
    input: Option<I>,
    editor: E,
}

/// Have the user enter a value. To use a custom editor, see [`enter_with`].
#[inline]
pub fn enter<I>() -> Interact<I, I::Editor>
where
    I: Edit + Default,
{
    enter_with(I::default_editor())
}

/// Have the user enter a value, through a custom editor.
#[inline]
pub fn enter_with<E>(editor: E) -> Interact<E::Input, E>
where
    E: Editor,
    E::Input: Default,
{
    edit_with(E::Input::default(), editor)
}

/// Have the user update a value. To use a custom editor, see [`edit_with`].
#[inline]
pub fn edit<I>(value: I) -> Interact<I, I::Editor>
where
    I: Edit,
{
    edit_with(value, I::default_editor())
}

/// Have the user update a value, through a custom editor.
#[inline]
pub fn edit_with<E>(value: E::Input, editor: E) -> Interact<E::Input, E>
where
    E: Editor,
{
    Interact {
        input: Some(value),
        editor,
    }
}

#[async_trait]
impl<E> Task for Interact<E::Input, E>
where
    E: Editor + Send,
    E::Input: Send,
    E::Output: Send + Sync,
{
    type Value = E::Output;

    async fn start<H>(&mut self, ctx: &mut Context<H>) -> Result<(), TaskError>
    where
        H: FeedbackHandler + Send,
    {
        if let Some(value) = self.input.take() {
            self.editor.write(value);
        }
        let component = self.editor.component(&mut ctx.gen);

        let initial = Feedback::Replace {
            id: Id::ROOT,
            component,
        };

        ctx.feedback.send(initial).await?;
        Ok(())
    }

    async fn on_event<H>(&mut self, event: Event, ctx: &mut Context<H>) -> TaskResult<Self::Value>
    where
        H: FeedbackHandler + Send,
    {
        if let Some(feedback) = self.editor.on_event(event, &mut ctx.gen) {
            ctx.feedback.send(feedback).await?;
        }
        match self.editor.read() {
            Ok(value) => Ok(TaskValue::Unstable(value)),
            Err(_) => Ok(TaskValue::Empty),
        }
    }
}

pub fn choose<T>(options: Vec<T>) -> Interact<usize, ChoiceEditor<T::Viewer>>
where
    T: View,
{
    choose_with(options)
}

pub fn choose_with<V>(options: Vec<V::Input>) -> Interact<usize, ChoiceEditor<V>>
where
    V: Viewer,
{
    Interact {
        input: None,
        editor: ChoiceEditor::new(options),
    }
}
