// use async_trait::async_trait;
// use uuid::Uuid;
//
// use top_derive::html;
//
// use crate::html::event::{Change, Event, Feedback};
// use crate::html::icon::Icon;
// use crate::html::{Handler, Html, ToHtml};
// use crate::prelude::TaskValue;
// use crate::task::tune::{ContentTune, Tune};
// use crate::task::Value;
//
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct VecEditor<E> {
//     /// Represents the list containing all elements.
//     group_id: Id,
//     /// Represents the plus button.
//     add_id: Id,
//     /// Represents of each of the elements with their respective identifiers.
//     rows: Vec<Row>,
//     editors: Vec<E>,
//     template: E,
// }
//
// impl<E> VecEditor<E> {
//     pub fn new(editors: Vec<E>, template: E) -> Self {
//         VecEditor {
//             group_id: Id::INVALID,
//             add_id: Id::INVALID,
//             rows: Vec::new(),
//             editors,
//             template,
//         }
//     }
// }
//
// impl<E> ToHtml for VecEditor<E>
// where
//     E: ToHtml,
// {
//     fn to_html(&self) -> Html {
//         let children: Html = self
//             .editors
//             .iter()
//             .zip(&self.rows)
//             .map(|(edit, row)| row.to_html(edit))
//             .collect();
//
//         let button = Row::add_button(self.add_id);
//
//         html! {r#"
//             <div class="column">
//                 <div id="{self.group_id}" class="column">
//                     {children}
//                 </div>
//                 {button}
//             </div>
//         "#}
//     }
// }
//
// impl<E> Editor for VecEditor<E>
// where
//     E: Editor + ToHtml + Clone,
// {
//     type Value = Vec<E::Value>;
//
//     fn start(&mut self, gen: &mut Generator) {
//         self.group_id = gen.next();
//         self.add_id = gen.next();
//         self.rows = self.editors.iter().map(|_| Row::new(gen)).collect();
//
//         for edit in &mut self.editors {
//             edit.start(gen);
//         }
//     }
//
//     fn on_event(&mut self, event: Event, gen: &mut Generator) -> Feedback {
//         match event {
//             Event::Press { id } if id == self.add_id => {
//                 // Add a new row
//                 let mut edit = self.template.clone();
//                 edit.start(gen);
//                 let row = Row::new(gen);
//                 let html = row.to_html(&edit);
//
//                 self.editors.push(edit);
//                 self.rows.push(row);
//
//                 Feedback::from(Change::AppendContent {
//                     id: self.group_id,
//                     html,
//                 })
//             }
//             Event::Press { id } if self.rows.iter().any(|row| row.sub_id == id) => {
//                 // Remove an existing row
//                 let index = self.rows.iter().position(|row| row.sub_id == id).unwrap();
//                 let Row { id, .. } = self.rows.remove(index);
//                 self.editors.remove(index);
//
//                 Feedback::from(Change::Remove { id })
//             }
//             _ => self
//                 .editors
//                 .iter_mut()
//                 .find_map(|edit| {
//                     let feedback = edit.on_event(event.clone(), gen);
//                     (!feedback.is_empty()).then(|| feedback)
//                 })
//                 .unwrap_or_default(),
//         }
//     }
//
//     fn value(&self) -> Result<Self::Value, EditorError> {
//         // TODO: Return all errors
//         self.editors
//             .iter()
//             .map(|edit| edit.value())
//             .collect::<Result<Vec<_>, _>>()
//     }
// }
//
// impl<E> ContentTune for VecEditor<E>
// where
//     E: Tune,
//     E::Tuner: Clone,
// {
//     type ContentTuner = E::Tuner;
//
//     fn tune_content(&mut self, tuner: Self::ContentTuner) {
//         self.editors
//             .iter_mut()
//             .for_each(|choice| choice.tune(tuner.clone()));
//     }
// }
