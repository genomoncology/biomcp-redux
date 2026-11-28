use crate::{Ctx, status};

use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    CivicMembership, Problem, Reason, Section, Source, SourceOutcome, SourceStatus, VariantCivic,
    VariantGet, VariantHeader, VariantNamespace,
};

pub(super) fn section(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: Deadline<'_>,
) -> (
    Section<VariantCivic>,
    Vec<SourceOutcome>,
    Vec<Problem>,
    Option<RequestError>,
    bool,
) {
    let leg = cx
        .fetch
        .leg(&biomcp_adapters::civic::SPEC, deadline)
        .fresh(cx.fresh);
    let result = candidates.first().map_or_else(
        || {
            Err(biomcp_fetch::SourceError {
                source: Source::Civic,
                reason: Reason::NoJoinId,
                status: None,
                tls_cause: None,
                retry_after: None,
            }
            .into())
        },
        |candidate| biomcp_adapters::civic::variant(&leg, query, candidate),
    );
    let error = result.as_ref().err().copied();
    let dropped = result
        .as_ref()
        .is_ok_and(|mapped| mapped.refusals == biomcp_adapters::Refusal::Refused);
    let mut source = status::source(Source::Civic, leg.finish(), error);
    let no_join = error.and_then(crate::status::reason) == Some(Reason::NoJoinId)
        || query.input().namespace() != VariantNamespace::CivicProfile
            && result.as_ref().is_ok_and(|mapped| {
                mapped
                    .value
                    .profiles
                    .iter()
                    .all(|profile| profile.membership == CivicMembership::Candidate)
            });
    if no_join {
        source.set_status(SourceStatus::Failed(Some(Reason::NoJoinId)));
        source.notices.push(biomcp_types::SourceNotice::RequestGuidance {
            data_date: None,
            message: "A CIViC profile may exist even when this allele cannot be joined. Use its CIViC profile ID or exact name with civic alone.".into(),
            retry_after_seconds: None,
        });
    }
    let (value, problems) = match result {
        Ok(mapped) => (
            if no_join {
                Section::unavailable(Source::Civic, Reason::NoJoinId)
            } else if mapped.value.profiles.is_empty() {
                Section::Empty {
                    source: Source::Civic,
                    record: None,
                }
            } else {
                Section::Data {
                    source: Source::Civic,
                    data: mapped.value,
                }
            },
            mapped.problems,
        ),
        Err(error) => (
            Section::unavailable(
                Source::Civic,
                crate::status::reason(error).unwrap_or(biomcp_types::Reason::Unavailable),
            ),
            Vec::new(),
        ),
    };
    (
        value,
        vec![source],
        problems,
        error.and_then(status::local),
        dropped,
    )
}

/// A selected profile that is not the one an exact-name input names is refused with the input's
/// own profile ID, never answered as a complete empty page (1251). Numeric inputs are refused
/// before any source is read.
pub(super) fn profile_refused(
    cx: &Ctx<'_>,
    query: &VariantGet,
    deadline: Deadline<'_>,
) -> Option<crate::Outcome> {
    let selected = query.civic_profile()?;
    if query.input().namespace() != VariantNamespace::CivicProfile
        || query.input().as_str().parse::<u64>().is_ok()
    {
        return None;
    }
    let leg = cx
        .fetch
        .leg(&biomcp_adapters::civic::SPEC, deadline)
        .fresh(cx.fresh);
    let id = match biomcp_adapters::civic::profile_id(&leg, query) {
        Ok(Some(id)) if id != selected => id,
        _ => return None,
    };
    let mut out = status::ready(biomcp_types::ReplyData::Usage(
        biomcp_types::UsageError::Arguments(biomcp_types::UsageArguments {
            argument: Some("--civic-profile".into()),
            message: Some(biomcp_types::ErrorMessage::ArgumentValue),
            // The refusal stands either way; a decimal ID is always valid text.
            valid_values: Some(
                biodata::Text::new(&id.to_string())
                    .map(|value| vec![value].into())
                    .unwrap_or_default(),
            ),
            next: Vec::new(),
        }),
    ));
    out.sources
        .push(status::source(Source::Civic, leg.finish(), None));
    Some(out)
}
