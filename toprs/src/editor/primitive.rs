//! This module contains basic editors for primitive types.

use std::fmt::Display;
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::str::FromStr;

use num_traits::PrimInt;

use crate::component::event::{Event, Feedback};
use crate::component::{ComponentCreator, Id, Widget};
use crate::editor::{Component, Editor, Report};

/// Basic editor for strings.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TextEditor(Id);

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
        self.0 = component.id();
        component
    }

    fn on_event(
        &mut self,
        event: Event,
        _: &mut ComponentCreator,
    ) -> Option<(Self::Output, Feedback)> {
        match event {
            Event::Update { id, value } if id == self.0 => {
                Some((Ok(value), Feedback::Valid { id }))
            }
            _ => None,
        }
    }
}

/// Basic editor for numbers.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NumberEditor<N>(Id, PhantomData<N>);

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
        self.0 = component.id();
        component
    }

    fn on_event(
        &mut self,
        event: Event,
        _: &mut ComponentCreator,
    ) -> Option<(Self::Output, Feedback)> {
        match event {
            Event::Update { id, value } => {
                if id == self.0 {
                    match value.parse::<N>() {
                        Ok(value) => Some((Ok(value), Feedback::Valid { id })),
                        Err(error) => Some((Err(error.into()), Feedback::Invalid { id })),
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Basic editor for numbers.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BooleanEditor(Id);

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
        self.0 = component.id();
        component
    }

    fn on_event(
        &mut self,
        event: Event,
        _: &mut ComponentCreator,
    ) -> Option<(Self::Output, Feedback)> {
        match event {
            Event::Update { id, value } => {
                if id == self.0 {
                    match value.parse() {
                        Ok(value) => Some((Ok(value), Feedback::Valid { id })),
                        Err(error) => Some((Err(error.into()), Feedback::Invalid { id })),
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
