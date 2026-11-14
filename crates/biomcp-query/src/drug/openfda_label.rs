//! Native SPL identities and unchanged explicit label headings.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{openfda_label as source, received_date};
use biomcp_fetch::RequestError;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugGet, DrugLabel, DrugLabelText, DrugPage, DrugSearch, DrugSection, DrugSource, PageInfo,
    Request, Section, Source, StepKind,
};
mod names;
mod search_page;
const SOURCE: Source = Source::OpenfdaLabel;
/// An omitted limit takes the request schema's largest, 50; the reply budget usually stops sooner.
fn fragment_limit(query: &DrugGet) -> u32 {
    query.section_limit().unwrap_or(50)
}
pub(super) fn search(cx: &Ctx<'_>, query: &DrugSearch) -> Outcome {
    let mut out = super::empty(
        (DrugPage {
            items: Box::default(),
            total: None,
        })
        .with_query(query.clone()),
    );
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    // A name too long for one bounded request is searched as typed, as it always was.
    let mut expanded = if query.widens_drug_names()
        && (query.name_snapshot().is_some() || !cx.fetch.disabled(SOURCE))
        && names::typed(query).is_some()
    {
        match names::prepare(cx, query, &mut deadline) {
            Ok(Some(saved)) => Some(saved),
            Ok(None) => {
                super::widen::missing(query, &mut out);
                return out;
            }
            Err(error) => {
                out.local = Some(error);
                return out;
            }
        }
    } else {
        None
    };
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let discovery = if let Some(saved) = expanded.as_mut() {
        names::search(&leg, query, saved)
    } else {
        match (query.indication(), query.query()) {
            (Some(indication), name) => {
                source::search_indication(&leg, indication, name, query.limit(), query.offset())
            }
            (None, Some(name)) => source::search(&leg, name, query.limit(), query.offset()),
            (None, None) => return super::unsupported(query.source(), out),
        }
    };
    let result = discovery.page.and_then(|page| {
        out.problems.extend(page.problems);
        let items = search_page::headers(&page.window.rows, query)?;
        Ok(page.window.with_rows(items))
    });
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    let remaining = leg.remaining();
    out.sources
        .push(status::source(SOURCE, leg.finish(), error));
    let sent = super::widen::retain(
        cx,
        query,
        expanded.as_ref(),
        deadline,
        result.is_ok(),
        &mut out,
    );
    disclosure(&mut out, discovery.free_text);
    if let Ok(window) = result {
        let total = window.total().unwrap_or(0);
        search_page::apply(cx, &sent, window, total, &mut out);
    }
    status::search(&mut out);
    names::finish(query, expanded, &mut out);
    command.finish(remaining);
    out
}
fn disclosure(out: &mut Outcome, selector: Option<biodata::Text>) {
    if let Some(selector) = selector
        && let Some(source) = out.sources.first_mut()
    {
        source.term = Some(selector);
        source
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
            data_date: None,
            message:
                "Exact and word name searches found no matching product; searched label free text."
                    .into(),
            retry_after_seconds: None,
        });
    }
}
/// Read one SPL record once and answer each query that names it, such as one per label heading.
pub(super) fn get(cx: &Ctx<'_>, queries: &[DrugGet]) -> Vec<Outcome> {
    let Some(first) = queries.first() else {
        return Vec::new();
    };
    let mut out = empty(first);
    if !first.supports_sections() {
        return queries
            .iter()
            .map(|query| super::unsupported(DrugSource::OpenfdaLabel, empty(query)))
            .collect();
    }
    let id = match first.native_input() {
        Ok(id) => id,
        Err(_) => {
            return queries
                .iter()
                .map(|query| super::rejected_input(query.source(), "input", empty(query)))
                .collect();
        }
    };
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&source::SPEC, deadline).fresh(cx.fresh);
    let result = source::get(&leg, &id);
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    let remaining = leg.remaining();
    out.sources
        .push(status::source(SOURCE, leg.finish(), error));
    if let Ok(page) = result {
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
            .map(|query| {
                let mut part = fork(&out, query);
                let cx = &sharing(cx, left);
                if let Some(record) = page.window.rows.first() {
                    let selected = select(cx, query, record, page.metadata.fetched, &mut part);
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
        command.finish(remaining);
        return outcomes;
    }
    command.finish(remaining);
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
    super::empty(if matches!(query.sections(), [DrugSection::Label]) {
        biomcp_types::DrugLabel::into_reply(query.clone(), None)
    } else {
        biomcp_types::DrugCard::into_reply(query.clone(), None)
    })
}
/// One query's outcome carrying the shared record read.
fn fork(read: &Outcome, query: &DrugGet) -> Outcome {
    super::fork(read, empty(query).body)
}
fn select(
    cx: &Ctx<'_>,
    query: &DrugGet,
    record: &source::Record,
    fetched: Option<i64>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    if query.sections().is_empty() {
        let card = record.card(received_date(fetched))?;
        for heading in source::HEADINGS
            .into_iter()
            .filter(|h| record.has_heading(h))
        {
            if let Some(heading) = crate::next::checked(
                heading.parse(),
                biomcp_types::Source::OpenfdaLabel,
                &mut out.problems,
            ) && let Some(request) = crate::next::checked(
                query.for_label(heading),
                biomcp_types::Source::OpenfdaLabel,
                &mut out.problems,
            ) {
                out.next.push(crate::next::Step::run(
                    StepKind::Section,
                    &[biomcp_types::Source::OpenfdaLabel],
                    Request::GetDrug(request),
                    cx.fresh,
                ));
            }
        }
        out.body = card.with_query(query.clone());
        return Ok(());
    }
    let Some(heading) = query.label_section().map(|h| h.as_str()) else {
        out.body = section_names(query, record)?;
        return Ok(());
    };
    if !source::HEADINGS.contains(&heading) {
        out.body = biomcp_types::ReplyData::Usage(invalid_heading(record)?);
        return Ok(());
    }
    if !record.has_heading(heading) {
        out.body = absent_heading(query, record, heading)?;
        return Ok(());
    }
    let offset = query.section_offset().unwrap_or(0);
    let label = match record.label(heading, offset, fragment_limit(query), cx.label_bytes)? {
        Some(mut data) => {
            data.retrieved = received_date(fetched);
            let returned = data.fragments.len() as u32;
            let total = data.total.unwrap_or(0);
            out.page = Some(page(offset, returned, total));
            if let Some(next) = offset
                .checked_add(returned)
                .filter(|n| u64::from(*n) < total)
                && let Some(request) = crate::next::checked(
                    query.at_section_offset(next),
                    biomcp_types::Source::OpenfdaLabel,
                    &mut out.problems,
                )
            {
                out.next.push(crate::next::Step::run(
                    StepKind::Page,
                    &[biomcp_types::Source::OpenfdaLabel],
                    Request::GetDrug(request),
                    cx.fresh,
                ));
            }
            Section::Data {
                source: SOURCE,
                data,
            }
        }
        None => Section::Empty {
            source: SOURCE,
            record: None,
        },
    };
    out.body = (DrugLabel {
        identity: record.header()?,
        hint: no_room(&label)?,
        label,
        sections: Box::default(),
    })
    .with_query(query.clone());
    Ok(())
}

/// Say why a section sharing a spent reply budget returned no text, since an empty text
/// alone reads as a section with none.
fn no_room(label: &Section<DrugLabelText>) -> Result<Option<biodata::Text>, RequestError> {
    match label {
        Section::Data { data, .. } if data.fragments.is_empty() && data.remaining.is_some() => {
            "No text: earlier sections in this reply used the label budget; follow next to read this section."
                .parse()
                .map(Some)
                .map_err(|_| biomcp_fetch::bad_reply(SOURCE, Some(200)))
        }
        _ => Ok(None),
    }
}

fn page(offset: u32, returned: u32, total: u64) -> PageInfo {
    PageInfo {
        offset,
        returned,
        total,
        pool: None,
        truncated: None,
    }
}

/// A bare label read: list the section names and say how to read one.
fn section_names(
    query: &DrugGet,
    record: &source::Record,
) -> Result<biomcp_types::ReplyData, RequestError> {
    Ok(DrugLabel {
        identity: record.header()?,
        label: Section::Empty {
            source: SOURCE,
            record: None,
        },
        sections: headings(record)?,
        hint: Some(
            "pass sections as label_section (--label-section on the command line), several separated by commas, to read their text"
                .parse()
                .map_err(|_| biomcp_fetch::bad_reply(SOURCE, Some(200)))?,
        ),
    }
    .with_query(query.clone()))
}

/// A heading this label does not carry: say so and list the headings it has, since an
/// empty section alone reads as a heading with no text.
fn absent_heading(
    query: &DrugGet,
    record: &source::Record,
    heading: &str,
) -> Result<biomcp_types::ReplyData, RequestError> {
    Ok(DrugLabel {
        identity: record.header()?,
        label: Section::Empty {
            source: SOURCE,
            record: None,
        },
        sections: headings(record)?,
        hint: Some(
            format!(
                "this label has no {heading} section; sections lists the sections it has, and label_section (--label-section on the command line) reads one"
            )
            .parse()
            .map_err(|_| biomcp_fetch::bad_reply(SOURCE, Some(200)))?,
        ),
    }
    .with_query(query.clone()))
}

fn headings(record: &source::Record) -> Result<Box<[biodata::Text]>, RequestError> {
    source::HEADINGS
        .iter()
        .filter(|h| record.has_heading(h))
        .map(|h| h.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| biomcp_fetch::bad_reply(SOURCE, Some(200)))
        .map(Into::into)
}

fn invalid_heading(record: &source::Record) -> Result<biomcp_types::UsageError, RequestError> {
    Ok(biomcp_types::UsageError::SourceSelector {
        source: SOURCE,
        message: biomcp_types::ErrorMessage::LabelSectionMissing,
        option: "--label-section".into(),
        valid_values: headings(record)?,
        next: vec![biomcp_types::Step {
            kind: StepKind::Configure,
            args: ["help", "get", "drug"].map(Into::into).into(),
        }],
    })
}
