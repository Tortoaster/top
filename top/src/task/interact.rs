use async_trait::async_trait;

use crate::editor::choice::ChoiceEditor;
use crate::editor::generic::Edit;
use crate::editor::Editor;
use crate::event::{Event, Feedback, FeedbackHandler};
use crate::id::Id;
use crate::task::{Context, Task, TaskError, TaskResult, TaskValue};
use crate::viewer::generic::View;
use crate::viewer::Viewer;

/// Basic interaction task. Supports both reading and writing. Use [`enter`], [`edit`], or
/// [`choose`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Interact<E>
where
    E: Editor,
{
    input: Option<E::Input>,
    pub(crate) editor: E,
}

/// Have the user enter a value. To use a custom editor, see [`enter_with`].
#[inline]
pub fn enter<T>() -> Interact<T::Editor>
where
    T: Edit,
{
    enter_with(T::edit())
}

/// Have the user enter a value, through a custom editor.
#[inline]
pub fn enter_with<E>(editor: E) -> Interact<E>
where
    E: Editor,
{
    Interact {
        input: None,
        editor,
    }
}

/// Have the user update a value. To use a custom editor, see [`edit_with`].
#[inline]
pub fn edit<T>(value: T) -> Interact<T::Editor>
where
    T: Edit,
{
    edit_with(value, T::edit())
}

/// Have the user update a value, through a custom editor.
#[inline]
pub fn edit_with<E>(value: E::Input, editor: E) -> Interact<E>
where
    E: Editor,
{
    Interact {
        input: Some(value),
        editor,
    }
}

/// Have the user select a value out of a list of options. To use a custom viewer for the options,
/// see [`choose_with`].
#[inline]
pub fn choose<T>(options: Vec<T>) -> Interact<ChoiceEditor<T::Viewer>>
where
    T: View,
{
    choose_with(options.into_iter().map(T::view).collect())
}

/// Have the user select a value out of a list of options, using a custom viewer.
#[inline]
pub fn choose_with<V>(options: Vec<V>) -> Interact<ChoiceEditor<V>>
where
    V: Viewer,
{
    Interact {
        input: None,
        editor: ChoiceEditor::new(options),
    }
}

#[async_trait]
impl<E> Task for Interact<E>
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
        self.editor.start(self.input.take(), &mut ctx.gen);
        let html = self.editor.as_html();

        let initial = Feedback::Replace { id: Id::ROOT, html };

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
        match self.editor.finish() {
            Ok(value) => Ok(TaskValue::Unstable(value)),
            Err(_) => Ok(TaskValue::Empty),
        }
    }
}
