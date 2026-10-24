//! Component restrictions are decided before reading replies or caches.
use crate::{Ctx, status};
use biodata::{
    Build, Reason, Section, Source, SourceOutcome, SourceState, VariantGet, VariantHeader,
    VariantPredictors,
};
use biomcp_fetch::{Deadline, RequestError};
pub(super) fn skipped<T>() -> super::sections::Result<T> {
    (
        Section::Skipped {
            source: Source::MyVariant,
            reason: Reason::Disabled,
        },
        vec![SourceOutcome {
            source: Source::MyVariant,
            state: SourceState::Skipped,
            reason: Some(Reason::Disabled),
            term: None,
            count: None,
            fetched: None,
            notices: Vec::new(),
        }],
        Vec::new(),
        None,
    )
}
pub(super) fn target(candidates: &[VariantHeader]) -> Option<(String, Build)> {
    let [candidate] = candidates else { return None };
    if candidate.source == Source::MyVariant {
        candidate
            .identity
            .representations
            .iter()
            .find_map(|r| r.build)
            .map(|build| (candidate.record.id.to_string(), build))
    } else {
        super::join_id(candidate).map(|id| (id, Build::Grch37))
    }
}
pub(super) fn measurements(
    cx: &Ctx<'_>,
    _query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: Deadline<'_>,
    conservation: bool,
) -> super::sections::Result<VariantPredictors> {
    use biomcp_adapters::myvariant::{self, components};
    let cadd = !conservation && !components::disabled(cx.fetch, components::CADD.name);
    let dbnsfp = !components::disabled(cx.fetch, components::DBNSFP.name);
    if cx.fetch.disabled(Source::MyVariant) || (!cadd && !dbnsfp) {
        return skipped();
    }
    let Some((id, build)) = target(candidates) else {
        return (Section::NotApplicable, Vec::new(), Vec::new(), None);
    };
    let leg = cx.fetch.leg(&myvariant::SPEC, deadline).fresh(cx.fresh);
    let result = myvariant::measurements::get(&leg, &id, build, conservation, cadd, dbnsfp);
    let error = result.as_ref().err().copied();
    let source = status::source(Source::MyVariant, leg.finish(), error);
    let mut problems = Vec::new();
    let section = match result {
        Ok(mapped) => {
            problems = mapped.problems;
            if mapped.value.scores.is_empty() && !problems.is_empty() {
                Section::Unavailable {
                    source: Source::MyVariant,
                    reason: Reason::NoJoinId,
                }
            } else if mapped.value.scores.is_empty() {
                Section::Empty {
                    source: Source::MyVariant,
                    record: candidates.first().map(|c| c.record.clone()),
                }
            } else {
                Section::Data {
                    source: Source::MyVariant,
                    data: mapped.value,
                }
            }
        }
        Err(RequestError::Source(error)) => Section::unavailable(Source::MyVariant, error.reason),
        Err(_) => Section::unavailable(Source::MyVariant, Reason::Unavailable),
    };
    (
        section,
        vec![source],
        problems,
        error.and_then(status::local),
    )
}
