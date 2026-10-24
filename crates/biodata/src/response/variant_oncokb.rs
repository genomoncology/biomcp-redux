use crate::{Build, Concept, PageInfo, Statement, Term, Text, VariantGet};
use serde::{Deserialize, Serialize};

/// Keyed OncoKB annotation, preserving the caller's assembly and tumor context.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VariantOncokb {
    /// Exact genomic query sent to OncoKB.
    pub hgvsg: Text,
    /// Explicit caller assembly, checked against the response query.
    pub build: Build,
    /// Explicit caller tumor type, checked against the response query.
    pub tumor_type: Text,
    /// Native dataset version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_version: Option<Text>,
    /// Native release date, kept in its published spelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<Text>,
    /// Native exact variant curation flag; never a clinical conclusion.
    pub variant_exist: bool,
    /// Native alternate allele curation flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allele_exist: Option<bool>,
    /// Source claims and text in native field order.
    pub records: Box<[OncokbVariantRecord]>,
    /// Disclosure bounds over the records, independent of identity paging.
    pub page: PageInfo,
    /// Continue without changing identity, build, tumor context or source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<VariantGet>,
}

/// A native annotation row; no level or classification is inferred from other fields.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum OncokbVariantRecord {
    /// A complete source statement with its original label.
    Statement {
        /// Common source-attributed claim.
        statement: Box<Statement>,
    },
    /// Native narrative, split only at checked character boundaries.
    Text {
        /// Original provider field name.
        field: Box<str>,
        /// Character offset into this provider field.
        offset: u32,
        /// Complete number of characters in this provider field.
        total: u32,
        /// This exact text fragment, without trimming or rewriting.
        text: Box<str>,
    },
    /// A source label without a representable source protein subject.
    UnmappedLabel {
        /// Native field.
        field: Box<str>,
        /// Whole native classification.
        classification: Term,
        /// No subject was invented.
        statement_unmapped: bool,
    },
    /// Native therapeutic, diagnostic or prognostic implication.
    Implication {
        /// Original provider collection name.
        field: Box<str>,
        /// Source evidence level, without a clinical interpretation.
        level: Term,
        /// Native tumor type; never replaced with the request's context.
        #[serde(skip_serializing_if = "Option::is_none")]
        tumor_type: Option<Concept>,
        /// Source-listed drugs, preserving order.
        #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
        drugs: Box<[Concept]>,
        /// Source alteration labels, which may describe a group or protein subject.
        #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
        alterations: Box<[Text]>,
        /// Native grouping cannot be represented by a common source profile ID.
        statement_unmapped: bool,
        /// Source PMID citations.
        citations: Box<[crate::Pmid]>,
        /// Source narrative, kept without trimming.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<Box<str>>,
        /// Native excluded contexts, never filled from the caller context.
        excluded_tumor_types: Box<[Concept]>,
        /// Native approved indication labels.
        approved_indications: Box<[Text]>,
        /// Native FDA level label, independent of the evidence level.
        #[serde(skip_serializing_if = "Option::is_none")]
        fda_level: Option<Term>,
    },
}
