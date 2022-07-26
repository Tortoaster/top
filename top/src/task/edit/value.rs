use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use async_trait::async_trait;
use uuid::Uuid;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::share::{ShareRead, ShareWrite};
use crate::task::edit::form::Form;
use crate::task::{TaskValue, Value};

#[derive(Clone, Debug)]
pub struct EditValue<S> {
    id: Uuid,
    share: S,
    label: Option<String>,
}

impl<S> EditValue<S> {
    pub fn new(share: S) -> Self {
        EditValue {
            id: Uuid::new_v4(),
            share,
            label: None,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
}

#[async_trait]
impl<S> Value for EditValue<S>
where
    S: ShareRead + Send + Sync,
    S::Value: Clone + Send + Sync,
{
    type Output = S::Value;

    async fn value(self) -> TaskValue<Self::Output> {
        self.share.read().as_ref().clone()
    }
}

#[async_trait]
impl<S> Handler for EditValue<S>
where
    S: ShareWrite + Send + Sync,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match event {
            Event::Update { id, value } if id == self.id => match value.parse::<S::Value>() {
                Ok(value) => {
                    let feedback = self.share.write(TaskValue::Unstable(value)).await;
                    Feedback::from(Change::Valid { id })
                        .merged_with(feedback)
                        .unwrap()
                }
                Err(_) => {
                    let feedback = self.share.write(TaskValue::Empty).await;
                    Feedback::from(Change::Invalid { id })
                        .merged_with(feedback)
                        .unwrap()
                }
            },
            _ => Feedback::new(),
        }
    }
}

#[async_trait]
impl<S> Refresh for EditValue<S>
where
    S: ShareRead + Send + Sync,
    S::Value: Display + Send + Sync,
{
    async fn refresh(&self, id: Uuid) -> Feedback {
        if self.share.id() == id {
            match self.share.read().await.deref() {
                TaskValue::Stable(value) | TaskValue::Unstable(value) => {
                    Feedback::from(Change::UpdateValue {
                        id: self.id,
                        value: value.to_string(),
                    })
                }
                TaskValue::Empty => Feedback::from(Change::Invalid { id: self.id }),
            }
        } else {
            Feedback::new()
        }
    }
}

#[async_trait]
impl<S> ToHtml for EditValue<S>
where
    S: ShareRead + Send + Sync,
    S::Value: Form + Send,
{
    async fn to_html(&self) -> Html {
        S::Value::form(self.share.read().await, &self.id, &self.label).await
    }
}
