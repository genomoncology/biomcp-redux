//! Component restrictions are decided before reading replies or caches.
use super::sections::Result;
use crate::{Ctx, status};
use biodata::Build;
use biomcp_fetch::Deadline;
use biomcp_types::{
    Reason, Section, Source, SourceNotice, VariantGet, VariantHeader, VariantPredictors,
};
pub(super) fn skipped<T>() -> Result<T> {
    (
        Section::Skipped {
            source: Source::MyVariant,
            reason: Reason::Disabled,
        },
        vec![biomcp_adapters::registry::outcome(
            Source::MyVariant,
            biomcp_types::SourceStatus::Skipped(Some(Reason::Disabled)),
        )],
        Vec::new(),
        None,
        false,
    )
}
pub(super) fn target(candidates: &[VariantHeader]) -> Option<(String, Build)> {
    let [candidate] = candidates else { return None };
    if candidate.source == Source::MyVariant
        && let Some(build) = super::identity::native_build(candidate)
    {
        return Some((candidate.record.id.to_string(), build));
    }
    [biomcp_types::VariantGet::DEFAULT_BUILD, Build::GRCh37]
        .into_iter()
        .find_map(|build| super::join_for_build(candidate, build).map(|id| (id, build)))
}
pub(super) fn grch37_id(candidates: &[VariantHeader]) -> Option<String> {
    let [candidate] = candidates else { return None };
    super::join_id(candidate)
}

pub(super) fn no_join<T>(query: &VariantGet) -> Result<T> {
    let section = if query.myvariant_build() == Build::GRCh37 {
        Section::NotApplicable
    } else {
        Section::unavailable(Source::MyVariant, Reason::NoJoinId)
    };
    (section, Vec::new(), Vec::new(), None, false)
}

pub(super) fn measurements(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: &mut Deadline<'_>,
    conservation: bool,
) -> Result<VariantPredictors> {
    use biomcp_adapters::myvariant::components;
    let cadd = !conservation && !components::disabled(cx.fetch, components::CADD.name);
    let dbnsfp = !components::disabled(cx.fetch, components::DBNSFP.name);
    if cx.fetch.disabled(Source::MyVariant) || (!cadd && !dbnsfp) {
        return skipped();
    }
    let Some(target) = target(candidates) else {
        return no_join(query);
    };
    let grch38 = target.1 == Build::GRCh38 && query.myvariant_build() == Build::GRCh38;
    if !cadd || !grch38 {
        let mut result = read(cx, candidates, deadline, target, conservation, cadd, dbnsfp);
        if conservation && grch38 && matches!(result.0, Section::Empty { .. }) {
            result.0 = Section::unavailable(Source::MyVariant, Reason::NotFound);
        }
        return result;
    }

    let native = dbnsfp.then(|| read(cx, candidates, deadline, target, false, false, true));
    let mut cadd = grch37_id(candidates).map_or_else(
        || no_join(query),
        |id| {
            read(
                cx,
                candidates,
                deadline,
                (id, Build::GRCh37),
                false,
                true,
                false,
            )
        },
    );
    let reason = match &cadd.0 {
        Section::Empty { .. } => Some(Reason::NotFound),
        Section::Data { data, .. } if data.scores.is_empty() => Some(Reason::BadReply),
        _ => None,
    };
    if let Some(reason) = reason {
        cadd.0 = Section::unavailable(Source::MyVariant, reason);
    }
    match native {
        Some(native) => with_cadd(native, cadd),
        None => cadd,
    }
}

fn read(
    cx: &Ctx<'_>,
    candidates: &[VariantHeader],
    deadline: &mut Deadline<'_>,
    (id, build): (String, Build),
    conservation: bool,
    cadd: bool,
    dbnsfp: bool,
) -> Result<VariantPredictors> {
    use biomcp_adapters::myvariant;
    let leg = cx.fetch.leg(&myvariant::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let result = myvariant::measurements::get(&leg, &id, build, conservation, cadd, dbnsfp);
    let error = result.as_ref().err().copied();
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let source = status::source(Source::MyVariant, leg.finish(), error);
    let mut dropped = false;
    let mut problems = Vec::new();
    let section = match result {
        Ok(mapped) => {
            dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
            problems = mapped.problems;
            if problems
                .iter()
                .any(|p| p.field.as_ref() == "subject_qualification")
            {
                Section::Unavailable {
                    source: Source::MyVariant,
                    reason: Reason::NoJoinId,
                }
            } else if mapped.value.scores.is_empty()
                && mapped.value.context.is_empty()
                && (problems.is_empty() || problems.iter().any(|p| p.field.as_ref() == "_id"))
            {
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
        Err(error) => Section::unavailable(
            Source::MyVariant,
            crate::status::reason(error).unwrap_or(biomcp_types::Reason::Unavailable),
        ),
    };
    (
        section,
        vec![source],
        problems,
        error.and_then(status::local),
        dropped,
    )
}

fn with_cadd(
    native: Result<VariantPredictors>,
    cadd: Result<VariantPredictors>,
) -> Result<VariantPredictors> {
    let (mut section, mut sources, mut problems, local, mut incomplete) = native;
    let (cadd, more_sources, more_problems, more_local, dropped) = cadd;
    match cadd {
        Section::Data { source, mut data } => {
            incomplete |= matches!(section, Section::Unavailable { .. });
            if let Section::Data { data: dbnsfp, .. } = section {
                data.scores = data
                    .scores
                    .into_vec()
                    .into_iter()
                    .chain(dbnsfp.scores)
                    .collect();
                data.components = data
                    .components
                    .into_vec()
                    .into_iter()
                    .chain(dbnsfp.components)
                    .collect();
                data.context = data
                    .context
                    .into_vec()
                    .into_iter()
                    .chain(dbnsfp.context)
                    .collect();
            }
            section = Section::Data { source, data };
        }
        Section::Unavailable { reason, .. } => {
            if let Some(source) = sources.first_mut() {
                source.notices.push(SourceNotice::RequestGuidance {
                    data_date: None,
                    message: format!("CADD unavailable in MyVariant's GRCh37 index ({reason}).")
                        .into(),
                    retry_after_seconds: None,
                });
            }
            incomplete = true;
        }
        _ => {}
    }
    sources.extend(more_sources);
    problems.extend(more_problems);
    (
        section,
        sources,
        problems,
        local.or(more_local),
        incomplete || dropped,
    )
}
