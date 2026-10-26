use biodata::{PartialDate, ProteinAudit, Text, UniProtIsoform};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Entry {
    pub primary_accession: UniProtIsoform,
    pub uni_protkb_id: Text,
    pub entry_type: Text,
    pub organism: Organism,
    pub protein_description: Description,
    pub sequence: Sequence,
    pub genes: Option<Vec<Gene>>,
    pub entry_audit: Option<Audit>,
}
impl Entry {
    pub(in crate::uniprot) fn admitted(&self, id: &UniProtIsoform) -> bool {
        self.primary_accession == *id && self.human_reviewed()
    }
    pub(in crate::uniprot) fn human_reviewed(&self) -> bool {
        self.organism.taxon_id == 9606
            && self.entry_type.as_str() == "UniProtKB reviewed (Swiss-Prot)"
    }
}
impl Reply for Entry {
    fn check(&self) -> Result<(), BadReply> {
        if self.sequence.length == 0 {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Organism {
    pub taxon_id: u32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Description {
    pub recommended_name: Name,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Name {
    pub full_name: Value,
}
#[derive(Debug, Deserialize)]
pub(in crate::uniprot) struct Value {
    pub value: Text,
}
#[derive(Debug, Deserialize)]
pub(in crate::uniprot) struct Sequence {
    pub length: u32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Gene {
    pub gene_name: Option<Value>,
    pub synonyms: Option<Vec<Value>>,
    pub ordered_locus_names: Option<Vec<Value>>,
    pub orf_names: Option<Vec<Value>>,
}
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::uniprot) struct Audit {
    pub entry_version: Option<u32>,
    pub sequence_version: Option<u32>,
    pub first_public_date: Option<PartialDate>,
    pub last_annotation_update_date: Option<PartialDate>,
    pub last_sequence_update_date: Option<PartialDate>,
}
impl From<Audit> for ProteinAudit {
    fn from(a: Audit) -> Self {
        Self {
            entry_version: a.entry_version,
            sequence_version: a.sequence_version,
            first_public: a.first_public_date,
            last_annotation_update: a.last_annotation_update_date,
            last_sequence_update: a.last_sequence_update_date,
            release: None,
        }
    }
}
