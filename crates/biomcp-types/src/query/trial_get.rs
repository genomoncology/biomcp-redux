use super::gene::{QueryInvalid, present};
use crate::{NctId, Text, TrialSection, TrialSource};
use biodata::TrialSiteStatus;
use serde::{Deserialize, Serialize};
/// Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct TrialGet(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// One to twenty exact NCT identifiers, in caller order.
    #[cfg_attr(feature="schema", schemars(length(min=1,max=20), extend("x-positional"=0)))]
    input: Box<[NctId]>,
    #[doc = " Optional intact source sections. Default cards only; diseases and summary require NCI. \
        NCI locations, contacts, references and documents are unsupported."]
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    #[cfg_attr(feature="schema", schemars(extend("x-trailing-section"=true)))]
    section: Box<[TrialSection]>,
    /// Whole-record count per source collection page, up to 200; locations fill the page, other collections and text fragments also stop at their byte budget.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 200)))]
    limit: u32,
    /// Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections.
    #[serde(default)]
    offset: u32,
    /// Checked stored source record, supplied by a continuation.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pool: Option<Text>,
    /// Latitude degrees; with --lon, locations give each site's straight-line miles and come nearest first; with no section, locations is read.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    lat: Option<Text>,
    /// Longitude degrees; used with --lat.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    lon: Option<Text>,
    /// ClinicalTrials.gov locations only: match the registry state label, ignoring ASCII case.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    state: Option<Text>,
    /// ClinicalTrials.gov locations only: match the site's exact registry recruitment status, such as RECRUITING.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "TrialSiteStatus"))]
    status: Option<TrialSiteStatus>,
    /// ClinicalTrials.gov locations only: positive straight-line radius with mi or km units; requires --lat and --lon.
    #[serde(
        default,
        deserialize_with = "present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    distance: Option<Text>,
    /// Explicit provider; defaults to ClinicalTrials.gov and never falls back.
    #[serde(default, skip_serializing_if = "TrialSource::is_default")]
    source: TrialSource,
}
fn default_limit() -> u32 {
    10
}
crate::query::validation::checked_query!(TrialGet, Fields);
impl TryFrom<Fields> for TrialGet {
    type Error = QueryInvalid;
    fn try_from(mut v: Fields) -> Result<Self, Self::Error> {
        if v.section.is_empty()
            && (v.site_filter().is_some() || (v.lat.is_some() && v.lon.is_some()))
        {
            v.section = Box::new([TrialSection::Locations]);
        }
        if v.source == TrialSource::Ctgov
            && v.section
                .iter()
                .any(|s| matches!(s, TrialSection::Diseases | TrialSection::Summary))
        {
            return Err(QueryInvalid::new(&["source", "section"]));
        }
        if !(1..=20).contains(&v.input.len()) {
            return Err(QueryInvalid::new(&["input"]));
        }
        if !(1..=200).contains(&v.limit) {
            return Err(QueryInvalid::new(&["limit"]));
        }
        if (v.pool.is_some() || v.offset != 0)
            && (v.input.len() != 1
                || v.section.len() != 1
                || v.section.first() == Some(&TrialSection::All))
        {
            return Err(QueryInvalid::new(&["pool", "offset", "input", "section"]));
        }
        v.validate_locations()?;
        Ok(Self(v))
    }
}
impl Fields {
    fn site_filter(&self) -> Option<&'static str> {
        if self.state.is_some() {
            Some("state")
        } else if self.status.is_some() {
            Some("status")
        } else {
            self.distance.as_ref().map(|_| "distance")
        }
    }

    fn validate_locations(&self) -> Result<(), QueryInvalid> {
        let locations = self
            .section
            .iter()
            .any(|s| matches!(s, TrialSection::Locations | TrialSection::All));
        if let Some(option) = self.site_filter() {
            if self.source != TrialSource::Ctgov {
                return Err(QueryInvalid::new(&[option, "source"]));
            }
            if !locations {
                return Err(QueryInvalid::new(&[option, "section"]));
            }
        }
        match (&self.lat, &self.lon) {
            (None, None) if self.distance.is_none() => {}
            (Some(lat), Some(lon)) => {
                super::trial::point(lat, lon)?;
                if !locations {
                    return Err(QueryInvalid::new(&["lat", "lon", "section"]));
                }
            }
            _ if self.distance.is_some() => {
                return Err(QueryInvalid::new(&["lat", "lon", "distance"]));
            }
            _ => return Err(QueryInvalid::new(&["lat", "lon"])),
        }
        if let Some(distance) = &self.distance {
            super::trial::validation::radius_miles(distance)?;
        }
        Ok(())
    }
}
impl TrialGet {
    /// Explicit source selected by the caller.
    pub fn source(&self) -> TrialSource {
        self.0.source
    }

    /// Select one independent provider for a generated section action.
    pub fn with_source(mut self, source: TrialSource) -> Self {
        self.0.source = source;
        self
    }
    /// Exact checked selectors.
    pub fn input(&self) -> &[NctId] {
        &self.0.input
    }
    /// Requested section words.
    pub fn sections(&self) -> &[TrialSection] {
        &self.0.section
    }
    /// Whole-record page limit.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Source character or record position.
    pub fn offset(&self) -> u32 {
        self.0.offset
    }
    /// Stored source record capability.
    pub fn pool(&self) -> Option<&Text> {
        self.0.pool.as_ref()
    }
    /// Checked point in degrees that site distances are measured from.
    pub fn point(&self) -> Option<(f64, f64)> {
        super::trial::point(self.0.lat.as_ref()?, self.0.lon.as_ref()?).ok()
    }
    /// Filter sites by registry state label.
    pub fn site_state(&self) -> Option<&str> {
        self.0.state.as_ref().map(Text::as_str)
    }
    /// Filter sites by their registry recruitment status.
    pub fn site_status(&self) -> Option<TrialSiteStatus> {
        self.0.status
    }
    /// Maximum straight-line distance in miles from the checked point.
    pub fn site_radius_miles(&self) -> Option<f64> {
        super::trial::validation::radius_miles(self.0.distance.as_ref()?).ok()
    }
    /// Whether any location filter was supplied.
    pub fn filters_sites(&self) -> bool {
        self.0.site_filter().is_some()
    }
    /// One card or section request, used by typed next steps.
    pub fn single(input: NctId, section: Option<TrialSection>, limit: u32) -> Self {
        Self(Fields {
            input: Box::new([input]),
            section: section.into_iter().collect(),
            limit,
            offset: 0,
            pool: None,
            lat: None,
            lon: None,
            state: None,
            status: None,
            distance: None,
            source: TrialSource::Ctgov,
        })
    }
    /// Continue one immutable source record at a checked position.
    pub fn page(&self, input: NctId, section: TrialSection, pool: Text, offset: u32) -> Self {
        let mut next = Self::single(input, Some(section), self.limit());
        if section == TrialSection::Locations {
            next.0.lat = self.0.lat.clone();
            next.0.lon = self.0.lon.clone();
            next.0.state = self.0.state.clone();
            next.0.status = self.0.status;
            next.0.distance = self.0.distance.clone();
        }
        next.0.source = self.source();
        next.0.pool = Some(pool);
        next.0.offset = offset;
        next
    }
    /// The same request for a checked subset of its own inputs, such as unread batch items.
    pub fn with_input(&self, input: Vec<NctId>) -> Self {
        let mut q = self.clone();
        if !input.is_empty() && input.iter().all(|id| self.0.input.contains(id)) {
            q.0.input = input.into();
        }
        q
    }
    /// Initial request without continuation state.
    pub fn identity(&self) -> Self {
        let mut q = self.clone();
        q.0.pool = None;
        q.0.offset = 0;
        q
    }
}
