use paste::paste;

use crate::component::Widget;
use crate::editor::{Component, Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::id::{Generator, Id};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitEditor;

impl Editor for UnitEditor {
    type Input = ();
    type Output = ();

    fn start(&mut self, _value: Option<Self::Input>, _gen: &mut Generator) {}

    fn component(&self) -> Component {
        Component::new(Id::INVALID, Widget::Group(Vec::new()))
    }

    fn on_event(&mut self, _event: Event, _gen: &mut Generator) -> Option<Feedback> {
        None
    }

    fn read(&self) -> Result<Self::Output, EditorError> {
        Ok(())
    }
}

macro_rules! tuple_editor {
    ($name:ident<$($editor:ident),*>) => {
        paste! {
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct $name<$($editor),*> {
                $([<$editor:snake>]: $editor),*
            }
        }

        impl<$($editor),*> $name<$($editor),*>
        where
            $($editor: Editor),*
        {
            paste! {
                pub fn new($(paste! { [<$editor:snake>] }: $editor),*) -> Self {
                    $name {
                        $([<$editor:snake>]),*
                    }
                }
            }
        }

        impl<$($editor),*> Editor for $name<$($editor),*>
        where
            $($editor: Editor),*
        {
            type Input = ($($editor::Input,)*);
            type Output = ($($editor::Output,)*);

            paste! {
                fn start(&mut self, value: Option<Self::Input>, gen: &mut Generator) {
                    match value {
                        None => {
                            $(self.[<$editor:snake>].start(None, gen);)*
                        }
                        Some(value) => {
                            $(self.[<$editor:snake>].start(Some(value.${index()}), gen);)*
                        }
                    }
                }
            }

            paste! {
                fn component(&self) -> Component {
                    let children = vec![
                        $(self.[<$editor:snake>].component()),*
                    ];
                    Component::new(Id::INVALID, Widget::Group(children))
                }
            }

            paste! {
                fn on_event(&mut self, event: Event, gen: &mut Generator) -> Option<Feedback> {
                    None
                        $(.or_else(|| self.[<$editor:snake>].on_event(event.clone(), gen)))*
                }
            }

            paste! {
                fn read(&self) -> Result<Self::Output, EditorError> {
                    Ok(($(self.[<$editor:snake>].read()?,)*))
                }
            }
        }
    }
}

tuple_editor!(MonupleEditor<A>);
tuple_editor!(CoupleEditor<A, B>);
tuple_editor!(TripleEditor<A, B, C>);
tuple_editor!(QuadrupleEditor<A, B, C, D>);
tuple_editor!(QuintupleEditor<A, B, C, D, E>);
tuple_editor!(SextupleEditor<A, B, C, D, E, F>);
tuple_editor!(SeptupleEditor<A, B, C, D, E, F, G>);
tuple_editor!(OctupleEditor<A, B, C, D, E, F, G, H>);
tuple_editor!(NonupleEditor<A, B, C, D, E, F, G, H, I>);
tuple_editor!(DecupleEditor<A, B, C, D, E, F, G, H, I, J>);
tuple_editor!(UndecupleEditor<A, B, C, D, E, F, G, H, I, J, K>);
tuple_editor!(DuodecupleEditor<A, B, C, D, E, F, G, H, I, J, K, L>);
