use std::fmt::Debug;

use async_trait::async_trait;

use crate::html::event::{Event, Feedback};
use crate::html::{Html, ToHtml};
use crate::task::{Result, Task, TaskValue};
use crate::viewer::generic::View;
use crate::viewer::Viewer;

/// Basic inspect (read-only interaction) task. Use [`view`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Inspect<V> {
    pub(in crate::task) viewer: V,
}

/// Show a value to the user. To use a custom editor, see [`view_with`].
#[inline]
pub fn view<T>(value: T) -> Inspect<T::Viewer>
where
    T: View,
{
    view_with(value.view())
}

/// Show a value to the user, through a custom editor.
#[inline]
pub fn view_with<V>(viewer: V) -> Inspect<V> {
    Inspect { viewer }
}

#[async_trait]
impl<V> ToHtml for Inspect<V>
where
    V: ToHtml + Send + Sync,
{
    async fn to_html(&self) -> Html {
        self.viewer.to_html().await
    }
}

#[async_trait]
impl<V> Task for Inspect<V>
where
    V: Viewer + ToHtml + Send + Sync,
{
    type Value = V::Value;
    type Share = V::Share;

    async fn on_event(&mut self, event: Event) -> Result<Feedback> {
        match event {
            Event::Redraw { id } => Ok(self.viewer.redraw(id).await),
            _ => Ok(Feedback::new()),
        }
    }

    async fn share(&self) -> Self::Share {
        self.viewer.share()
    }

    async fn value(self) -> Result<TaskValue<Self::Value>> {
        Ok(self.viewer.value().await)
    }
}
