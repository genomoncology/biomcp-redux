use biodata::Text;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Allele {
    #[serde(rename = "@id")]
    pub id: Option<Text>,
    #[serde(rename = "errorType")]
    pub error_type: Option<Text>,
    #[serde(default, rename = "genomicAlleles")]
    pub genomic: Vec<Genomic>,
    #[serde(default, rename = "externalRecords")]
    pub external: External,
}
#[derive(Default, Deserialize)]
pub(super) struct External {
    #[serde(default, rename = "dbSNP")]
    pub rs: Vec<Rs>,
    #[serde(default, rename = "ClinVarVariations")]
    pub clinvar: Vec<Clinvar>,
}
#[derive(Deserialize)]
pub(super) struct Rs {
    pub rs: u64,
}
#[derive(Deserialize)]
pub(super) struct Clinvar {
    #[serde(rename = "variationId")]
    pub id: u64,
}
#[derive(Deserialize)]
pub(super) struct Genomic {
    pub chromosome: Option<biodata::Contig>,
    #[serde(rename = "referenceGenome")]
    pub assembly: Option<Text>,
    #[serde(rename = "referenceSequence")]
    pub reference: Text,
    pub hgvs: Vec<Text>,
    pub coordinates: Vec<Coordinate>,
}
#[derive(Deserialize)]
pub(super) struct Coordinate {
    pub start: u32,
    pub end: u32,
    #[serde(rename = "referenceAllele")]
    pub reference: String,
    pub allele: String,
}
impl Reply for Allele {
    fn check(&self) -> Result<(), BadReply> {
        if let Some(code) = &self.error_type {
            return if self.id.is_none() && code.as_str().len() <= 128 && code.as_str().bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') { Ok(()) } else { Err(BadReply) };
        }
        let key = self
            .id
            .as_ref()
            .and_then(|v| v.as_str().strip_prefix("http://reg.genome.network/allele/"));
        if !key.is_some_and(|k| {
            k.parse::<biodata::VariantInput>()
                .is_ok_and(|v| v.namespace().ok() == Some(biodata::VariantNamespace::Ca))
        }) || self.genomic.is_empty()
            || self.genomic.len() > 1000
        {
            return Err(BadReply);
        }
        if self.genomic.iter().any(|g| {
            g.hgvs.is_empty()
                || g.coordinates.is_empty()
                || g.coordinates.iter().any(|c| c.start > c.end)
        }) {
            return Err(BadReply);
        }
        Ok(())
    }
}
