// use std::fmt::Display;
// use std::str::FromStr;
//
// use top_derive::html;
//
// use crate::edit::primitive::InputEditor;
// use crate::edit::{Editor, EditorError};
// use crate::html::event::{Event, Feedback};
// use crate::html::id::Generator;
// use crate::html::{Html, ToHtml};
// use crate::task::tune::{InputTuner, Tune};
//
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct DisplayFromStrEditor<T> {
//     edit: InputEditor<T>,
// }
//
// impl<T> DisplayFromStrEditor<T>
// where
//     T: FromStr,
// {
//     pub fn new(value: Option<T>) -> Self {
//         let edit = match value {
//             None => match "".parse::<T>() {
//                 Ok(value) => InputEditor::new(value),
//                 Err(_) => InputEditor::empty(),
//             },
//             Some(value) => InputEditor::new(value),
//         };
//
//         DisplayFromStrEditor { edit }
//     }
// }
//
// impl<T> ToHtml for DisplayFromStrEditor<T>
// where
//     T: Display,
// {
//     fn to_html(&self) -> Html {
//         let value = self.edit.value.as_ref().map(ToString::to_string);
//         html! {r#"
//             <label for="{self.edit.id}" class="label">{self.edit.tuner.label}</label>
//             <input id="self.edit.id" class="input" value="{value}" onblur="update(this)"/>
//         "#}
//     }
// }
//
// impl<T> Editor for DisplayFromStrEditor<T>
// where
//     T: Clone + Display + FromStr,
// {
//     type Value = T;
//
//     fn start(&mut self, gen: &mut Generator) {
//         self.edit.start(gen)
//     }
//
//     fn on_event(&mut self, event: Event, gen: &mut Generator) -> Feedback {
//         self.edit.on_event(event, gen)
//     }
//
//     fn value(&self) -> Result<Self::Value, EditorError> {
//         self.edit.value()
//     }
// }
//
// impl<T> Tune for DisplayFromStrEditor<T> {
//     type Tuner = InputTuner;
//
//     fn tune(&mut self, tuner: Self::Tuner) {
//         self.edit.tune(tuner)
//     }
// }
