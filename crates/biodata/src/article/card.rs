use crate::{Capped, Doi, PartialDate, Pmcid, Pmid, PublicationType, Text};
use serde::{Deserialize, Serialize};
/// Complete PubMed article facts; card disclosure caps authors without dropping their total.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Article<A = Box<[Text]>> {
    /// PubMed identity.
    pub pmid: Pmid,
    /// DOI assigned to this article.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<Doi>,
    /// PMC identity assigned to this article.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmcid: Option<Pmcid>,
    /// Whole title with inline text preserved.
    pub title: Text,
    /// Journal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal: Option<Text>,
    /// Publication date at source precision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<PartialDate>,
    /// Every clinically meaningful source publication type.
    pub types: Box<[PublicationType]>,
    /// Related correction notices or originals.
    pub corrections: Box<[Correction]>,
    /// Authors in source order.
    pub authors: A,
    /// Whole abstract, preserving labels and source order.
    pub abstract_parts: Box<[AbstractPart]>,
}
impl Article {
    /// Disclose the first ten authors and retain the whole count.
    pub fn card(self) -> Article<Capped<Text>> {
        Article { pmid:self.pmid, doi:self.doi, pmcid:self.pmcid, title:self.title,
            journal:self.journal, date:self.date, types:self.types, corrections:self.corrections,
            authors:Capped { total:self.authors.len() as u64,
                items:self.authors.into_iter().take(10).collect() }, abstract_parts:self.abstract_parts }
    }
}
/// One source-labelled abstract paragraph.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AbstractPart {
    /// Source label, when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Text>,
    /// Whole paragraph text.
    pub text: Text,
}
crate::source::canonical_enum! {
    /// Source correction relationship kind.
    CorrectionKind, SourceUnknown {
        /// Retraction relationship.
        Retraction => "retraction",
        /// Erratum relationship.
        Erratum => "erratum",
        /// Expression of concern relationship.
        ExpressionOfConcern => "expression_of_concern",
    }
}
crate::source::canonical_enum! {
    /// Direction of a source correction relationship.
    CorrectionRelation, SourceUnknown {
        /// The related record is the notice.
        Notice => "notice",
        /// The related record is the original.
        Original => "original",
    }
}
/// One correction relationship supplied by PubMed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Correction {
    /// Correction category.
    pub kind: CorrectionKind,
    /// Whether the related record is a notice or original.
    pub relation: CorrectionRelation,
    /// Related PMID, when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmid: Option<Pmid>,
}
