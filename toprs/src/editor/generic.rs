pub use toprs_derive::DefaultEditor;

use crate::editor::primitive::{BooleanEditor, NumberEditor, TextEditor};
use crate::editor::tuple::*;
use crate::editor::{Editor, Report};

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
    ($name:ident<$($editor:ident, $output:ident),*>) => {
        impl<$($editor, $output),*> DefaultEditor for ($($editor,)*)
        where
            $(
                $editor: DefaultEditor<Editor = $output> + Clone + Default,
                $output: Editor<Input = $editor, Output = Report<$editor>>
            ),*
        {
            type Editor = $name<$($editor::Editor, $editor),*>;

            fn default_editor() -> Self::Editor {
                $name::new($($editor::default_editor()),*)
            }
        }
    }
}

impl_default_editor_for_tuple!(MonupleEditor<A, AEditor>);
impl_default_editor_for_tuple!(CoupleEditor<A, AEditor, B, BEditor>);
impl_default_editor_for_tuple!(TripleEditor<A, AEditor, B, BEditor, C, CEditor>);
impl_default_editor_for_tuple!(QuadrupleEditor<A, AEditor, B, BEditor, C, CEditor, D, DEditor>);
impl_default_editor_for_tuple!(QuintupleEditor<A, AEditor, B, BEditor, C, CEditor, D, DEditor, E, EEditor>);
impl_default_editor_for_tuple!(SextupleEditor<A, AEditor, B, BEditor, C, CEditor, D, DEditor, E, EEditor, F, FEditor>);
impl_default_editor_for_tuple!(SeptupleEditor<A, AEditor, B, BEditor, C, CEditor, D, DEditor, E, EEditor, F, FEditor, G, GEditor>);
impl_default_editor_for_tuple!(OctupleEditor<A, AEditor, B, BEditor, C, CEditor, D, DEditor, E, EEditor, F, FEditor, G, GEditor, H, HEditor>);
impl_default_editor_for_tuple!(NonupleEditor<A, AEditor, B, BEditor, C, CEditor, D, DEditor, E, EEditor, F, FEditor, G, GEditor, H, HEditor, I, IEditor>);
impl_default_editor_for_tuple!(DecupleEditor<A, AEditor, B, BEditor, C, CEditor, D, DEditor, E, EEditor, F, FEditor, G, GEditor, H, HEditor, I, IEditor, J, JEditor>);
impl_default_editor_for_tuple!(UndecupleEditor<A, AEditor, B, BEditor, C, CEditor, D, DEditor, E, EEditor, F, FEditor, G, GEditor, H, HEditor, I, IEditor, J, JEditor, K, KEditor>);
impl_default_editor_for_tuple!(DuodecupleEditor<A, AEditor, B, BEditor, C, CEditor, D, DEditor, E, EEditor, F, FEditor, G, GEditor, H, HEditor, I, IEditor, J, JEditor, K, KEditor, L, LEditor>);

// impl<A, AEditor, B, BEditor> DefaultEditor for (A, B)
// where
//     A: DefaultEditor<Editor =AEditor> + Clone + Default,
//     AEditor: Editor<Input = A, Output = Report<A>>,
//     B: DefaultEditor<Editor =BEditor> + Clone + Default,
//     BEditor: Editor<Input = B, Output = Report<B>>,
// {
//     type Editor = TupleEditor<A::Editor, A, B::Editor, B>;
//
//     fn default_editor() -> Self::Editor {
//         TupleEditor::new(A::default_editor(), B::default_editor())
//     }
// }
