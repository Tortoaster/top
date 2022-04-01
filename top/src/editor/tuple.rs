use paste::paste;

use crate::component::Widget;
use crate::editor::{Component, Editor, EditorError};
use crate::event::{Event, Feedback};
use crate::id::Generator;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitEditor;

impl Editor for UnitEditor {
    type Input = ();
    type Output = ();

    fn component(&mut self, gen: &mut Generator) -> Component {
        Component::new(
            gen.next(),
            Widget::Group {
                children: Vec::new(),
                horizontal: false,
            },
        )
    }

    fn on_event(&mut self, _event: Event, _gen: &mut Generator) -> Option<Feedback> {
        None
    }

    fn read(&self) -> Result<Self::Output, EditorError> {
        Ok(())
    }

    fn write(&mut self, _value: Self::Input) {}
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
                fn component(&mut self, gen: &mut Generator) -> Component {
                    let children = vec![
                        $(self.[<$editor:snake>].component(gen)),*
                    ];
                    Component::new(gen.next(), Widget::Group { children, horizontal: false })
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

            paste! {
                fn write(&mut self, value: Self::Input) {
                    $(self.[<$editor:snake>].write(value.${index()});)*
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
