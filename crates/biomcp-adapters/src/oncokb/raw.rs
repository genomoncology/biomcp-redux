use biodata::{EntrezGeneId, Oncokb, OncokbCard, OncokbGeneType, OncokbTranscript, Text};
use biomcp_fetch::{BadReply, Reply};
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
    pub entrez_gene_id: EntrezGeneId,
    hugo_symbol: biodata::GeneSymbol,
    gene_type: OncokbGeneType,
    #[serde(default, deserialize_with = "optional_text")]
    highest_sensitive_level: Option<Text>,
    #[serde(default, deserialize_with = "optional_text")]
    highest_resistance_level: Option<Text>,
    #[serde(default, deserialize_with = "optional_text")]
    summary: Option<Text>,
    #[serde(default, deserialize_with = "optional_text")]
    background: Option<Text>,
    #[serde(default, deserialize_with = "optional_text")]
    grch37_isoform: Option<Text>,
    #[serde(rename = "grch37RefSeq")]
    #[serde(default, deserialize_with = "optional_text")]
    grch37_refseq: Option<Text>,
    #[serde(default, deserialize_with = "optional_text")]
    grch38_isoform: Option<Text>,
    #[serde(rename = "grch38RefSeq")]
    #[serde(default, deserialize_with = "optional_text")]
    grch38_refseq: Option<Text>,
}
impl Gene {
    pub(super) fn mapped(self) -> super::GeneData {
        let _symbol = self.hugo_symbol;
        super::GeneData {
            card: OncokbCard {
                gene_type: self.gene_type,
                highest_sensitive_level: self.highest_sensitive_level,
                highest_resistance_level: self.highest_resistance_level,
                version: None,
            },
            section: Oncokb {
                version: None,
                summary: self.summary,
                background: self.background,
                grch37: OncokbTranscript {
                    isoform: self.grch37_isoform,
                    refseq: self.grch37_refseq,
                },
                grch38: OncokbTranscript {
                    isoform: self.grch38_isoform,
                    refseq: self.grch38_refseq,
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

fn entrez<'de, D: serde::Deserializer<'de>>(d: D) -> Result<EntrezGeneId, D::Error> {
    let value = u32::deserialize(d)?;
    EntrezGeneId::new(value).map_err(serde::de::Error::custom)
}

fn optional_text<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<Text>, D::Error> {
    let value = Option::<String>::deserialize(d)?;
    Ok(value.and_then(|text| Text::new(&text).ok()))
}
