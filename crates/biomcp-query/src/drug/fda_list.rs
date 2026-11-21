//! Paging and text matching shared by FDA drug lists.
use crate::{Ctx, Outcome, status};
use biomcp_types::{DrugSearch, Paged, Request, Source, StepKind};

pub(super) fn page<T>(
    cx: &Ctx<'_>,
    q: &DrugSearch,
    rows: impl Iterator<Item = T>,
    out: &mut Outcome,
) -> Box<[T]> {
    let rows = rows.collect::<Vec<_>>();
    let total = rows.len() as u64;
    let offset = q.offset();
    let items: Box<_> = rows
        .into_iter()
        .skip(offset as usize)
        .take(q.limit() as usize)
        .collect();
    let returned = items.len() as u32;
    out.page = Some(status::page_info(offset, returned, total));
    if let Some(next) = offset
        .checked_add(returned)
        .filter(|next| returned > 0 && u64::from(*next) < total)
    {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[q.source().source()],
            Request::SearchDrug(q.at_offset(next)),
            cx.fresh,
        ));
    }
    items
}

pub(super) fn matches(needle: Option<&str>, cells: &[&str]) -> bool {
    needle.is_none_or(|needle| {
        cells
            .iter()
            .any(|cell| cell.to_lowercase().contains(needle))
    })
}

pub(super) fn source(
    source: Source,
    out: &mut Outcome,
    report: biomcp_fetch::LegReport,
    error: Option<biomcp_fetch::RequestError>,
) {
    out.local = error.and_then(status::local);
    let mut source = status::source(source, report, error);
    if let Some(page) = out.page.as_ref() {
        source.count = Some(page.total);
        source.retrieved_count = Some(page.returned);
    }
    out.sources.push(source);
    status::search(out);
}
