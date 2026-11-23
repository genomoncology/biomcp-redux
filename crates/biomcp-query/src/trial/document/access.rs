//! Exact manifest membership precedes the bounded read or original-byte download.
use crate::{Ctx, Outcome, status};
use biodata::TrialDocument;
use biomcp_adapters::ctgov;
use biomcp_fetch::Leg;
use biomcp_types::{
    Reason, ReplyBody, ReplyData, Section, Source, TrialDocumentGet, TrialDocumentReadResult,
    TrialDocumentResult, TrialDocumentSaveQuery, UsageError,
};

const SOURCE: Source = Source::ClinicalTrialsGov;

/// A typed source result before the binary writes a caller-owned file.
#[derive(Debug)]
pub struct Download {
    /// Result and attributed request evidence.
    pub out: Outcome,
    /// Original PDF and checked manifest metadata, never cached as a saved file.
    pub file: Option<(TrialDocument, Vec<u8>)>,
}

enum Action {
    Read,
    Download,
    RefuseSave,
}

enum Document {
    Text(
        TrialDocument,
        biomcp_types::PdfText,
        Option<biomcp_adapters::pdf::Embedded>,
    ),
    File(TrialDocument, Vec<u8>),
}

/// Read a document or report the publisher URL when this surface cannot save.
pub(crate) fn run(cx: &Ctx<'_>, query: &TrialDocumentGet) -> Outcome {
    let action = if query.save.is_some() {
        Action::RefuseSave
    } else {
        Action::Read
    };
    if query.replace && query.save.is_none() {
        return status::ready(ReplyData::Usage(UsageError::argument("--save".into())));
    }
    execute(cx, query, action).out
}

/// A disallowed destination returns the source URL without downloading.
pub fn download(cx: &Ctx<'_>, query: &TrialDocumentGet, allow: bool) -> Download {
    if let Some(option) = query
        .find
        .as_ref()
        .map(|_| "--find")
        .or(query.section_offset.map(|_| "--section-offset"))
    {
        return Download {
            out: status::ready(ReplyData::Usage(UsageError::argument(option.into()))),
            file: None,
        };
    }
    let action = if allow {
        Action::Download
    } else {
        Action::RefuseSave
    };
    execute(cx, query, action)
}

fn execute(cx: &Ctx<'_>, query: &TrialDocumentGet, action: Action) -> Download {
    let command = cx.fetch.command_deadline(cx.budget);
    let leg = cx
        .fetch
        .leg(&ctgov::SPEC, command.deadline())
        .fresh(cx.fresh);
    let mut out = status::ready(placeholder(query, None));
    let result = select(&leg, query, &mut out).and_then(|selected| {
        let Some(document) = selected else {
            return Ok(None);
        };
        match action {
            Action::RefuseSave => {
                out.body = ReplyData::Usage(UsageError::ArticleSaveRefused(document.url));
                Ok(None)
            }
            Action::Download => ctgov::document::download(&leg, &document)
                .map(|bytes| Some(Document::File(document, bytes))),
            Action::Read => ctgov::document::text(&leg, &document)
                .map(|(text, read)| Some(Document::Text(document, text, read))),
        }
    });
    let (result, error) = match result {
        Ok(result) => (result, None),
        Err(error) => {
            status::item(&mut out, false, true, false);
            out.local = status::local(error);
            out.body = placeholder(
                query,
                Some(status::reason(error).unwrap_or(Reason::Unavailable)),
            );
            (None, Some(error))
        }
    };
    let remaining = leg.command_remaining();
    out.sources
        .push(status::source(SOURCE, leg.finish(), error));
    command.finish(remaining);
    let file = match result {
        Some(Document::File(document, bytes)) => Some((document, bytes)),
        Some(Document::Text(document, content, read)) => {
            super::text::answer(&mut out, query, document, content, read, cx.fresh);
            None
        }
        None => None,
    };
    Download { out, file }
}

fn select(
    leg: &Leg<'_>,
    query: &TrialDocumentGet,
    out: &mut Outcome,
) -> Result<Option<TrialDocument>, biomcp_fetch::RequestError> {
    let (detail, _) = ctgov::detail::get(leg, query.input)?;
    let incomplete = detail.document_refusals == biomcp_adapters::Refusal::Refused;
    out.include_refusals(detail.refusals);
    status::item(out, true, false, false);
    out.problems.extend(detail.problems);
    let selected = detail
        .documents
        .into_vec()
        .into_iter()
        .find(|d| d.filename == query.file);
    if selected.is_none() {
        if incomplete {
            status::item(out, false, true, false);
            out.body = placeholder(query, Some(Reason::BadReply));
        } else {
            out.body = ReplyData::Usage(UsageError::TextPositionError("file"));
        }
    }
    Ok(selected)
}

fn placeholder(query: &TrialDocumentGet, reason: Option<Reason>) -> ReplyData {
    fn section<T>(reason: Option<Reason>) -> Section<T> {
        reason.map_or(
            Section::Empty {
                source: SOURCE,
                record: None,
            },
            |reason| Section::unavailable(SOURCE, reason),
        )
    }
    match &query.save {
        Some(path) => TrialDocumentResult {
            file: section(reason),
        }
        .with_query(TrialDocumentSaveQuery {
            input: query.input,
            file: query.file.clone(),
            save: path.clone(),
            replace: query.replace,
        }),
        None => TrialDocumentReadResult {
            content: section(reason),
        }
        .with_query(query.clone()),
    }
}
