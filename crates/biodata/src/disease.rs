use crate::{Attribution, Concept, Curie, FiniteNumber, Text};
use serde::{Deserialize, Serialize};
/// A native source association without clinical interpretation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseAssociation {
    /// The exact requested concept, including a queried parent.
    pub requested_id: Curie,
    /// Native returned subject.
    pub subject: Concept,
    /// Native source predicate.
    pub predicate: Curie,
    /// Native returned object.
    pub object: Concept,
    /// Original source subject before source normalization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_subject: Option<Curie>,
    /// Original source object before source normalization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_object: Option<Curie>,
    /// Native row provenance and version.
    pub attribution: Attribution,
    /// Native primary knowledge source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_knowledge_source: Option<Curie>,
    /// Native aggregator knowledge source, distinct from the providing collection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregator_knowledge_source: Option<Curie>,
    /// Native providing collection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provided_by: Option<Text>,
    /// Source evidence terms without interpretation.
    pub evidence: Box<[Concept]>,
    /// Native qualifiers, including frequency or onset if supplied.
    pub qualifiers: Box<[Concept]>,
    /// Source negation; absent is not false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negated: Option<bool>,
    /// Native subject closure.
    pub subject_closure: Box<[Curie]>,
    /// Native object closure.
    pub object_closure: Box<[Curie]>,
}
/// A publisher relation with distinct endpoints and predicate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseRelation {
    /// Unchanged native cross-reference description when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Native subject.
    pub subject: DiseaseEndpoint,
    /// Publisher predicate, without equivalence promotion.
    pub predicate: Curie,
    /// Native object.
    pub object: DiseaseEndpoint,
    /// Publisher record and version.
    pub attribution: Attribution,
}
/// A publisher synonym with its scope and language.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseSynonym {
    /// Complete synonym text.
    pub value: Text,
    /// Publisher scope predicate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Curie>,
    /// Source language tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Text>,
    /// Publisher record and version.
    pub attribution: Attribution,
}
/// One source-native selector without a local interpretation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StatisticSelector {
    /// Source field name.
    pub name: Text,
    /// Source selector code.
    pub code: Text,
    /// Publisher label when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Text>,
}
/// One native statistics row with requested and returned dimensions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseStatistic {
    /// Explicit selected native site ID.
    pub requested_site: Text,
    /// Settings sent to the publisher.
    pub requested_selectors: Box<[StatisticSelector]>,
    /// Settings actually returned, never filled from request.
    pub returned_selectors: Box<[StatisticSelector]>,
    /// Native annual row year.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
    /// Native measure name.
    pub measure: Text,
    /// Observed estimate; missing stays absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed: Option<StatisticEstimate>,
    /// Modeled estimate kept separately.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modeled: Option<StatisticEstimate>,
    /// Publisher record and dataset version when supplied.
    pub attribution: Attribution,
}
/// Source estimate and uncertainty without computed quantities.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StatisticEstimate {
    /// Native estimate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<FiniteNumber>,
    /// Native standard error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_error: Option<FiniteNumber>,
    /// Native lower confidence bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_ci: Option<FiniteNumber>,
    /// Native upper confidence bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_ci: Option<FiniteNumber>,
    /// Native count; no inferred denominator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

/// One native endpoint with an identifier and an optional source label.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseEndpoint {
    /// Exact source identifier.
    pub id: Curie,
    /// Source label when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Text>,
}
