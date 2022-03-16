pub use toprs_derive::DefaultEditor;

use crate::editor::{Editor, Report};
use crate::prelude::{NumberEditor, TextEditor};

/// Specifies the default editor for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`DefaultEditor`].
pub trait DefaultEditor: Sized {
    type Editor: Editor<Input = Self, Output = Report<Self>>;

    /// Specifies the default editor for this type.
    fn default_editor() -> Self::Editor;
}

impl DefaultEditor for String {
    type Editor = TextEditor;

    fn default_editor() -> Self::Editor {
        TextEditor::default()
    }
}

impl DefaultEditor for i32 {
    type Editor = NumberEditor;

    fn default_editor() -> Self::Editor {
        NumberEditor::default()
    }
}
