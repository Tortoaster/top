use paste::paste;

use crate::component::event::{Event, Feedback};
use crate::component::{ComponentCreator, Widget};
use crate::editor::{Component, Editor, Report};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitEditor;

impl Editor for UnitEditor {
    type Input = ();
    type Output = Report<()>;

    fn start(&mut self, _initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        ctx.create(Widget::Group(Vec::new()))
    }

    fn on_event(&mut self, _event: Event, _ctx: &mut ComponentCreator) -> Option<Feedback> {
        None
    }

    fn value(&self) -> &Self::Output {
        &Ok(())
    }

    fn finish(self) -> Self::Output {
        Ok(())
    }
}

macro_rules! none {
    ($_:ident) => {
        None
    };
}

// TODO: Remove clones, report, and trait bounds; reuse state of inner editors
macro_rules! tuple_editor {
    ($name:ident<$($editor:ident, $output:ident),*>) => {
        paste! {
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct $name<$($editor, $output),*> {
                $([<$editor:snake>]: $editor,)*
                value: Report<($($output,)*)>,
            }
        }

        impl<$($editor, $output),*> $name<$($editor, $output),*>
        where
            $(
                $editor: Editor<Output = Report<$output>>,
                $output: Default
            ),*
        {
            paste! {
                pub fn new($(paste! { [<$editor:snake>] }: $editor),*) -> Self {
                    $name {
                        $([<$editor:snake>],)*
                        value: Ok(($($output::default(),)*)),
                    }
                }
            }
        }

        impl<$($editor, $output),*> $name<$($editor, $output),*>
        where
            $(
                $editor: Editor<Output = Report<$output>>,
                $output: Clone
            ),*
        {
            paste! {
                pub fn value(&self) -> Report<($($output,)*)> {
                    $(let [<$editor:snake>] = self.[<$editor:snake>].value().clone()?;)*
                    Ok(($([<$editor:snake>],)*))
                }
            }
        }

        impl<$($editor, $output),*> Editor for $name<$($editor, $output),*>
        where
            $(
                $editor: Editor<Output = Report<$output>>,
                $output: Clone
            ),*
        {
            type Input = ($($editor::Input,)*);
            type Output = Report<($($output,)*)>;

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
                    let feedback = None
                        $(.or_else(|| self.[<$editor:snake>].on_event(event.clone(), ctx)))*;

                    if feedback.is_some() {
                        let value = self.value();
                        self.value = value;
                    }

                    feedback
                }
            }

            fn value(&self) -> &Self::Output {
                &self.value
            }

            paste! {
                fn finish(self) -> Self::Output {
                    Ok(($(self.[<$editor:snake>].finish()?,)*))
                }
            }
        }
    }
}

// Beautiful, don't touch
tuple_editor!(MonupleEditor<A, AValue>);
tuple_editor!(CoupleEditor<A, AValue, B, BValue>);
tuple_editor!(TripleEditor<A, AValue, B, BValue, C, CValue>);
tuple_editor!(QuadrupleEditor<A, AValue, B, BValue, C, CValue, D, DValue>);
tuple_editor!(QuintupleEditor<A, AValue, B, BValue, C, CValue, D, DValue, E, EValue>);
tuple_editor!(SextupleEditor<A, AValue, B, BValue, C, CValue, D, DValue, E, EValue, F, FValue>);
tuple_editor!(SeptupleEditor<A, AValue, B, BValue, C, CValue, D, DValue, E, EValue, F, FValue, G, GValue>);
tuple_editor!(OctupleEditor<A, AValue, B, BValue, C, CValue, D, DValue, E, EValue, F, FValue, G, GValue, H, HValue>);
tuple_editor!(NonupleEditor<A, AValue, B, BValue, C, CValue, D, DValue, E, EValue, F, FValue, G, GValue, H, HValue, I, IValue>);
tuple_editor!(DecupleEditor<A, AValue, B, BValue, C, CValue, D, DValue, E, EValue, F, FValue, G, GValue, H, HValue, I, IValue, J, JValue>);
tuple_editor!(UndecupleEditor<A, AValue, B, BValue, C, CValue, D, DValue, E, EValue, F, FValue, G, GValue, H, HValue, I, IValue, J, JValue, K, KValue>);
tuple_editor!(DuodecupleEditor<A, AValue, B, BValue, C, CValue, D, DValue, E, EValue, F, FValue, G, GValue, H, HValue, I, IValue, J, JValue, K, KValue, L, LValue>);
