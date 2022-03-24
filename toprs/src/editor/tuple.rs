use paste::paste;

use crate::component::event::{Event, Feedback};
use crate::component::{ComponentCreator, Widget};
use crate::editor::{Component, Editor, Report};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitEditor;

impl Editor for UnitEditor {
    type Input = ();
    type Output = ();

    fn start(&mut self, _initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        ctx.create(Widget::Group(Vec::new()))
    }

    fn on_event(&mut self, _event: Event, _ctx: &mut ComponentCreator) -> Option<Feedback> {
        None
    }

    fn value(&self) -> Report<Self::Output> {
        Ok(())
    }
}

macro_rules! none {
    ($_:ident) => {
        None
    };
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
                fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
                    let ($([<initial_ $editor:snake>],)*) = match initial {
                        Some(($([<value_ $editor:snake>],)*)) => ($(Some([<value_ $editor:snake>]),)*),
                        None => ($(none!($editor),)*),
                    };
                    let components = vec![
                        $(self.[<$editor:snake>].start([<initial_ $editor:snake>], ctx)),*
                    ];
                    let widget = Widget::Group(components);
                    let component = ctx.create(widget);
                    component
                }
            }

            paste! {
                fn on_event(&mut self, event: Event, ctx: &mut ComponentCreator) -> Option<Feedback> {
                    None
                        $(.or_else(|| self.[<$editor:snake>].on_event(event.clone(), ctx)))*
                }
            }

            paste! {
                fn value(&self) -> Report<Self::Output> {
                    Ok(($(self.[<$editor:snake>].value()?,)*))
                }
            }
        }
    }
}

// Beautiful, don't touch
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
