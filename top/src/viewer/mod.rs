use async_trait::async_trait;
use uuid::Uuid;

use crate::html::event::Feedback;
use crate::prelude::TaskValue;
use crate::share::SharedValue;

pub mod convert;
pub mod generic;
pub mod primitive;

/// Viewers describe how tasks should be displayed to the user.
#[async_trait]
pub trait Viewer {
    /// The type of data this viewer produces.
    type Value;
    type Share: SharedValue;

    async fn redraw(&self, id: Uuid) -> Feedback;

    fn share(&self) -> Self::Share;

    /// Get the current value of this viewer.
    async fn value(&self) -> TaskValue<Self::Value>;
}
