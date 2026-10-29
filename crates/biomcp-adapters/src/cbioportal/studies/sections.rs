//! Complete native metadata collections with request-bound cache admission.
use super::super::SPEC;
use biodata::{StudyId, StudyProfile, StudyProfileId, StudySampleList, StudySampleListId, Text};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, PathSegment, Reply, RequestError};
use serde::Deserialize;
/// Measured MSK DETAILED response plus fifteen percent, rounded upward.
const MAX_SAMPLE_LIST_BYTES: u64 = 1_526_432;
/// Complete source collection, after dropping records belonging to another study.
#[derive(Debug)]
pub struct Records<T> {
    /// Checked records in their original source order.
    pub rows: Vec<T>,
    /// Original response acquisition time, including cache reads.
    pub acquired: Option<u64>,
    /// Number of rejected records with a different study identity.
    pub rejected: usize,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    molecular_profile_id: StudyProfileId,
    study_id: StudyId,
    molecular_alteration_type: Text,
    datatype: Text,
    name: Box<str>,
    description: Box<str>,
    generic_assay_type: Option<Text>,
    patient_level: bool,
    show_profile_in_analysis_tab: bool,
}
impl Profile {
    fn mapped(self) -> StudyProfile {
        StudyProfile {
            molecular_profile_id: self.molecular_profile_id,
            study_id: self.study_id,
            molecular_alteration_type: self.molecular_alteration_type,
            datatype: self.datatype,
            name: self.name,
            description: self.description,
            generic_assay_type: self.generic_assay_type,
            patient_level: self.patient_level,
            show_profile_in_analysis_tab: self.show_profile_in_analysis_tab,
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SampleList {
    sample_list_id: StudySampleListId,
    study_id: StudyId,
    category: Text,
    name: Box<str>,
    description: Box<str>,
    sample_count: Option<u64>,
    // Unknown sampleIds is discarded by serde without storing or counting members.
}
impl SampleList {
    fn mapped(self) -> StudySampleList {
        StudySampleList {
            sample_list_id: self.sample_list_id,
            study_id: self.study_id,
            category: self.category,
            name: self.name,
            description: self.description,
            sample_count: self.sample_count,
        }
    }
}
#[derive(Deserialize)]
#[serde(transparent)]
struct Profiles(Vec<Profile>);
#[derive(Deserialize)]
#[serde(transparent)]
struct SampleLists(Vec<SampleList>);
impl Reply for Profiles {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl Reply for SampleLists {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
fn endpoint(id: &StudyId, name: &str) -> Result<biomcp_fetch::Endpoint, RequestError> {
    Ok(SPEC
        .get("/studies")
        .with_segments(&[PathSegment::new(id.as_str())?, PathSegment::new(name)?])
        .fresh(Freshness::Days(1))
        .bytes(MAX_SAMPLE_LIST_BYTES))
}
/// Exact DETAILED profiles; embedded source study records are never retained.
pub fn profiles(leg: &Leg<'_>, id: &StudyId) -> Result<Records<StudyProfile>, RequestError> {
    let (reply, metadata) = leg.json_received_checked_cache::<Profiles>(
        &endpoint(id, "molecular-profiles")?,
        &[("projection", Arg::Text("DETAILED"))],
        |r| r.0.iter().all(|v| v.study_id == *id),
    )?;
    let rejected = reply.0.iter().filter(|v| v.study_id != *id).count();
    Ok(Records {
        rows: reply
            .0
            .into_iter()
            .filter(|v| v.study_id == *id)
            .map(Profile::mapped)
            .collect(),
        acquired: metadata
            .fetched
            .map(u64::try_from)
            .transpose()
            .map_err(|_| bad())?,
        rejected,
    })
}
/// Exact DETAILED metadata; source sampleCount remains optional and member IDs have no field.
pub fn sample_lists(leg: &Leg<'_>, id: &StudyId) -> Result<Records<StudySampleList>, RequestError> {
    let (reply, metadata) = leg.json_received_checked_cache::<SampleLists>(
        &endpoint(id, "sample-lists")?,
        &[("projection", Arg::Text("DETAILED"))],
        |r| r.0.iter().all(|v| v.study_id == *id),
    )?;
    let rejected = reply.0.iter().filter(|v| v.study_id != *id).count();
    Ok(Records {
        rows: reply
            .0
            .into_iter()
            .filter(|v| v.study_id == *id)
            .map(SampleList::mapped)
            .collect(),
        acquired: metadata
            .fetched
            .map(u64::try_from)
            .transpose()
            .map_err(|_| bad())?,
        rejected,
    })
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: biomcp_types::Source::Cbioportal,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
