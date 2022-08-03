use std::collections::BTreeSet;

use async_trait::async_trait;
use futures::future;
use uuid::Uuid;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::share::{ShareChildren, ShareRead, ShareUpdate, ShareWrite};
use crate::task::view::view_shared;
use crate::task::view::view_shared::ViewShared;
use crate::task::{TaskValue, Value};

#[derive(Clone, Debug)]
pub struct ViewVec<S, T> {
    container_id: Uuid,
    share: S,
    tasks: Vec<T>,
}

impl<S, T> ViewVec<S, T>
where
    S: ShareChildren,
    S::Child: ShareRead<Value = T::Output> + Clone,
    T: Value,
    T::Output: ViewShared<S::Child, Task = T>,
{
    pub fn new(share: S) -> Self {
        let tasks: Vec<T> = share.children().iter().cloned().map(view_shared).collect();
        ViewVec {
            container_id: Uuid::new_v4(),
            share,
            tasks,
        }
    }
}

#[async_trait]
impl<S, T> Value for ViewVec<S, T>
where
    S: ShareRead + Send + Sync,
    S::Value: Clone,
    T: Send + Sync,
{
    type Output = S::Value;

    async fn value(&self) -> TaskValue<Self::Output> {
        self.share.read().as_ref().clone()
    }
}

#[async_trait]
impl<S, T> Handler for ViewVec<S, T>
where
    S: ShareChildren + Send,
    S::Child: ShareWrite,
    T: Value + Handler + Send + Sync,
    T::Output: Clone,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match event {
            _ => future::join_all(
                self.tasks
                    .iter_mut()
                    .map(|task| task.on_event(event.clone())),
            )
            .await
            .into_iter()
            .collect(),
        }
    }
}

#[async_trait]
impl<S, T> Refresh for ViewVec<S, T>
where
    S: ShareChildren + ShareUpdate + Send + Sync,
    S::Child: ShareRead + Clone,
    <S::Child as ShareRead>::Value: ViewShared<S::Child, Task = T>,
    T: ToHtml + Send + Sync,
{
    async fn refresh(&mut self, ids: &BTreeSet<Uuid>) -> Feedback {
        if self.share.updated(&ids) {
            self.tasks = self
                .share
                .children()
                .iter()
                .cloned()
                .map(view_shared)
                .collect();
            Feedback::from(Change::Replace {
                id: self.container_id,
                html: self.to_html().await,
            })
        } else {
            Feedback::new()
        }
    }
}

#[async_trait]
impl<S, T> ToHtml for ViewVec<S, T>
where
    S: Send + Sync,
    T: ToHtml + Send + Sync,
{
    async fn to_html(&self) -> Html {
        let children: Html = future::join_all(self.tasks.iter().map(ToHtml::to_html))
            .await
            .into_iter()
            .collect();

        Html(format!(
            r#"<div id="{}" class="column">{children}</div>"#,
            self.container_id
        ))
    }
}
