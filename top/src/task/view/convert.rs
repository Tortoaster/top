// use std::fmt::{Debug, Display};
//
// use async_trait::async_trait;
//
// use crate::html::{Html, ToHtml};
// use crate::task::tune::Tune;
// use crate::view::primitive::OutputViewer;
// use crate::view::Viewer;
//
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct DisplayViewer<T> {
//     view: OutputViewer<String>,
//     value: T,
// }
//
// impl<T> DisplayViewer<T> {
//     pub fn new(value: T) -> Self
//     where
//         T: Display,
//     {
//         DisplayViewer {
//             view: OutputViewer::new(value.to_string()),
//             value,
//         }
//     }
// }
//
// #[async_trait]
// impl<T> ToHtml for DisplayViewer<T>
// where
//     T: Send + Sync,
// {
//     async fn to_html(&self) -> Html {
//         self.view.to_html().await
//     }
// }
//
// impl<T> Viewer for DisplayViewer<T>
// where
//     T: Clone,
// {
//     type Value = T;
//
//     fn value(&self) -> Self::Value {
//         self.value.clone()
//     }
// }
//
// impl<T> Tune for DisplayViewer<T> {
//     type Tuner = <OutputViewer<String> as Tune>::Tuner;
//
//     fn tune(&mut self, tuner: Self::Tuner) {
//         self.view.tune(tuner);
//     }
// }
//
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct DebugViewer<T> {
//     view: OutputViewer<String>,
//     value: T,
// }
//
// impl<T> DebugViewer<T> {
//     pub fn new(value: T) -> Self
//     where
//         T: Debug,
//     {
//         DebugViewer {
//             view: OutputViewer::new(format!("{:?}", value)),
//             value,
//         }
//     }
// }
//
// #[async_trait]
// impl<T> ToHtml for DebugViewer<T>
// where
//     T: Send + Sync,
// {
//     async fn to_html(&self) -> Html {
//         self.view.to_html().await
//     }
// }
//
// impl<T> Viewer for DebugViewer<T>
// where
//     T: Clone,
// {
//     type Value = T;
//
//     fn value(&self) -> Self::Value {
//         self.value.clone()
//     }
// }
//
// impl<T> Tune for DebugViewer<T> {
//     type Tuner = <OutputViewer<String> as Tune>::Tuner;
//
//     fn tune(&mut self, tuner: Self::Tuner) {
//         self.view.tune(tuner);
//     }
// }
