use std::fmt::Debug;

use crate::html::{Html, ToHtml};
use crate::task::inspect::Inspect;
use crate::task::interact::Interact;

pub trait Tune {
    type Tuner;

    fn tune(&mut self, tuner: Self::Tuner);
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InputTuner {
    pub label: Option<String>,
}

impl InputTuner {
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringTuner {
    pub color: Color,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Color {
    Black,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Brown,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl ToHtml for Color {
    fn to_html(&self) -> Html {
        match self {
            Color::Black => Html("black".to_owned()),
            Color::White => Html("white".to_owned()),
            Color::Red => Html("red".to_owned()),
            Color::Orange => Html("orange".to_owned()),
            Color::Yellow => Html("yellow".to_owned()),
            Color::Green => Html("green".to_owned()),
            Color::Blue => Html("blue".to_owned()),
            Color::Purple => Html("purple".to_owned()),
            Color::Pink => Html("pink".to_owned()),
            Color::Brown => Html("brown".to_owned()),
        }
    }
}

impl StringTuner {
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl<E> Interact<E>
where
    E: Tune,
{
    pub fn tune(mut self, tuner: E::Tuner) -> Self {
        self.editor.tune(tuner);
        self
    }
}

impl<V> Inspect<V>
where
    V: Tune,
{
    pub fn tune(mut self, tuner: V::Tuner) -> Self {
        self.viewer.tune(tuner);
        self
    }
}
