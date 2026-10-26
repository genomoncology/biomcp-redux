use super::{Outcome, disease, encode, usage_envelope};
use biomcp_query::Action;
use biomcp_types::{ErrorEnvelope, Request};
use std::io;

pub(super) fn check(action: &Action) -> Option<io::Result<Outcome>> {
    if let Action::Run {
        request: Request::GetCellLine(q),
        ..
    } = action
        && matches!(q.id(), biomcp_types::CellLineInput::Source(id) if biomcp_adapters::cellosaurus::native_database(id).is_none())
    {
        return Some(usage_envelope(ErrorEnvelope::arguments()));
    }
    if let Action::Run {
        request: Request::GetCellLine(query),
        ..
    } = action
        && query.section() == Some(biomcp_types::CellLineSection::All)
        && (query.section_limit().is_some() || query.section_offset() != 0)
    {
        return Some(usage_envelope(ErrorEnvelope::cell_line_all_paging()));
    }
    if disease::unsupported(action) {
        return Some(usage_envelope(ErrorEnvelope::arguments()));
    }
    if let Action::Run {
        request: Request::SearchTrial(q),
        ..
    } = action
    {
        if q.nci_disease_invalid() {
            return Some(usage_envelope(ErrorEnvelope::nci_disease()));
        }
        if let Some(option) = q.unsupported() {
            let source = if q.source() == biomcp_types::TrialSource::Nci {
                biomcp_types::Source::Nci
            } else {
                biomcp_types::Source::ClinicalTrialsGov
            };
            return Some(usage_envelope(ErrorEnvelope::unsupported_trial_filter(
                source, option,
            )));
        }
    }

    if let Action::Run {
        request: Request::SearchArticle(query),
        ..
    } = action
        && let Some((source, option)) = biomcp_query::article::unsupported(query)
    {
        return Some(encode(
            ErrorEnvelope::unsupported_filter(source, option),
            crate::exit::Status::Usage,
        ));
    }
    None
}
