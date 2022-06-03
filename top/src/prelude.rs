// pub use crate::viewer::convert::{DebugViewer, DisplayViewer};
pub use crate::share::{Share, SharedReadMapExt};
pub use crate::task::{Task, TaskValue};
// pub use crate::editor::convert::DisplayFromStrEditor;
pub use crate::task::inspect::{view, view_shared, view_with};
// pub use crate::task::interact::{choose, choose_with, edit, edit_with, enter};
pub use crate::task::interact::{edit, edit_shared, edit_with, enter};
pub use crate::task::parallel::TaskParallelExt;
pub use crate::task::sequential::{Button, TaskSequentialExt, Trigger};
pub use crate::task::tune::{Color, InputTuner, OutputTuner};

pub mod derive {
    pub use crate::editor::generic::Edit;
    pub use crate::editor::Editor;
    pub use crate::html::event::{Change, Event};
}
