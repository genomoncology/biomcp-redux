use super::gene::{QueryInvalid, present};
use crate::Text;
use serde::{Deserialize, Serialize};
mod source;
pub use source::{DiseaseOntology, DiseaseSource};
biodata::wire_enum! {
    /// One named disease page.
    DiseaseSection {
        /// Publisher scoped synonyms.
        Synonyms => "synonyms",
        /// Native cross references.
        Xrefs => "xrefs",
        /// Native parent relations.
        Hierarchy => "hierarchy",
        /// Native gene association rows.
        Genes => "genes",
        /// Native phenotype association rows.
        Phenotypes => "phenotypes",
        /// Native stage incidence rows.
        Incidence => "incidence",
        /// Annual survival rows.
        Survival => "survival",
    }
}
fn default_limit() -> u32 {
    5
}
fn get_source() -> DiseaseSource {
    DiseaseSource::MyDisease
}
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
    /// Literal source search text; absent for the SEER site catalog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    text: Option<Text>,
    /// One explicit source, default OLS.
    #[serde(default)]
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
impl TryFrom<SearchFields> for DiseaseSearch {
    type Error = QueryInvalid;
    fn try_from(mut v: SearchFields) -> Result<Self, Self::Error> {
        if !(1..=50).contains(&v.limit)
            || !matches!(v.source, DiseaseSource::Ols | DiseaseSource::Seer)
            || (v.source == DiseaseSource::Ols && v.text.is_none())
            || (v.source == DiseaseSource::Seer && (v.text.is_some() || v.ontology.is_some()))
        {
            return Err(QueryInvalid);
        }
        if v.source == DiseaseSource::Ols {
            v.ontology.get_or_insert_with(DiseaseOntology::default);
        }
        Ok(Self(v))
    }
}
/// Exact concept or native SEER site with independently paged section.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct DiseaseGet(GetFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Exact ontology CURIE or an explicit SEER catalog site ID.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    id: Text,
    /// Default MyDisease; no automatic fallback.
    #[serde(default = "get_source")]
    source: DiseaseSource,
    /// One OLS ontology.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "DiseaseOntology", extend("default" = "mondo")))]
    ontology: Option<DiseaseOntology>,
    /// One named section; absent means identity card only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-trailing-section" = true)))]
    section: Option<DiseaseSection>,
    /// Native rows per section page.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    section_limit: u32,
    /// Native row offset within the selected section.
    #[serde(default)]
    section_offset: u32,
    /// Native sex code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sex: Option<Text>,
    /// Native race code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    race: Option<Text>,
    /// Native age-range code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    age_range: Option<Text>,
    /// Requested native dataset selector, not proof of returned membership.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dataset: Option<Text>,
    /// Incidence-only native period code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    period: Option<Text>,
    /// Survival-only interval code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    interval: Option<Text>,
}
impl TryFrom<GetFields> for DiseaseGet {
    type Error = QueryInvalid;
    fn try_from(mut v: GetFields) -> Result<Self, Self::Error> {
        let statistic = matches!(
            v.section,
            Some(DiseaseSection::Incidence | DiseaseSection::Survival)
        );
        if !(1..=50).contains(&v.section_limit)
            || (v.source == DiseaseSource::Seer
                && (!statistic || !v.id.as_str().bytes().all(|b| b.is_ascii_digit())))
            || (v.source != DiseaseSource::Seer
                && (statistic || v.id.as_str().parse::<biodata::Curie>().is_err()))
            || (v.source != DiseaseSource::Seer
                && [
                    &v.sex,
                    &v.race,
                    &v.age_range,
                    &v.dataset,
                    &v.period,
                    &v.interval,
                ]
                .iter()
                .any(|v| v.is_some()))
            || (v.source != DiseaseSource::Ols && v.ontology.is_some())
            || (v.section != Some(DiseaseSection::Incidence) && v.period.is_some())
            || (v.section != Some(DiseaseSection::Survival) && v.interval.is_some())
        {
            return Err(QueryInvalid);
        }
        if v.source == DiseaseSource::Ols {
            v.ontology.get_or_insert_with(DiseaseOntology::default);
        }
        Ok(Self(v))
    }
}
