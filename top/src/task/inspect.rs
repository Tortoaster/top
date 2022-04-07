use std::fmt::Debug;

use async_trait::async_trait;

use crate::event::{Event, Feedback, FeedbackHandler};
use crate::id::Id;
use crate::task::{Context, Task, TaskError, TaskResult, TaskValue};
use crate::viewer::generic::View;
use crate::viewer::Viewer;

/// Basic inspect (read-only interaction) task. Use [`view`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Inspect<V> {
    pub(crate) viewer: V,
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
    V: Viewer + Send,
{
    type Value = V::Output;

    async fn start<H>(&mut self, ctx: &mut Context<H>) -> Result<(), TaskError>
    where
        H: FeedbackHandler + Send,
    {
        let html = self.viewer.as_html();
        let feedback = Feedback::Replace { id: Id::ROOT, html };
        ctx.feedback.send(feedback).await?;
        Ok(())
    }

    async fn on_event<H>(&mut self, _event: Event, _ctx: &mut Context<H>) -> TaskResult<Self::Value>
    where
        H: FeedbackHandler + Send,
    {
        Ok(TaskValue::Stable(self.viewer.finish()))
    }
}
