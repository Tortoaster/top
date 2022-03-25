pub use top_derive::DefaultEditor;

use crate::editor::container::VecEditor;
use crate::editor::primitive::{BooleanEditor, NumberEditor, TextEditor};
use crate::editor::tuple::*;
use crate::editor::Editor;

/// Specifies the default editor for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`DefaultEditor`].
pub trait DefaultEditor: Sized {
    type Editor: Editor<Input = Self, Output = Self>;

    /// Specifies the default editor for this type.
    fn default_editor() -> Self::Editor;
}

impl DefaultEditor for String {
    type Editor = TextEditor;

    fn default_editor() -> Self::Editor {
        TextEditor::new()
    }
}

macro_rules! impl_default_editor_for_integer {
    ($($ty:ty),*) => {
        $(
            impl DefaultEditor for $ty {
                type Editor = NumberEditor<$ty>;

                fn default_editor() -> Self::Editor {
                    NumberEditor::new()
                }
            }
        )*
    };
}

impl_default_editor_for_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl DefaultEditor for bool {
    type Editor = BooleanEditor;

    fn default_editor() -> Self::Editor {
        BooleanEditor::new()
    }
}

impl DefaultEditor for () {
    type Editor = UnitEditor;

    fn default_editor() -> Self::Editor {
        UnitEditor
    }
}

macro_rules! impl_default_editor_for_tuple {
    ($name:ident<$($editor:ident),*>) => {
        impl<$($editor),*> DefaultEditor for ($($editor,)*)
        where
            $($editor: DefaultEditor),*
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

impl<T> DefaultEditor for Vec<T>
where
    T: DefaultEditor,
    <T as DefaultEditor>::Editor: Clone,
{
    type Editor = VecEditor<T::Editor>;

    fn default_editor() -> Self::Editor {
        VecEditor::new(T::default_editor())
    }
}
