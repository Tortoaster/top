use async_trait::async_trait;
use paste::paste;

use crate::editor::{Editor, EditorError};
use crate::html::event::{Event, Feedback};
use crate::html::id::Generator;
use crate::html::{Html, ToHtml};
use crate::share::Share;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitEditor;

#[async_trait]
impl ToHtml for UnitEditor {
    async fn to_html(&self) -> Html {
        Html::default()
    }
}

#[async_trait]
impl Editor for UnitEditor {
    type Value = ();
    type Share = Share<Result<Self::Value, EditorError>>;

    fn start(&mut self, _gen: &mut Generator) {}

    async fn on_event(&mut self, _event: Event, _gen: &mut Generator) -> Feedback {
        Feedback::new()
    }

    fn share(&self) -> Self::Share {
        Share::new(Ok(()))
    }

    fn value(self) -> Result<Self::Value, EditorError> {
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
                            $(self.[<$editor:snake>].start(Some(value.0), gen);)*
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

// tuple_editor!(MonupleEditor<A>);
// tuple_editor!(CoupleEditor<A, B>);
// tuple_editor!(TripleEditor<A, B, C>);
// tuple_editor!(QuadrupleEditor<A, B, C, D>);
// tuple_editor!(QuintupleEditor<A, B, C, D, E>);
// tuple_editor!(SextupleEditor<A, B, C, D, E, F>);
// tuple_editor!(SeptupleEditor<A, B, C, D, E, F, G>);
// tuple_editor!(OctupleEditor<A, B, C, D, E, F, G, H>);
// tuple_editor!(NonupleEditor<A, B, C, D, E, F, G, H, I>);
// tuple_editor!(DecupleEditor<A, B, C, D, E, F, G, H, I, J>);
// tuple_editor!(UndecupleEditor<A, B, C, D, E, F, G, H, I, J, K>);
// tuple_editor!(DuodecupleEditor<A, B, C, D, E, F, G, H, I, J, K, L>);
