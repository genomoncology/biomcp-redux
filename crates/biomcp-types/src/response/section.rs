use crate::{Reason, Source, Text};
use serde::{Deserialize, Serialize};
/// The source's own entry for an entity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
pub struct SourceRecord {
    /// Native authority; unknown means the source did not state one.
    pub namespace: biodata::IdentifierNamespace,
    /// Native source identifier.
    pub id: Text,
    /// Link to that entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Option<Text>,
}
impl SourceRecord {
    /// Name the identifier a source was asked, under its authority when the
    /// identifier does not carry one, such as `UniProt` `P33981` or `HGNC:1097`.
    pub fn asked(namespace: Option<&str>, id: impl std::fmt::Display) -> Option<Self> {
        Some(Self {
            namespace: match namespace {
                Some(namespace) => Text::new(namespace).ok()?.into(),
                None => biodata::IdentifierNamespace::Unknown,
            },
            id: Text::new(&id.to_string()).ok()?,
            url: None,
        })
    }
}
/// A requested section has exactly one state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "state", rename_all = "kebab-case")]
#[cfg_attr(feature = "schema", schemars(rename = "Section_{T}"))]
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
        #[serde(skip_serializing_if = "Option::is_none")]
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

impl<T> Section<T> {
    /// Preserve policy skips when a source cannot supply a requested section.
    pub fn unavailable(source: Source, reason: Reason) -> Self {
        if matches!(reason, Reason::Disabled | Reason::NoKey) {
            Self::Skipped { source, reason }
        } else {
            Self::Unavailable { source, reason }
        }
    }
}
