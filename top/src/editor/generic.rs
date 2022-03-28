pub use top_derive::Edit;

use crate::editor::container::{OptionEditor, VecEditor};
use crate::editor::primitive::{BooleanEditor, NumberEditor, TextEditor};
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

macro_rules! impl_default_editor_for_integer {
    ($($ty:ty),*) => {
        $(
            impl Edit for $ty {
                type Editor = NumberEditor<$ty>;

                fn default_editor() -> Self::Editor {
                    NumberEditor::new()
                }
            }
        )*
    };
}

impl_default_editor_for_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Edit for bool {
    type Editor = BooleanEditor;

    fn default_editor() -> Self::Editor {
        BooleanEditor::new()
    }
}

impl Edit for () {
    type Editor = UnitEditor;

    fn default_editor() -> Self::Editor {
        UnitEditor
    }
}

macro_rules! impl_default_editor_for_tuple {
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

impl_default_editor_for_tuple!(MonupleEditor<A>);
impl_default_editor_for_tuple!(CoupleEditor<A, B>);
impl_default_editor_for_tuple!(TripleEditor<A, B, C>);
impl_default_editor_for_tuple!(QuadrupleEditor<A, B, C, D>);
impl_default_editor_for_tuple!(QuintupleEditor<A, B, C, D, E>);
impl_default_editor_for_tuple!(SextupleEditor<A, B, C, D, E, F>);
impl_default_editor_for_tuple!(SeptupleEditor<A, B, C, D, E, F, G>);
impl_default_editor_for_tuple!(OctupleEditor<A, B, C, D, E, F, G, H>);
impl_default_editor_for_tuple!(NonupleEditor<A, B, C, D, E, F, G, H, I>);
impl_default_editor_for_tuple!(DecupleEditor<A, B, C, D, E, F, G, H, I, J>);
impl_default_editor_for_tuple!(UndecupleEditor<A, B, C, D, E, F, G, H, I, J, K>);
impl_default_editor_for_tuple!(DuodecupleEditor<A, B, C, D, E, F, G, H, I, J, K, L>);

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
