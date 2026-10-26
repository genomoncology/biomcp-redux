//! Exact manifest membership precedes the shared original-byte download.
use crate::{Ctx, Outcome, status};
use biodata::TrialDocument;
use biomcp_adapters::ctgov;
use biomcp_fetch::SourceError;
use biomcp_types::{QueryBody, Section, Source, Status, TrialDocumentGet, TrialDocumentResult};
/// A typed source result before the binary writes a caller-owned file.
#[derive(Debug)]
pub struct Download {
    /// Result and attributed request evidence.
    pub out: Outcome,
    /// Original PDF and checked manifest metadata, never cached as a saved file.
    pub file: Option<(TrialDocument, Vec<u8>)>,
}
/// A disallowed surface or destination returns the source URL without downloading.
pub fn download(cx: &Ctx<'_>, query: &TrialDocumentGet, allow: bool) -> Download {
    let command = cx.fetch.command_deadline(cx.budget);
    let leg = cx
        .fetch
        .leg(&ctgov::SPEC, command.deadline())
        .fresh(cx.fresh);
    let mut out = Outcome {
        status: Status::Complete,
        body: QueryBody::TrialDocument(TrialDocumentResult {
            file: Section::Empty {
                source: Source::ClinicalTrialsGov,
                record: None,
            },
        }),
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    };
    let result = ctgov::detail::get(&leg, query.input).and_then(|(detail, _)| {
        let Some(document) = detail
            .documents
            .into_vec()
            .into_iter()
            .find(|d| d.filename == query.file)
        else {
            out.body = QueryBody::TextPositionError;
            return Ok(None);
        };
        if !allow {
            out.body = QueryBody::ArticleSaveRefused(document.url);
            return Ok(None);
        }
        ctgov::document::download(&leg, &document).map(|bytes| Some((document, bytes)))
    });
    let error = result.as_ref().err().copied();
    if let Some(error) = error {
        out.status = Status::Failed;
        out.local = status::local(error);
        out.body = QueryBody::TrialDocument(TrialDocumentResult {
            file: Section::unavailable(Source::ClinicalTrialsGov, crate::status::reason(error)),
        });
    }
    let remaining = leg.remaining();
    out.sources.push(status::source(
        Source::ClinicalTrialsGov,
        leg.finish(),
        error,
    ));
    command.finish(remaining);
    Download {
        out,
        file: result.ok().flatten(),
    }
}
