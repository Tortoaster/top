//! This module contains basic editors for primitive types.

use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use num_traits::PrimInt;

use crate::component::event::{Event, Feedback};
use crate::component::{ComponentCreator, Id, Widget};
use crate::editor::{Component, Editor, Report};

/// Basic editor for strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextEditor {
    id: Id,
    value: Report<String>,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            id: Id::default(),
            value: Ok(String::new()),
        }
    }
}

impl Editor for TextEditor {
    type Input = String;
    type Output = Report<String>;

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::TextField {
            value: initial.unwrap_or_default(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } if id == self.id => {
                self.value = Ok(value);
                Some(Feedback::Valid { id })
            }
            _ => None,
        }
    }

    fn value(&self) -> &Self::Output {
        &self.value
    }

    fn finish(self) -> Self::Output {
        self.value
    }
}

/// Basic editor for numbers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NumberEditor<N> {
    id: Id,
    value: Report<N>,
}

impl<N> NumberEditor<N>
where
    N: Default,
{
    pub fn new() -> Self {
        Self {
            id: Id::default(),
            value: Ok(N::default()),
        }
    }
}

impl<N> Editor for NumberEditor<N>
where
    N: Default + Display + FromStr<Err = ParseIntError> + PrimInt,
{
    type Input = N;
    type Output = Report<N>;

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::NumberField {
            value: initial.unwrap_or_default().to_string(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } => {
                if id == self.id {
                    match value.parse::<N>() {
                        Ok(value) => {
                            self.value = Ok(value);
                            Some(Feedback::Valid { id })
                        }
                        Err(error) => {
                            self.value = Err(error.into());
                            Some(Feedback::Invalid { id })
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn value(&self) -> &Self::Output {
        &self.value
    }

    fn finish(self) -> Self::Output {
        self.value
    }
}

/// Basic editor for numbers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BooleanEditor {
    id: Id,
    value: Report<bool>,
}

impl BooleanEditor {
    pub fn new() -> Self {
        Self {
            id: Id::default(),
            value: Ok(false),
        }
    }
}

impl Editor for BooleanEditor {
    type Input = bool;
    type Output = Report<bool>;

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        let widget = Widget::Checkbox {
            checked: initial.unwrap_or_default(),
            label: None,
            disabled: false,
        };
        let component = ctx.create(widget);
        // TODO: Type-safe way of guaranteeing that editors have a proper identifier.
        self.id = component.id();
        component
    }

    fn on_event(&mut self, event: Event, _: &mut ComponentCreator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } => {
                if id == self.id {
                    match value.parse() {
                        Ok(value) => {
                            self.value = Ok(value);
                            Some(Feedback::Valid { id })
                        }
                        Err(error) => {
                            self.value = Err(error.into());
                            Some(Feedback::Invalid { id })
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn value(&self) -> &Self::Output {
        &self.value
    }

    fn finish(self) -> Self::Output {
        self.value
    }
}

/// Combines two editors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TupleEditor<A, X, B, Y> {
    a: A,
    b: B,
    // TODO: Remove, reuse state of inner editors
    value: Report<(X, Y)>,
}

impl<A, X, B, Y> TupleEditor<A, X, B, Y>
where
    A: Editor<Output = Report<X>>,
    X: Default,
    B: Editor<Output = Report<Y>>,
    Y: Default,
{
    pub fn new(a: A, b: B) -> Self {
        TupleEditor {
            a,
            b,
            value: Ok((X::default(), Y::default())),
        }
    }
}

impl<A, X, B, Y> TupleEditor<A, X, B, Y>
where
    A: Editor<Output = Report<X>>,
    X: Clone,
    B: Editor<Output = Report<Y>>,
    Y: Clone,
{
    // TODO: Get rid of cloning somehow
    pub fn value(&self) -> Report<(X, Y)> {
        let a = self.a.value().clone()?;
        let b = self.b.value().clone()?;
        Ok((a, b))
    }
}

impl<A, X, B, Y> Editor for TupleEditor<A, X, B, Y>
where
    A: Editor<Output = Report<X>>,
    X: Clone,
    B: Editor<Output = Report<Y>>,
    Y: Clone,
{
    type Input = (A::Input, B::Input);
    type Output = Report<(X, Y)>;

    fn start(&mut self, initial: Option<Self::Input>, ctx: &mut ComponentCreator) -> Component {
        let (initial_a, initial_b) = match initial {
            Some((value_a, value_b)) => (Some(value_a), Some(value_b)),
            None => (None, None),
        };
        let components = vec![self.a.start(initial_a, ctx), self.b.start(initial_b, ctx)];
        let widget = Widget::Group(components);
        let component = ctx.create(widget);
        component
    }

    fn on_event(&mut self, event: Event, ctx: &mut ComponentCreator) -> Option<Feedback> {
        let feedback = self
            .a
            .on_event(event.clone(), ctx)
            .or_else(|| self.b.on_event(event, ctx));

        if feedback.is_some() {
            let value = self.value();
            self.value = value;
        }

        feedback
    }

    fn value(&self) -> &Self::Output {
        &self.value
    }

    fn finish(self) -> Self::Output {
        Ok((self.a.finish()?, self.b.finish()?))
    }
}
