use async_trait::async_trait;

use crate::component::event::{EventHandler, Feedback};
use crate::component::Id;
use crate::editor::generic::DefaultEditor;
use crate::editor::{Editor, Report};
use crate::task::value::TaskValue;
use crate::task::{Executor, Task, TaskError};

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

    async fn execute(
        mut self,
        executor: &mut Executor<impl EventHandler + Send>,
    ) -> Result<TaskValue<Self::Value>, TaskError> {
        let component = self.editor.start(
            self.value.as_ref().cloned().into_option(),
            &mut executor.ctx,
        );

        let initial = Feedback::Replace {
            id: Id::ROOT,
            component,
        };
        executor.events.send(initial).await;

        while let Some(event) = executor.events.receive().await {
            let feedback = self.editor.respond_to(event, &mut executor.ctx);
            if let Some(feedback) = feedback {
                executor.events.send(feedback).await;
            }
        }

        Ok(self.value)
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
