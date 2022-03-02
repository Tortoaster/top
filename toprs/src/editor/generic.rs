pub use toprs_derive::Edit;

use crate::editor::Editor;
use crate::prelude::{NumberField, TextField};

pub trait Edit {
    type Editor: Editor<Read = Self>;
}

impl Edit for String {
    type Editor = TextField;
}

impl Edit for i32 {
    type Editor = NumberField;
}
