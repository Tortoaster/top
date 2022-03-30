use async_trait::async_trait;

use crate::component::event::{Event, Feedback, FeedbackHandler};
use crate::component::Id;
use crate::editor::choice::ChoiceEditor;
use crate::editor::generic::Edit;
use crate::editor::Editor;
use crate::task::value::TaskValue;
use crate::task::{Context, Error, Task};
use crate::viewer::generic::View;
use crate::viewer::Viewer;

/// Basic interaction task. Supports both reading and writing. Use [`enter`] or [`update`] to
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
    I: Default + Edit,
{
    enter_with(I::default_editor())
}

/// Have the user enter a value, through a custom editor.
#[inline]
pub fn enter_with<I, E>(editor: E) -> Interact<I, E>
where
    I: Default,
    E: Editor<Input = I>,
{
    update_with(I::default(), editor)
}

/// Have the user update a value. To use a custom editor, see [`update_with`].
#[inline]
pub fn update<I>(value: I) -> Interact<I, I::Editor>
where
    I: Edit,
{
    update_with(value, I::default_editor())
}

/// Have the user update a value, through a custom editor.
#[inline]
pub fn update_with<I, E>(value: I, editor: E) -> Interact<I, E>
where
    E: Editor<Input = I>,
{
    Interact {
        input: Some(value),
        editor,
    }
}

#[async_trait]
impl<I, O, E> Task for Interact<I, E>
where
    I: Send,
    O: Send + Sync,
    E: Editor<Input = I, Output = O> + Send,
{
    type Value = O;

    async fn start<H: FeedbackHandler + Send>(
        &mut self,
        ctx: &mut Context<H>,
    ) -> Result<(), Error<H::Error>> {
        if let Some(value) = self.input.take() {
            self.editor.write(value);
        }
        let component = self.editor.component(&mut ctx.components);

        let initial = Feedback::Replace {
            id: Id::ROOT,
            component,
        };

        ctx.feedback.send(initial).await?;
        Ok(())
    }

    async fn on_event<H: FeedbackHandler + Send>(
        &mut self,
        event: Event,
        ctx: &mut Context<H>,
    ) -> Result<TaskValue<Self::Value>, Error<H::Error>> {
        if let Some(feedback) = self.editor.on_event(event, &mut ctx.components) {
            ctx.feedback.send(feedback).await?;
        }
        match self.editor.read() {
            Ok(value) => Ok(TaskValue::Unstable(value)),
            Err(_) => Ok(TaskValue::Empty),
        }
    }

    async fn finish(self) -> TaskValue<Self::Value> {
        match self.editor.read() {
            Ok(value) => TaskValue::Stable(value),
            Err(_) => TaskValue::Empty,
        }
    }
}

pub fn choose<V, F>(options: Vec<F>) -> Interact<usize, ChoiceEditor<V>>
where
    V: Viewer<Input = F>,
    F: View<Viewer = V>,
{
    Interact {
        input: None,
        editor: ChoiceEditor::new(options),
    }
}
