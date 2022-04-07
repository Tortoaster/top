use std::fmt::Debug;

use crate::html::Color;
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
