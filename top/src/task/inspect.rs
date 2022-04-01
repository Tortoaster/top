use std::fmt::Debug;

use async_trait::async_trait;
use either::Either;

use crate::event::{Event, Feedback, FeedbackHandler};
use crate::id::Id;
use crate::task::{Context, Task, TaskError, TaskResult, TaskValue};
use crate::viewer::generic::View;
use crate::viewer::Viewer;

/// Basic inspect (read-only interaction) task. Supports both reading. Use [`view`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Inspect<V>
where
    V: Viewer,
{
    viewer: Either<V::Input, V>,
}

/// Show a value to the user. To use a custom editor, see [`view_with`].
#[inline]
pub fn view<T>(value: T) -> Inspect<T::Viewer>
where
    T: View,
{
    view_with(value)
}

/// Show a value to the user, through a custom editor.
#[inline]
pub fn view_with<V>(value: V::Input) -> Inspect<V>
where
    V: Viewer,
{
    Inspect {
        viewer: Either::Left(value),
    }
}

#[async_trait]
impl<V> Task for Inspect<V>
where
    V: Viewer + Send,
    V::Input: Clone + Debug + Send,
    V::Output: Send + Sync,
{
    type Value = V::Output;

    async fn start<H>(&mut self, ctx: &mut Context<H>) -> Result<(), TaskError>
    where
        H: FeedbackHandler + Send,
    {
        if let Either::Left(input) = &self.viewer {
            self.viewer = Either::Right(V::start(input.clone()))
        };

        let component = self.viewer.as_ref().unwrap_right().component(&mut ctx.gen);

        let initial = Feedback::Replace {
            id: Id::ROOT,
            component,
        };

        ctx.feedback.send(initial).await?;
        Ok(())
    }

    async fn on_event<H>(&mut self, _event: Event, _ctx: &mut Context<H>) -> TaskResult<Self::Value>
    where
        H: FeedbackHandler + Send,
    {
        match &self.viewer {
            Either::Left(_) => Ok(TaskValue::Empty),
            Either::Right(viewer) => Ok(TaskValue::Stable(viewer.read())),
        }
    }
}
