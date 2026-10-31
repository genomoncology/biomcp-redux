use super::{Outcome, ResponseMeta};
use biomcp_query::Action;
use biomcp_types::{DiseaseSection, DiseaseSource, QueryBody, Request};
use std::io;
pub(super) fn unsupported(action: &Action) -> bool {
    match action {
        Action::Run {
            request: Request::SearchDisease(q),
            ..
        } => !matches!(q.source(), DiseaseSource::Ols | DiseaseSource::Seer),
        Action::Run {
            request: Request::GetDisease(q),
            ..
        } => match q.source() {
            DiseaseSource::Ols => !matches!(
                q.section(),
                None | Some(DiseaseSection::Synonyms | DiseaseSection::Xrefs)
            ),
            DiseaseSource::MyDisease => {
                let supported = biomcp_adapters::mydisease::supported_id(q.id().as_str());
                !supported
                    || !matches!(
                        q.section(),
                        None | Some(
                            DiseaseSection::Synonyms
                                | DiseaseSection::Xrefs
                                | DiseaseSection::Hierarchy
                        )
                    )
            }
            DiseaseSource::Monarch => !matches!(
                q.section(),
                Some(DiseaseSection::Genes | DiseaseSection::Phenotypes)
            ),
            DiseaseSource::Seer => !matches!(
                q.section(),
                Some(DiseaseSection::Incidence | DiseaseSection::Survival)
            ),
        },
        _ => false,
    }
}
pub(super) fn prepare(action: Action) -> Result<Action, biomcp_adapters::seer::SelectorError> {
    if let Action::Run {
        request: Request::GetDisease(query),
        fresh,
    } = action
    {
        if query.source() != DiseaseSource::Seer {
            return Ok(Action::Run {
                request: Request::GetDisease(query),
                fresh,
            });
        }
        let selection = biomcp_adapters::seer::selection(&query)?;
        let query = query
            .with_demographics(selection.sex, selection.race, selection.age_range)
            .map_err(|_| biomcp_adapters::seer::SelectorError {
                selector: biomcp_adapters::seer::Selector::Sex,
            })?;
        return Ok(Action::Run {
            request: Request::GetDisease(query),
            fresh,
        });
    }
    Ok(action)
}
pub(super) fn selector_error(error: biomcp_adapters::seer::SelectorError) -> io::Result<Outcome> {
    let values = error
        .selector
        .valid_codes()
        .iter()
        .map(|code| code.parse())
        .collect::<Result<Vec<biodata::Text>, _>>()
        .map_err(io::Error::other)?;
    let mut envelope = biomcp_types::ErrorEnvelope::source_selector(
        biomcp_types::Source::Seer,
        error.selector.flag().into(),
        values.into(),
    );
    envelope.next.push(biomcp_types::Step {
        kind: biomcp_types::StepKind::Disambiguate,
        args: ["search", "disease", "--source", "seer"]
            .map(Into::into)
            .into(),
    });
    super::usage_envelope(envelope)
}
pub(super) fn encode(meta: ResponseMeta, request: Request, body: QueryBody) -> io::Result<Outcome> {
    match (request, body) {
        (Request::SearchDisease(q), QueryBody::DiseasePage(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::SearchDisease(q), QueryBody::DiseaseSites(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::GetDisease(q), QueryBody::DiseaseCard(body)) => meta.encode(q, body),
        (Request::GetDisease(q), QueryBody::DiseaseSynonyms(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseXrefs(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseHierarchy(body)) => meta.encode(q, Some(body)),
        (Request::GetDisease(q), QueryBody::DiseaseGenes(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::GetDisease(q), QueryBody::DiseasePhenotypes(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::GetDisease(q), QueryBody::DiseaseIncidence(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        (Request::GetDisease(q), QueryBody::DiseaseSurvival(body)) => {
            let body = Some(body);
            meta.encode(q, body)
        }
        _ => Err(io::Error::other("disease response mismatch")),
    }
}
