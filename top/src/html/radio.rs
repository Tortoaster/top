use crate::html::{AsHtml, Html, RADIO_GROUP, REGISTRY};
use crate::id::Id;
use serde_json::json;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RadioGroup {
    id: Id,
    children: Vec<Html>,
}

impl RadioGroup {
    pub const fn new(id: Id, children: Vec<Html>) -> Self {
        RadioGroup { id, children }
    }
}

impl AsHtml for RadioGroup {
    fn as_html(&self) -> Html {
        let html = REGISTRY
            .render(
                RADIO_GROUP,
                &json!({
                    "id": self.id,
                    "options": self.children,
                }),
            )
            .expect("failed to render template");

        Html(html)
    }
}
