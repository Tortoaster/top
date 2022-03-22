pub use toprs_derive::DefaultEditor;

use crate::editor::primitive::BooleanEditor;
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

macro_rules! impl_default_editor_for_primitive {
    ($($ty:ty),*) => {
        $(
            impl DefaultEditor for $ty {
                type Editor = NumberEditor<$ty>;

                fn default_editor() -> Self::Editor {
                    NumberEditor::default()
                }
            }
        )*
    };
}

impl_default_editor_for_primitive!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl DefaultEditor for bool {
    type Editor = BooleanEditor;

    fn default_editor() -> Self::Editor {
        BooleanEditor::default()
    }
}
