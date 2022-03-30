pub use top_derive::Edit;

use crate::editor::container::{OptionEditor, VecEditor};
use crate::editor::primitive::{BooleanEditor, CharEditor, FloatEditor, IntegerEditor, TextEditor};
use crate::editor::tuple::*;
use crate::editor::Editor;

/// Specifies the default editor for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`Edit`].
pub trait Edit: Sized {
    type Editor: Editor<Input = Self, Output = Self>;

    /// Specifies the default editor for this type.
    fn default_editor() -> Self::Editor;
}

impl Edit for String {
    type Editor = TextEditor;

    fn default_editor() -> Self::Editor {
        TextEditor::new()
    }
}

macro_rules! impl_edit_for_integer {
    ($($ty:ty),*) => {
        $(
            impl Edit for $ty {
                type Editor = IntegerEditor<$ty>;

                fn default_editor() -> Self::Editor {
                    IntegerEditor::new()
                }
            }
        )*
    };
}

impl_edit_for_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_edit_for_float {
    ($($ty:ty),*) => {
        $(
            impl Edit for $ty {
                type Editor = FloatEditor<$ty>;

                fn default_editor() -> Self::Editor {
                    FloatEditor::new()
                }
            }
        )*
    };
}

impl_edit_for_float!(f32, f64);

impl Edit for bool {
    type Editor = BooleanEditor;

    fn default_editor() -> Self::Editor {
        BooleanEditor::new()
    }
}

impl Edit for char {
    type Editor = CharEditor;

    fn default_editor() -> Self::Editor {
        CharEditor::new()
    }
}

impl Edit for () {
    type Editor = UnitEditor;

    fn default_editor() -> Self::Editor {
        UnitEditor
    }
}

macro_rules! impl_edit_for_tuple {
    ($name:ident<$($editor:ident),*>) => {
        impl<$($editor),*> Edit for ($($editor,)*)
        where
            $($editor: Edit),*
        {
            type Editor = $name<$($editor::Editor),*>;

            fn default_editor() -> Self::Editor {
                $name::new($($editor::default_editor()),*)
            }
        }
    }
}

impl_edit_for_tuple!(MonupleEditor<A>);
impl_edit_for_tuple!(CoupleEditor<A, B>);
impl_edit_for_tuple!(TripleEditor<A, B, C>);
impl_edit_for_tuple!(QuadrupleEditor<A, B, C, D>);
impl_edit_for_tuple!(QuintupleEditor<A, B, C, D, E>);
impl_edit_for_tuple!(SextupleEditor<A, B, C, D, E, F>);
impl_edit_for_tuple!(SeptupleEditor<A, B, C, D, E, F, G>);
impl_edit_for_tuple!(OctupleEditor<A, B, C, D, E, F, G, H>);
impl_edit_for_tuple!(NonupleEditor<A, B, C, D, E, F, G, H, I>);
impl_edit_for_tuple!(DecupleEditor<A, B, C, D, E, F, G, H, I, J>);
impl_edit_for_tuple!(UndecupleEditor<A, B, C, D, E, F, G, H, I, J, K>);
impl_edit_for_tuple!(DuodecupleEditor<A, B, C, D, E, F, G, H, I, J, K, L>);

impl<T> Edit for Vec<T>
where
    T: Edit,
    <T as Edit>::Editor: Clone,
{
    type Editor = VecEditor<T::Editor>;

    fn default_editor() -> Self::Editor {
        VecEditor::new(T::default_editor())
    }
}

impl<T> Edit for Option<T>
where
    T: Edit,
{
    type Editor = OptionEditor<T::Editor>;

    fn default_editor() -> Self::Editor {
        OptionEditor::new(T::default_editor())
    }
}
