use crate::{Reason, Source, Text};
use serde::{Deserialize, Serialize};
/// The source's own entry for an entity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRecord {
    /// Native source identifier.
    pub id: Text,
    /// Link to that entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Text>,
}
/// A requested section has exactly one state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum Section<T> {
    /// The source returned data.
    Data {
        #[doc = "Source identity."]
        source: Source,
        #[doc = "Whole section data."]
        data: T,
    },
    /// The source answered with no data.
    Empty {
        #[doc = "Source identity."]
        source: Source,
        #[doc = "Native entry, if any."]
        record: Option<SourceRecord>,
    },
    /// A requested source failed.
    Unavailable {
        #[doc = "Source identity."]
        source: Source,
        #[doc = "Typed reason."]
        reason: Reason,
    },
    /// User settings prevented the source request.
    Skipped {
        #[doc = "Source identity."]
        source: Source,
        #[doc = "Typed reason."]
        reason: Reason,
    },
    /// Entity facts rule this section out.
    NotApplicable,
}
