// use crate::editor::container::{OptionEditor, VecEditor};
use crate::editor::primitive::{
    BooleanEditor, CharEditor, FloatEditor, IntegerEditor, StringEditor,
};
use crate::editor::tuple::*;
use crate::editor::Editor;

/// Specifies the default editor for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`Edit`].
pub trait Edit: Sized {
    type Editor: Editor<Output = Self>;

    /// Specifies the default editor for this type.
    fn edit(value: Option<Self>) -> Self::Editor;
}

impl Edit for String {
    type Editor = StringEditor;

    fn edit(value: Option<Self>) -> Self::Editor {
        StringEditor::new(value)
    }
}

macro_rules! impl_edit_for_integer {
    ($($ty:ty),*) => {
        $(
            impl Edit for $ty {
                type Editor = IntegerEditor<$ty>;

                fn edit(value: Option<Self>) -> Self::Editor {
                    IntegerEditor::new(value)
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

                fn edit(value: Option<Self>) -> Self::Editor {
                    FloatEditor::new(value)
                }
            }
        )*
    };
}

impl_edit_for_float!(f32, f64);

impl Edit for bool {
    type Editor = BooleanEditor;

    fn edit(value: Option<Self>) -> Self::Editor {
        BooleanEditor::new(value)
    }
}

impl Edit for char {
    type Editor = CharEditor;

    fn edit(value: Option<Self>) -> Self::Editor {
        CharEditor::new(value)
    }
}

impl Edit for () {
    type Editor = UnitEditor;

    fn edit(_: Option<Self>) -> Self::Editor {
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

// impl_edit_for_tuple!(MonupleEditor<A>);
// impl_edit_for_tuple!(CoupleEditor<A, B>);
// impl_edit_for_tuple!(TripleEditor<A, B, C>);
// impl_edit_for_tuple!(QuadrupleEditor<A, B, C, D>);
// impl_edit_for_tuple!(QuintupleEditor<A, B, C, D, E>);
// impl_edit_for_tuple!(SextupleEditor<A, B, C, D, E, F>);
// impl_edit_for_tuple!(SeptupleEditor<A, B, C, D, E, F, G>);
// impl_edit_for_tuple!(OctupleEditor<A, B, C, D, E, F, G, H>);
// impl_edit_for_tuple!(NonupleEditor<A, B, C, D, E, F, G, H, I>);
// impl_edit_for_tuple!(DecupleEditor<A, B, C, D, E, F, G, H, I, J>);
// impl_edit_for_tuple!(UndecupleEditor<A, B, C, D, E, F, G, H, I, J, K>);
// impl_edit_for_tuple!(DuodecupleEditor<A, B, C, D, E, F, G, H, I, J, K, L>);

// impl<T> Edit for Vec<T>
// where
//     T: Edit,
// {
//     type Editor = VecEditor<T::Editor>;
//
//     fn edit(self) -> Self::Editor {
//         VecEditor::new(self.into_iter().map(T::edit).collect())
//     }
// }
//
// impl<T> Edit for Option<T>
// where
//     T: Edit,
// {
//     type Editor = OptionEditor<T::Editor>;
//
//     fn edit(self) -> Self::Editor {
//         OptionEditor::new(self.map(T::edit))
//     }
// }
