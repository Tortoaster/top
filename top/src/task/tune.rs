use std::fmt::Debug;

use async_trait::async_trait;

use crate::html::{Html, ToHtml};
use crate::task::inspect::Inspect;

pub trait Tune {
    type Tuner;

    fn tune(&mut self, tuner: Self::Tuner);
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OutputTuner {
    pub color: Color,
}

impl OutputTuner {
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
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

#[async_trait]
impl ToHtml for Color {
    async fn to_html(&self) -> Html {
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

impl<V> Inspect<V>
where
    V: Tune,
{
    pub fn tune(mut self, tuner: V::Tuner) -> Self {
        self.viewer.tune(tuner);
        self
    }
}

pub trait ContentTune {
    type ContentTuner;

    fn tune_content(&mut self, tuner: Self::ContentTuner);
}

impl<V> Inspect<V>
where
    V: ContentTune,
{
    pub fn tune_content(mut self, tuner: V::ContentTuner) -> Self {
        self.viewer.tune_content(tuner);
        self
    }
}
