//! Standalone GWAS requests. Shared command registration follows separately.
use crate::{GeneSymbol, Pmid, QueryInvalid, RsId, Text};
use biodata::{GwasStudyId, GwasTraitId, ScientificPValue};
use serde::{Deserialize, Serialize};
/// Native source order, echoed without a configurable sort command.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum GwasOrder {
    /// Ascending source p value.
    #[default]
    #[serde(rename = "p_value,asc")]
    PValueAscending,
}
/// Checked association search selectors with native conjunction semantics.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct GwasSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Exact source rsID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    variant: Option<RsId>,
    /// Literal native efo_trait label; never reported-trait prose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    r#trait: Option<Text>,
    /// Exact native ontology identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    trait_id: Option<GwasTraitId>,
    /// Exact native mapped gene; legacy positional query aliases this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gene: Option<GeneSymbol>,
    /// Exact source study accession.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    study: Option<GwasStudyId>,
    /// Exact source publication identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pubmed: Option<Pmid>,
    /// Inclusive exact threshold under native ascending order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    p_value: Option<ScientificPValue>,
    /// Source child-trait expansion; requires trait_id.
    #[serde(default)]
    include_child_traits: bool,
    /// Source extended gene set; requires gene.
    #[serde(default)]
    extended_geneset: bool,
    /// Source order; threshold requests use this same native order.
    #[serde(default)]
    order: GwasOrder,
    /// Maximum association rows.
    #[serde(default = "limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Native association offset.
    #[serde(default)]
    offset: u32,
}
fn limit() -> u32 {
    3
}
impl TryFrom<SearchFields> for GwasSearch {
    type Error = QueryInvalid;
    fn try_from(f: SearchFields) -> Result<Self, Self::Error> {
        if f.variant.is_none()
            && f.r#trait.is_none()
            && f.trait_id.is_none()
            && f.gene.is_none()
            && f.study.is_none()
            && f.pubmed.is_none()
            || f.include_child_traits && f.trait_id.is_none()
            || f.extended_geneset && f.gene.is_none()
            || !(1..=50).contains(&f.limit)
        {
            return Err(QueryInvalid);
        }
        Ok(Self(f))
    }
}
impl GwasSearch {
    /// Checked variant selector.
    pub fn variant(&self) -> Option<&RsId> {
        self.0.variant.as_ref()
    }
    /// Checked trait selector.
    pub fn trait_label(&self) -> Option<&Text> {
        self.0.r#trait.as_ref()
    }
    /// Checked trait_id selector.
    pub fn trait_id(&self) -> Option<&GwasTraitId> {
        self.0.trait_id.as_ref()
    }
    /// Checked gene selector.
    pub fn gene(&self) -> Option<&GeneSymbol> {
        self.0.gene.as_ref()
    }
    /// Checked study selector.
    pub fn study(&self) -> Option<&GwasStudyId> {
        self.0.study.as_ref()
    }
    /// Checked pubmed selector.
    pub fn pubmed(&self) -> Option<&Pmid> {
        self.0.pubmed.as_ref()
    }
    /// Checked p_value selector.
    pub fn p_value(&self) -> Option<&ScientificPValue> {
        self.0.p_value.as_ref()
    }
    /// Native include_child_traits selector.
    pub fn include_child_traits(&self) -> bool {
        self.0.include_child_traits
    }
    /// Native extended_geneset selector.
    pub fn extended_geneset(&self) -> bool {
        self.0.extended_geneset
    }
    /// Native order selector.
    pub fn order(&self) -> GwasOrder {
        self.0.order
    }
    /// Native limit selector.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Native offset selector.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Continue without changing source selectors.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut next = self.clone();
        next.0.offset = offset;
        next
    }
}
