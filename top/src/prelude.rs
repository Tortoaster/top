pub use crate::editor::convert::DisplayFromStrEditor;
pub use crate::task::inspect::{view, view_with};
pub use crate::task::interact::{choose, choose_with, edit, edit_with, enter};
pub use crate::task::parallel::TaskParallelExt;
pub use crate::task::sequential::{has_value, if_stable, if_unstable, if_value};
pub use crate::task::sequential::{Action, TaskSequentialExt};
pub use crate::task::tune::{Color, InputTuner, OutputTuner};
pub use crate::task::Task;
pub use crate::viewer::convert::{DebugViewer, DisplayViewer};

pub mod derive {
    pub use crate::editor::generic::Edit;
    pub use crate::editor::{Editor, EditorError};
    pub use crate::html::event::{Change, Event};
    pub use crate::html::id::{Generator, Id};
}
