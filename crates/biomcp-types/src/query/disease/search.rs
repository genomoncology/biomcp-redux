//! Disease search inputs for literal discovery, site catalogs and HPO releases.
use super::{DiseaseOntology, DiseaseSource, default_limit};
use crate::query::gene::{QueryInvalid, present};
use serde::{Deserialize, Serialize};

/// Search by text, SEER site name, or HPO annotations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct DiseaseSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Each HPO ID or a descendant must have a positive annotation.
    #[serde(default, skip_serializing_if = "<[biodata::HpId]>::is_empty")]
    #[cfg_attr(feature = "schema", schemars(length(max=20), extend("x-comma-separated" = true)))]
    phenotype: Box<[biodata::HpId]>,
    /// HPO release tag (vYYYY-MM-DD), required with --phenotype.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "schema",
        schemars(regex(pattern = r"^v[0-9]{4}-[0-9]{2}-[0-9]{2}$"))
    )]
    hpo_release: Option<Box<str>>,
    /// Search text or optional SEER site name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(length(min=1),extend("x-positional" = 0)))]
    text: Option<Box<str>>,
    /// HPO for --phenotype, OLS by default, SEER for sites.
    #[serde(default)]
    #[cfg_attr(
        feature = "schema",
        schemars(with = "super::source::DiseaseSearchSource", extend("default" = "ols"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<DiseaseSource>,
    /// OLS ontology, default MONDO.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "DiseaseOntology", extend("default" = "mondo")))]
    ontology: Option<DiseaseOntology>,
    /// Rows per page.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Row offset.
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
        let source = v.source.unwrap_or(if v.phenotype.is_empty() {
            DiseaseSource::Ols
        } else {
            DiseaseSource::Hpo
        });
        if source == DiseaseSource::Hpo {
            if v.phenotype.is_empty() || v.phenotype.len() > 20 {
                return Err(QueryInvalid::new(&["phenotype"]));
            }
            if v.hpo_release.as_deref().is_none_or(|tag| !release_tag(tag)) {
                return Err(QueryInvalid::new(&["hpo_release"]));
            }
            if v.text.is_some() || v.ontology.is_some() {
                return Err(QueryInvalid::new(&["phenotype", "text", "ontology"]));
            }
        } else if !v.phenotype.is_empty() || v.hpo_release.is_some() {
            return Err(QueryInvalid::new(&["source", "phenotype", "hpo_release"]));
        }
        match source {
            DiseaseSource::Ols => {
                if v.text.as_ref().is_none_or(|t| t.trim().is_empty()) {
                    return Err(QueryInvalid::new(&["source", "text"]));
                }
                v.ontology.get_or_insert_with(DiseaseOntology::default);
            }
            DiseaseSource::Seer => {
                if v.text.as_ref().is_some_and(|t| t.trim().is_empty()) || v.ontology.is_some() {
                    return Err(QueryInvalid::new(&["source", "text", "ontology"]));
                }
            }
            DiseaseSource::Hpo => (),
            _ => return Err(QueryInvalid::new(&["source"])),
        }
        v.source = Some(source);
        Ok(Self(v))
    }
}
impl DiseaseSearch {
    /// Select the SEER site catalog with the standard page bounds.
    pub fn seer() -> Result<Self, QueryInvalid> {
        Self::try_from(SearchFields {
            text: None,
            source: Some(DiseaseSource::Seer),
            phenotype: Box::default(),
            hpo_release: None,
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
        self.0.source.unwrap_or_default()
    }
    /// The named HPO release and requested terms, when this is an HPO search.
    pub fn hpo(&self) -> Option<(&str, &[biodata::HpId])> {
        self.0
            .hpo_release
            .as_deref()
            .map(|tag| (tag, self.0.phenotype.as_ref()))
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

fn release_tag(tag: &str) -> bool {
    match tag.as_bytes() {
        [b'v', year @ .., b'-', m1, m2, b'-', d1, d2] if year.len() == 4 => {
            year.iter().chain([m1, m2, d1, d2]).all(u8::is_ascii_digit)
        }
        _ => false,
    }
}
