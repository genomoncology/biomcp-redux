use crate::{
    Attribution, Concept, Curie, PageInfo, PartialDate, Pmid, Statement, Text, VariantAssertionsGet,
};
use serde::{Deserialize, Serialize};
/// One source-ordered page, with native record and classification boundaries intact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantAssertions {
    /// Source assertion rows in native order.
    pub assertions: Box<[VariantAssertion]>,
    /// Page bounds over native rows.
    pub page: PageInfo,
    /// Typed continuation preserving source and selectors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<VariantAssertionsGet>,
}
impl super::ResponseBody for VariantAssertions {
    const SCHEMA: &'static str = "biomcp.variant_assertions/1";
    type Query = VariantAssertionsGet;
}
/// A native record/domain pair, never a vote or merged classification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantAssertion {
    /// Aggregate/submission/document context.
    pub kind: AssertionKind,
    /// Source classification domain.
    pub domain: AssertionDomain,
    /// Native category has no approved common-model proposition.
    #[serde(default, skip_serializing_if = "is_false")]
    pub statement_unmapped: bool,
    /// Supported mapping into the common model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<Statement>,
    /// Native assertion whose category the common model cannot represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmapped: Option<UnmappedAssertion>,
    /// Original source detail, returned only for an explicitly selected record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<AssertionDetail>,
}
impl VariantAssertion {
    /// Record provenance without relying on the clinical mapping.
    pub fn attribution(&self) -> Option<&Attribution> {
        self.statement
            .as_ref()
            .map(|s| &s.attribution)
            .or_else(|| self.unmapped.as_ref().map(|s| &s.attribution))
    }
}
/// Source classification retained when its category has no approved shared proposition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UnmappedAssertion {
    /// Unchanged source label.
    pub classification: Text,
    /// Original record provenance.
    pub attribution: Attribution,
    /// Native review status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<Text>,
    /// Native conditions, including identifiers without supplied labels.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub conditions: Box<[AssertionCondition]>,
}
/// Source details that do not fit the compact shared statement.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AssertionDetail {
    /// Assertion method labels, not inferred from clinical testing.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub methods: Box<[Text]>,
    /// All source conditions; ID-only conditions stay ID-only.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub conditions: Box<[AssertionCondition]>,
    /// Source origin labels without reducing multiple observations.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub origins: Box<[Text]>,
    /// Source citations, including assertion-method citations.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub citations: Box<[Pmid]>,
    /// Source prose, kept separately from structured criteria.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Text>,
    /// Criteria explicitly marked unmet.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub unmet_criteria: Box<[Text]>,
    /// Source publication date; never an inferred evaluation date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<PartialDate>,
    /// Source approval date, distinct from evaluation and publication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<PartialDate>,
    /// Native document versions offered by the source.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub versions: Box<[Text]>,
}
/// A native condition with independently optional identifier and label.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AssertionCondition {
    /// Identifier exactly supplied by the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Curie>,
    /// Label only when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Text>,
}
impl AssertionCondition {
    /// A common concept exists only when the source supplied a label.
    pub fn concept(&self) -> Option<Concept> {
        self.label.clone().map(|label| Concept {
            id: self.id.clone(),
            label,
        })
    }
}
crate::macros::wire_enum! {
    /// Native record scope.
    AssertionKind {
        /// Variant aggregate.
        Vcv=>"vcv",
        /// Variant-condition aggregate.
        Rcv=>"rcv",
        /// Submitted classification.
        Scv=>"scv",
        /// ClinGen expert-panel document.
        Erepo=>"erepo",
    }
}
crate::macros::wire_enum! {
    /// Source classification domains remain separate.
    AssertionDomain {
        /// Germline source classification.
        Germline=>"germline",
        /// Somatic clinical impact.
        SomaticClinicalImpact=>"somatic_clinical_impact",
        /// Somatic oncogenicity.
        Oncogenicity=>"oncogenicity",
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}
