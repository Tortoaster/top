use crate::editor::Editor;

pub mod interaction;

#[derive(Debug)]
pub struct TaskId(usize);

#[derive(Debug)]
pub struct Task<T, E> {
    value: Option<TaskValue<T>>,
    pub editor: E,
}

impl<T, E> Task<T, E> where E: Editor<Read = T> {}

#[derive(Debug)]
pub enum TaskValue<T> {
    Stable(T),
    Unstable(T),
}
