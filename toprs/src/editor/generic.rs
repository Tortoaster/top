pub use toprs_derive::DefaultEditor;

use crate::editor::Editor;
use crate::prelude::{NumberEditor, TextEditor};

/// Specifies the default editor for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`DefaultEditor`].
pub trait DefaultEditor {
    // TODO: Generate entire editor, or just component?
    type Editor: Editor<Output = Self, Input = Self>;

    /// Specifies the default editor for this type.
    fn default_editor() -> Self::Editor;
}

impl DefaultEditor for String {
    type Editor = TextEditor;

    fn default_editor() -> Self::Editor {
        TextEditor::new()
    }
}

impl DefaultEditor for i32 {
    type Editor = NumberEditor;

    fn default_editor() -> Self::Editor {
        NumberEditor::new()
    }
}
