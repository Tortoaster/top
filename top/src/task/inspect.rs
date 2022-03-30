use std::fmt::Debug;

use async_trait::async_trait;
use either::Either;

use crate::component::event::{Event, Feedback, FeedbackHandler};
use crate::component::id::Id;
use crate::task::{Context, Error, Task, TaskValue};
use crate::viewer::generic::View;
use crate::viewer::Viewer;

/// Basic inspect (read-only interaction) task. Supports both reading. Use [`view`] to construct one.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Inspect<I, V> {
    viewer: Either<I, V>,
}

/// Show a value to the user. To use a custom editor, see [`view_with`].
#[inline]
pub fn view<I>(value: I) -> Inspect<I, I::Viewer>
where
    I: View,
{
    view_with(value)
}

/// Show a value to the user, through a custom editor.
#[inline]
pub fn view_with<V>(value: V::Input) -> Inspect<V::Input, V>
where
    V: Viewer,
{
    Inspect {
        viewer: Either::Left(value),
    }
}

#[async_trait]
impl<I, O, V> Task for Inspect<I, V>
where
    I: Clone + Debug + Send,
    O: Send + Sync,
    V: Viewer<Input = I, Output = O> + Send,
{
    type Value = O;

    async fn start<H: FeedbackHandler + Send>(
        &mut self,
        ctx: &mut Context<H>,
    ) -> Result<(), Error<H::Error>> {
        if let Either::Left(input) = &self.viewer {
            self.viewer = Either::Right(V::start(input.clone()))
        };

        let component = self
            .viewer
            .as_ref()
            .unwrap_right()
            .component(&mut ctx.components);

        let initial = Feedback::Replace {
            id: Id::ROOT,
            component,
        };

        ctx.feedback.send(initial).await?;
        Ok(())
    }

    async fn on_event<H: FeedbackHandler + Send>(
        &mut self,
        _event: Event,
        _ctx: &mut Context<H>,
    ) -> Result<TaskValue<Self::Value>, Error<H::Error>> {
        match &self.viewer {
            Either::Left(_) => Ok(TaskValue::Empty),
            Either::Right(viewer) => Ok(TaskValue::Stable(viewer.read())),
        }
    }

    async fn finish(self) -> TaskValue<Self::Value> {
        match self.viewer {
            Either::Left(_) => TaskValue::Empty,
            Either::Right(viewer) => TaskValue::Stable(viewer.read()),
        }
    }
}
