use crate::editor::container::SequentialEditor;
use crate::task::{Task, TaskValue};
use std::marker::PhantomData;

pub trait TaskExt: Task {
    fn then<T, F>(self, f: F) -> Then<Self, T, F>
    where
        F: FnOnce(Self::Output) -> T,
        T: Task,
        Self: Sized,
    {
        Then {
            task: self,
            f,
            _then: PhantomData,
        }
    }
}

impl<T> TaskExt for T where T: Task {}

pub struct Then<T1, T2, F> {
    task: T1,
    f: F,
    _then: PhantomData<T2>,
}

impl<T1, T2, F> Task for Then<T1, T2, F>
where
    T1: Task,
    T2: Task,
    F: FnOnce(T1::Output) -> T2,
{
    type Output = T2::Output;
    type Editor = SequentialEditor<T1::Editor>;

    fn get_value(self) -> TaskValue<Self::Output> {
        (self.f)(self.task.get_value().unwrap()).get_value()
    }

    fn get_editor(self) -> Self::Editor {
        SequentialEditor::new(self.task.get_editor(), "Next".to_owned())
    }
}
