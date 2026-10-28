use super::{GeneStatus, LocusGroup, LocusType, Mane, Summary};
use crate::{
    EnsemblGeneId, GeneLocation, GeneSymbol, GenomicInterval, HgncId, NcbiGeneId, OmimId, Text,
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
    /// Source-stated location, including unparsed publisher text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeneLocation>,
    /// All stated previous symbols.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub previous_symbols: Box<[GeneSymbol]>,
    /// All stated HGNC aliases.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub aliases: Box<[Text]>,
    /// All stated UniProt base accessions.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub uniprot: Box<[UniProtAcc]>,
    /// All stated OMIM identifiers.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub omim: Box<[OmimId]>,
    /// Versioned MANE Select transcript pair, if stated.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub mane: Box<[Mane]>,
    /// Source intervals with explicit assembly and counting rule.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub spans: Box<[GenomicInterval]>,
    /// Gene name.
    pub name: Text,
    /// Whole attributed summary, if stated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Summary>,
}
