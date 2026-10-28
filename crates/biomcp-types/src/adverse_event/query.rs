use super::{AdverseEventSection, AdverseEventSerious, AdverseEventSource};
use crate::QueryInvalid;
use biodata::{PartialDate, ReportId, Text};
use serde::{Deserialize, Serialize};
fn default_source() -> AdverseEventSource {
    AdverseEventSource::Faers
}
fn default_limit() -> u32 {
    5
}
fn full_day(date: Option<PartialDate>) -> bool {
    date.is_none_or(|d| d.day().is_some())
}
#[doc = " Search five report headers by default. VAERS accepts MMR, COVID19, COVID-19 vaccine or \
    one exact qualified native influenza code, matched without case; use influenza to list \
    codes and labels. Tables remain separate and are never summed. One report may list \
    several vaccines. VAERS returns five reported symptom rows by default; seriousness and \
    age select independent tables. Counts are reports, not people. Reports do not show that a \
    vaccine caused an event and cannot measure risk or compare vaccine safety."]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "SearchFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "SearchFields"))]
pub struct AdverseEventSearch(SearchFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct SearchFields {
    /// Literal drug text, MAUDE device phrase or exact supported VAERS vaccine.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    text: Text,
    /// FAERS by default; VAERS accepts only exact supported vaccine names.
    #[serde(default = "default_source")]
    source: AdverseEventSource,
    /// Independent VAERS table; omitted means reported symptoms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 1)))]
    section: Option<super::VaersSection>,
    /// Retained VAERS table for provider-free continuation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    snapshot: Option<Text>,
    /// FAERS MedDRA term phrase match, not exact equality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reaction: Option<Text>,
    /// FAERS report flag: death or hospitalization; never a reaction outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    serious: Option<AdverseEventSerious>,
    /// Inclusive first FDA receipt lower bound, YYYY-MM-DD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "date")))]
    received_from: Option<PartialDate>,
    /// Inclusive first FDA receipt upper bound, YYYY-MM-DD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "date")))]
    received_to: Option<PartialDate>,
    /// Native report headers or VAERS rows per page; default five.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    limit: u32,
    /// Native row offset. VAERS continuation requires its retained snapshot.
    #[serde(default)]
    offset: u32,
}
impl TryFrom<SearchFields> for AdverseEventSearch {
    type Error = QueryInvalid;
    fn try_from(v: SearchFields) -> Result<Self, Self::Error> {
        if !(1..=50).contains(&v.limit)
            || !full_day(v.received_from)
            || !full_day(v.received_to)
            || v.received_from
                .zip(v.received_to)
                .is_some_and(|(a, b)| a.to_string() > b.to_string())
            || (v.source != AdverseEventSource::Vaers
                && (v.section.is_some() || v.snapshot.is_some()))
            || (v.source == AdverseEventSource::Vaers
                && super::vaers::vaers_vaccine_code(v.text.as_str()).is_none())
            || (v.source != AdverseEventSource::Faers
                && (v.reaction.is_some()
                    || v.serious.is_some()
                    || v.received_from.is_some()
                    || v.received_to.is_some()))
        {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl AdverseEventSearch {
    /// Selected VAERS grouping.
    pub fn vaers_section(&self) -> super::VaersSection {
        self.0.section.unwrap_or(super::VaersSection::Symptoms)
    }
    /// Checked retained table identity.
    pub fn snapshot(&self) -> Option<&Text> {
        self.0.snapshot.as_ref()
    }
    /// Bind navigation to one native table.
    pub fn with_snapshot(&self, snapshot: Text) -> Self {
        let mut q = self.clone();
        q.0.snapshot = Some(snapshot);
        q
    }
    /// Begin a separate grouping without borrowing its sibling denominator.
    pub fn with_vaers_section(&self, section: super::VaersSection) -> Self {
        let mut q = self.clone();
        q.0.section = Some(section);
        q.0.snapshot = None;
        q.0.offset = 0;
        q
    }
    /// Restart without an expired retained table.
    pub fn restart(&self) -> Self {
        let mut q = self.clone();
        q.0.snapshot = None;
        q.0.offset = 0;
        q
    }
    /// Exact supported vaccine code, never an inferred code bridge.
    pub fn vaccine(&self) -> Option<&'static str> {
        super::vaers::vaers_vaccine_code(self.0.text.as_str())
    }
    /// Literal source text.
    pub fn text(&self) -> &Text {
        &self.0.text
    }
    /// Selected source.
    pub fn source(&self) -> AdverseEventSource {
        self.0.source
    }
    /// Reaction phrase.
    pub fn reaction(&self) -> Option<&Text> {
        self.0.reaction.as_ref()
    }
    /// Report flag.
    pub fn serious(&self) -> Option<AdverseEventSerious> {
        self.0.serious
    }
    /// Inclusive lower first-receipt date.
    pub fn received_from(&self) -> Option<PartialDate> {
        self.0.received_from
    }
    /// Inclusive upper first-receipt date.
    pub fn received_to(&self) -> Option<PartialDate> {
        self.0.received_to
    }
    /// Report page size.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Report offset.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Continue while preserving every filter.
    pub fn at_offset(&self, offset: u32) -> Self {
        let mut q = self.clone();
        q.0.offset = offset;
        q
    }
}
/// Exact native report identity and an optional independently paged section.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "GetFields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "GetFields"))]
pub struct AdverseEventGet(GetFields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GetFields {
    /// Published report ID with no source prefix.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    id: ReportId,
    /// FAERS by default; MAUDE is explicit.
    #[serde(default = "default_source")]
    #[cfg_attr(feature = "schema", schemars(with="str", extend("enum"=["faers", "maude"])))]
    source: AdverseEventSource,
    /// FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 1)))]
    section: Option<AdverseEventSection>,
    /// Checked retained report snapshot for provider-free continuation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    snapshot: Option<Text>,
    /// Absolute source-array index; required only for identifier sections.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    drug_index: Option<u32>,
    /// Whole source rows per section page.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 50)))]
    section_limit: u32,
    /// Offset within the selected native section.
    #[serde(default)]
    section_offset: u32,
}
impl TryFrom<GetFields> for AdverseEventGet {
    type Error = QueryInvalid;
    fn try_from(v: GetFields) -> Result<Self, Self::Error> {
        if v.source == AdverseEventSource::Vaers
            || (v.source == AdverseEventSource::Maude
                && !v.id.as_str().bytes().all(|b| b.is_ascii_digit()))
            || !(1..=50).contains(&v.section_limit)
            || v.section.is_some_and(|s| s.source() != v.source)
            || v.drug_index.is_some() != v.section.is_some_and(AdverseEventSection::is_identifier)
        {
            return Err(QueryInvalid);
        }
        Ok(Self(v))
    }
}
impl AdverseEventGet {
    /// Exact published identity.
    pub fn id(&self) -> &ReportId {
        &self.0.id
    }
    /// Explicit family.
    pub fn source(&self) -> AdverseEventSource {
        self.0.source
    }
    /// Named section.
    pub fn section(&self) -> Option<AdverseEventSection> {
        self.0.section
    }
    /// Retained report identity, absent for an initial get.
    pub fn snapshot(&self) -> Option<&Text> {
        self.0.snapshot.as_ref()
    }
    /// Bind all subsequent section pages to this checked report.
    pub fn with_snapshot(&self, snapshot: Text) -> Self {
        let mut q = self.clone();
        q.0.snapshot = Some(snapshot);
        q
    }
    /// Restart the exact report without an expired snapshot.
    pub fn restart(&self) -> Self {
        let mut q = self.clone();
        q.0.snapshot = None;
        q.0.section_offset = 0;
        q
    }
    /// Absolute native drug index, only for identifier sections.
    pub fn drug_index(&self) -> Option<u32> {
        self.0.drug_index
    }
    /// Whole row limit.
    pub fn section_limit(&self) -> u32 {
        self.0.section_limit
    }
    /// Native section offset.
    pub fn section_offset(&self) -> u32 {
        self.0.section_offset
    }
    /// Continue while preserving source, identity and drug index.
    pub fn at_section_offset(&self, offset: u32) -> Self {
        let mut q = self.clone();
        q.0.section_offset = offset;
        q
    }
    /// Select a section through the same validation used at deserialization.
    pub fn with_section(
        &self,
        section: AdverseEventSection,
        drug_index: Option<u32>,
    ) -> Result<Self, QueryInvalid> {
        let mut fields = self.0.clone();
        fields.section = Some(section);
        fields.drug_index = drug_index;
        fields.section_offset = 0;
        fields.try_into()
    }
}
