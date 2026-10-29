use crate::native_text::NativeText;
use biodata::{FiniteNumber, Text};

use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Response {
    #[serde(rename = "_embedded")]
    pub embedded: Option<Embedded>,
    #[serde(rename = "_links")]
    pub(super) links: Links,
    pub page: Page,
}
#[derive(Debug, Deserialize)]
pub(super) struct Embedded {
    pub associations: Vec<Association>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Links {
    #[serde(rename = "self")]
    pub(super) current: Link,
}
#[derive(Debug, Deserialize)]
pub(super) struct Link {
    pub(super) href: Text,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Page {
    pub(super) size: u32,
    pub total_elements: u64,
    total_pages: u64,
    pub(super) number: u32,
}
/// A source association retains publisher labels and numbers unchanged.
#[derive(Debug, Deserialize)]
pub struct Association {
    /// Stable association record identity.
    pub association_id: u64,
    /// Catalog study accession.
    pub accession_id: Text,
    /// Source-reported traits.
    pub reported_trait: Vec<NativeText>,
    /// Source mapped ontology terms.
    pub efo_traits: Vec<Trait>,
    /// Source effect alleles, never interpreted as a classification.
    pub snp_allele: Vec<EffectAllele>,
    /// Reported association p value.
    pub p_value: FiniteNumber,
    /// Source exact mantissa, independent of rounded p_value.
    #[serde(rename = "pvalue_mantissa")]
    pub p_value_mantissa: Option<u64>,
    /// Source exponent, including values below the double range.
    #[serde(rename = "pvalue_exponent")]
    pub p_value_exponent: Option<i64>,
    /// Source author attribution.
    #[serde(rename = "first_author")]
    pub author: Option<String>,
    /// Source gene mapping strings.
    #[serde(rename = "mapped_genes")]
    pub mapped_gene: Option<Vec<NativeText>>,
    /// Native mapping locations.
    pub locations: Option<Vec<NativeText>>,
    /// Source interval lower bound.
    pub ci_lower: Option<FiniteNumber>,
    /// Source interval upper bound.
    pub ci_upper: Option<FiniteNumber>,
    /// Reported effect beta, preserving units and prose.
    pub beta: Option<String>,
    /// Reported odds ratio where provided.
    pub or_per_copy_num: Option<FiniteNumber>,
    /// Source allele frequency text, such as NR.
    pub risk_frequency: Option<String>,
    /// Source-reported effect interval.
    pub range: Option<String>,
    /// Source publication identity.
    pub pubmed_id: Option<String>,
}
/// Source trait identity and label.
#[derive(Debug, Deserialize)]
pub struct Trait {
    /// Source ontology ID.
    pub efo_id: Text,
    /// Whole source label.
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    pub efo_trait: biodata::Text,
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
        let offset = u64::from(self.page.number) * u64::from(self.page.size);
        let expected = self
            .page
            .total_elements
            .saturating_sub(offset)
            .min(self.page.size.into());
        if rows.len() as u64 != expected {
            return Err(BadReply);
        }
        let mut ids = std::collections::BTreeSet::new();
        if rows.iter().any(|r| {
            r.association_id == 0
                || !ids.insert(r.association_id)
                || r.exact_p_value().is_none()
                    && (r.p_value_mantissa.is_some()
                        || r.p_value_exponent.is_some()
                        || f64::from(r.p_value) == 0.0)
        }) {
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

impl Association {
    /// Decode exact native parts without converting the probability to a double.
    pub fn exact_p_value(&self) -> Option<biodata::ScientificPValue> {
        let mantissa = self.p_value_mantissa?;
        format!("{mantissa}e{}", self.p_value_exponent?)
            .parse()
            .ok()
    }
}
