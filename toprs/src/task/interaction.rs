use crate::editor::generic::DefaultEditor;
use crate::editor::Editor;
use crate::task::value::OptionExt;
use crate::task::{Task, TaskValue};

#[derive(Debug)]
pub struct View<T, E> {
    value: T,
    editor: E,
}

impl<T, E> Task for View<T, E>
where
    E: Editor<Output = T> + Send,
{
    type Output = T;
    type Editor = E;

    fn get_value(self) -> TaskValue<Self::Output> {
        TaskValue::Stable(self.value)
    }

    fn get_editor(self) -> Self::Editor {
        self.editor
    }
}

pub fn view<T>(value: T) -> View<T, T::Editor>
where
    T: Clone + DefaultEditor,
{
    let editor = T::default_editor();
    View { value, editor }
}

pub fn view_with<T, E>(value: T, editor: E) -> View<T, E>
where
    E: Editor<Output = T>,
{
    View { value, editor }
}

#[derive(Debug)]
pub struct Enter<T, E> {
    value: Option<T>,
    editor: E,
}

impl<T, E> Task for Enter<T, E>
where
    E: Editor<Output = T> + Send,
{
    type Output = T;
    type Editor = E;

    fn get_value(self) -> TaskValue<Self::Output> {
        self.value.into_unstable()
    }

    fn get_editor(self) -> Self::Editor {
        self.editor
    }
}

pub fn enter<T>() -> Enter<T, T::Editor>
where
    T: DefaultEditor,
{
    let editor = T::default_editor();
    Enter {
        value: None,
        editor,
    }
}

pub fn enter_with<T, E>(editor: E) -> Enter<T, E>
where
    E: Editor<Output = T>,
{
    Enter {
        value: None,
        editor,
    }
}

#[derive(Debug)]
pub struct Update<T, E> {
    value: Option<T>,
    editor: E,
}

impl<T, E> Task for Update<T, E>
where
    E: Editor<Output = T> + Send,
{
    type Output = T;
    type Editor = E;

    fn get_value(self) -> TaskValue<Self::Output> {
        self.value.into_unstable()
    }

    fn get_editor(self) -> Self::Editor {
        self.editor
    }
}

pub fn update<T>(value: T) -> Update<T, T::Editor>
where
    T: DefaultEditor,
{
    let editor = T::default_editor();
    Update {
        value: Some(value),
        editor,
    }
}

pub fn update_with<T, E>(value: T, editor: E) -> Update<T, E>
where
    E: Editor<Output = T>,
{
    Update {
        value: Some(value),
        editor,
    }
}
