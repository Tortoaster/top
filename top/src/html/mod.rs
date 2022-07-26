//! This module contains functionality for generating user interfaces for tasks.

use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::html::event::{Event, Feedback};

pub mod event;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct Html(pub String);

impl Html {
    pub async fn wrapper(title: &str) -> Html {
        Html(format!(
            r#"
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
                            <div id="00000000-0000-0000-0000-000000000000"></div>
                        </div>
                    </section>
                </body>
            </html>
        "#
        ))
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

#[async_trait]
pub trait ToHtml {
    async fn to_html(&self) -> Html;
}

#[async_trait]
pub trait Handler {
    async fn on_event(&mut self, event: Event) -> Feedback;
}

#[async_trait]
pub trait Refresh {
    async fn refresh(&self, ids: &BTreeSet<Uuid>) -> Feedback;
}
