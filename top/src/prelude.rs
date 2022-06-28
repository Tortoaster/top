pub use crate::share::{Shared, SharedReadMapExt};
pub use crate::task::edit::{edit, edit_share, enter};
pub use crate::task::parallel::TaskParallelExt;
pub use crate::task::sequential::{Button, TaskSequentialExt, Trigger};
pub use crate::task::view::value::Color;
pub use crate::task::view::{view, view_shared};
pub use crate::task::TaskValue;

pub mod derive {
    pub use crate::html::event::{Change, Event};
    pub use crate::task::edit::Edit;
}
