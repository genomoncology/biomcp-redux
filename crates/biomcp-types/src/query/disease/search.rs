//! Disease discovery through OLS or the SEER catalog.
use super::{DiseaseOntology, DiseaseSource, default_limit};
use crate::query::gene::{QueryInvalid, present};
use serde::{Deserialize, Serialize};
/// Literal discovery text or an explicit SEER catalog page.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct DiseaseSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Literal source search text; optional site-name filter for SEER.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(length(min=1),extend("x-positional" = 0)))]
    text: Option<Box<str>>,
    /// One explicit source, default OLS.
    #[serde(default)]
    #[cfg_attr(
        feature = "schema",
        schemars(with = "super::source::DiseaseSearchSource", extend("default" = "ols"))
    )]
    source: DiseaseSource,
    /// Exactly one ontology, default MONDO.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "DiseaseOntology", extend("default" = "mondo")))]
    ontology: Option<DiseaseOntology>,
    /// Maximum source rows on one page.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Native row offset.
    #[serde(default)]
    offset: u32,
}
crate::query::validation::checked_query!(DiseaseSearch, SearchFields);
impl TryFrom<SearchFields> for DiseaseSearch {
    type Error = QueryInvalid;
    fn try_from(mut v: SearchFields) -> Result<Self, Self::Error> {
        if !(1..=50).contains(&v.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        if !matches!(v.source, DiseaseSource::Ols | DiseaseSource::Seer) {
            return Err(QueryInvalid::new(&["source"]));
        }
        if v.source == DiseaseSource::Ols && v.text.as_ref().is_none_or(|t| t.trim().is_empty()) {
            return Err(QueryInvalid::new(&["source", "text"]));
        }
        if v.source == DiseaseSource::Seer
            && (v.text.as_ref().is_some_and(|t| t.trim().is_empty()) || v.ontology.is_some())
        {
            return Err(QueryInvalid::new(&["source", "text", "ontology"]));
        }
        if v.source == DiseaseSource::Ols {
            v.ontology.get_or_insert_with(DiseaseOntology::default);
        }
        Ok(Self(v))
    }
}
impl DiseaseSearch {
    /// Select the SEER site catalog with the standard page bounds.
    pub fn seer() -> Result<Self, QueryInvalid> {
        Self::try_from(SearchFields {
            text: None,
            source: DiseaseSource::Seer,
            ontology: None,
            limit: default_limit(),
            offset: 0,
        })
    }

    /// Select the SEER site catalog in the largest accepted pages.
    pub fn seer_widest() -> Result<Self, QueryInvalid> {
        let query = Self::seer()?;
        Self::try_from(SearchFields {
            limit: 50,
            ..query.0
        })
    }

    /// Select a name-filtered SEER catalog.
    pub fn seer_named(text: &str) -> Result<Self, QueryInvalid> {
        let query = Self::seer()?;
        Self::try_from(SearchFields {
            text: Some(text.into()),
            ..query.0
        })
    }

    /// Literal search text, absent for a native site catalog.
    pub fn text(&self) -> Option<&str> {
        self.0.text.as_deref()
    }
    /// Explicit service.
    pub fn source(&self) -> DiseaseSource {
        self.0.source
    }
    /// Selected OLS ontology, absent for other sources.
    pub fn ontology(&self) -> Option<DiseaseOntology> {
        self.0.ontology
    }
    /// Maximum source rows.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Native row offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
}
impl crate::Paged for DiseaseSearch {
    type Offset = u32;
    fn set_page_offset(&mut self, offset: Self::Offset) {
        self.0.offset = offset;
    }
}
