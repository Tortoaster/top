use crate::html::{AsHtml, Html};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Span<'a> {
    content: &'a str,
    color: Color,
}

impl<'a> Span<'a> {
    pub const fn new(content: &'a str) -> Self {
        Span {
            content,
            color: Color::Black,
        }
    }

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

impl AsHtml for Color {
    fn as_html(&self) -> Html {
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

impl AsHtml for Span<'_> {
    fn as_html(&self) -> Html {
        let html = format!(
            "<span style=\"color: {};\">{}</span>",
            self.color.as_html(),
            self.content
        );

        Html(html)
    }
}
