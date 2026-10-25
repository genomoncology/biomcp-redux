//! Research compatibility projections; the CLI retains the entity schemas.
use crate::{Problem, SourceOutcome};
use serde::{Deserialize, Serialize};

/// A citable exact source record, without inferred identity or relevance.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ResearchHit {
    /// Exact quoted get argument list, excluding the get verb.
    pub id: String,
    /// Original source title, or its exact identifier when no title is supplied.
    pub title: crate::Text,
    /// Nonempty canonical source record URL.
    pub url: crate::Text,
}
/// Forward-compatible envelope metadata; entity fields use their CLI schemas.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ResearchEnvelope {
    /// Original source outcomes, including their terms.
    #[serde(default)]
    pub sources: Vec<SourceOutcome>,
    /// Original problems, without translation.
    #[serde(default)]
    pub problems: Vec<Problem>,
}
/// Search disclosures preserve the original CLI response without inventing hits.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ResearchSearchMetadata<M> {
    /// Complete original CLI envelope.
    pub envelope: M,
    /// Native rows that lack an exact citable get target.
    pub omitted_rows: u32,
    /// Explains where unprojected rows remain available.
    pub note: String,
}
/// Research search response.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ResearchSearch<M> {
    /// Citable records in unchanged CLI order.
    pub results: Vec<ResearchHit>,
    /// Original response and projection disclosures.
    pub metadata: ResearchSearchMetadata<M>,
}
/// Research fetch response using the existing CLI Markdown view.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ResearchFetch<M> {
    /// Unchanged caller identifier.
    pub id: String,
    /// Original source title, or exact identifier.
    pub title: crate::Text,
    /// Complete Markdown view of the bounded CLI record.
    pub text: String,
    /// Nonempty canonical source URL.
    pub url: crate::Text,
    /// Complete original CLI envelope.
    pub metadata: M,
}
