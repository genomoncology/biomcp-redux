use crate::raw::Field;
use biodata::{NcbiGeneId, Text};

use biomcp_fetch::{BadReply, Reply};
use biomcp_types::{Oncokb, OncokbCard, OncokbGeneType, OncokbTranscript};
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(transparent)]
pub(super) struct Genes(pub Vec<Gene>);
impl Reply for Genes {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Gene {
    #[serde(deserialize_with = "entrez")]
    pub entrez_gene_id: NcbiGeneId,
    hugo_symbol: biodata::GeneSymbol,
    gene_type: OncokbGeneType,
    highest_sensitive_level: Option<Field<String>>,
    highest_resistance_level: Option<Field<String>>,
    summary: Option<Field<String>>,
    background: Option<Field<String>>,
    grch37_isoform: Option<Field<String>>,
    #[serde(rename = "grch37RefSeq")]
    grch37_refseq: Option<Field<String>>,
    grch38_isoform: Option<Field<String>>,
    #[serde(rename = "grch38RefSeq")]
    grch38_refseq: Option<Field<String>>,
}
impl Gene {
    pub(super) fn mapped(self, problems: &mut Vec<biomcp_types::Problem>) -> super::GeneData {
        let mut text = |value, field| {
            crate::mapped::field_text(
                problems,
                biomcp_types::Source::Oncokb,
                Text::new(&self.entrez_gene_id.to_string()).ok(),
                field,
                value,
            )
        };
        let _symbol = self.hugo_symbol;
        super::GeneData {
            card: OncokbCard {
                gene_type: self.gene_type,
                highest_sensitive_level: text(
                    self.highest_sensitive_level,
                    "highest_sensitive_level",
                ),
                highest_resistance_level: text(
                    self.highest_resistance_level,
                    "highest_resistance_level",
                ),
                version: None,
            },
            section: Oncokb {
                version: None,
                summary: text(self.summary, "summary"),
                background: text(self.background, "background"),
                grch37: OncokbTranscript {
                    isoform: text(self.grch37_isoform, "grch37.isoform"),
                    refseq: text(self.grch37_refseq, "grch37.refseq"),
                },
                grch38: OncokbTranscript {
                    isoform: text(self.grch38_isoform, "grch38.isoform"),
                    refseq: text(self.grch38_refseq, "grch38.refseq"),
                },
            },
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Info {
    pub data_version: Version,
}
#[derive(Deserialize)]
pub(super) struct Version {
    pub version: Text,
}
impl Reply for Info {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}

fn entrez<'de, D: serde::Deserializer<'de>>(d: D) -> Result<NcbiGeneId, D::Error> {
    let value = u32::deserialize(d)?;
    NcbiGeneId::new(value).map_err(serde::de::Error::custom)
}
