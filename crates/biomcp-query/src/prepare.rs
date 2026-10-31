//! Capability and selector checks run before transport admission.
use crate::Action;
use biomcp_types::{DiseaseSection, DiseaseSource, ErrorEnvelope, Request};
impl Action {
    /// Prepare source selectors and preserve existing usage refusal envelopes.
    pub(crate) fn prepare(self) -> Result<Self, ErrorEnvelope> {
        let action = prepare_disease(self).map_err(selector_error)?;
        if let Some(error) = unsupported(&action) { return Err(error); }
        Ok(action)
    }
}
fn disease_unsupported(action: &Action) -> bool {
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
fn prepare_disease(action: Action) -> Result<Action, biomcp_adapters::seer::SelectorError> {
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
fn selector_error(error: biomcp_adapters::seer::SelectorError) -> ErrorEnvelope {
    let values = error
        .selector
        .valid_codes()
        .iter()
        .map(|code| code.parse())
        .collect::<Result<Vec<biodata::Text>, _>>();
    let Ok(values) = values else { return ErrorEnvelope::arguments(); };
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
    envelope
}
fn unsupported(action: &Action) -> Option<ErrorEnvelope> {
    if let Action::Run {
        request: Request::GetCellLine(q),
        ..
    } = action
        && matches!(q.id(), biomcp_types::CellLineInput::Source(id) if biomcp_adapters::cellosaurus::native_database(id).is_none())
    {
        return Some(ErrorEnvelope::arguments());
    }
    if let Action::Run {
        request: Request::GetCellLine(query),
        ..
    } = action
        && query.section() == Some(biomcp_types::CellLineSection::All)
        && (query.section_limit().is_some() || query.section_offset() != 0)
    {
        return Some(ErrorEnvelope::cell_line_all_paging());
    }
    if disease_unsupported(action) {
        return Some(ErrorEnvelope::arguments());
    }
    if let Action::Run {
        request: Request::SearchTrial(q),
        ..
    } = action
    {
        if q.nci_disease_invalid() {
            return Some(ErrorEnvelope::nci_disease());
        }
        if let Some(option) = q.unsupported() {
            let source = if q.source() == biomcp_types::TrialSource::Nci {
                biomcp_types::Source::Nci
            } else {
                biomcp_types::Source::ClinicalTrialsGov
            };
            return Some(ErrorEnvelope::unsupported_trial_filter(
                source, option,
            ));
        }
    }

    if let Action::Run {
        request: Request::SearchArticle(query),
        ..
    } = action
        && let Some((source, option)) = crate::article::unsupported(query)
    {
        return Some(ErrorEnvelope::unsupported_filter(source, option));
    }
    None
}
