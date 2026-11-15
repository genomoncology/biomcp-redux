//! Explicit PDF download orchestration; the binary owns atomic file I/O.
use super::sections::pdf::{load, pdf_error};
use crate::{Ctx, Outcome, status};
use biomcp_types::ReplyBody;

use biomcp_adapters::{pdf, semantic_scholar};
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    ArticleGet, ArticleInput, ArticlePdf, ArticlePdfResult, ArticleSectionWord, PdfLink, Reason,
    ReplyData, Request, Section, Source, SourceNotice, StepKind,
};
/// Download only when the caller supplied a path and the interface permits local writes.
pub fn download(cx: &Ctx<'_>, query: &ArticlePdf, allow_save: bool) -> Download {
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    let mut out = crate::status::ready(
        (ArticlePdfResult {
            file: Section::NotApplicable,
        })
        .with_query(query.clone()),
    );
    let mut file = None;
    if let Err(error) = execute(cx, query, allow_save, &mut deadline, &mut out, &mut file) {
        out.local = out.local.or(status::local(error));
        crate::status::item(&mut out, false, true, false);
        out.body = (ArticlePdfResult {
            file: Section::unavailable(
                Source::SemanticScholar,
                crate::status::reason(error).unwrap_or(biomcp_types::Reason::Unavailable),
            ),
        })
        .with_query(query.clone());
    }
    command.finish(deadline.remaining());
    Download { out, file }
}
fn execute<'a>(
    cx: &Ctx<'a>,
    query: &ArticlePdf,
    allow_save: bool,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
    file: &mut Option<(PdfLink, Vec<u8>)>,
) -> Result<(), RequestError> {
    if let ArticleInput::EuropePmc(id) = &query.input {
        europepmc_refused(query, id, out);
        return Ok(());
    }
    let section = load(cx, &query.input, deadline, out)?;
    let Section::Data { data: link, .. } = section else {
        crate::status::item(out, false, true, false);
        out.body = (ArticlePdfResult {
            file: Section::unavailable(Source::SemanticScholar, Reason::NotOpen),
        })
        .with_query(query.clone());
        return Ok(());
    };
    if !allow_save {
        out.body = ReplyData::Usage(biomcp_types::UsageError::PdfSaveRefused(link));
        return Ok(());
    }
    let leg = cx
        .fetch
        .leg(&semantic_scholar::PUBLISHER, *deadline)
        .fresh(true);
    let started = leg.remaining();
    let result = pdf::download(&leg, link.url.as_str()).map_err(|error| {
        if matches!(error, RequestError::Input(_)) {
            pdf_error(Reason::Rejected)
        } else {
            error
        }
    });
    *deadline = super::consume(*deadline, started, leg.remaining());
    let mut source = status::source(
        Source::SemanticScholar,
        leg.finish(),
        result.as_ref().err().copied(),
    );
    if let Err(error) = &result
        && let Some(refused) = pdf::refused(error)
    {
        source
            .notices
            .push(refusal(&link.url, refused, open_text(&link.url, &query.input), out));
    }
    if let Some(existing) = out
        .sources
        .iter_mut()
        .find(|s| s.source == Source::SemanticScholar)
    {
        status::retain_source_state(existing, &source);
    } else {
        out.sources.push(source);
    }
    let bytes = result?;
    if !bytes.starts_with(b"%PDF-") {
        return Err(pdf_error(Reason::NotPdf));
    }
    *file = Some((link, bytes));
    Ok(())
}

/// The open full-text identity for a refused PDF: the PMC or preprint the link
/// names, else the caller's PubMed, PMC or Europe PMC identity.
fn open_text(url: &biodata::Text, input: &ArticleInput) -> Option<ArticleInput> {
    pdf::publisher::named_text(url.as_str()).or_else(|| match input {
        ArticleInput::Doi(_) => None,
        input => Some(input.clone()),
    })
}

/// Say who refused the PDF and why, and point to the open full text when one is named.
fn refusal(
    url: &biodata::Text,
    (status, meaning): (u16, &str),
    open: Option<ArticleInput>,
    out: &mut Outcome,
) -> SourceNotice {
    let url = url.as_str();
    let europepmc = pdf::publisher::europepmc(url);
    let host = if europepmc {
        "Europe PMC".to_owned()
    } else {
        pdf::publisher::host(url).unwrap_or_else(|| "The publisher".to_owned())
    };
    let why = if europepmc {
        "Europe PMC does not give PDFs to automated clients such as BioMCP, and BioMCP does \
         not get around that."
    } else {
        "This does not show that the paper is closed."
    };
    let next = if open.is_some_and(|input| fulltext_next(input, out)) {
        "Read its open full text with the next command, or open the URL in a browser."
    } else {
        "Open the URL in a browser."
    };
    SourceNotice::RequestGuidance {
        data_date: None,
        message: format!(
            "{host} refused the PDF at {url} with HTTP {status}{meaning}, so no file was \
             saved. {why} {next}"
        )
        .into(),
        retry_after_seconds: None,
    }
}

/// Offer `get article ID fulltext`, the open text reader for PMC and Europe PMC.
fn fulltext_next(input: ArticleInput, out: &mut Outcome) -> bool {
    let Ok(query) = ArticleGet::with_sections([input].into(), [ArticleSectionWord::Fulltext].into())
    else {
        return false;
    };
    out.next.push(crate::next::Step::run(
        StepKind::Section,
        &super::next::text_sources(&query.input()[0]),
        Request::GetArticle(query),
        false,
    ));
    true
}

/// Europe PMC refuses automated PDF requests (see `pdf::publisher::europepmc`), so a
/// preprint ID gets a plain refusal and its open full text without a request.
fn europepmc_refused(query: &ArticlePdf, id: &biomcp_types::EuropePmcId, out: &mut Outcome) {
    let mut source = biomcp_adapters::registry::outcome(
        Source::EuropePmc,
        biomcp_types::SourceStatus::Skipped(Some(Reason::Rejected)),
    );
    fulltext_next(query.input.clone(), out);
    source.notices.push(SourceNotice::RequestGuidance {
        data_date: None,
        message: format!(
            "Europe PMC does not give PDFs to automated clients such as BioMCP: its PDF \
             links answer HTTP 403 Forbidden, and BioMCP does not get around that, so no \
             file was saved. Europe PMC's full text for {id}, where open, is readable with \
             the next command; https://europepmc.org/article/PPR/{id} opens in a browser."
        )
        .into(),
        retry_after_seconds: None,
    });
    out.sources.push(source);
    crate::status::item(out, false, true, false);
    out.body = (ArticlePdfResult {
        file: Section::unavailable(Source::EuropePmc, Reason::Rejected),
    })
    .with_query(query.clone());
}

/// Bounded download before the binary decides how to install the file.
#[derive(Debug)]
pub struct Download {
    /// Typed source outcome.
    pub out: Outcome,
    /// Valid PDF bytes and source metadata; never cached or parsed.
    pub file: Option<(PdfLink, Vec<u8>)>,
}
