//! Historical COSMIC fragments remain associated with their MyVariant parent record.
use crate::{Ctx, status};

use biodata::Build;
use biomcp_fetch::Deadline;
use biomcp_types::{Reason, Section, Source, VariantCosmic, VariantGet, VariantHeader};
pub(super) fn section(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: &mut Deadline<'_>,
) -> super::sections::Result<VariantCosmic> {
    if biomcp_adapters::myvariant::components::disabled(
        cx.fetch,
        biomcp_adapters::myvariant::components::COSMIC.name,
    ) {
        return super::components::skipped();
    }
    let Some((id, build)) = super::components::target(candidates) else {
        return super::components::no_join(query);
    };
    let cross_build = build == Build::GRCh38 && query.myvariant_build() == Build::GRCh38;
    let (id, build) = if cross_build {
        let Some(id) = super::components::grch37_id(candidates) else {
            return super::components::no_join(query);
        };
        (id, Build::GRCh37)
    } else {
        (id, build)
    };
    let leg = cx
        .fetch
        .leg(&biomcp_adapters::myvariant::SPEC, *deadline)
        .fresh(cx.fresh);
    let started = leg.remaining();
    let result = biomcp_adapters::myvariant::cosmic::get(&leg, &id, build);
    let error = result.as_ref().err().copied();
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let source = status::source(Source::MyVariant, leg.finish(), error);
    let mut dropped = false;
    let mut problems = Vec::new();
    let result = result.map(|mapped| {
        dropped |= mapped.refusals == biomcp_adapters::Refusal::Refused;
        problems = mapped.problems;
        mapped.value
    });
    let section = match result {
        Ok(mut data) if !data.records.is_empty() => {
            data.total = data.records.len() as u64;
            data.records = data
                .records
                .into_vec()
                .into_iter()
                .skip(query.section_offset() as usize)
                .take(query.section_limit() as usize)
                .collect();
            let offset = query
                .section_offset()
                .saturating_add(data.records.len() as u32);
            if (offset as u64) < data.total {
                data.next = Some(query.at_section_offset(offset));
            }
            Section::Data {
                source: Source::MyVariant,
                data,
            }
        }
        Ok(_) if cross_build => Section::unavailable(Source::MyVariant, Reason::NotFound),
        Ok(_) => Section::Empty {
            source: Source::MyVariant,
            record: candidates.first().map(|candidate| candidate.record.clone()),
        },
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
