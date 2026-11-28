//! Unpublished records report sample counts separately from their study card.
use super::{SPEC, raw, study_card};
use crate::native_text::NativeText;
use biodata::GwasStudyId;
use biomcp_fetch::{Arg, BadReply, Freshness, Input, Leg, Read, Reply, RequestError};
use biomcp_types::{GwasStudy, GwasStudySample, Problem, ProblemCode, Source};
use serde::Deserialize;

/// Read one unpublished study and the Catalog's sample breakdown.
pub fn unpublished_study(
    leg: &Leg<'_>,
    id: &GwasStudyId,
) -> Result<crate::Mapped<GwasStudy>, RequestError> {
    let path = format!("/unpublished-studies/{id}");
    let mut study = study_card(leg, id, path.clone())?;
    study.value.unpublished = true;
    match read_samples(leg, &path) {
        Ok(samples) => study.value.samples = samples,
        Err(_) => {
            study.refusals = crate::Refusal::Refused;
            study.problems.push(Problem {
                source: Source::Gwas,
                record: crate::mapped::record(id),
                field: "samples".into(),
                code: ProblemCode::CollectionIncomplete,
            });
        }
    }
    Ok(study)
}

#[derive(Debug, Deserialize)]
struct Samples {
    #[serde(rename = "_embedded")]
    embedded: Option<Embedded>,
}
#[derive(Debug, Deserialize)]
struct Embedded {
    unpublished_ancestries: Vec<Sample>,
}
#[derive(Debug, Deserialize)]
struct Sample {
    sample_size: u64,
    ancestry_category: NativeText,
    #[serde(rename = "_links")]
    links: raw::Links,
}
impl Reply for Samples {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}

fn read_samples(leg: &Leg<'_>, study_path: &str) -> Result<Box<[GwasStudySample]>, RequestError> {
    let path = format!("{study_path}/unpublished-ancestries");
    let prefix = format!("{}{path}/", SPEC.base);
    let endpoint = SPEC.get("").with_path(path)?.fresh(Freshness::Days(1));
    let response = leg
        .call(
            &endpoint,
            Input::Params(&[
                ("sort", Arg::Text("sample_size")),
                ("direction", Arg::Text("desc")),
            ]),
            Read::<Samples>::json().check(&|reply| {
                if reply.embedded.as_ref().is_none_or(|rows| {
                    rows.unpublished_ancestries
                        .iter()
                        .all(|row| raw::owned(&row.links.current.href, &prefix))
                }) {
                    Ok(())
                } else {
                    Err(BadReply)
                }
            }),
        )?
        .value;
    Ok(response
        .embedded
        .into_iter()
        .flat_map(|rows| rows.unpublished_ancestries)
        .map(|row| GwasStudySample {
            ancestry: row.ancestry_category.into(),
            count: row.sample_size,
        })
        .collect())
}
