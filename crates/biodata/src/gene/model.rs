use super::{GeneStatus, LocusGroup, LocusType, Mane, Summary};
use crate::{
    Cytoband, EnsemblGeneId, GeneSymbol, GenomicInterval, HgncId, NcbiGeneId, OmimId, Text,
    UniProtAcc,
};
use serde::{Deserialize, Serialize};

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
    pub entrez_id: Option<NcbiGeneId>,
    /// Ensembl gene identifier, if stated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<EnsemblGeneId>,
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
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub mane: Box<[Mane]>,
    /// Source intervals with explicit assembly and counting rule.
    pub spans: Box<[GenomicInterval]>,
    /// Gene name.
    pub name: Text,
    /// Whole attributed summary, if stated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Summary>,
}
