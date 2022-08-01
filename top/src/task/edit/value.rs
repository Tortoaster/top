use std::collections::BTreeSet;
use std::fmt::Display;

use async_trait::async_trait;
use uuid::Uuid;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::share::{ShareRead, ShareUpdate, ShareWrite};
use crate::task::edit::form::{FromForm, IntoForm};
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
    S: ShareWrite + ShareUpdate + Send + Sync,
    S::Value: FromForm,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match event {
            Event::Update { id, value } if id == self.id => {
                let value = S::Value::from_form(value);
                let change = match value {
                    TaskValue::Stable(_) | TaskValue::Unstable(_) | TaskValue::Empty => {
                        Change::Valid { id }
                    }
                    TaskValue::Error(_) => Change::Invalid { id },
                };
                self.share.write(value);
                let feedback = Feedback::update_share(self.share.id());
                feedback.merged_with(Feedback::from(change)).unwrap()
            }
            _ => Feedback::new(),
        }
    }
}

#[async_trait]
impl<S> Refresh for EditValue<S>
where
    S: ShareRead + ShareUpdate + Send + Sync,
    // TODO: Don't use Display, use IntoForm
    S::Value: Display + Send + Sync,
{
    async fn refresh(&mut self, ids: &BTreeSet<Uuid>) -> Feedback {
        if self.share.updated(ids) {
            match self.share.read().as_ref() {
                TaskValue::Stable(value) | TaskValue::Unstable(value) => {
                    Feedback::from(Change::UpdateValue {
                        id: self.id,
                        value: value.to_string(),
                    })
                }
                TaskValue::Error(_) => Feedback::from(Change::Invalid { id: self.id }),
                TaskValue::Empty => Feedback::from(Change::UpdateValue {
                    id: self.id,
                    value: String::new(),
                }),
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
    S::Value: IntoForm + Send,
{
    async fn to_html(&self) -> Html {
        S::Value::into_form(
            self.share.read().as_ref(),
            &self.id,
            self.label.as_deref().unwrap_or_default(),
        )
    }
}
