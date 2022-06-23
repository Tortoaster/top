use async_trait::async_trait;
use futures::lock::MutexGuard;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::icon::Icon;
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::share::{Share, ShareRead, Shared};
use crate::task::edit::Edit;
use crate::task::{OptionExt, TaskValue, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditOption<S>
where
    S: Share,
{
    edit: Edit<S>,
    share: SharedOption<S>,
    id: Uuid,
    /// Represents the plus button if there is no value present.
    add_id: Uuid,
    /// Represents the row containing the edit and the minus button if a value is present.
    row: Row,
    /// True if this edit contains a value, false otherwise.
}

impl<T> EditOption<Shared<T>> {
    pub fn new(value: Option<T>) -> Self {
        let enabled = value.is_some();
        EditOption::new_shared(Shared::new(value.into_unstable()), enabled)
    }
}

impl<S> EditOption<S>
where
    S: ShareRead,
{
    pub fn new_shared(share: S, enabled: bool) -> Self {
        EditOption {
            edit: Edit::new_shared(share),
            id: Uuid::new_v4(),
            add_id: Uuid::new_v4(),
            row: Row::new(),
            enabled,
        }
    }
}

#[async_trait]
impl<S> Value for EditOption<S>
where
    S: Value + Send + Sync,
{
    type Output = Option<S::Output>;
    type Share = S::Share;

    async fn share(&self) -> Self::Share {
        self.editor.share().await
    }

    async fn value(self) -> TaskValue<Self::Output> {
        if self.enabled {
            self.editor.value().await.map(Option::Some)
        } else {
            TaskValue::Empty
        }
    }
}

#[async_trait]
impl<S> Handler for EditOption<S>
where
    S: ToHtml + Handler + Send + Sync,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match event {
            Event::Press { id } if id == self.add_id && !self.enabled => {
                // Add value
                let html = self.row.to_html(&mut self.editor).await;
                self.enabled = true;

                Feedback::from(Change::ReplaceContent { id: self.id, html })
            }
            Event::Press { id } if id == self.row.sub_id && self.enabled => {
                // Remove value

                let html = Row::add_button(self.add_id).await;
                self.enabled = false;

                Feedback::from(Change::ReplaceContent { id: self.id, html })
            }
            _ => {
                if self.enabled {
                    self.editor.on_event(event).await
                } else {
                    Feedback::new()
                }
            }
        }
    }
}

#[async_trait]
impl<S> Refresh for EditOption<S> {
    async fn refresh(&self, id: Uuid) -> Feedback {
        todo!()
    }
}

#[async_trait]
impl<S> ToHtml for EditOption<S>
where
    S: ToHtml + Send + Sync,
{
    async fn to_html(&self) -> Html {
        let content = if self.enabled {
            self.row.to_html(&self.edit).await
        } else {
            Row::add_button(self.add_id).await
        };

        html! {r#"
            <div id="{self.id}">
                {content}
            </div>
        "#}
    }
}

#[derive(Clone, Debug)]
pub struct SharedOption<S> where S: Share {
    share: S,
    value: Shared<S::Value>,
    enabled: bool,
}

impl<S> Share for SharedOption<S>
where
    S: Share,
{
    type Value = Option<S::Value>;

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        if self.enabled {
            self.share.clone_value().await.map(Option::Some)
        } else {
            TaskValue::Unstable(None)
        }
    }
}

impl<S> ShareRead for SharedOption<S>
where
    S: ShareRead
{
    async fn read(&self) -> MutexGuard<'_, TaskValue<Self::Value>> {
        if self.enabled {
            let x = self.share.read().await
        } else {
            TaskValue::Unstable(None)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    id: Uuid,
    sub_id: Uuid,
}

impl Row {
    fn new() -> Self {
        Row {
            id: Uuid::new_v4(),
            sub_id: Uuid::new_v4(),
        }
    }

    /// Creates a row consisting of the edit and a button to remove it.
    async fn to_html(&self, content: &impl ToHtml) -> Html {
        html! {r#"
            <div id="{self.id}" class="level">
                {content}
                <button id="{self.sub_id}" class="button is-outlined" type="button" onclick="press(this)">
                    {Icon::Minus}
                </button>
            </div>
        "#}
    }

    async fn add_button(id: Uuid) -> Html {
        html! {r#"
            <button id="{id}" class="button is-outlined" type="button" onclick="press(this)">
                {Icon::Plus}
            </button>
        "#}
    }
}
