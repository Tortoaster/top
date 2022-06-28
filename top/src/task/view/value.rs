use std::fmt::Display;

use async_trait::async_trait;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::prelude::TaskValue;
use crate::share::{Share, ShareId, ShareRead, Shared};
use crate::task::Value;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ViewValue<S> {
    id: Uuid,
    share: S,
    color: Color,
}

impl<T> ViewValue<Shared<T>>
where
    T: Clone + Send,
{
    pub fn new(value: T) -> Self {
        ViewValue::new_shared(Shared::new(TaskValue::Stable(value)))
    }
}

impl<S> ViewValue<S>
where
    S: Share,
{
    pub fn new_shared(share: S) -> Self {
        ViewValue {
            id: Uuid::new_v4(),
            share,
            color: Color::default(),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

#[async_trait]
impl<S> Value for ViewValue<S>
where
    S: Share + Clone + Send + Sync,
    S::Value: Send + Sync,
{
    type Output = S::Value;
    type Share = S;

    async fn share(&self) -> Self::Share {
        self.share.clone()
    }

    async fn value(self) -> TaskValue<Self::Output> {
        self.share.clone_value().await
    }
}

#[async_trait]
impl<S> Handler for ViewValue<S>
where
    S: Send,
{
    async fn on_event(&mut self, _event: Event) -> Feedback {
        Feedback::new()
    }
}

#[async_trait]
impl<S> Refresh for ViewValue<S>
where
    S: ShareId + ShareRead + Send + Sync,
    S::Value: ToHtml + Display + Send + Sync,
{
    async fn refresh(&self, id: Uuid) -> Feedback {
        if self.share.id() == id {
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
impl<S> ToHtml for ViewValue<S>
where
    S: ShareRead + Send + Sync,
    S::Value: ToHtml + Display + Send + Sync,
{
    async fn to_html(&self) -> Html {
        let value = self.share.read().await;
        html! {r#"
            <div id="{self.id}">
                <span style="color: {self.color};">{value}</span>
            </div>
        "#}
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Color {
    Black,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Brown,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

#[async_trait]
impl ToHtml for Color {
    async fn to_html(&self) -> Html {
        match self {
            Color::Black => Html("black".to_owned()),
            Color::White => Html("white".to_owned()),
            Color::Red => Html("red".to_owned()),
            Color::Orange => Html("orange".to_owned()),
            Color::Yellow => Html("yellow".to_owned()),
            Color::Green => Html("green".to_owned()),
            Color::Blue => Html("blue".to_owned()),
            Color::Purple => Html("purple".to_owned()),
            Color::Pink => Html("pink".to_owned()),
            Color::Brown => Html("brown".to_owned()),
        }
    }
}
