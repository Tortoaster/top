use std::marker::PhantomData;

use crate::editor::container::SequentialEditor;
use crate::editor::{Editor, Report};
use crate::task::Task;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Then<T1, F, T2> {
    first: T1,
    f: F,
    _then: PhantomData<T2>,
}

impl<T1, E1, O1, F, T2, E2> Task for Then<T1, F, T2>
where
    T1: Task<Editor = E1>,
    E1: Editor<Output = Report<O1>> + Send,
    F: Fn(O1) -> T2 + Send,
    T2: Task<Editor = E2>,
    E2: Editor + Send,
{
    type Editor = SequentialEditor<E1, F, E2>;

    fn editor(self) -> Self::Editor {
        SequentialEditor::new(self.first.editor(), self.f)
    }
}

pub trait TaskExt: Task {
    fn then<E1, O1, F, T2>(self, f: F) -> Then<Self, F, T2>
    where
        Self: Task<Editor = E1> + Sized,
        E1: Editor<Output = Report<O1>>,
        F: Fn(O1) -> T2,
        T2: Task,
    {
        Then {
            first: self,
            f,
            _then: PhantomData,
        }
    }
}

impl<T> TaskExt for T where T: Task {}
