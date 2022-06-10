pub use top_derive::Edit;

use crate::editor::container::OptionEditor;
use crate::editor::primitive::InputEditor;
use crate::editor::tuple::*;
use crate::html::{Handler, ToHtml};
use crate::share::{Share, SharedId, SharedRead, SharedValue, SharedWrite};
use crate::task::Value;

/// Specifies the default editor for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`Edit`].
pub trait Edit: Sized {
    type Editor: Value<Output = Self> + Handler + ToHtml;

    /// Specifies the default editor for this type.
    fn edit(value: Option<Self>) -> Self::Editor;
}

/// Have the user enter a value. To use a custom editor, see [`edit_with`].
#[inline]
pub fn enter<T>() -> T::Editor
where
    T: Edit,
{
    T::edit(None)
}

/// Have the user update a value. To use a custom editor, see [`edit_with`].
#[inline]
pub fn edit<T>(value: T) -> T::Editor
where
    T: Edit,
{
    T::edit(Some(value))
}

// /// Have the user select a value out of a list of options. To use a custom viewer for the options,
// /// see [`choose_with`].
// #[inline]
// pub fn choose<T>(options: Vec<T>) -> Interact<ChoiceEditor<T::Viewer>>
// where
//     T: View,
// {
//     choose_with(options.into_iter().map(T::view).collect())
// }
//
// /// Have the user select a value out of a list of options, using a custom viewer.
// #[inline]
// pub fn choose_with<V>(options: Vec<V>) -> Interact<ChoiceEditor<V>> {
//     Interact {
//         editor: ChoiceEditor::new(options),
//     }
// }

/// For some types, the HTML-representation starts with a valid value by default. For example, a
/// number input starts at 0, which is a valid number, and a text field starts empty, which is a
/// valid string. In these cases, the editor should be initialized with a default value, rather than
/// [`EditorError::Empty`].
macro_rules! impl_edit_for_default {
    ($($ty:ty),*) => {
        $(
            impl Edit for $ty {
                type Editor = InputEditor<Share<$ty>, $ty>;

                fn edit(value: Option<Self>) -> Self::Editor {
                    InputEditor::new(Some(value.unwrap_or_default()))
                }
            }
        )*
    };
}

impl_edit_for_default!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, String
);

impl Edit for char {
    type Editor = InputEditor<Share<char>, char>;

    fn edit(value: Option<Self>) -> Self::Editor {
        InputEditor::new(value)
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
//     T::Editor: ToHtml + Clone,
// {
//     type Editor = VecEditor<T::Editor>;
//
//     fn edit(value: Option<Self>) -> Self::Editor {
//         VecEditor::new(
//             value
//                 .into_iter()
//                 .flatten()
//                 .map(|value| T::edit(Some(value)))
//                 .collect(),
//             T::edit(None),
//         )
//     }
// }

impl<T> Edit for Option<T>
where
    T: Edit,
    T::Editor: ToHtml + Send + Sync,
    <T::Editor as Value>::Share: Sync,
{
    type Editor = OptionEditor<T::Editor>;

    fn edit(value: Option<Self>) -> Self::Editor {
        let enabled = value.as_ref().map(Option::is_some).unwrap_or_default();

        OptionEditor::new(T::edit(value.flatten()), enabled)
    }
}

pub trait SharedEdit<S>: Sized {
    type Editor: Value<Output = Self> + Handler + ToHtml;

    fn edit_shared(share: S) -> Self::Editor;
}

#[inline]
pub fn edit_shared<S>(share: S) -> <S::Value as SharedEdit<S>>::Editor
where
    S: SharedRead,
    S::Value: SharedEdit<S>,
{
    <S::Value>::edit_shared(share)
}

macro_rules! impl_shared_edit {
    ($($ty:ty),*) => {
        $(
            impl<S> SharedEdit<S> for $ty
            where
                S: SharedRead<Value = $ty>
                    + SharedWrite<Value = $ty>
                    + SharedId
                    + SharedValue<Value = $ty>
                    + Clone
                    + Send
                    + Sync,
            {
                type Editor = InputEditor<S, $ty>;

                fn edit_shared(share: S) -> Self::Editor {
                    InputEditor::new_shared(share)
                }
            }
        )*
    };
}

impl_shared_edit!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, String
);
