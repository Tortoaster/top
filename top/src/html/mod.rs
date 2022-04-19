//! This module contains functionality for generating user interfaces for tasks.

use std::fmt::{Display, Formatter};

use serde::Serialize;

use top_derive::html;

pub mod event;
pub mod icon;
pub mod id;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct Html(pub String);

impl Html {
    pub fn wrapper(title: &str) -> Html {
        html! {r#"
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <title>{title}</title>
                    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css">
                    <script src="https://kit.fontawesome.com/e94af86b8c.js" crossorigin="anonymous"></script>
                    <script src="top/top.js"></script>
                </head>
                <body>
                    <section class="section">
                        <div class="container">
                            <div id="top-0"></div>
                        </div>
                    </section>
                </body>
            </html>
        "#}
    }
}

impl Display for Html {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromIterator<Html> for Html {
    fn from_iter<T: IntoIterator<Item = Html>>(iter: T) -> Self {
        let html: String = iter.into_iter().map(|html| html.0).collect();
        Html(html)
    }
}

pub trait ToHtml {
    fn to_html(&self) -> Html;
}

macro_rules! impl_to_html {
    ($($ty:ty),*) => {
        $(
            impl ToHtml for $ty {
                fn to_html(&self) -> Html {
                    Html(self.to_string())
                }
            }
        )*
    };
}

impl_to_html!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, &str,
    String
);

impl ToHtml for Html {
    fn to_html(&self) -> Html {
        self.clone()
    }
}

impl<T> ToHtml for Option<T>
where
    T: ToHtml,
{
    fn to_html(&self) -> Html {
        self.as_ref().map(ToHtml::to_html).unwrap_or_default()
    }
}

impl<T, E> ToHtml for Result<T, E>
where
    T: ToHtml,
{
    fn to_html(&self) -> Html {
        self.as_ref().map(ToHtml::to_html).unwrap_or_default()
    }
}

impl<T> ToHtml for Vec<T>
where
    T: ToHtml,
{
    fn to_html(&self) -> Html {
        self.iter().map(ToHtml::to_html).collect()
    }
}
