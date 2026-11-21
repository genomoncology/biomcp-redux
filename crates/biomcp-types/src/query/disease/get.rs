//! Exact disease reads and independent section paging.
use super::{DiseaseId, DiseaseOntology, DiseaseSection, DiseaseSource, default_limit, get_source};
use crate::{
    Text,
    query::gene::{QueryInvalid, present},
};
use serde::{Deserialize, Serialize};
/// Disease details and sections.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(with = "GetFields", extend("x-reply-query-titles" = ["DiseaseSearch"]))
)]
pub struct DiseaseGet {
    #[serde(flatten)]
    pub(super) fields: GetFields<DiseaseId>,
    #[serde(skip)]
    pub(super) record: Text,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "GetFields"))]
pub(super) struct GetFields<I = Text> {
    /// Disease CURIE, SEER site code or name, or exact curation name.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    pub(super) id: I,
    /// Default MyDisease, or both sources for gene validity.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "DiseaseSource"))]
    pub(super) source: Option<DiseaseSource>,
    /// OLS ontology. Defaults to the ID’s ontology or MONDO.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "DiseaseOntology"))]
    pub(super) ontology: Option<DiseaseOntology>,
    /// Omit for the identity card.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 1)))]
    pub(super) section: Option<DiseaseSection>,
    /// Rows per page. Default 5, Monarch genes 2, phenotypes 1.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "u32", range(min = 1, max = 50)))]
    pub(super) section_limit: Option<u32>,
    /// Section row offset.
    #[serde(default)]
    pub(super) section_offset: u32,
    /// Native sex code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) sex: Option<Text>,
    /// Native race code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) race: Option<Text>,
    /// Native age-range code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) age_range: Option<Text>,
    /// Requested dataset code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) dataset: Option<Text>,
    /// Incidence period code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) period: Option<Text>,
    /// Survival interval code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) interval: Option<Text>,
    /// Exact gene validity classification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pub(super) classification: Option<Text>,
    /// Exact GenCC submitter or ClinGen panel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pub(super) submitter: Option<Text>,
}
crate::query::validation::checked_query!(DiseaseGet, GetFields);
impl DiseaseGet {
    /// Check an exact OLS concept within its source-listed ontology.
    pub fn ols(id: Text, ontology: DiseaseOntology) -> Result<Self, QueryInvalid> {
        Self::try_from(GetFields {
            id,
            source: Some(DiseaseSource::Ols),
            ontology: Some(ontology),
            section: None,
            section_limit: None,
            section_offset: 0,
            sex: None,
            race: None,
            age_range: None,
            dataset: None,
            period: None,
            interval: None,
            classification: None,
            submitter: None,
        })
    }

    /// Read an exact native disease CURIE through the default disease command.
    pub fn from_id(id: biodata::Curie) -> Result<Self, QueryInvalid> {
        let record = Text::new(id.as_str()).map_err(|_| QueryInvalid::new(&["id"]))?;
        Ok(Self {
            record,
            fields: GetFields {
                id: DiseaseId::Concept(id),
                source: Some(get_source()),
                ontology: None,
                section: None,
                section_limit: Some(default_limit()),
                section_offset: 0,
                sex: None,
                race: None,
                age_range: None,
                dataset: None,
                period: None,
                interval: None,
                classification: None,
                submitter: None,
            },
        })
    }

    /// Read Orphanet epidemiology for an exact concept or Orphanet code.
    pub fn orphadata_prevalence(id: biodata::Curie) -> Result<Self, QueryInvalid> {
        Self::try_from(GetFields {
            id: Text::new(id.as_str()).map_err(|_| QueryInvalid::new(&["id"]))?,
            source: Some(DiseaseSource::Orphadata),
            ontology: None,
            section: Some(DiseaseSection::Prevalence),
            section_limit: None,
            section_offset: 0,
            sex: None,
            race: None,
            age_range: None,
            dataset: None,
            period: None,
            interval: None,
            classification: None,
            submitter: None,
        })
    }

    /// Original edge-admitted spelling for source record attribution.
    pub fn record(&self) -> &Text {
        &self.record
    }
    /// Exact source identifier.
    pub fn id(&self) -> &DiseaseId {
        &self.fields.id
    }
    /// Explicit service.
    pub fn source(&self) -> DiseaseSource {
        self.fields.source.unwrap_or_else(get_source)
    }
    /// Default single publisher, if the section does not read both curation exports.
    pub fn default_source(&self) -> Option<DiseaseSource> {
        (self.section() != Some(DiseaseSection::GeneValidity)).then(get_source)
    }
    /// Selected OLS ontology.
    pub fn ontology(&self) -> Option<DiseaseOntology> {
        self.fields.ontology
    }
    /// Selected source section.
    pub fn section(&self) -> Option<DiseaseSection> {
        self.fields.section
    }
    /// Maximum section rows.
    pub fn section_limit(&self) -> u32 {
        self.fields.section_limit.unwrap_or_else(default_limit)
    }
    /// Section row offset.
    pub fn section_offset(&self) -> u32 {
        self.fields.section_offset
    }
    /// Continue the selected section while keeping its selectors.
    pub fn at_section_offset(&self, offset: u32) -> Self {
        let mut q = self.clone();
        q.fields.section_offset = offset;
        q
    }
    /// Select an available section from this exact card.
    pub fn with_section(&self, section: DiseaseSection) -> Self {
        let mut q = self.clone();
        q.fields.section = Some(section);
        q.fields.section_offset = 0;
        q
    }
    /// Apply source-resolved demographics to a checked SEER query.
    pub fn with_demographics(
        &self,
        sex: Text,
        race: Text,
        age_range: Text,
    ) -> Result<Self, QueryInvalid> {
        if self.source() != DiseaseSource::Seer {
            return Err(QueryInvalid::new(&["source"]));
        }
        let mut fields = self.fields.clone();
        fields.sex = Some(sex);
        fields.race = Some(race);
        fields.age_range = Some(age_range);
        Ok(Self {
            fields,
            record: self.record.clone(),
        })
    }
    /// Continue SEER discovery using a publisher-issued native site code.
    pub fn with_seer_site(&self, site: Text) -> Result<Self, QueryInvalid> {
        if self.source() != DiseaseSource::Seer
            || !site.as_str().bytes().all(|b| b.is_ascii_digit())
        {
            return Err(QueryInvalid::new(&["source", "id"]));
        }
        let mut query = self.clone();
        query.record = site.clone();
        query.fields.id = DiseaseId::SeerSite(site);
        Ok(query)
    }
    /// Native sex selector.
    pub fn sex(&self) -> Option<&Text> {
        self.fields.sex.as_ref()
    }
    /// Native race selector.
    pub fn race(&self) -> Option<&Text> {
        self.fields.race.as_ref()
    }
    /// Native age range selector.
    pub fn age_range(&self) -> Option<&Text> {
        self.fields.age_range.as_ref()
    }
    /// Native dataset selector.
    pub fn dataset(&self) -> Option<&Text> {
        self.fields.dataset.as_ref()
    }
    /// Native period selector.
    pub fn period(&self) -> Option<&Text> {
        self.fields.period.as_ref()
    }
    /// Native interval selector.
    pub fn interval(&self) -> Option<&Text> {
        self.fields.interval.as_ref()
    }
}

impl DiseaseGet {
    /// Selected curation publishers; omitted source reads each publisher independently.
    pub fn validity_sources(&self) -> Option<&'static [crate::Source]> {
        use crate::Source;
        if self.section() != Some(DiseaseSection::GeneValidity) {
            return None;
        }
        match self.fields.source {
            None => Some(DiseaseSection::GeneValidity.sources()),
            Some(DiseaseSource::ClinGen) => Some(&[Source::ClinGen]),
            Some(DiseaseSource::Gencc) => Some(&[Source::Gencc]),
            _ => None,
        }
    }

    /// Exact classification from the source.
    pub fn classification(&self) -> Option<&Text> {
        self.fields.classification.as_ref()
    }

    /// Exact source submitter or expert panel.
    pub fn submitter(&self) -> Option<&Text> {
        self.fields.submitter.as_ref()
    }
}
