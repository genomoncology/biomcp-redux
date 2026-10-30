//! Complete native metadata collections with request-bound cache admission.
use super::super::SPEC;
use crate::mapped::Problems;
use crate::raw::Field;
use biodata::{StudyId, StudyProfile, StudyProfileId, StudySampleList, StudySampleListId, Text};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, PathSegment, Reply, RequestError};
use serde::Deserialize;
/// Measured MSK DETAILED response plus fifteen percent, rounded upward.
const MAX_SAMPLE_LIST_BYTES: u64 = 1_526_432;
/// Complete source collection, after dropping records belonging to another study.
#[derive(Debug)]
pub struct Records<T> {
    /// Rejected metadata on retained records.
    pub problems: Vec<biomcp_types::Problem>,
    /// Checked records in their original source order.
    pub rows: Vec<Option<T>>,
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
    molecular_alteration_type: Option<Field<Text>>,
    datatype: Option<Field<Text>>,
    name: Option<Field<Box<str>>>,
    description: Option<Field<Box<str>>>,
    generic_assay_type: Option<Field<Text>>,
    patient_level: Option<Field<bool>>,
    show_profile_in_analysis_tab: Option<Field<bool>>,
}
impl Profile {
    fn mapped(self, problems: &mut Problems) -> StudyProfile {
        problems.record = crate::mapped::record(&self.molecular_profile_id);
        StudyProfile {
            molecular_profile_id: self.molecular_profile_id,
            study_id: self.study_id,
            molecular_alteration_type: problems
                .required_field("molecularAlterationType", self.molecular_alteration_type),
            datatype: problems.required_field("datatype", self.datatype),
            name: problems.required_field("name", self.name),
            description: problems.required_field("description", self.description),
            generic_assay_type: problems.field("genericAssayType", self.generic_assay_type),
            patient_level: problems.required_field("patientLevel", self.patient_level),
            show_profile_in_analysis_tab: problems.required_field(
                "showProfileInAnalysisTab",
                self.show_profile_in_analysis_tab,
            ),
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SampleList {
    sample_list_id: StudySampleListId,
    study_id: StudyId,
    category: Option<Field<Text>>,
    name: Option<Field<Box<str>>>,
    description: Option<Field<Box<str>>>,
    sample_count: Option<Field<u64>>,
    // Unknown sampleIds is discarded by serde without storing or counting members.
}
impl SampleList {
    fn mapped(self, problems: &mut Problems) -> StudySampleList {
        problems.record = crate::mapped::record(&self.sample_list_id);
        StudySampleList {
            sample_list_id: self.sample_list_id,
            study_id: self.study_id,
            category: problems.required_field("category", self.category),
            name: problems.required_field("name", self.name),
            description: problems.required_field("description", self.description),
            sample_count: problems.field("sampleCount", self.sample_count),
        }
    }
}
#[derive(Deserialize)]
#[serde(transparent)]
struct Profiles(Vec<super::super::scope::Row<Profile, super::super::scope::Study>>);
#[derive(Deserialize)]
#[serde(transparent)]
struct SampleLists(Vec<super::super::scope::Row<SampleList, super::super::scope::Study>>);
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
        |r| r.0.iter().all(|v| v.scope.study_id == *id),
    )?;
    let mut problems = Problems {
        source: biomcp_types::Source::Cbioportal,
        record: None,
        list: Vec::new(),
    };
    let total = reply.0.len();
    let rows: Vec<_> = reply
        .0
        .into_iter()
        .map(|v| {
            problems.record = None;
            let row = problems.field("molecularProfileId", Some(v.value))?;
            (row.study_id == *id).then(|| row.mapped(&mut problems))
        })
        .collect();
    Ok(Records {
        rejected: total - rows.iter().flatten().count(),
        rows,
        problems: problems.list,
        acquired: metadata
            .fetched
            .map(u64::try_from)
            .transpose()
            .map_err(|_| bad())?,
    })
}
/// Exact DETAILED metadata; source sampleCount remains optional and member IDs have no field.
pub fn sample_lists(leg: &Leg<'_>, id: &StudyId) -> Result<Records<StudySampleList>, RequestError> {
    let (reply, metadata) = leg.json_received_checked_cache::<SampleLists>(
        &endpoint(id, "sample-lists")?,
        &[("projection", Arg::Text("DETAILED"))],
        |r| r.0.iter().all(|v| v.scope.study_id == *id),
    )?;
    let mut problems = Problems {
        source: biomcp_types::Source::Cbioportal,
        record: None,
        list: Vec::new(),
    };
    let total = reply.0.len();
    let rows: Vec<_> = reply
        .0
        .into_iter()
        .map(|v| {
            problems.record = None;
            let row = problems.field("sampleListId", Some(v.value))?;
            (row.study_id == *id).then(|| row.mapped(&mut problems))
        })
        .collect();
    Ok(Records {
        rejected: total - rows.iter().flatten().count(),
        rows,
        problems: problems.list,
        acquired: metadata
            .fetched
            .map(u64::try_from)
            .transpose()
            .map_err(|_| bad())?,
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
