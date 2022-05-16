use async_trait::async_trait;

use crate::html::{Html, ToHtml};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Icon {
    Plus,
    Minus,
    Check,
    XMark,
}

#[async_trait]
impl ToHtml for Icon {
    async fn to_html(&self) -> Html {
        let html = match self {
            Icon::Plus => r#"<i class="fas fa-plus"></i>"#,
            Icon::Minus => r#"<i class="fas fa-minus"></i>"#,
            Icon::Check => r#"<i class="fa-solid fa-check"></i>"#,
            Icon::XMark => r#"<i class="fa-solid fa-xmark"></i>"#,
        };

        Html(format!("<span class=\"icon is-small\">{html}</span>"))
    }
}
