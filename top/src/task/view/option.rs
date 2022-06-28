use std::fmt::{Debug, Display};
use std::str::FromStr;

use async_trait::async_trait;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::prelude::TaskValue;
use crate::share::{OptionShare, Share, ShareId, ShareRead, ShareWrite, Shared};
use crate::task::view::value::ViewValue;
use crate::task::{OptionExt, Value};

#[derive(Clone, Debug)]
pub struct ViewOption<S> {
    share: OptionShare<S>,
    view: ViewValue<S>,
    container_id: Uuid,
}

impl<T> ViewOption<Shared<T>>
where
    T: Clone + Send,
{
    pub fn new(value: Option<T>) -> Self {
        let enabled = value.is_some();
        let share = OptionShare::new(Shared::new(value.into_unstable()), enabled);
        ViewOption::new_shared(share)
    }
}

impl<S> ViewOption<S>
where
    S: Share + Clone,
{
    pub fn new_shared(share: OptionShare<S>) -> Self {
        let view = ViewValue::new_shared(share.inner().clone());
        ViewOption {
            share,
            view,
            container_id: Uuid::new_v4(),
        }
    }
}

#[async_trait]
impl<S> Value for ViewOption<S>
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
impl<S> Handler for ViewOption<S>
where
    S: ShareId + ShareWrite + Clone + Send + Sync,
    S::Value: FromStr + Clone + Send,
    <S::Value as FromStr>::Err: Send,
{
    async fn on_event(&mut self, _event: Event) -> Feedback {
        Feedback::new()
    }
}

#[async_trait]
impl<S> Refresh for ViewOption<S>
where
    S: ShareRead + ShareId + Send + Sync,
    S::Value: ToHtml + Display + Clone + Send + Sync,
{
    async fn refresh(&self, id: Uuid) -> Feedback {
        if self.share.id() == id {
            if self.share.is_some().await {
                let feedback = Feedback::from(Change::ReplaceContent {
                    id: self.container_id,
                    html: filled_row(&self.view).await,
                });
                feedback.merged_with(self.view.refresh(id).await).unwrap()
            } else {
                Feedback::from(Change::ReplaceContent {
                    id: self.container_id,
                    html: empty_row().await,
                })
            }
        } else {
            Feedback::new()
        }
    }
}

#[async_trait]
impl<S> ToHtml for ViewOption<S>
where
    S: ShareRead + Send + Sync,
    S::Value: ToHtml + Display + Send + Sync,
{
    async fn to_html(&self) -> Html {
        let content = if self.share.is_some().await {
            filled_row(&self.view).await
        } else {
            empty_row().await
        };

        html! {r#"
            <div id="{self.container_id}">
                {content}
            </div>
        "#}
    }
}

async fn empty_row() -> Html {
    Html::default()
}

async fn filled_row(inner: &impl ToHtml) -> Html {
    html! {r#"<div class="box">{inner}</div>"#}
}
