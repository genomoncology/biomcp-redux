//! Map full current-label section reads and phrase searches to the reply.
use super::SOURCE;
use crate::{Ctx, Outcome};
use biomcp_adapters::{openfda_label as source, received_date};
use biomcp_fetch::RequestError;
use biomcp_types::{DrugGet, ReplyBody, Section, SourceStatus};
use biomcp_types::{DrugLabel, DrugLabelText, Request, StepKind};

/// An omitted fragment limit keeps the default page size; the text budget may stop it sooner.
const DEFAULT_FRAGMENT_LIMIT: u32 = 50;
/// An identity card with one next call per label heading the record carries.
fn card(
    cx: &Ctx<'_>,
    query: &DrugGet,
    record: &source::Record,
    fetched: Option<i64>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
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
    Ok(())
}
pub(super) fn select(
    cx: &Ctx<'_>,
    query: &DrugGet,
    record: &source::Record,
    fetched: Option<i64>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    if query.sections().is_empty() {
        return card(cx, query, record, fetched, out);
    }
    if !query.label_find().is_empty() {
        return find(query, record, fetched, out);
    }
    let recent_major_changes = recent(query, record, &mut out.problems);
    let Some(asked) = query.label_section().map(|h| h.as_str()) else {
        out.body = section_names(query, record, recent_major_changes)?;
        return Ok(());
    };
    // A name openFDA does not use is absent from this label like a field the label lacks.
    let Some(heading) =
        biomcp_types::openfda_label_field(asked).filter(|heading| record.has_heading(heading))
    else {
        out.body = absent_heading(query, record, asked, recent_major_changes)?;
        return Ok(());
    };
    let offset = query.section_offset().unwrap_or(0);
    let limit = query.section_limit().unwrap_or(DEFAULT_FRAGMENT_LIMIT);
    let label = match record.label(heading, offset, limit, cx.label_bytes)? {
        Some(mut data) => {
            data.retrieved = received_date(fetched);
            let returned = data.fragments.len() as u32;
            let total = data.total.unwrap_or(0);
            out.page = Some(crate::status::page_info(offset, returned, total));
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
        recent_major_changes,
        hint: no_room(&label)?.or(printed_name(asked, heading)?),
        label,
        sections: Box::default(),
    })
    .with_query(query.clone());
    Ok(())
}

/// Search the whole section and cite the result without sending its full text.
fn find(
    query: &DrugGet,
    record: &source::Record,
    fetched: Option<i64>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let identity = record.header()?;
    let asked = query.label_section().map_or("", biodata::Text::as_str);
    let result = biomcp_types::openfda_label_field(asked).map_or(Ok(None), |heading| {
        record.find_label(heading, query.label_find())
    });
    let (label, hint) = match result {
        Ok(Some(mut data)) => {
            data.retrieved = received_date(fetched);
            let hint = printed_name(asked, data.heading.as_str())?;
            (
                Section::Data {
                    source: SOURCE,
                    data,
                },
                hint,
            )
        }
        Ok(None) => {
            out.body = absent_heading(query, record, asked, Box::default())?;
            return Ok(());
        }
        Err(error) => {
            fail_source(out, error);
            crate::status::item(out, true, false, true);
            (
                Section::unavailable(SOURCE, biomcp_types::Reason::BadReply),
                None,
            )
        }
    };
    out.body = DrugLabel {
        identity,
        recent_major_changes: Box::default(),
        label,
        sections: Box::default(),
        hint,
    }
    .with_query(query.clone());
    Ok(())
}

/// Keep the source read's counts and notices when its label section cannot be used.
pub(super) fn fail_source(out: &mut Outcome, error: RequestError) {
    if let Some(source) = out.sources.first_mut() {
        source.set_status(SourceStatus::Failed(crate::status::reason(error)));
    }
}

/// The label's recent major changes, except beside a read of that section, which already
/// carries its text.
fn recent(
    query: &DrugGet,
    record: &source::Record,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Box<[biodata::Text]> {
    if query
        .label_section()
        .is_some_and(|h| h.as_str().starts_with("recent_major_changes"))
    {
        return Box::default();
    }
    let (changes, rejected) = record.recent_major_changes();
    problems.extend(rejected);
    changes
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

/// Name the openFDA field read when the caller asked by the printed label's name for it, so the
/// reply says which field the text came from.
fn printed_name(asked: &str, heading: &str) -> Result<Option<biodata::Text>, RequestError> {
    if asked == heading {
        return Ok(None);
    }
    format!("{asked} is the printed label's name for this section; this reply reads openFDA's {heading} field")
        .parse()
        .map(Some)
        .map_err(|_| biomcp_fetch::bad_reply(SOURCE, Some(200)))
}

/// A bare label read: list the section names and say how to read one.
fn section_names(
    query: &DrugGet,
    record: &source::Record,
    recent_major_changes: Box<[biodata::Text]>,
) -> Result<biomcp_types::ReplyData, RequestError> {
    Ok(DrugLabel {
        identity: record.header()?,
        recent_major_changes,
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

/// A heading this label does not carry: say so, naming the label, and list the headings it
/// has, since an empty section alone reads as a heading with no text and an agent must be
/// able to quote which label lacks it.
fn absent_heading(
    query: &DrugGet,
    record: &source::Record,
    heading: &str,
    recent_major_changes: Box<[biodata::Text]>,
) -> Result<biomcp_types::ReplyData, RequestError> {
    let identity = record.header()?;
    let hint = format!(
        "{} has no {heading} section; sections lists the sections it has, and label_section (--label-section on the command line) reads one",
        label_name(&identity)
    );
    Ok(DrugLabel {
        identity,
        recent_major_changes,
        label: Section::Empty {
            source: SOURCE,
            record: None,
        },
        sections: headings(record)?,
        hint: Some(
            hint.parse()
                .map_err(|_| biomcp_fetch::bad_reply(SOURCE, Some(200)))?,
        ),
    }
    .with_query(query.clone()))
}

/// The label as an agent can cite it: brand (or generic) name, SPL set ID and version.
fn label_name(identity: &biomcp_types::DrugHeader) -> String {
    let set = identity.id.to_string();
    let version = identity
        .version
        .as_ref()
        .map(|v| format!(", version {v}"))
        .unwrap_or_default();
    match identity.name.as_ref().map(|n| n.to_string()) {
        Some(name) if name != set => format!("{name}'s label (set {set}{version})"),
        _ => format!("The label (set {set}{version})"),
    }
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
