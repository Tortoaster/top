use std::fmt::{Debug, Display};

use crate::html::{Html, ToHtml};
use crate::task::tune::Tune;
use crate::viewer::primitive::OutputViewer;
use crate::viewer::Viewer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayViewer<T> {
    viewer: OutputViewer<String>,
    value: T,
}

impl<T> DisplayViewer<T> {
    pub fn new(value: T) -> Self
    where
        T: Display,
    {
        DisplayViewer {
            viewer: OutputViewer::new(value.to_string()),
            value,
        }
    }
}

impl<T> ToHtml for DisplayViewer<T> {
    fn to_html(&self) -> Html {
        self.viewer.to_html()
    }
}

impl<T> Viewer for DisplayViewer<T>
where
    T: Clone,
{
    type Value = T;

    fn value(&self) -> Self::Value {
        self.value.clone()
    }
}

impl<T> Tune for DisplayViewer<T> {
    type Tuner = <OutputViewer<String> as Tune>::Tuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.viewer.tune(tuner);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugViewer<T> {
    viewer: OutputViewer<String>,
    value: T,
}

impl<T> DebugViewer<T> {
    pub fn new(value: T) -> Self
    where
        T: Debug,
    {
        DebugViewer {
            viewer: OutputViewer::new(format!("{:?}", value)),
            value,
        }
    }
}

impl<T> ToHtml for DebugViewer<T> {
    fn to_html(&self) -> Html {
        self.viewer.to_html()
    }
}

impl<T> Viewer for DebugViewer<T>
where
    T: Clone,
{
    type Value = T;

    fn value(&self) -> Self::Value {
        self.value.clone()
    }
}

impl<T> Tune for DebugViewer<T> {
    type Tuner = <OutputViewer<String> as Tune>::Tuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.viewer.tune(tuner);
    }
}
