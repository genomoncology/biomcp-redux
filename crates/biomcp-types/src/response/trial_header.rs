use crate::{NctId, PartialDate, Text, TrialPhase, TrialStatus};
use serde::{Deserialize, Serialize};
/// Small source-attributed trial discovery record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(transform = narrowable))]
pub struct TrialHeader {
    /// Exact registry identity.
    pub nct_id: NctId,
    /// Source brief title, absent when missing or rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Text>,
    /// Registry overall status, absent when missing or rejected; independent of site status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrialStatus>,
    /// Source phases; absent phases remain absent.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub phases: Box<[TrialPhase]>,
    /// Source condition names.
    pub conditions: crate::Capped<Text>,
    /// Source intervention names.
    pub interventions: crate::Capped<Text>,
    /// Lead sponsor as registered, absent when the row carries none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sponsor: Option<Text>,
    /// Source start date with its actual or estimated qualifier, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<biodata::TrialDate>,
    /// Source primary completion date with its qualifier, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_completion: Option<biodata::TrialDate>,
    /// Whether the registry has posted results, absent when the row carries no flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_results: Option<bool>,
    /// Last posted registry update, absent when missing or rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<PartialDate>,
    /// Publisher's canonical record URL.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Text,
}

/// Default rows always carry these; a caller's chosen fields may leave them out.
#[cfg(feature = "schema")]
fn narrowable(schema: &mut schemars::Schema) {
    if let Some(required) = schema
        .get_mut("required")
        .and_then(|required| required.as_array_mut())
    {
        required
            .retain(|name| !matches!(name.as_str(), Some("conditions" | "interventions" | "url")));
    }
}
