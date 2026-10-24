//! Native registry discovery; no local clinical interpretation or reordering.
use biodata::{NctId, Text, TrialHeader, TrialSearch, TrialPhase, TrialStatus, StudyType, PartialDate};
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, Reply, RequestError, SourceSpec};
use serde::Deserialize;
use std::{collections::BTreeMap, time::Duration};
/// Publisher terms apply to this registry; individual records are not a blanket CC0 grant.
pub static SPEC: SourceSpec = SourceSpec {
    source: biodata::Source::ClinicalTrialsGov,
    base: "https://clinicaltrials.gov/api/v2",
    key: KeyUse::None,
    quota: &crate::quotas::CTGOV,
    deadline: Duration::from_secs(15),
    terms: biomcp_fetch::Terms {
        name: "ClinicalTrials.gov attribution and source processing date; publisher terms",
        url: "https://clinicaltrials.gov/about-site/terms-conditions",
        usage: biodata::Use::Restricted,
    },
};
const SEARCH: Endpoint = SPEC.get("/studies").fresh(Freshness::Days(1));
/// Small source processing version endpoint, also the bounded health probe.
pub const VERSION: Endpoint = SPEC.get("/version").fresh(Freshness::Days(1));
/// A native search page with opaque source cursor.
pub struct SearchPage {
    /// Source order, untouched.
    pub items: Vec<TrialHeader>,
    /// Exact native total.
    pub total: u64,
    /// Opaque next page.
    pub next: Option<Text>,
}
/// Exact parameters; resolved gene text is explicit and attributed to HGNC by the caller.
pub fn parameters(query: &TrialSearch, gene: Option<&str>) -> BTreeMap<String,String> {
    let mut p = BTreeMap::from([
        ("format".into(),"json".into()), ("countTotal".into(),"true".into()),
        ("pageSize".into(),if query.count_only() { "1".into() } else { query.limit().to_string() }),
        ("fields".into(),"NCTId,BriefTitle,OverallStatus,Phase,StudyType,Condition,InterventionName,LastUpdatePostDate".into()),
    ]);
    for (name,value) in [("query.cond",query.condition()),("query.intr",query.intervention()),("pageToken",query.next_page())] {
        if let Some(value) = value { p.insert(name.into(),value.as_str().into()); }
    }
    let mut text = Vec::new();
    if let Some(value) = query.query() { text.push(value.as_str().to_owned()); }
    if let Some(value) = query.variant() { text.push(value.as_str().to_owned()); }
    if let Some(value) = gene { text.push(value.to_owned()); }
    if !text.is_empty() { p.insert("query.term".into(),text.join(" AND ")); }
    if let Some(status) = query.status() { p.insert("filter.overallStatus".into(), wire(status)); }
    let mut advanced = Vec::new();
    if let Some(id) = query.id() { advanced.push(format!("AREA[NCTId]{id}")); }
    if let Some(phase) = query.phase() {
        advanced.extend(phase.phases().iter().map(|phase| format!("AREA[Phase]{}",wire(phase))));
    }
    for (field,value) in [("EligibilityCriteria",query.criteria()),("LocationFacility",query.facility()),("LeadSponsorName",query.sponsor())] {
        if let Some(value) = value { advanced.push(format!("AREA[{field}]{}", quote(value.as_str()))); }
    }
    if let Some(study_type) = query.study_type() { advanced.push(format!("AREA[StudyType]{}",wire(study_type))); }
    if let Some(age) = query.age() {
        advanced.push(format!("(AREA[MinimumAge]RANGE[MIN,{age} years] OR AREA[MinimumAge]MISSING) AND (AREA[MaximumAge]RANGE[{age} years,MAX] OR AREA[MaximumAge]MISSING)"));
    }
    if query.updated_since().is_some() || query.updated_until().is_some() {
        advanced.push(format!("AREA[LastUpdatePostDate]RANGE[{},{}]",query.updated_since().map_or("MIN".into(),ToString::to_string),query.updated_until().map_or("MAX".into(),ToString::to_string)));
    }
    if query.has_results() { advanced.push("AREA[ResultsFirstPostDate]RANGE[MIN,MAX]".into()); }
    if !advanced.is_empty() { p.insert("filter.advanced".into(),advanced.join(" AND ")); }
    let mut facets = Vec::new();
    if let Some(sex) = query.sex() {
        facets.push(format!("sex:{}",match sex { biodata::TrialSex::Female=>"f", biodata::TrialSex::Male=>"m", biodata::TrialSex::All=>"a" }));
    }
    if let Some(sponsor) = query.sponsor_type() { facets.push(format!("funderType:{}",wire(sponsor))); }
    if !facets.is_empty() { p.insert("aggFilters".into(),facets.join(",")); }
    if let (Some(lat),Some(lon),Some(distance)) = (query.lat(),query.lon(),query.distance()) {
        p.insert("filter.geo".into(),format!("distance({},{},{})",lat.as_str(),lon.as_str(),distance.as_str()));
    }
    p
}
fn quote(value: &str) -> String { format!("\"{}\"",value.replace('\\',"\\\\").replace('"',"\\\"")) }
fn wire(value: &impl serde::Serialize) -> String {
    // Closed shared enums serialize only canonical JSON strings.
    serde_json::to_value(value).ok().and_then(|v| v.as_str().map(str::to_owned)).unwrap_or_default()
}
/// Read and validate native discovery results before shared cache admission.
pub fn search(leg: &Leg<'_>, params: &BTreeMap<String,String>) -> Result<SearchPage,RequestError> {
    let args: Vec<_> = params.iter().map(|(k,v)| (k.as_str(),Arg::Text(v))).collect();
    let raw = leg.json::<RawPage>(&SEARCH,&args)?;
    Ok(SearchPage { items: raw.studies.into_iter().map(|row| row.protocol_section.header()).collect(), total: raw.total_count, next: raw.next_page_token })
}
/// Source processing timestamp, distinct from request time and record update dates.
pub fn version(leg: &Leg<'_>) -> Result<Text,RequestError> { Ok(leg.json::<Version>(&VERSION,&[])?.data_timestamp) }
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Version { data_timestamp: Text }
impl Reply for Version { fn check(&self) -> Result<(),BadReply> { Ok(()) } }
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct RawPage { total_count: u64, #[serde(default)] studies: Vec<Study>, next_page_token: Option<Text> }
impl Reply for RawPage {
    fn check(&self) -> Result<(),BadReply> {
        let mut ids = std::collections::BTreeSet::new();
        if self.studies.len() > 100 || self.studies.len() as u64 > self.total_count || self.studies.iter().any(|r| !ids.insert(r.protocol_section.identification_module.nct_id)) { return Err(BadReply); }
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Study { protocol_section: Protocol }
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Protocol {
    identification_module: Identification,
    status_module: Status,
    design_module: Design,
    conditions_module: Option<Conditions>,
    arms_interventions_module: Option<Interventions>,
}
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Identification { nct_id: NctId, brief_title: Text }
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Status { overall_status: TrialStatus, last_update_post_date_struct: Updated }
#[derive(Deserialize)]
struct Updated { date: PartialDate }
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Design { study_type: StudyType, #[serde(default)] phases: Box<[TrialPhase]> }
#[derive(Deserialize)]
struct Conditions { #[serde(default)] conditions: Box<[Text]> }
#[derive(Deserialize)]
struct Interventions { #[serde(default)] interventions: Vec<Intervention> }
#[derive(Deserialize)]
struct Intervention { name: Text }
impl Protocol {
    fn header(self) -> TrialHeader {
        TrialHeader { nct_id: self.identification_module.nct_id,
            url: Text::new(&format!("https://clinicaltrials.gov/study/{}",self.identification_module.nct_id)).unwrap_or_else(|_| self.identification_module.brief_title.clone()),
            title: self.identification_module.brief_title, status: self.status_module.overall_status,
            phases: self.design_module.phases, study_type: self.design_module.study_type,
            conditions: self.conditions_module.map_or_else(Box::default,|v|v.conditions),
            interventions: self.arms_interventions_module.map_or_else(Vec::new,|v|v.interventions.into_iter().map(|i|i.name).collect()).into(),
            updated: self.status_module.last_update_post_date_struct.date }
    }
}
