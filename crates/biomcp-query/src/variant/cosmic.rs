//! Historical COSMIC fragments remain associated with their MyVariant parent record.
use crate::{Ctx, status};
use biodata::{Build, Reason, Section, Source, VariantCosmic, VariantGet, VariantHeader};
use biomcp_fetch::{Deadline, RequestError};
pub(super) fn section(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: Deadline<'_>,
) -> super::components::Result<VariantCosmic> {
    let Some(candidate) = candidates.first() else { return (Section::NotApplicable, Vec::new(), Vec::new(), None); };
    let target = if candidate.source == Source::MyVariant {
        candidate.identity.representations.iter().find_map(|r| r.build)
            .map(|build| (candidate.record.id.to_string(), build))
    } else { super::join_id(candidate).map(|id| (id, Build::Grch37)) };
    let Some((id, build)) = target else { return (Section::NotApplicable, Vec::new(), Vec::new(), None); };
    let leg = cx.fetch.leg(&biomcp_adapters::myvariant::SPEC, deadline).fresh(cx.fresh);
    let result = biomcp_adapters::myvariant::cosmic::get(&leg, &id, build);
    let error = result.as_ref().err().copied();
    let source = status::source(Source::MyVariant, leg.finish(), error);
    let section = match result {
        Ok(mut data) if !data.records.is_empty() => {
            data.total = data.records.len() as u64;
            data.records = data.records.into_vec().into_iter().skip(query.section_offset() as usize).take(query.section_limit() as usize).collect();
            let offset = query.section_offset().saturating_add(data.records.len() as u32);
            if (offset as u64) < data.total { data.next = Some(query.at_section_offset(offset)); }
            Section::Data { source: Source::MyVariant, data }
        },
        Ok(_) => Section::Empty { source: Source::MyVariant, record: Some(candidate.record.clone()) },
        Err(RequestError::Source(error)) => Section::unavailable(Source::MyVariant, error.reason),
        Err(_) => Section::unavailable(Source::MyVariant, Reason::Unavailable),
    };
    (section, vec![source], Vec::new(), error.and_then(status::local))
}
