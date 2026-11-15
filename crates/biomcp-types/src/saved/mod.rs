//! Saved searches and lists a user keeps across cache cleans and upgrades.
//! Records hold words, names, identifiers and times, never source content.
use serde::{Deserialize, Serialize};

/// One record a user kept: the identifier a get command reads.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SavedRef {
    /// Entity word of the get command, such as gene or trial.
    pub entity: crate::Text,
    /// Exact identifier the get command accepts.
    pub id: crate::Text,
    /// Source the identifier belongs to, when identifiers are source-specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::Source>,
    /// Unix seconds when the reference was kept.
    pub as_of: u64,
    /// Source data date of the kept record, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_date: Option<crate::SourceTimestamp>,
}

/// A search kept by name; its command words are what runs again.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SavedSearch {
    /// Random 16-byte identifier in hex.
    pub id: crate::Text,
    /// Name, unique in its project without regard to case.
    pub name: crate::Text,
    /// Project holding the search.
    pub project: crate::Text,
    /// Command words as given, without the save option.
    pub args: Vec<Box<str>>,
    /// Request schema identity at save time.
    pub schema: crate::Text,
    /// SHA-256 of the checked request at save time, in hex.
    pub request_key: crate::Text,
    /// Unix seconds when the search was saved.
    pub created: u64,
    /// Unix seconds when the search was last checked for changes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checked: Option<u64>,
}

/// One kept reference in a list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SavedListItem {
    /// The kept record.
    #[serde(rename = "ref")]
    pub reference: SavedRef,
    /// The user's note, which is user data and never instructions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<crate::Text>,
    /// Unix seconds when the item was added.
    pub added: u64,
}

/// A named list of kept references.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SavedList {
    /// Random 16-byte identifier in hex.
    pub id: crate::Text,
    /// Name, unique in its project without regard to case.
    pub name: crate::Text,
    /// Project holding the list.
    pub project: crate::Text,
    /// Unix seconds when the list was made.
    pub created: u64,
    /// Unix seconds when an item was last added or removed.
    pub modified: u64,
    /// Items in the order they were added.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SavedListItem>,
}

/// Saved searches and lists selected by a saved command.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Saved {
    /// Saved searches, by project and name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub searches: Vec<SavedSearch>,
    /// Saved lists, by project and name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lists: Vec<SavedList>,
    /// IDs a list remove named that the list did not hold.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing: Vec<crate::Text>,
}
impl crate::ResponseBody for Saved {
    const SCHEMA: &'static str = "biomcp.saved/1";
    type Query = crate::EmptyQuery;
}
