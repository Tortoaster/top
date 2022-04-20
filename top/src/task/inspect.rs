use std::fmt::Debug;

use async_trait::async_trait;

use crate::html::event::Event;
use crate::html::id::Generator;
use crate::html::{Html, ToHtml};
use crate::task::{Context, Task, TaskError, TaskResult, TaskValue};
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
impl<V> Task for Inspect<V>
where
    V: Viewer + ToHtml + Send,
{
    type Value = V::Value;

    async fn start(&mut self, _gen: &mut Generator) -> Result<Html, TaskError> {
        Ok(self.viewer.to_html())
    }

    async fn on_event(&mut self, _event: Event, _ctx: &mut Context) -> TaskResult<Self::Value> {
        Ok(TaskValue::Stable(self.viewer.finish()))
    }
}
