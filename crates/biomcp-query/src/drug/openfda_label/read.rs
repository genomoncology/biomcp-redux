//! Share one source read across requested headings for a label.
use super::{SOURCE, section::select};
use crate::status;
use crate::{Ctx, Outcome};
use biomcp_adapters::openfda_label as source;
use biomcp_fetch::Deadline;
use biomcp_types::DrugSource;
use biomcp_types::{DrugGet, DrugSection, ReplyBody, Section};

/// Read each SPL set once, with one source budget across the request.
pub(in crate::drug) fn get(cx: &Ctx<'_>, queries: &[DrugGet]) -> Vec<Outcome> {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let outcomes = queries
        .chunk_by(|a, b| a.input() == b.input())
        .flat_map(|group| read(cx, group, deadline))
        .collect();
    command.finish(deadline.remaining());
    outcomes
}

/// Read one SPL record once and answer each query that names it, such as one per label heading.
fn read(cx: &Ctx<'_>, queries: &[DrugGet], deadline: Deadline<'_>) -> Vec<Outcome> {
    let Some(first) = queries.first() else {
        return Vec::new();
    };
    let mut out = empty(first);
    if !first.supports_sections() {
        return queries
            .iter()
            .map(|query| super::super::unsupported(DrugSource::OpenfdaLabel, empty(query)))
            .collect();
    }
    let id = match first.native_input() {
        Ok(id) => id,
        Err(_) => {
            return queries
                .iter()
                .map(|query| super::super::rejected_input(query.source(), "input", empty(query)))
                .collect();
        }
    };
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let (page, error) = match source::get(&leg, &id) {
        Ok(page) => (Some(page), None),
        Err(error) => (None, Some(error)),
    };
    out.local = error.and_then(status::local);
    out.sources
        .push(status::source(SOURCE, leg.finish(), error));
    if let Some(page) = page {
        out.problems.extend(page.problems);
        out.include_refusals(page.window.refusals());
        crate::status::item(&mut out, true, false, false);
        if let Some(source) = out.sources.first_mut() {
            source.count = Some(page.window.total().unwrap_or(0));
            source.retrieved_count = Some(page.window.rows.len() as u32);
        }
        // The label budget is per reply: sections named together share it in order, and a
        // section left without room returns no text and continues by its remaining fragments.
        let mut left = cx.label_bytes;
        let outcomes = queries
            .iter()
            .enumerate()
            .map(|(at, query)| {
                let mut part = fork(&out, query);
                let cx = &sharing(cx, left);
                if let Some(record) = page.window.rows.first() {
                    let selected = select(cx, query, record, page.metadata.fetched, &mut part);
                    // Recent major changes describe the label, not a heading: once per reply.
                    if at > 0
                        && let biomcp_types::ReplyData::DrugLabel {
                            body: Some(body), ..
                        } = &mut part.body
                    {
                        body.recent_major_changes = Box::default();
                    }
                    left = left.saturating_sub(text_bytes(&part));
                    if let Err(error) = selected {
                        crate::status::item(&mut part, false, true, false);
                        part.local = status::local(error);
                    }
                } else {
                    crate::status::item(&mut part, false, false, false);
                }
                part
            })
            .collect();
        return outcomes;
    }
    queries.iter().map(|query| fork(&out, query)).collect()
}
/// The same context with the label budget still left in this reply.
fn sharing<'a>(cx: &Ctx<'a>, label_bytes: usize) -> Ctx<'a> {
    Ctx {
        fetch: cx.fetch,
        // As in resolve::within: drug source credentials remain in the borrowed Fetch, no drug
        // source contacts NCBI, and the contact (a Secret) cannot be copied.
        ncbi_email: None,
        budget: cx.budget,
        fresh: cx.fresh,
        drug_name_limit: cx.drug_name_limit,
        label_bytes,
        fit_deadline: cx.fit_deadline,
    }
}
/// Escaped bytes of label text a part returned, as the adapter counts them.
fn text_bytes(part: &Outcome) -> usize {
    match &part.body {
        biomcp_types::ReplyData::DrugLabel {
            body: Some(body), ..
        } => match &body.label {
            Section::Data { data, .. } => {
                serde_json::to_string(&data.text).map_or(0, |text| text.len().saturating_sub(2))
            }
            _ => 0,
        },
        _ => 0,
    }
}
fn empty(query: &DrugGet) -> Outcome {
    super::super::empty(if matches!(query.sections(), [DrugSection::Label]) {
        biomcp_types::DrugLabel::into_reply(query.clone(), None)
    } else {
        biomcp_types::DrugCard::into_reply(query.clone(), None)
    })
}
/// One query's outcome carrying the shared record read.
fn fork(read: &Outcome, query: &DrugGet) -> Outcome {
    super::super::fork(read, empty(query).body)
}
