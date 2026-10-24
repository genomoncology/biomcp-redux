use biodata::{FiniteNumber, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Response {
    #[serde(rename = "_embedded")]
    pub embedded: Option<Embedded>,
    #[serde(rename = "_links")]
    links: Links,
    pub page: Page,
}
#[derive(Debug, Deserialize)]
pub(super) struct Embedded {
    pub associations: Vec<Association>,
}
#[derive(Debug, Deserialize)]
struct Links {
    #[serde(rename = "self")]
    current: Link,
}
#[derive(Debug, Deserialize)]
struct Link {
    href: Text,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Page {
    size: u32,
    pub total_elements: u64,
    total_pages: u64,
    number: u32,
}
/// A source association retains publisher labels and numbers unchanged.
#[derive(Debug, Deserialize)]
pub struct Association {
    /// Stable association record identity.
    pub association_id: u64,
    /// Catalog study accession.
    pub accession_id: Text,
    /// Source-reported traits.
    pub reported_trait: Vec<Text>,
    /// Source mapped ontology terms.
    pub efo_traits: Vec<Trait>,
    /// Source effect alleles, never interpreted as a classification.
    pub snp_allele: Vec<EffectAllele>,
    /// Reported association p value.
    pub p_value: FiniteNumber,
    /// Reported effect beta, preserving units and prose.
    #[serde(default, deserialize_with = "optional_text")]
    pub beta: Option<Text>,
    /// Reported odds ratio where provided.
    pub or_per_copy_num: Option<FiniteNumber>,
    /// Source allele frequency text, such as NR.
    #[serde(default, deserialize_with = "optional_text")]
    pub risk_frequency: Option<Text>,
    /// Source-reported effect interval.
    #[serde(default, deserialize_with = "optional_text")]
    pub range: Option<Text>,
    /// Source publication identity.
    pub pubmed_id: Option<Text>,
}
/// Source trait identity and label.
#[derive(Debug, Deserialize)]
pub struct Trait {
    /// Source ontology ID.
    pub efo_id: Text,
    /// Whole source label.
    pub efo_trait: Text,
}
/// Effect allele qualified by its exact source rsID.
#[derive(Debug, Deserialize)]
pub struct EffectAllele {
    /// Source rsID.
    pub rs_id: Text,
    /// Source effect allele text, not normalized.
    pub effect_allele: Text,
}
impl Reply for Response {
    fn check(&self) -> Result<(), BadReply> {
        let rows = self
            .embedded
            .as_ref()
            .map(|e| e.associations.as_slice())
            .unwrap_or_default();
        if self.page.size == 0
            || rows.len() > self.page.size as usize
            || self.page.total_pages != self.page.total_elements.div_ceil(self.page.size.into())
            || self.page.total_elements == 0 && !rows.is_empty()
            || self.page.total_elements > 0 && self.embedded.is_none()
        {
            return Err(BadReply);
        }
        let mut ids = std::collections::BTreeSet::new();
        if rows
            .iter()
            .any(|r| r.association_id == 0 || !ids.insert(r.association_id))
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
impl Response {
    pub(super) fn matches(&self, id: &str, page: u32, size: u32, url: &str) -> bool {
        self.page.number == page
            && self.page.size == size
            && self.links.current.href.as_str() == url
            && self.embedded.as_ref().is_none_or(|e| {
                e.associations
                    .iter()
                    .all(|a| a.snp_allele.iter().any(|s| s.rs_id.as_str() == id))
            })
    }
}

fn optional_text<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<Text>, D::Error> {
    let value = Option::<String>::deserialize(d)?;
    Ok(value.and_then(|v| Text::new(&v).ok()))
}
