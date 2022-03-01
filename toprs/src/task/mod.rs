use crate::editor::Editor;
use crate::task::value::TaskValue;

pub mod combinator;
pub mod interaction;
pub mod value;

pub trait Task {
    type Output;
    // TODO: Lock output
    type Editor: Editor;

    fn get_value(self) -> TaskValue<Self::Output>;

    fn get_editor(self) -> Self::Editor;
}
