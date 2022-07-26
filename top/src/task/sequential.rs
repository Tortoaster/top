// use std::ops::Deref;
//
// use async_trait::async_trait;
// use either::Either;
// use uuid::Uuid;
//
// use top_derive::html;
//
// use crate::html::event::{Change, Event, Feedback};
// use crate::html::{Handler, Html, Refresh, ToHtml};
// use crate::task::{TaskValue, Value};
//
// /// Basic sequential task. Consists of a current task, along with one or more [`Continuation`]s that
// /// decide when the current task should finish and what to do with the result.
// #[derive(Debug)]
// pub struct Sequential<T, U, C, F> {
//     id: Uuid,
//     current: Either<T, U>,
//     trigger: Trigger,
//     condition: C,
//     transform: F,
// }
//
// impl<T, U, C, F> Sequential<T, U, C, F>
// where
//     T: Value + Send + Sync,
//     T::Share: ShareRead + Clone + Send + Sync,
//     U: ToHtml + Send + Sync,
//     C: Fn(&TaskValue<<T::Share as ShareConsume>::Value>) -> bool + Send + Sync,
//     F: Fn(TaskValue<<T::Share as ShareConsume>::Value>) -> U + Send + Sync,
// {
//     async fn transform(&mut self) -> Result<Feedback, TransformError> {
//         match &self.current {
//             Either::Left(task) => {
//                 let share = task.share().await;
//                 if (self.condition)(share.read().await.deref()) {
//                     let next = (self.transform)(share.consume().await);
//                     let html = next.to_html().await;
//                     self.current = Either::Right(next);
//                     Ok(Feedback::from(Change::ReplaceContent { id: self.id, html }))
//                 } else {
//                     Err(TransformError::FalseCondition)
//                 }
//             }
//             Either::Right(_) => Err(TransformError::InvalidState),
//         }
//     }
// }
//
// enum TransformError {
//     InvalidState,
//     FalseCondition,
// }
//
// #[async_trait]
// impl<T, U, C, F> ToHtml for Sequential<T, U, C, F>
// where
//     T: ToHtml + Send + Sync,
//     U: ToHtml + Send + Sync,
//     C: Send + Sync,
//     F: Send + Sync,
// {
//     async fn to_html(&self) -> Html {
//         match &self.current {
//             Either::Left(task) => {
//                 let task = task.to_html().await;
//
//                 let buttons = match &self.trigger {
//                     Trigger::Button(button) => button.to_html().await,
//                     _ => Html::default(),
//                 };
//
//                 let id = self.id;
//
//                 html! {r#"
//                     <div id={id}>
//                         {task}
//                         {buttons}
//                     </div>
//                 "#}
//             }
//             Either::Right(task) => task.to_html().await,
//         }
//     }
// }
//
// #[async_trait]
// impl<T, U, C, F> Handler for Sequential<T, U, C, F>
// where
//     T: Value + Handler + Send + Sync,
//     T::Output: Clone + Send,
//     T::Share: ShareRead + Clone + Send + Sync,
//     U: ToHtml + Handler + Send + Sync,
//     C: Fn(&TaskValue<<T::Share as ShareConsume>::Value>) -> bool + Send + Sync,
//     F: Fn(TaskValue<<T::Share as ShareConsume>::Value>) -> U + Send + Sync,
// {
//     async fn on_event(&mut self, event: Event) -> Feedback {
//         match &mut self.current {
//             Either::Left(task) => {
//                 let feedback = task.on_event(event.clone()).await;
//
//                 match &self.trigger {
//                     Trigger::Update => self.transform().await.unwrap_or(feedback),
//                     Trigger::Button(action) => {
//                         if let Event::Press { id } = &event {
//                             if action.1 == *id {
//                                 self.transform().await.unwrap_or(feedback)
//                             } else {
//                                 feedback
//                             }
//                         } else {
//                             feedback
//                         }
//                     }
//                 }
//             }
//             Either::Right(task) => task.on_event(event).await,
//         }
//     }
// }
//
// #[async_trait]
// impl<T, U, C, F> Refresh for Sequential<T, U, C, F>
// where
//     T: Refresh + Send + Sync,
//     U: Refresh + Send + Sync,
//     C: Send + Sync,
//     F: Send + Sync,
// {
//     async fn refresh(&self, id: Uuid) -> Feedback {
//         match &self.current {
//             Either::Left(task) => task.refresh(id).await,
//             Either::Right(task) => task.refresh(id).await,
//         }
//     }
// }
//
// #[async_trait]
// impl<T, U, C, F> Value for Sequential<T, U, C, F>
// where
//     T: Value + Send + Sync,
//     T::Output: Clone + Send,
//     T::Share: ShareRead + Clone + Send + Sync,
//     U: Value + ToHtml + Send + Sync,
//     C: Fn(&TaskValue<<T::Share as ShareConsume>::Value>) -> bool + Send + Sync,
//     F: Fn(TaskValue<<T::Share as ShareConsume>::Value>) -> U + Send + Sync,
// {
//     type Output = U::Output;
//     type Share = ();
//
//     async fn share(&self) -> Self::Share {
//         ()
//     }
//
//     async fn value(self) -> TaskValue<Self::Output> {
//         match self.current {
//             Either::Left(_) => TaskValue::Empty,
//             Either::Right(t) => t.value().await,
//         }
//     }
// }
//
// #[derive(Debug)]
// pub enum Trigger {
//     /// Trigger as soon as possible.
//     Update,
//     /// Trigger when the user presses a button.
//     Button(Button),
// }
//
// /// Actions that are represented as buttons in the user interface, used in [`Continuation`]s. When
// /// the user presses the associated button, and the associated predicate in the continuation is met,
// /// the current task is consumed and the next task will be created from the resulting value.
// #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
// pub struct Button(&'static str, Uuid);
//
// impl Button {
//     pub fn new(label: &'static str) -> Self {
//         Button(label, Uuid::new_v4())
//     }
// }
//
// #[async_trait]
// impl ToHtml for Button {
//     async fn to_html(&self) -> Html {
//         html! {r#"
//             <button id="{self.1}" class="button is-link" type="button" onclick="press(this)">
//                 {self.0}
//             </button>
//         "#}
//     }
// }
//
// /// Adds the [`steps`] method to any task, allowing it to become a sequential task through the
// /// [`Steps`] builder.
// pub trait TaskSequentialExt: Value + Sized {
//     fn then<U, C, F>(
//         self,
//         trigger: Trigger,
//         condition: C,
//         transform: F,
//     ) -> Sequential<Self, U, C, F>
//     where
//         C: Fn(&TaskValue<Self::Output>) -> bool,
//         F: Fn(TaskValue<Self::Output>) -> U,
//     {
//         Sequential {
//             id: Uuid::new_v4(),
//             current: Either::Left(self),
//             trigger,
//             condition,
//             transform,
//         }
//     }
// }
//
// impl<T> TaskSequentialExt for T where T: Value {}
