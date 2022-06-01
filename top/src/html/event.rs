use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::html::Html;

/// Interaction events from the user, such as checking a checkbox or pressing a button.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    Update { id: Uuid, value: String },
    Press { id: Uuid },
    Redraw { id: Uuid },
}

/// Changes to the user interface in response to [`Event`]s, such as confirming a value is valid, or
/// replacing the content after the user presses a button.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Change {
    /// Replace this element with new html.
    ReplaceContent { id: Uuid, html: Html },
    /// Add html to this element.
    AppendContent { id: Uuid, html: Html },
    /// Remove this element.
    Remove { id: Uuid },
    /// The value of this html is valid.
    Valid { id: Uuid },
    /// The value of this html is invalid.
    Invalid { id: Uuid },

    /// Change the value of an input.
    UpdateValue { id: Uuid, value: String },
}

impl Change {
    fn id(&self) -> Uuid {
        match self {
            Change::ReplaceContent { id, .. }
            | Change::AppendContent { id, .. }
            | Change::Remove { id, .. }
            | Change::Valid { id, .. }
            | Change::Invalid { id, .. }
            | Change::UpdateValue { id, .. } => *id,
        }
    }

    fn merge_with(&mut self, other: Change) -> Result<(), ()> {
        match self {
            Change::ReplaceContent { html, .. } => match other {
                Change::ReplaceContent { html: other, .. } => *html = other,
                Change::AppendContent { html: other, .. } => html.0.push_str(&other.0),
                Change::Remove { .. } => *self = other,
                Change::Valid { .. } | Change::Invalid { .. } | Change::UpdateValue { .. } => {
                    return Err(());
                }
            },
            Change::AppendContent { html, .. } => match other {
                Change::ReplaceContent { .. } | Change::Remove { .. } => *self = other,
                Change::AppendContent { html: other, .. } => html.0.push_str(&other.0),
                Change::Valid { .. } | Change::Invalid { .. } | Change::UpdateValue { .. } => {
                    return Err(());
                }
            },
            Change::Remove { .. } => match other {
                Change::ReplaceContent { .. } | Change::AppendContent { .. } => return Err(()),
                Change::Remove { .. } => {}
                Change::Valid { .. } | Change::Invalid { .. } | Change::UpdateValue { .. } => {
                    return Err(());
                }
            },
            Change::Valid { .. } | Change::Invalid { .. } => *self = other,
            Change::UpdateValue { value, .. } => match other {
                Change::ReplaceContent { .. } | Change::AppendContent { .. } => return Err(()),
                Change::Remove { .. } => *self = other,
                Change::Valid { .. } | Change::Invalid { .. } => {}
                Change::UpdateValue { value: other, .. } => *value = other,
            },
        }

        Ok(())
    }
}

#[must_use]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Feedback {
    changes: BTreeMap<Uuid, Change>,
    shares: BTreeSet<Uuid>,
}

impl Feedback {
    /// Creates new feedback with no changes.
    pub fn new() -> Self {
        Feedback {
            changes: BTreeMap::new(),
            shares: BTreeSet::new(),
        }
    }

    pub fn update_share(share: Uuid) -> Self {
        let mut shares = BTreeSet::new();
        shares.insert(share);
        Feedback {
            changes: BTreeMap::new(),
            shares,
        }
    }

    /// Combines two pieces of feedback, giving [`other`] in ambiguous cases. For example, if this
    /// feedback inserts something in an element while [`other`] removes that element, it gets
    /// removed. The other way around (first removing, then inserting) makes no sense and will
    /// result in an error.
    pub fn merged_with(mut self, mut other: Self) -> Result<Self, ()> {
        for (id, other) in other.changes {
            match self.changes.get_mut(&id) {
                None => {
                    self.changes.insert(id, other);
                }
                Some(change) => change.merge_with(other)?,
            }
        }
        self.shares.append(&mut other.shares);

        Ok(self)
    }

    pub fn changes(self) -> Vec<Change> {
        self.changes.into_values().collect()
    }

    pub fn shares(&self) -> &BTreeSet<Uuid> {
        &self.shares
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}

impl From<Change> for Feedback {
    fn from(change: Change) -> Self {
        let mut changes = BTreeMap::new();
        changes.insert(change.id(), change);
        Feedback {
            changes,
            shares: BTreeSet::new(),
        }
    }
}
