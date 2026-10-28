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
    /// Source gene curation flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene_exist: Option<bool>,
    /// Source investigated-but-unknown flag, never used to create a classification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vus: Option<bool>,
    /// Source hotspot flag, without an actionability conclusion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotspot: Option<bool>,
    /// Native alternate allele curation flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allele_exist: Option<bool>,
    /// Source claims and text in native field order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
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
    /// Native non-PubMed citation, preserving source text and link.
    Citation {
        /// Provider collection owning this citation.
        field: Box<str>,
        /// Native text, without trimming.
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<Box<str>>,
        /// Source link.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        link: Option<Text>,
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
        /// Native PMID citations remain available when the source subject is absent.
        #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
        citations: Box<[crate::Pmid]>,
        /// No subject was invented.
        #[serde(default = "unmapped")]
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
        /// Reserved common statement slot; native alteration groups leave it absent.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        statement: Option<Box<Statement>>,
        /// Native grouping cannot be represented by a common source profile ID.
        #[serde(default = "unmapped")]
        statement_unmapped: bool,
        /// Source PMID citations.
        #[serde(default)]
        #[serde(skip_serializing_if = "<[_]>::is_empty")]
        citations: Box<[crate::Pmid]>,
        /// Native non-PubMed citations, with their complete source content and links.
        #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
        abstracts: Box<[OncokbCitation]>,
        /// Source narrative, kept without trimming.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<Box<str>>,
        /// Native excluded contexts, never filled from the caller context.
        #[serde(default)]
        #[serde(skip_serializing_if = "<[_]>::is_empty")]
        excluded_tumor_types: Box<[Concept]>,
        /// Native approved indication labels.
        #[serde(default)]
        #[serde(skip_serializing_if = "<[_]>::is_empty")]
        approved_indications: Box<[Text]>,
        /// Native FDA level label, independent of the evidence level.
        #[serde(skip_serializing_if = "Option::is_none")]
        fda_level: Option<Term>,
    },
}

fn unmapped() -> bool {
    true
}

/// One native non-PubMed source citation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OncokbCitation {
    /// Complete native citation content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// Native citation link.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub link: Option<Text>,
}
