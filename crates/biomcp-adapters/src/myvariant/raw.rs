use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Page {
    pub total: u64,
    pub hits: Vec<Hit>,
}
#[derive(Deserialize)]
pub(super) struct Hit {
    #[serde(rename = "_id")]
    pub id: biodata::Text,
    pub dbsnp: Option<Dbsnp>,
    pub snpeff: Option<crate::raw::Field<super::transcripts::Snpeff>>,
    pub clinvar: Option<Clinvar>,
    pub chrom: Option<SourceChromosome>,
    pub hg19: Option<SourceSpan>,
    pub hg38: Option<SourceSpan>,
}
#[derive(Deserialize)]
pub(super) struct Dbsnp {
    pub rsid: Option<OneOrMany>,
    pub gene: Option<Genes>,
}
#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum Genes {
    One(Gene),
    Many(Vec<Gene>),
}
#[derive(Deserialize)]
pub(super) struct Gene {
    pub symbol: Option<biodata::Text>,
}
#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum OneOrMany {
    One(biodata::Text),
    Many(Vec<biodata::Text>),
}
#[derive(Deserialize)]
pub(super) struct Clinvar {
    pub variant_id: u64,
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        if self.total > 10_000 || self.hits.len() > 1000 || self.hits.len() as u64 > self.total {
            return Err(BadReply);
        }
        if self.hits.iter().any(|h| h.check().is_err()) {
            return Err(BadReply);
        }
        Ok(())
    }
}

impl Reply for Hit {
    fn check(&self) -> Result<(), BadReply> {
        if self
            .id
            .as_str()
            .parse::<biomcp_types::VariantInput>()
            .is_ok_and(|i| i.namespace().ok() == Some(biomcp_types::VariantNamespace::Chromosome))
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}

impl Hit {
    pub(super) fn matches(&self, query: &str) -> bool {
        if query.starts_with("rs") {
            self.dbsnp
                .as_ref()
                .and_then(|d| d.rsid.as_ref())
                .is_some_and(|ids| match ids {
                    OneOrMany::One(id) => id.as_str() == query,
                    OneOrMany::Many(ids) => ids.iter().any(|id| id.as_str() == query),
                })
        } else if let Some(id) = query.strip_prefix("clinvar.variant_id:") {
            self.clinvar
                .as_ref()
                .is_some_and(|c| c.variant_id.to_string() == id)
        } else {
            self.id.as_str() == query
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum SourceChromosome {
    Name(biodata::Text),
    Malformed(serde::de::IgnoredAny),
}
#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum SourceSpan {
    Bounds { start: u64, end: u64 },
    Malformed(serde::de::IgnoredAny),
}
