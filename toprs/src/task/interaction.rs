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
    T: Send,
    E: Editor<Output = Report<T>> + Send,
{
    type Value = T;

    async fn execute(
        mut self,
        executor: &mut Executor<impl EventHandler + Send>,
    ) -> Result<TaskValue<Self::Value>, TaskError> {
        let component = self.editor.start(&mut executor.ctx);

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
pub fn enter<T>() -> Interact<T, T::Editor>
where
    T: DefaultEditor,
{
    enter_with(T::default_editor(None))
}

/// Have the user enter a value, through a custom editor.
pub fn enter_with<T, E>(editor: E) -> Interact<T, E>
where
    E: Editor<Output = Report<T>>,
{
    Interact {
        value: TaskValue::Empty,
        editor,
    }
}
