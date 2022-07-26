use std::collections::BTreeSet;
use std::fmt::Display;

use async_trait::async_trait;
use uuid::Uuid;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::share::ShareRead;
use crate::task::{TaskValue, Value};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ViewDisplay<S> {
    id: Uuid,
    share: S,
}

impl<S> ViewDisplay<S> {
    pub fn new(share: S) -> Self {
        ViewDisplay {
            id: Uuid::new_v4(),
            share,
        }
    }
}

#[async_trait]
impl<S> Value for ViewDisplay<S>
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
impl<S> Handler for ViewDisplay<S>
where
    S: Send,
{
    async fn on_event(&mut self, _event: Event) -> Feedback {
        Feedback::new()
    }
}

#[async_trait]
impl<S> Refresh for ViewDisplay<S>
where
    S: ShareRead + Send + Sync,
    S::Value: Display + Send + Sync,
{
    async fn refresh(&self, ids: &BTreeSet<Uuid>) -> Feedback {
        if self.share.updated(&ids) {
            Feedback::from(Change::Replace {
                id: self.id,
                html: self.to_html().await,
            })
        } else {
            Feedback::new()
        }
    }
}

#[async_trait]
impl<S> ToHtml for ViewDisplay<S>
where
    S: ShareRead + Send + Sync,
    S::Value: Display + Send + Sync,
{
    async fn to_html(&self) -> Html {
        Html(format!(
            r#"<div id="{}"><span>{}</span></div>"#,
            self.id,
            self.share
                .read()
                .as_ref()
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default()
        ))
    }
}
