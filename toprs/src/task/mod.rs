use crate::editor::Editor;

pub mod combinator;
pub mod interaction;
pub mod value;

pub trait Task {
    type Editor: Editor + Send;

    fn editor(self) -> Self::Editor;
}
