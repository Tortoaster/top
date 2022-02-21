use crate::editor::{Component, Editor};
use std::convert::Infallible;

#[derive(Debug, Default)]
pub struct TextField {
    value: String,
    label: Option<String>,
    disabled: bool,
}

impl TextField {
    pub const fn new() -> Self {
        TextField {
            value: String::new(),
            label: None,
            disabled: false,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl Editor for TextField {
    type Read = String;
    type Write = String;
    type Error = Infallible;

    fn ui(&self) -> Component {
        Component::TextField {
            value: self.value.clone(),
            label: self.label.clone(),
            disabled: self.disabled,
        }
    }

    fn read_value(&self) -> Self::Read {
        self.value.clone()
    }

    fn write_value(&mut self, value: Self::Write) -> Result<(), Self::Error> {
        self.value = value;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct NumberField {
    value: i32,
    label: Option<String>,
    disabled: bool,
}

impl NumberField {
    pub const fn new() -> Self {
        NumberField {
            value: 0,
            label: None,
            disabled: false,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl Editor for NumberField {
    type Read = i32;
    type Write = i32;
    type Error = Infallible;

    fn ui(&self) -> Component {
        Component::NumberField {
            value: self.value,
            label: self.label.clone(),
            disabled: self.disabled,
        }
    }

    fn read_value(&self) -> Self::Read {
        self.value
    }

    fn write_value(&mut self, value: Self::Write) -> Result<(), Self::Error> {
        self.value = value;
        Ok(())
    }
}
