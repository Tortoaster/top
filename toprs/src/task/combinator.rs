// use std::marker::PhantomData;
//
// use crate::editor::container::SequentialEditor;
// use crate::task::{Task, TaskValue};
//
// #[derive(Debug)]
// pub struct Then<T1, T2, F> {
//     task: T1,
//     f: F,
//     _then: PhantomData<T2>,
// }
//
// #[derive(Debug)]
// pub struct Action(pub &'static str);
//
// impl Action {
//     pub const OK: Self = Action("Ok");
//     pub const CANCEL: Self = Action("Cancel");
//     pub const YES: Self = Action("Yes");
//     pub const NO: Self = Action("No");
//     pub const NEXT: Self = Action("Next");
//     pub const PREVIOUS: Self = Action("Previous");
//     pub const FINISH: Self = Action("Finish");
//     pub const CONTINUE: Self = Action("Continue");
//     pub const NEW: Self = Action("New");
//     pub const EDIT: Self = Action("Edit");
//     pub const DELETE: Self = Action("Delete");
//     pub const REFRESH: Self = Action("Refresh");
//     pub const CLOSE: Self = Action("Close");
// }
//
// impl<T1, T2, F> Task for Then<T1, T2, F>
// where
//     T1: Task,
//     T2: Task + Send,
//     F: FnOnce(T1::Output) -> T2,
// {
//     type Output = T2::Output;
//     type Editor = SequentialEditor<T1::Editor, T2>;
//
//     fn get_value(self) -> TaskValue<Self::Output> {
//         todo!()
//     }
//
//     fn get_editor(self) -> Self::Editor {
//         SequentialEditor::default()
//             .with_editor(self.task.get_editor())
//             .with_button("Next".to_owned())
//     }
// }
//
// pub trait TaskExt: Task {
//     fn then<T2, F>(self, f: F) -> Then<Self, T2, F>
//     where
//         F: FnOnce(Self::Output) -> T2,
//         T2: Task,
//         Self: Sized,
//     {
//         Then {
//             task: self,
//             f,
//             _then: PhantomData,
//         }
//     }
// }
//
// impl<T> TaskExt for T where T: Task {}
