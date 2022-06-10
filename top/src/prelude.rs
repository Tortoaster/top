pub use crate::editor::generic::{edit, edit_shared, enter};
pub use crate::share::{Shared, SharedReadMapExt};
pub use crate::task::parallel::TaskParallelExt;
pub use crate::task::sequential::{Button, TaskSequentialExt, Trigger};
pub use crate::task::TaskValue;
pub use crate::viewer::generic::{view, view_shared};

pub mod derive {
    pub use crate::editor::generic::Edit;
    pub use crate::html::event::{Change, Event};
}
