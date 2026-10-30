use super::{OrcidClientId, OrcidId};
use crate::{PartialDate, Text};
use serde::{Deserialize, Serialize};
/// Only public source visibility is admitted; legacy uppercase is an alias.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = ["public", "PUBLIC"])))]
pub enum OrcidPublicVisibility {
    /// A public source assertion.
    #[serde(rename = "public", alias = "PUBLIC")]
    Public,
}
/// Native ORCID external identifier before any article pivot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
pub struct OrcidExternalId {
    /// Open native external-id-type.
    pub namespace: Text,
    /// Unchanged native identifier value.
    pub id: Text,
    /// Open native relationship, including self and part-of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pub relationship: Option<Text>,
    /// Native identifier URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Option<Text>,
}
/// Source and assertion origin remain distinct provider claims.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OrcidAssertionSource {
    /// Native source ORCID iD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_orcid: Option<OrcidId>,
    /// Native ORCID client ID; no person identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_client_id: Option<OrcidClientId>,
    /// Native source display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<Text>,
    /// Native assertion origin ORCID iD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_origin_orcid: Option<OrcidId>,
    /// Native origin client ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_origin_client_id: Option<OrcidClientId>,
    /// Native origin display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertion_origin_name: Option<Text>,
}
/// One public summary, retained in native group order without representative selection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OrcidWorkAssertion {
    /// Source assertion identifier scoped to this ORCID record.
    pub put_code: std::num::NonZeroU64,
    /// Native title; some summaries have none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Text>,
    /// Native subtitle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<Text>,
    /// Native translated title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translated_title: Option<Text>,
    /// Open native work type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pub work_type: Option<Text>,
    /// Native partial publication date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<PartialDate>,
    /// Native journal title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_title: Option<Text>,
    /// Native work URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Option<Text>,
    /// Every native assertion identifier in order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub external_ids: Box<[OrcidExternalId]>,
    /// Native source and origin provenance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<OrcidAssertionSource>,
    /// Only public assertions belong to this product.
    pub visibility: OrcidPublicVisibility,
    /// Native display index retained without sorting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_index: Option<Text>,
}
/// One native work group is one group, regardless of assertion multiplicity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OrcidWorkGroup {
    /// Native group identifiers in source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub external_ids: Box<[OrcidExternalId]>,
    /// Every public summary in native source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub assertions: Box<[OrcidWorkAssertion]>,
}
impl std::str::FromStr for OrcidPublicVisibility {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "public" | "PUBLIC" => Ok(Self::Public),
            _ => Err(crate::Error::AuthorIdSyntax),
        }
    }
}
impl std::fmt::Display for OrcidPublicVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("public")
    }
}
