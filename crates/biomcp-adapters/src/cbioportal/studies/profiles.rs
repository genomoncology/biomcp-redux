//! A study's molecular profiles of one native kind, and the caller's choice among them.
use super::rows::endpoint;
use crate::raw::Field;
use biodata::{StudyId, StudyProfileId, Text};
use biomcp_fetch::{Arg, BadReply, Input, Leg, Read, Reply, RequestError};
use biomcp_types::{Source, StudyGet};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    study_id: StudyId,
    molecular_profile_id: StudyProfileId,
    molecular_alteration_type: Text,
    datatype: Option<Field<Text>>,
}
#[derive(Deserialize)]
#[serde(transparent)]
struct Profiles(Vec<Profile>);
impl Reply for Profiles {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
/// The study's mutation profiles, in source order.
pub fn mutation_profiles(leg: &Leg<'_>, q: &StudyGet) -> Result<Vec<StudyProfileId>, RequestError> {
    profiles_of(leg, q, "MUTATION_EXTENDED", None)
}
/// The study's discrete copy-number (GISTIC) profiles, in source order.
pub fn copy_number_profiles(
    leg: &Leg<'_>,
    q: &StudyGet,
) -> Result<Vec<StudyProfileId>, RequestError> {
    profiles_of(leg, q, "COPY_NUMBER_ALTERATION", Some("DISCRETE"))
}
/// The study's structural variant profiles, in source order.
pub fn structural_variant_profiles(
    leg: &Leg<'_>,
    q: &StudyGet,
) -> Result<Vec<StudyProfileId>, RequestError> {
    profiles_of(leg, q, "STRUCTURAL_VARIANT", None)
}
fn profiles_of(
    leg: &Leg<'_>,
    q: &StudyGet,
    kind: &str,
    datatype: Option<&str>,
) -> Result<Vec<StudyProfileId>, RequestError> {
    let check = |p: &Profiles| {
        p.0.iter()
            .all(|p| p.study_id == *q.id())
            .then_some(())
            .ok_or(BadReply)
    };
    let endpoint = endpoint("/studies", &[q.id().as_str(), "molecular-profiles"])?;
    let params = [("projection", Arg::Text("SUMMARY"))];
    let profiles = leg.call(
        &endpoint,
        Input::Params(&params),
        Read::<Profiles>::json().check(&check),
    )?;
    Ok(profiles
        .value
        .0
        .into_iter()
        .filter(|p| {
            p.molecular_alteration_type.as_str() == kind
                && datatype.is_none_or(|want| {
                    p.datatype
                        .as_ref()
                        .and_then(Field::as_ref)
                        .is_some_and(|d| d.as_str() == want)
                })
        })
        .map(|p| p.molecular_profile_id)
        .collect())
}
/// The caller's profile when the study lists it, else the first; none is an unavailable section.
pub(super) fn selected(
    profiles: Vec<StudyProfileId>,
    q: &StudyGet,
) -> Result<StudyProfileId, RequestError> {
    let unavailable = biomcp_fetch::SourceError {
        source: Source::Cbioportal,
        reason: biomcp_types::Reason::Unavailable,
        status: None,
        tls_cause: None,
        retry_after: None,
    };
    let found = profiles
        .into_iter()
        .find(|id| q.molecular_profile().is_none_or(|asked| asked == id));
    found.ok_or_else(|| unavailable.into())
}
