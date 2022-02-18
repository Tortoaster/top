pub use toprs_derive::IntoEditor;

use crate::editor::Editor;

pub trait IntoEditor<E>
where
    E: Editor,
{
    fn into_editor() -> E;
}
