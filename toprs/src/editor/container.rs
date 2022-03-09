// use std::marker::PhantomData;
//
// use crate::component::Component;
// use crate::editor::Editor;
// use crate::prelude::Task;
//
// #[derive(Debug)]
// pub struct SequentialEditor<E, T2> {
//     editor: Option<E>,
//     buttons: Vec<String>,
//     _then: PhantomData<T2>,
// }
//
// impl<E, T2> Default for SequentialEditor<E, T2> {
//     fn default() -> Self {
//         SequentialEditor {
//             editor: None,
//             buttons: Vec::new(),
//             _then: PhantomData,
//         }
//     }
// }
//
// impl<E, T2> SequentialEditor<E, T2> {
//     pub fn with_editor(mut self, editor: E) -> Self {
//         self.editor = Some(editor);
//         self
//     }
//
//     pub fn with_button(mut self, button: String) -> Self {
//         self.buttons.push(button);
//         self
//     }
// }
//
// impl<E, T2> Editor for SequentialEditor<E, T2>
// where
//     E: Editor,
//     T2: Task,
// {
//     type Output = E::Output;
//     type Input = E::Input;
//     type Error = E::Error;
//
//     fn new(&self) -> Component {
//         Component::Column(vec![
//             self.editor.as_ref().unwrap().reset(),
//             Component::Row(
//                 self.buttons
//                     .iter()
//                     .cloned()
//                     .map(|text| Component::Button {
//                         text,
//                         disabled: false,
//                     })
//                     .collect(),
//             ),
//         ])
//     }
//
//     fn finish(&self) -> Self::Output {
//         self.editor.as_ref().unwrap().finish()
//     }
//
//     fn write_value(&mut self, value: Self::Input) -> Result<(), Self::Error> {
//         self.editor.as_mut().unwrap().write_value(value)
//     }
// }
