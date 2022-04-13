use std::fmt::Debug;

use async_trait::async_trait;

use crate::event::{Event, Feedback};
use crate::html::{AsHtml, Div, DivType};
use crate::id::Id;
use crate::task::{Context, Task, TaskError, TaskResult, TaskValue};
use crate::viewer::generic::View;
use crate::viewer::Viewer;

/// Basic inspect (read-only interaction) task. Use [`view`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Inspect<V> {
    id: Id,
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
    Inspect {
        id: Id::INVALID,
        viewer,
    }
}

#[async_trait]
impl<V> Task for Inspect<V>
where
    V: Viewer + AsHtml + Send,
{
    type Value = V::Value;

    async fn start(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        self.id = ctx.gen.next();

        let html = Div::new(vec![self.viewer.as_html()])
            .with_id(self.id)
            .with_div_type(DivType::Section)
            .as_html();
        let feedback = Feedback::Insert { id: Id::ROOT, html };

        ctx.feedback.send(feedback).await?;

        Ok(())
    }

    async fn on_event(&mut self, _event: Event, _ctx: &mut Context) -> TaskResult<Self::Value> {
        Ok(TaskValue::Stable(self.viewer.finish()))
    }

    async fn finish(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        ctx.feedback.send(Feedback::Remove { id: self.id }).await?;

        Ok(())
    }
}
