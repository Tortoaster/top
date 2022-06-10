use std::fmt::Debug;

use async_trait::async_trait;

use crate::html::event::{Event, Feedback};
use crate::html::{Handler, Html, ToHtml};
use crate::share::SharedRead;
use crate::task::{TaskValue, Value};
use crate::viewer::generic::{SharedView, View};
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

#[inline]
pub fn view_shared<S>(share: S) -> Inspect<<S::Value as SharedView<S>>::Viewer>
where
    S: SharedRead,
    S::Value: SharedView<S>,
{
    view_with(<S::Value>::view_shared(share))
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
impl<V> Handler for Inspect<V>
where
    V: Viewer + Send + Sync,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match event {
            Event::Redraw { id } => self.viewer.redraw(id).await,
            _ => Feedback::new(),
        }
    }
}

#[async_trait]
impl<V> Value for Inspect<V>
where
    V: Viewer + ToHtml + Send + Sync,
{
    type Output = V::Value;
    type Share = V::Share;

    async fn share(&self) -> Self::Share {
        self.viewer.share()
    }

    async fn value(self) -> TaskValue<Self::Output> {
        self.viewer.value().await
    }
}
