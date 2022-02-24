use crate::editor::Editor;
use crate::task::{Task, TaskValue};

pub fn view<T, E>(value: T, editor: E) -> Task<T, E>
where
    E: Editor<Read = T>,
{
    Task {
        value: Some(TaskValue::Stable(value)),
        editor,
    }
}

pub fn enter<T, E>(editor: E) -> Task<T, E>
where
    E: Editor<Read = T>,
{
    Task {
        value: None,
        editor,
    }
}

pub fn update<T, E>(value: T, editor: E) -> Task<T, E>
where
    E: Editor<Read = T>,
{
    Task {
        value: Some(TaskValue::Unstable(value)),
        editor,
    }
}
