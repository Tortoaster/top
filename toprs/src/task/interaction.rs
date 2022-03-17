use async_trait::async_trait;

use crate::component::event::{Event, Feedback, FeedbackHandler};
use crate::component::Id;
use crate::editor::generic::DefaultEditor;
use crate::editor::{Editor, Report};
use crate::task::value::TaskValue;
use crate::task::{Context, Error, Task};

/// Basic interaction task. Supports both reading and writing. Use [`enter`] or [`update`] to
/// construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Interact<T, E> {
    value: TaskValue<T>,
    editor: E,
}

#[async_trait]
impl<T, E> Task for Interact<T, E>
where
    T: Clone + Send,
    E: Editor<Input = T, Output = Report<T>> + Send,
{
    type Value = T;

    async fn start<H: FeedbackHandler + Send>(
        &mut self,
        ctx: &mut Context<H>,
    ) -> Result<(), Error<H::Error>> {
        let component = self.editor.start(
            self.value.as_ref().cloned().into_option(),
            &mut ctx.components,
        );

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
        if let Some((value, feedback)) = self.editor.on_event(event, &mut ctx.components) {
            if let Ok(value) = value {
                self.value = TaskValue::Unstable(value);
            }
            ctx.feedback.send(feedback).await?;
        }

        Ok(self.value.clone())
    }
}

/// Have the user enter a value. To use a custom editor, see [`enter_with`].
#[inline]
pub fn enter<T>() -> Interact<T, T::Editor>
where
    T: Default + DefaultEditor,
{
    enter_with(T::default_editor())
}

/// Have the user enter a value, through a custom editor.
#[inline]
pub fn enter_with<T, E>(editor: E) -> Interact<T, E>
where
    T: Default,
    E: Editor<Output = Report<T>>,
{
    update_with(T::default(), editor)
}

/// Have the user update a value. To use a custom editor, see [`update_with`].
#[inline]
pub fn update<T>(value: T) -> Interact<T, T::Editor>
where
    T: DefaultEditor,
{
    update_with(value, T::default_editor())
}

/// Have the user update a value, through a custom editor.
#[inline]
pub fn update_with<T, E>(value: T, editor: E) -> Interact<T, E>
where
    E: Editor<Output = Report<T>>,
{
    Interact {
        value: TaskValue::Unstable(value),
        editor,
    }
}
