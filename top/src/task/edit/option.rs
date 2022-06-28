use std::fmt::Display;
use std::str::FromStr;

use async_trait::async_trait;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::icon::Icon;
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::prelude::TaskValue;
use crate::share::{OptionShare, Share, ShareId, ShareRead, ShareWrite, Shared};
use crate::task::edit::form::Form;
use crate::task::edit::value::EditValue;
use crate::task::{OptionExt, Value};

#[derive(Clone, Debug)]
pub struct EditOption<S> {
    share: OptionShare<S>,
    edit: EditValue<S>,
    container_id: Uuid,
    button_id: Uuid,
}

impl<T> EditOption<Shared<T>>
where
    T: Clone + Send,
{
    pub fn new(value: Option<T>) -> Self {
        let enabled = value.is_some();
        let share = OptionShare::new(Shared::new(value.into_unstable()), enabled);
        EditOption::new_shared(share)
    }
}

impl<S> EditOption<S>
where
    S: Share + Clone,
{
    pub fn new_shared(share: OptionShare<S>) -> Self {
        let edit = EditValue::new_shared(share.inner().clone());
        EditOption {
            share,
            edit,
            container_id: Uuid::new_v4(),
            button_id: Uuid::new_v4(),
        }
    }
}

#[async_trait]
impl<S> Value for EditOption<S>
where
    S: Share + Clone + Send + Sync,
{
    type Output = Option<S::Value>;
    type Share = OptionShare<S>;

    async fn share(&self) -> Self::Share {
        self.share.clone()
    }

    async fn value(self) -> TaskValue<Self::Output> {
        self.share.clone_value().await
    }
}

#[async_trait]
impl<S> Handler for EditOption<S>
where
    S: ShareId + ShareWrite + Clone + Send + Sync,
    S::Value: Form + FromStr + Clone + Send,
    <S::Value as FromStr>::Err: Send,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match event {
            Event::Press { id } if id == self.button_id => {
                if self.share.is_some().await {
                    let feedback = self.share.disable().await;
                    let html = add_row(self.button_id).await;
                    Feedback::from(Change::ReplaceContent {
                        id: self.container_id,
                        html,
                    })
                    .merged_with(feedback)
                    .unwrap()
                } else {
                    let feedback = self.share.enable().await;
                    let html = remove_row(self.button_id, &self.edit).await;
                    Feedback::from(Change::ReplaceContent {
                        id: self.container_id,
                        html,
                    })
                    .merged_with(feedback)
                    .unwrap()
                }
            }
            _ => {
                if self.share.is_some().await {
                    self.edit.on_event(event).await
                } else {
                    Feedback::new()
                }
            }
        }
    }
}

#[async_trait]
impl<S> Refresh for EditOption<S>
where
    S: ShareRead + ShareId + Send + Sync,
    S::Value: Form + Display + Clone + Send + Sync,
{
    async fn refresh(&self, id: Uuid) -> Feedback {
        if self.share.id() == id {
            if self.share.is_some().await {
                let html = remove_row(self.button_id, &self.edit).await;
                let feedback = Feedback::from(Change::ReplaceContent {
                    id: self.container_id,
                    html,
                });
                feedback.merged_with(self.edit.refresh(id).await).unwrap()
            } else {
                let html = add_row(self.button_id).await;
                Feedback::from(Change::ReplaceContent {
                    id: self.container_id,
                    html,
                })
            }
        } else {
            Feedback::new()
        }
    }
}

#[async_trait]
impl<S> ToHtml for EditOption<S>
where
    S: ShareRead + Send + Sync,
    S::Value: ToHtml + Form + Send,
{
    async fn to_html(&self) -> Html {
        let content = if self.share.is_some().await {
            remove_row(self.button_id, &self.edit).await
        } else {
            add_row(self.button_id).await
        };

        html! {r#"
            <div id="{self.container_id}">
                {content}
            </div>
        "#}
    }
}

async fn add_row(id: Uuid) -> Html {
    html! {r#"
        <button id="{id}" class="button is-outlined" type="button" onclick="press(this)">
            {Icon::Plus}
        </button>
    "#}
}

async fn remove_row(id: Uuid, inner: &impl ToHtml) -> Html {
    html! {r#"
        <div class="level">
            {inner}
            <button id="{id}" class="button is-outlined" type="button" onclick="press(this)">
                {Icon::Minus}
            </button>
        </div>
    "#}
}
