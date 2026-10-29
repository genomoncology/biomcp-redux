use crate::native_text::NativeText;
use biodata::Text;
use biomcp_fetch::{BadReply, Reply};
use biomcp_types::{GwasAncestry, GwasCountry, GwasLocus, GwasReportedGene};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Links {
    #[serde(rename = "self")]
    pub current: Link,
}
#[derive(Debug, Deserialize)]
pub(super) struct Link {
    pub href: Text,
}
pub(super) fn owned(url: &Text, prefix: &str) -> bool {
    url.as_str()
        .strip_prefix(prefix)
        .is_some_and(|id| !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()))
}
#[derive(Debug, Deserialize)]
pub(super) struct Ancestries {
    #[serde(rename = "_embedded")]
    pub embedded: AncestryRows,
}
#[derive(Debug, Deserialize)]
pub(super) struct AncestryRows {
    pub ancestries: Vec<Ancestry>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Ancestry {
    #[serde(rename = "type")]
    stage: Text,
    number_of_individuals: Option<u64>,
    ancestral_groups: Vec<Group>,
    country_of_origin: Vec<Country>,
    country_of_recruitment: Vec<Country>,
    #[serde(rename = "_links")]
    pub links: Links,
}
#[derive(Debug, Deserialize)]
struct Group {
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    ancestral_group: biodata::Text,
}
#[derive(Debug, Deserialize)]
struct Country {
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    country_name: biodata::Text,
    major_area: Option<NativeText>,
    region: Option<NativeText>,
}
impl Country {
    fn row(self) -> GwasCountry {
        GwasCountry {
            country: self.country_name,
            major_area: self.major_area.map(Into::into),
            region: self.region.map(Into::into),
        }
    }
}
impl Ancestry {
    pub(super) fn row(self) -> GwasAncestry {
        GwasAncestry {
            stage: self.stage,
            number_of_individuals: self.number_of_individuals,
            ancestry_groups: self
                .ancestral_groups
                .into_iter()
                .map(|g| g.ancestral_group)
                .collect(),
            country_of_origin: self
                .country_of_origin
                .into_iter()
                .map(Country::row)
                .collect(),
            country_of_recruitment: self
                .country_of_recruitment
                .into_iter()
                .map(Country::row)
                .collect(),
            source_url: self.links.current.href,
        }
    }
}
impl Reply for Ancestries {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Loci {
    #[serde(rename = "_embedded")]
    pub embedded: LocusRows,
}
#[derive(Debug, Deserialize)]
pub(super) struct LocusRows {
    pub loci: Vec<Locus>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Locus {
    description: Option<NativeText>,
    strongest_risk_alleles: Vec<Allele>,
    author_reported_genes: Vec<Gene>,
    #[serde(rename = "_links")]
    pub links: Links,
}
#[derive(Debug, Deserialize)]
struct Allele {
    risk_allele_name: Text,
}
#[derive(Debug, Deserialize)]
struct Gene {
    gene_name: Option<NativeText>,
    ensembl_gene_ids: Vec<Text>,
    entrez_gene_ids: Vec<Text>,
}
impl Locus {
    pub(super) fn row(self) -> GwasLocus {
        GwasLocus {
            description: self.description.map(Into::into),
            strongest_risk_alleles: self
                .strongest_risk_alleles
                .into_iter()
                .map(|a| a.risk_allele_name)
                .collect(),
            author_reported_genes: self
                .author_reported_genes
                .into_iter()
                .map(|g| GwasReportedGene {
                    name: g.gene_name.map(Into::into),
                    ensembl_ids: g.ensembl_gene_ids.into(),
                    entrez_ids: g.entrez_gene_ids.into(),
                })
                .collect(),
            source_url: Some(self.links.current.href),
        }
    }
}
impl Reply for Loci {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
