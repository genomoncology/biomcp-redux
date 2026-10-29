use biodata::Text;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Allele {
    #[serde(rename = "@id")]
    pub id: Option<Text>,
    #[serde(rename = "errorType")]
    pub error_type: Option<Text>,
    #[serde(rename = "description")]
    pub error_description: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "genomicAlleles")]
    pub genomic: Option<Vec<Genomic>>,
    #[serde(rename = "transcriptAlleles")]
    pub transcripts: Option<Vec<Transcript>>,
    #[serde(rename = "externalRecords")]
    pub external: Option<External>,
}
#[derive(Deserialize)]
pub(super) struct External {
    #[serde(rename = "dbSNP")]
    pub rs: Option<Vec<Rs>>,
    #[serde(rename = "ClinVarVariations")]
    pub clinvar: Option<Vec<Clinvar>>,
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
    pub chromosome: Option<crate::raw::Field<Text>>,
    #[serde(rename = "referenceGenome")]
    pub assembly: Option<crate::raw::Field<Text>>,
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
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if (200..300).contains(&status) || matches!(status, 400 | 422) && self.error_type.is_some()
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
    fn check(&self) -> Result<(), BadReply> {
        if let Some(code) = &self.error_type {
            return if self.id.is_none()
                && code.as_str().len() <= 128
                && code
                    .as_str()
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_')
            {
                Ok(())
            } else {
                Err(BadReply)
            };
        }
        let genomic = self.genomic.as_deref().ok_or(BadReply)?;
        let key = self
            .id
            .as_ref()
            .and_then(|v| v.as_str().strip_prefix("http://reg.genome.network/allele/"));
        if !key.is_some_and(|k| {
            k.parse::<biomcp_types::VariantInput>()
                .is_ok_and(|v| v.namespace().ok() == Some(biomcp_types::VariantNamespace::Ca))
        }) || genomic.is_empty()
            || genomic.len() > 1000
        {
            return Err(BadReply);
        }
        if genomic.iter().any(|g| {
            g.hgvs.is_empty()
                || g.coordinates.is_empty()
                || g.coordinates.iter().any(|c| c.start > c.end)
        }) {
            return Err(BadReply);
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub(super) struct Transcript {
    #[serde(rename = "MANE")]
    pub mane: Option<Mane>,
}
#[derive(Deserialize)]
pub(super) struct Mane {
    pub nucleotide: Option<ManeForms>,
    pub protein: Option<ManeForms>,
}
#[derive(Deserialize)]
pub(super) struct ManeForms {
    #[serde(rename = "Ensembl")]
    pub ensembl: Option<Hgvs>,
    #[serde(rename = "RefSeq")]
    pub refseq: Option<Hgvs>,
}
#[derive(Deserialize)]
pub(super) struct Hgvs {
    pub hgvs: Text,
}
