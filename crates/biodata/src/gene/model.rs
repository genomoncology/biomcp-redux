use super::{GeneStatus, HgncStatus, LocusGroup, LocusType, ManeSelect, MatchKind, Summary};
use crate::{
    Cytoband, EnsemblGeneId, EntrezGeneId, GeneSymbol, GenomicInterval, HgncId, OmimId, Text,
    UniProtAcc,
};
use serde::{Deserialize, Serialize};

/// A source-stated match, preserving the text that matched.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Match {
    /// The match category.
    pub kind: MatchKind,
    /// The source text that matched the input.
    pub text: Text,
}
/// Enough gene identity to choose a search result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GeneHeader {
    /// HGNC identifier.
    pub hgnc_id: HgncId,
    /// Source-preserved symbol.
    pub symbol: GeneSymbol,
    /// Gene name.
    pub name: Text,
    /// HGNC locus group.
    pub locus_group: LocusGroup,
    /// A single stated band, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Cytoband>,
    /// HGNC record status.
    pub status: GeneStatus,
    /// Match information when this header answers an input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<Match>,
}
/// Human gene identity and whole source annotations, independent of response cards.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Gene {
    /// HGNC identifier.
    pub hgnc_id: HgncId,
    /// Source-preserved symbol.
    pub symbol: GeneSymbol,
    /// NCBI Gene identifier, if stated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrez_id: Option<EntrezGeneId>,
    /// Ensembl gene identifier, if stated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<EnsemblGeneId>,
    /// Gene name.
    pub name: Text,
    /// HGNC record status.
    pub status: GeneStatus,
    /// HGNC locus group.
    pub locus_group: LocusGroup,
    /// HGNC locus type, if stated and recognized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locus_type: Option<LocusType>,
    /// Single stated band, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Cytoband>,
    /// All stated previous symbols.
    pub previous_symbols: Box<[GeneSymbol]>,
    /// All stated HGNC aliases.
    pub aliases: Box<[Text]>,
    /// All stated UniProt base accessions.
    pub uniprot: Box<[UniProtAcc]>,
    /// All stated OMIM identifiers.
    pub omim: Box<[OmimId]>,
    /// Versioned MANE Select transcript pair, if stated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mane_select: Option<ManeSelect>,
    /// Source intervals with explicit assembly and counting rule.
    pub spans: Box<[GenomicInterval]>,
    /// Whole attributed summary, if stated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Summary>,
}
/// Resolution outcomes preserve ambiguity and withdrawn identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "resolution", rename_all = "snake_case", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub enum GeneResolution {
    /// One resolved identity and its source match.
    Resolved {
        /// The whole gene entity.
        gene: Box<Gene>,
        /// How the source matched the input.
        r#match: Match,
    },
    /// Every candidate, with no silent selection.
    Ambiguous {
        /// Ordered candidates supplied by the resolver.
        candidates: Box<[GeneHeader]>,
    },
    /// The original withdrawn identity and stated successors.
    Withdrawn {
        /// Original HGNC identifier.
        hgnc_id: HgncId,
        /// Original symbol.
        symbol: GeneSymbol,
        /// Stated HGNC withdrawn-file status.
        hgnc_status: HgncStatus,
        /// All stated merge destinations.
        merged_into: Box<[HgncId]>,
    },
    /// No source identity matched.
    NotFound {},
}
