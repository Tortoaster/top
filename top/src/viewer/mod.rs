use crate::html::AsHtml;

pub mod convert;
pub mod generic;
pub mod primitive;

/// Viewers describe how tasks should be displayed to the user.
pub trait Viewer: AsHtml {
    /// The type of data this viewer starts with.
    type Input;
    /// The type of data this viewer produces, usually [`Self::Input`] for interaction tasks.
    type Output;

    fn start(value: Self::Input) -> Self;

    // TODO: Allow borrow and consume
    /// Get the current value of this viewer.
    fn finish(&self) -> Self::Output;
}
