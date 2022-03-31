use crate::component::Component;
use crate::id::Generator;

pub mod convert;
pub mod generic;
pub mod primitive;

/// Viewers describe how tasks should be displayed to the user.
pub trait Viewer {
    /// The type of data this viewer starts with.
    type Input;
    /// The type of data this viewer produces, usually [`Self::Input`] for interaction tasks.
    type Output;

    fn start(value: Self::Input) -> Self;

    /// Create the user interface for this viewer.
    fn component(&self, ctx: &mut Generator) -> Component;

    // TODO: Allow borrow and consume
    /// Get the current value of this viewer.
    fn read(&self) -> Self::Output;
}
