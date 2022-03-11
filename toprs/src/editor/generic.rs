pub use toprs_derive::DefaultEditor;

use crate::editor::Editor;
use crate::prelude::{NumberEditor, TextEditor};

/// Specifies the default editor for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`DefaultEditor`].
pub trait DefaultEditor: Sized {
    type Editor: Editor<Output = Self>;

    /// Specifies the default editor for this type.
    fn default_editor(value: Option<Self>) -> Self::Editor;
}

impl DefaultEditor for String {
    type Editor = TextEditor;

    fn default_editor(value: Option<Self>) -> Self::Editor {
        match value {
            None => TextEditor::new(),
            Some(value) => TextEditor::with_value(value),
        }
    }
}

impl DefaultEditor for i32 {
    type Editor = NumberEditor;

    fn default_editor(value: Option<Self>) -> Self::Editor {
        match value {
            None => NumberEditor::new(),
            Some(value) => NumberEditor::with_value(value),
        }
    }
}
