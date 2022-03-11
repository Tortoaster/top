use crate::editor::generic::DefaultEditor;
use crate::editor::Editor;
use crate::task::Task;

/// Basic view interaction task. See [`view`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct View<E> {
    editor: E,
}

/// Displays the provided value to the user. To use a custom editor, see [`view_with`].
pub fn view<T>(value: T) -> View<T::Editor>
where
    T: Clone + DefaultEditor,
{
    view_with(value.clone(), T::default_editor(Some(value)))
}

/// Display the provided value to the user, through a custom editor.
pub fn view_with<T, E>(value: T, editor: E) -> View<E>
where
    E: Editor<Output = T>,
{
    View { editor }
}

impl<T, E> Task for View<E>
where
    E: Editor<Output = T> + Send,
{
    type Editor = E;

    fn editor(self) -> Self::Editor {
        self.editor
    }
}

/// Basic enter interaction task. See [`enter`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Enter<E> {
    editor: E,
}

/// Have the user enter a value. To use a custom editor, see [`enter_with`].
pub fn enter<T>() -> Enter<T::Editor>
where
    T: DefaultEditor,
{
    enter_with(T::default_editor(None))
}

/// Have the user enter a value, through a custom editor.
pub fn enter_with<T, E>(editor: E) -> Enter<E>
where
    E: Editor<Output = T>,
{
    Enter { editor }
}

impl<T, E> Task for Enter<E>
where
    E: Editor<Output = T> + Send,
{
    type Editor = E;

    fn editor(self) -> Self::Editor {
        self.editor
    }
}

/// Basic update interaction task. See [`update`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Update<E> {
    editor: E,
}

/// Have the user update a value. To use a custom editor, see [`update_with`].
pub fn update<T>(value: T) -> Update<T::Editor>
where
    T: Clone + DefaultEditor,
{
    update_with(value.clone(), T::default_editor(Some(value)))
}

/// Have the user update a value, through a custom editor.
pub fn update_with<T, E>(value: T, editor: E) -> Update<E>
where
    E: Editor<Output = T>,
{
    Update { editor }
}

impl<T, E> Task for Update<E>
where
    E: Editor<Output = T> + Send,
{
    type Editor = E;

    fn editor(self) -> Self::Editor {
        self.editor
    }
}
