use crate::html::{Html, ToHtml};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Icon {
    Plus,
    Minus,
}

impl ToHtml for Icon {
    fn to_html(&self) -> Html {
        let html = match self {
            Icon::Plus => "<i class=\"fas fa-plus\"></i>",
            Icon::Minus => "<i class=\"fas fa-minus\"></i>",
        };

        Html(format!("<span class=\"icon is-small\">{html}</span>"))
    }
}
