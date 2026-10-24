//! Explicit PDF download orchestration; the transport owns atomic file I/O.
use crate::{Ctx, Outcome, status};
use super::sections::pdf::{bad, filename, load, reason};
use biodata::{ArticlePdf, ArticlePdfResult, PdfSaved, QueryBody, Reason, Section, Source, Status, Text};
use biomcp_adapters::{pdf, semantic_scholar};
use biomcp_fetch::{Deadline, RequestError, SaveTarget};
/// Download only when the caller supplied a path and the interface permits local writes.
pub fn run(cx: &Ctx<'_>, query: &ArticlePdf, allow_save: bool) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    let mut out = Outcome { status: Status::Complete, body: QueryBody::ArticlePdf(ArticlePdfResult {file: Section::NotApplicable}), page: None, sources: Vec::new(), problems: Vec::new(), next: Vec::new(), local: None };
    if let Err(error) = execute(cx, query, allow_save, &mut deadline, &mut out) {
        out.local = out.local.or(status::local(error));
        out.status = Status::Failed;
        out.body = QueryBody::ArticlePdf(ArticlePdfResult {file: Section::unavailable(Source::SemanticScholar, reason(error))});
    }
    command.finish(deadline.remaining());
    out
}
fn execute<'a>(cx: &Ctx<'a>, query: &ArticlePdf, allow_save: bool, deadline: &mut Deadline<'a>, out: &mut Outcome) -> Result<(), RequestError> {
    let section = load(cx, &query.input, deadline, out)?;
    let Section::Data {data: link, ..} = section else {
        out.status = Status::Failed;
        out.body = QueryBody::ArticlePdf(ArticlePdfResult {file: Section::unavailable(Source::SemanticScholar, Reason::NotOpen)});
        return Ok(());
    };
    let target = if allow_save { SaveTarget::new(query.save.as_str(), &filename(&query.input), query.replace).ok() } else { None };
    let Some(target) = target else {out.body = QueryBody::PdfSaveRefused(link); return Ok(());};
    let leg = cx.fetch.leg(&semantic_scholar::PUBLISHER, *deadline).fresh(true);
    let started = leg.remaining();
    let result = pdf::download(&leg, link.url.as_str()).map_err(|error| if matches!(error, RequestError::Input(_)) {bad(Reason::Rejected)} else {error});
    *deadline = super::consume(*deadline, started, leg.remaining());
    let source = status::source(Source::SemanticScholar, leg.finish(), result.as_ref().err().copied());
    if let Some(existing) = out.sources.iter_mut().find(|s| s.source == Source::SemanticScholar) {status::retain_source_state(existing, &source);} else {out.sources.push(source);}
    let bytes = result?;
    if !bytes.starts_with(b"%PDF-") {return Err(bad(Reason::NotPdf));}
    let saved = match target.write(&bytes) {Ok(saved) => saved, Err(_) => {out.body = QueryBody::PdfSaveRefused(link); return Ok(());}};
    let data = PdfSaved {
        path: Text::new(saved.path.to_string_lossy()).map_err(|_| bad(Reason::BadReply))?,
        bytes: saved.bytes,
        sha256: Text::new(saved.sha256).map_err(|_| bad(Reason::BadReply))?,
        pdf: link,
    };
    out.body = QueryBody::ArticlePdf(ArticlePdfResult {file: Section::Data {source: Source::SemanticScholar, data}});
    Ok(())
}
