use crate::editor::{Editor, FormatParams, Html};
use std::convert::Infallible;

#[derive(Debug, Default)]
pub struct TextField(String, FormatParams);

impl TextField {
    pub fn new(params: FormatParams) -> Self {
        TextField(Default::default(), params)
    }
}

impl Editor for TextField {
    type Read = String;
    type Write = String;
    type Error = Infallible;

    fn html(&self) -> Html {
        format!(
            "<label>{}<input type='text' value='{}' {}/></label>",
            self.1.label.as_ref().unwrap_or(&String::new()),
            self.0,
            if self.1.disabled { "disabled" } else { "" },
        )
    }

    fn read_value(&self) -> Self::Read {
        self.0.clone()
    }

    fn write_value(&mut self, value: Self::Write) -> Result<(), Self::Error> {
        self.0 = value;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct NumberField(i32, FormatParams);

impl NumberField {
    pub fn new(params: FormatParams) -> Self {
        NumberField(Default::default(), params)
    }
}

impl Editor for NumberField {
    type Read = i32;
    type Write = i32;
    type Error = Infallible;

    fn html(&self) -> Html {
        format!(
            "<label>{}<input type='number' value='{}' {}/></label>",
            self.1.label.as_ref().unwrap_or(&String::new()),
            self.0,
            if self.1.disabled { "disabled" } else { "" },
        )
    }

    fn read_value(&self) -> Self::Read {
        self.0.clone()
    }

    fn write_value(&mut self, value: Self::Write) -> Result<(), Self::Error> {
        self.0 = value;
        Ok(())
    }
}
