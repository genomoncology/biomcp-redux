use super::gene::QueryInvalid;
use crate::{GeneInput, NctId, PartialDate, StudyType, Text, TrialSex, TrialStatus};
use serde::{Deserialize, Serialize};
crate::macros::wire_enum! {
    /// Registry funding category facet.
    SponsorType {
        /// National Institutes of Health.
        Nih => "nih",
        /// Other United States federal agency.
        Fed => "fed",
        /// Industry.
        Industry => "industry",
        /// Other sponsor category.
        Other => "other",
    }
}
/// Checked registry search with source-native filters.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct TrialSearch(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// General registry discovery text.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text", extend("x-positional" = 0)))]
    query: Option<Text>,
    /// Condition-only source text.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    condition: Option<Text>,
    /// Literal source intervention text.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    intervention: Option<Text>,
    /// Resolve one checked gene before registry discovery.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "GeneInput"))]
    gene: Option<GeneInput>,
    /// Caller-supplied variant discovery text; no eligibility inference.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    variant: Option<Text>,
    /// Literal eligibility-field discovery text.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    criteria: Option<Text>,
    /// Exact registry overall status.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "TrialStatus"))]
    status: Option<TrialStatus>,
    /// Registry phase: 1, 2, 3, 4, early-1, not-applicable, 1/2 or 2/3.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "PhaseFilter"))]
    phase: Option<PhaseFilter>,
    /// Exact registry study type.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "StudyType"))]
    study_type: Option<StudyType>,
    /// Registry sex facet.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "TrialSex"))]
    sex: Option<TrialSex>,
    /// Age in years; unstated source bounds count as no limit.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "u32"))]
    age: Option<u32>,
    /// Exact registry facility field; cannot combine with radius.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    facility: Option<Text>,
    /// Radius center latitude in degrees.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    lat: Option<Text>,
    /// Radius center longitude in degrees.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    lon: Option<Text>,
    /// Source distance with explicit mi or km units.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    distance: Option<Text>,
    /// Exact lead sponsor field.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    sponsor: Option<Text>,
    /// Registry sponsor funding facet.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "SponsorType"))]
    sponsor_type: Option<SponsorType>,
    /// First included registry update date.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "PartialDate"))]
    updated_since: Option<PartialDate>,
    /// Last included registry update date.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "PartialDate"))]
    updated_until: Option<PartialDate>,
    /// Exact NCT identity in a conjunction with other filters.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "NctId"))]
    id: Option<NctId>,
    /// Opaque registry continuation; keep filters unchanged.
    #[serde(
        default,
        deserialize_with = "super::gene::present",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    next_page: Option<Text>,
    /// Source total only; no returned study rows.
    #[serde(default)]
    count_only: bool,
    /// Require posted source results.
    #[serde(default)]
    has_results: bool,
    /// Headers per page; default ten, maximum one hundred.
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "schema", schemars(range(min = 1, max = 100)))]
    limit: u32,
}
fn default_limit() -> u32 {
    10
}
impl TryFrom<Fields> for TrialSearch {
    type Error = QueryInvalid;
    fn try_from(f: Fields) -> Result<Self, Self::Error> {
        if !(1..=100).contains(&f.limit)
            || f.age.is_some_and(|v| v > 150)
            || f.updated_since
                .zip(f.updated_until)
                .is_some_and(|(a, b)| a.to_string() > b.to_string())
            || (f.facility.is_some()
                && (f.lat.is_some() || f.lon.is_some() || f.distance.is_some()))
            || [f.lat.is_some(), f.lon.is_some(), f.distance.is_some()]
                .iter()
                .filter(|v| **v)
                .count()
                % 3
                != 0
        {
            return Err(QueryInvalid);
        }
        if let (Some(lat), Some(lon), Some(distance)) = (&f.lat, &f.lon, &f.distance) {
            let lat = lat.as_str().parse::<f64>().map_err(|_| QueryInvalid)?;
            let lon = lon.as_str().parse::<f64>().map_err(|_| QueryInvalid)?;
            let distance = distance
                .as_str()
                .strip_suffix("mi")
                .or_else(|| distance.as_str().strip_suffix("km"))
                .ok_or(QueryInvalid)?
                .parse::<f64>()
                .map_err(|_| QueryInvalid)?;
            if !(-90.0..=90.0).contains(&lat)
                || !(-180.0..=180.0).contains(&lon)
                || !distance.is_finite()
                || distance <= 0.0
            {
                return Err(QueryInvalid);
            }
        }
        Ok(Self(f))
    }
}
impl TrialSearch {
    /// General registry discovery text.
    pub fn query(&self) -> Option<&Text> {
        self.0.query.as_ref()
    }
    /// Condition-only source text.
    pub fn condition(&self) -> Option<&Text> {
        self.0.condition.as_ref()
    }
    /// Literal source intervention text.
    pub fn intervention(&self) -> Option<&Text> {
        self.0.intervention.as_ref()
    }
    /// Resolve one checked gene before registry discovery.
    pub fn gene(&self) -> Option<&GeneInput> {
        self.0.gene.as_ref()
    }
    /// Caller-supplied variant discovery text; no eligibility inference.
    pub fn variant(&self) -> Option<&Text> {
        self.0.variant.as_ref()
    }
    /// Literal eligibility-field discovery text.
    pub fn criteria(&self) -> Option<&Text> {
        self.0.criteria.as_ref()
    }
    /// Exact registry overall status.
    pub fn status(&self) -> Option<&TrialStatus> {
        self.0.status.as_ref()
    }
    /// Registry phase: 1, 2, 3, 4, early-1, not-applicable, 1/2 or 2/3.
    pub fn phase(&self) -> Option<&PhaseFilter> {
        self.0.phase.as_ref()
    }
    /// Exact registry study type.
    pub fn study_type(&self) -> Option<&StudyType> {
        self.0.study_type.as_ref()
    }
    /// Registry sex facet.
    pub fn sex(&self) -> Option<&TrialSex> {
        self.0.sex.as_ref()
    }
    /// Age in years; unstated source bounds count as no limit.
    pub fn age(&self) -> Option<&u32> {
        self.0.age.as_ref()
    }
    /// Exact registry facility field; cannot combine with radius.
    pub fn facility(&self) -> Option<&Text> {
        self.0.facility.as_ref()
    }
    /// Radius center latitude in degrees.
    pub fn lat(&self) -> Option<&Text> {
        self.0.lat.as_ref()
    }
    /// Radius center longitude in degrees.
    pub fn lon(&self) -> Option<&Text> {
        self.0.lon.as_ref()
    }
    /// Source distance with explicit mi or km units.
    pub fn distance(&self) -> Option<&Text> {
        self.0.distance.as_ref()
    }
    /// Exact lead sponsor field.
    pub fn sponsor(&self) -> Option<&Text> {
        self.0.sponsor.as_ref()
    }
    /// Registry sponsor funding facet.
    pub fn sponsor_type(&self) -> Option<&SponsorType> {
        self.0.sponsor_type.as_ref()
    }
    /// First included registry update date.
    pub fn updated_since(&self) -> Option<&PartialDate> {
        self.0.updated_since.as_ref()
    }
    /// Last included registry update date.
    pub fn updated_until(&self) -> Option<&PartialDate> {
        self.0.updated_until.as_ref()
    }
    /// Exact NCT identity in a conjunction with other filters.
    pub fn id(&self) -> Option<&NctId> {
        self.0.id.as_ref()
    }
    /// Opaque registry continuation; keep filters unchanged.
    pub fn next_page(&self) -> Option<&Text> {
        self.0.next_page.as_ref()
    }
    /// Source count without study rows.
    pub fn count_only(&self) -> bool {
        self.0.count_only
    }
    /// Posted results restriction.
    pub fn has_results(&self) -> bool {
        self.0.has_results
    }
    /// Bounded page size.
    pub fn limit(&self) -> u32 {
        self.0.limit
    }
    /// Continue the identical source query with its opaque cursor.
    pub fn at_page(&self, token: Text) -> Self {
        let mut next = self.clone();
        next.0.next_page = Some(token);
        next
    }
}

crate::macros::wire_enum! {
    /// Explicit source phase selection, including the registry's combined phases.
    PhaseFilter {
        /// Early phase 1.
        Early1 => "early-1",
        /// Phase 1.
        One => "1",
        /// Phase 2.
        Two => "2",
        /// Phase 3.
        Three => "3",
        /// Phase 4.
        Four => "4",
        /// Not applicable.
        NotApplicable => "not-applicable",
        /// Combined phase 1 and phase 2.
        OneTwo => "1/2",
        /// Combined phase 2 and phase 3.
        TwoThree => "2/3",
    }
}
impl PhaseFilter {
    /// Exact registry phases required together.
    pub fn phases(self) -> &'static [crate::TrialPhase] {
        use crate::TrialPhase as P;
        match self {
            Self::Early1 => &[P::Early1],
            Self::One => &[P::Phase1],
            Self::Two => &[P::Phase2],
            Self::Three => &[P::Phase3],
            Self::Four => &[P::Phase4],
            Self::NotApplicable => &[P::NotApplicable],
            Self::OneTwo => &[P::Phase1, P::Phase2],
            Self::TwoThree => &[P::Phase2, P::Phase3],
        }
    }
}
