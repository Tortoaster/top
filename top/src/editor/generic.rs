use crate::editor::container::{OptionEditor, VecEditor};
use crate::editor::primitive::InputEditor;
use crate::editor::Editor;
// use crate::editor::container::{OptionEditor, VecEditor};
use crate::editor::tuple::*;
use crate::html::AsHtml;

/// Specifies the default editor for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`Edit`].
pub trait Edit: Sized {
    type Editor: Editor<Output = Self>;

    /// Specifies the default editor for this type.
    fn edit(value: Option<Self>) -> Self::Editor;
}

impl Edit for String {
    type Editor = InputEditor<String>;

    fn edit(value: Option<Self>) -> Self::Editor {
        InputEditor::new(value.unwrap_or_default())
    }
}

macro_rules! impl_edit_for_number {
    ($($ty:ty),*) => {
        $(
            impl Edit for $ty {
                type Editor = InputEditor<$ty>;

                fn edit(value: Option<Self>) -> Self::Editor {
                    InputEditor::new(value.unwrap_or_default())
                }
            }
        )*
    };
}

impl_edit_for_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl Edit for bool {
    type Editor = InputEditor<bool>;

    fn edit(value: Option<Self>) -> Self::Editor {
        InputEditor::new(value.unwrap_or_default())
    }
}

impl Edit for char {
    type Editor = InputEditor<char>;

    fn edit(value: Option<Self>) -> Self::Editor {
        match value {
            None => InputEditor::empty(),
            Some(value) => InputEditor::new(value),
        }
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

impl<T> Edit for Vec<T>
where
    T: Edit,
    T::Editor: AsHtml + Clone,
{
    type Editor = VecEditor<T::Editor>;

    fn edit(value: Option<Self>) -> Self::Editor {
        VecEditor::new(
            value
                .into_iter()
                .flatten()
                .map(|value| T::edit(Some(value)))
                .collect(),
            T::edit(None),
        )
    }
}

impl<T> Edit for Option<T>
where
    T: Edit,
    T::Editor: AsHtml,
{
    type Editor = OptionEditor<T::Editor>;

    fn edit(value: Option<Self>) -> Self::Editor {
        let enabled = value.as_ref().map(Option::is_some).unwrap_or_default();

        OptionEditor::new(T::edit(value.flatten()), enabled)
    }
}
