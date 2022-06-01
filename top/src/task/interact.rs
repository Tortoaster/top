use async_trait::async_trait;

use crate::editor::generic::Edit;
use crate::editor::Editor;
use crate::html::event::{Event, Feedback};
use crate::html::{Html, ToHtml};
use crate::task::{Result, Task, TaskValue};

/// Basic interaction task. Supports both reading and writing. Use [`enter`], [`edit`], or
/// [`choose`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Interact<E> {
    pub(in crate::task) editor: E,
}

/// Have the user enter a value. To use a custom editor, see [`edit_with`].
#[inline]
pub fn enter<T>() -> Interact<T::Editor>
where
    T: Edit,
{
    edit_with(T::edit(None))
}

/// Have the user update a value. To use a custom editor, see [`edit_with`].
#[inline]
pub fn edit<T>(value: T) -> Interact<T::Editor>
where
    T: Edit,
{
    edit_with(T::edit(Some(value)))
}

/// Have the user enter a value, through a custom editor.
#[inline]
pub fn edit_with<E>(editor: E) -> Interact<E> {
    Interact { editor }
}

// /// Have the user select a value out of a list of options. To use a custom viewer for the options,
// /// see [`choose_with`].
// #[inline]
// pub fn choose<T>(options: Vec<T>) -> Interact<ChoiceEditor<T::Viewer>>
// where
//     T: View,
// {
//     choose_with(options.into_iter().map(T::view).collect())
// }
//
// /// Have the user select a value out of a list of options, using a custom viewer.
// #[inline]
// pub fn choose_with<V>(options: Vec<V>) -> Interact<ChoiceEditor<V>> {
//     Interact {
//         editor: ChoiceEditor::new(options),
//     }
// }

#[async_trait]
impl<E> ToHtml for Interact<E>
where
    E: ToHtml + Send + Sync,
{
    async fn to_html(&self) -> Html {
        self.editor.to_html().await
    }
}

#[async_trait]
impl<E> Task for Interact<E>
where
    E: Editor + ToHtml + Send + Sync,
{
    type Value = E::Value;
    type Share = E::Share;

    async fn on_event(&mut self, event: Event) -> Result<Feedback> {
        Ok(self.editor.on_event(event).await)
    }

    async fn share(&self) -> Self::Share {
        self.editor.share()
    }

    async fn value(self) -> Result<TaskValue<Self::Value>> {
        Ok(self.editor.value().await)
    }
}
