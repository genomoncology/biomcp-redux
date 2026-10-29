//! Source assertions without ranking, merging or clinical inference.
use crate::{GeneRef, PartialDate, Pmid, Text};
use serde::{Deserialize, Serialize};
mod clinvar;
pub use clinvar::{ClinvarAlleleOriginCode, ClinvarReviewStatusCode};
mod coding;
pub use coding::*;
mod reference;
pub use reference::{Curie, VariantRef};
/// One source claim, kept in its own framework and context.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Statement {
    /// Source-owned subject reference.
    pub subject: VariantRef,
    /// Claim domain.
    pub proposition: Proposition,
    /// Source-stated relationship.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    /// Source-stated assessment direction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// Whole source classification label.
    pub classification: Term,
    /// Source evidence level or review status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strength: Option<Term>,
    /// Source disease or tumor context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Concept>,
    /// Source therapies in their native order.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub therapies: Box<[Concept]>,
    /// Source-stated origin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allele_origin: Option<Term>,
    /// Source gene context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gene_context: Option<GeneRef>,
    /// Source criterion labels in source order.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub criteria: Box<[Text]>,
    /// Source publications.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub citations: Box<[Pmid]>,
    /// One source record and its provenance.
    pub attribution: Attribution,
}
/// A source label with explicitly supported codings in source order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Term {
    /// Original label.
    pub label: Text,
    /// Documented framework codings.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub codings: Box<[Coding]>,
}
/// One source concept with its optional ontology identifier.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Concept {
    /// Source-stated concept identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Curie>,
    /// Original concept label.
    pub label: Text,
}
/// Provenance for one source record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Attribution {
    /// Source responsible for the claim.
    pub source: Text,
    /// Native assertion record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<Text>,
    /// Record version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Text>,
    /// Dataset release.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<Text>,
    /// Source submitter or panel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitter: Option<Text>,
    /// Source evaluation date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluated: Option<PartialDate>,
    /// Source record link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Option<Text>,
}
crate::wire_enum! {
    /// Domain of a source claim.

    #[cfg_attr(feature = "schema", schemars(extend("x-enum-open" = true)))]
    Proposition {
        /// Published pathogenicity value.
        Pathogenicity => "pathogenicity",
        /// Published oncogenicity value.
        Oncogenicity => "oncogenicity",
        /// Published clinical_significance value.
        ClinicalSignificance => "clinical_significance",
        /// Published therapeutic_response value.
        TherapeuticResponse => "therapeutic_response",
        /// Published diagnostic value.
        Diagnostic => "diagnostic",
        /// Published prognostic value.
        Prognostic => "prognostic",
        /// Published functional_impact value.
        FunctionalImpact => "functional_impact",
    }
}
crate::wire_enum! {
    /// Assessment direction stated by the source.

    #[cfg_attr(feature = "schema", schemars(extend("x-enum-open" = true)))]
    Direction {
        /// Published supports value.
        Supports => "supports",
        /// Published disputes value.
        Disputes => "disputes",
        /// Published neutral value.
        Neutral => "neutral",
    }
}
crate::wire_enum! {
    /// VA-Spec 1.1 proposition relationships in native snake_case.

    #[cfg_attr(feature = "schema", schemars(extend("x-enum-open" = true)))]
    Predicate {
        /// Published is_causal_for value.
        IsCausalFor => "is_causal_for",
        /// Published is_oncogenic_for value.
        IsOncogenicFor => "is_oncogenic_for",
        /// Published has_clinical_significance_for value.
        HasClinicalSignificanceFor => "has_clinical_significance_for",
        /// Published predicts_sensitivity_to value.
        PredictsSensitivityTo => "predicts_sensitivity_to",
        /// Published predicts_resistance_to value.
        PredictsResistanceTo => "predicts_resistance_to",
        /// Published is_diagnostic_inclusion_criterion_for value.
        IsDiagnosticInclusionCriterionFor => "is_diagnostic_inclusion_criterion_for",
        /// Published is_diagnostic_exclusion_criterion_for value.
        IsDiagnosticExclusionCriterionFor => "is_diagnostic_exclusion_criterion_for",
        /// Published associated_with_better_outcome_for value.
        AssociatedWithBetterOutcomeFor => "associated_with_better_outcome_for",
        /// Published associated_with_worse_outcome_for value.
        AssociatedWithWorseOutcomeFor => "associated_with_worse_outcome_for",
        /// Published impacts_function_of value.
        ImpactsFunctionOf => "impacts_function_of",
    }
}

impl Statement {
    /// Build one explicit source claim with absent qualifiers and empty source lists.
    pub fn new(
        subject: VariantRef,
        proposition: Proposition,
        classification: Term,
        attribution: Attribution,
    ) -> Self {
        Self {
            subject,
            proposition,
            predicate: None,
            direction: None,
            classification,
            strength: None,
            condition: None,
            therapies: Box::default(),
            allele_origin: None,
            gene_context: None,
            criteria: Box::default(),
            citations: Box::default(),
            attribution,
        }
    }
}
