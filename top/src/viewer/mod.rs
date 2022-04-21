pub mod convert;
pub mod generic;
pub mod primitive;

/// Viewers describe how tasks should be displayed to the user.
pub trait Viewer {
    /// The type of data this viewer produces.
    type Value;

    // TODO: Allow borrow and consume
    /// Get the current value of this viewer.
    fn value(&self) -> Self::Value;
}
