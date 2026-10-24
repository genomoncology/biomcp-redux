//! Explicit PDF outline and shared extracted-text loading.
use super::pdf_parts::Document;
use crate::{Action, Ctx, Outcome, status, store::Store};
use biodata::{
    ArticleInput, ArticleResolution, OpenAccessStatus, PdfText, QueryBody, Reason, Request,
    Section, Source, SourceNotice, Status, StepKind, Text, TextOrigin,
};
use biomcp_adapters::{pdf, semantic_scholar};
use biomcp_fetch::{Deadline, RequestError, SourceError};

pub(super) fn run<'a>(cx: &Ctx<'a>, deadline: &mut Deadline<'a>, out: &mut Outcome) {
    let QueryBody::ArticleCards(cards) = &out.body else {
        return;
    };
    let inputs: Vec<_> = cards
        .items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let Some(ArticleResolution::Resolved { card, .. }) = &item.answer else {
                return None;
            };
            let input = card
                .pmid
                .map(ArticleInput::Pmid)
                .or_else(|| card.doi.clone().map(ArticleInput::Doi))?;
            Some((index, input))
        })
        .collect();
    for (index, input) in inputs {
        let result = load(cx, &input, deadline, out);
        let section = match result {
            Ok(section) => map(section, |document| document.outline),
            Err(error) => {
                out.local = out.local.or(status::local(error));
                Section::unavailable(Source::SemanticScholar, reason(error))
            }
        };
        if matches!(section, Section::Data { .. }) && out.next.len() < 5 {
            if let Ok(text) = biodata::ArticleText::part(input, 1) {
                out.next.push((
                    StepKind::Section,
                    Action::Run {
                        request: Request::ArticleText(text.with_pdf()),
                        fresh: false,
                    },
                ));
            }
        }
        let QueryBody::ArticleCards(cards) = &mut out.body else {
            return;
        };
        if let Some(item) = cards.items.get_mut(index) {
            if matches!(
                section,
                Section::Unavailable { .. } | Section::Skipped { .. }
            ) {
                item.status = Status::Partial;
            }
            if let Some(sections) = &mut item.sections {
                sections.pdf = Some(section);
            }
        }
    }
    if let QueryBody::ArticleCards(cards) = &out.body {
        out.status = status::status(cards.items.iter().map(|item| item.status));
    }
}
pub(in crate::article) fn load<'a>(
    cx: &Ctx<'a>,
    input: &ArticleInput,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<Section<Document>, RequestError> {
    if cx.fetch.disabled(Source::SemanticScholar) {
        return Ok(Section::unavailable(
            Source::SemanticScholar,
            Reason::Disabled,
        ));
    }
    let article = serde_json::to_string(input).map_err(|_| bad(Reason::BadReply))?;
    let mut store = Store::open(cx.fetch.cache_access()?, *deadline)?;
    if !cx.fresh {
        if let Some(encoded) = store.pdf_get(&article)? {
            let document: Document =
                serde_json::from_str(&encoded).map_err(|_| bad(Reason::BadReply))?;
            notice(out);
            return Ok(Section::Data {
                source: Source::SemanticScholar,
                data: document,
            });
        }
    }
    let leg = cx
        .fetch
        .leg(&semantic_scholar::SPEC, *deadline)
        .fresh(cx.fresh);
    let started = leg.remaining();
    let result = semantic_scholar::pdf(&leg, input);
    *deadline = super::super::consume(*deadline, started, leg.remaining());
    out.sources.push(status::source(
        Source::SemanticScholar,
        leg.finish(),
        result.as_ref().err().copied(),
    ));
    notice(out);
    let Some(link) = result?
        .open_access_pdf
        .filter(|link| !link.url.is_empty() && link.status != OpenAccessStatus::Closed)
    else {
        return Ok(Section::unavailable(
            Source::SemanticScholar,
            Reason::NotOpen,
        ));
    };
    let leg = cx.fetch.leg(&pdf::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let result = download(&leg, &link);
    *deadline = super::super::consume(*deadline, started, leg.remaining());
    let source = status::source(
        Source::SemanticScholar,
        leg.finish(),
        result.as_ref().err().copied(),
    );
    if let Some(existing) = out
        .sources
        .iter_mut()
        .find(|s| s.source == Source::SemanticScholar)
    {
        status::retain_source_state(existing, &source);
    } else {
        out.sources.push(source);
    }
    let section = result?;
    if let Section::Data { data, .. } = &section {
        store.set_deadline(*deadline);
        let encoded = serde_json::to_string(data).map_err(|_| bad(Reason::BadReply))?;
        store.pdf_put(&article, data.url(), &encoded)?;
    }
    Ok(section)
}
fn download(
    leg: &biomcp_fetch::Leg<'_>,
    link: &semantic_scholar::PdfLink,
) -> Result<Section<Document>, RequestError> {
    let bytes = pdf::download(leg, &link.url).map_err(|error| {
        if matches!(error, RequestError::Input(_)) {
            bad(Reason::Rejected)
        } else {
            error
        }
    })?;
    if !bytes.starts_with(b"%PDF-") {
        return Ok(Section::unavailable(
            Source::SemanticScholar,
            Reason::NotPdf,
        ));
    }
    let extracted: pdf::Extraction =
        serde_json::from_slice(&leg.pdf_worker(&bytes)?).map_err(|_| bad(Reason::BadReply))?;
    let section = match extracted {
        pdf::Extraction::Encrypted => {
            Section::unavailable(Source::SemanticScholar, Reason::Unsupported)
        }
        pdf::Extraction::Empty => Section::Empty {
            source: Source::SemanticScholar,
            record: None,
        },
        pdf::Extraction::BadReply => {
            Section::unavailable(Source::SemanticScholar, Reason::BadReply)
        }
        pdf::Extraction::Data {
            pages,
            text,
            truncated,
        } => {
            let outline = PdfText {
                origin: TextOrigin::Pdf,
                url: Text::new(&link.url).map_err(|_| bad(Reason::BadReply))?,
                status: link.status,
                license: link.license(),
                pages,
                truncated,
                parts: Box::default(),
            };
            Section::Data {
                source: Source::SemanticScholar,
                data: Document::new(outline, text).ok_or_else(|| bad(Reason::BadReply))?,
            }
        }
    };
    Ok(section)
}
pub(in crate::article) fn reason(error: RequestError) -> Reason {
    match error {
        RequestError::Source(error) => error.reason,
        _ => Reason::Unavailable,
    }
}
fn bad(reason: Reason) -> RequestError {
    SourceError {
        source: Source::SemanticScholar,
        reason,
        status: None,
        retry_after: None,
    }
    .into()
}
fn notice(out: &mut Outcome) {
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|s| s.source == Source::SemanticScholar)
    {
        if !source.notices.contains(&SourceNotice::PdfText) {
            source.notices.push(SourceNotice::PdfText);
        }
    } else {
        let mut source = status::source(Source::SemanticScholar, Default::default(), None);
        source.notices.push(SourceNotice::PdfText);
        out.sources.push(source);
    }
}
fn map<T, U>(section: Section<T>, f: impl FnOnce(T) -> U) -> Section<U> {
    match section {
        Section::Data { source, data } => Section::Data {
            source,
            data: f(data),
        },
        Section::Empty { source, record } => Section::Empty { source, record },
        Section::Unavailable { source, reason } => Section::Unavailable { source, reason },
        Section::Skipped { source, reason } => Section::Skipped { source, reason },
        Section::NotApplicable => Section::NotApplicable,
    }
}

pub(in crate::article) fn read<'a>(
    cx: &Ctx<'a>,
    query: &biodata::ArticleText,
    selection: &crate::article::protein_change::Selection,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let section = load(cx, query.input(), deadline, out)?;
    let content = match section {
        Section::Data { source, data } => {
            let Some(content) = super::pdf_text::select(&data, query, selection, out) else {
                out.body = QueryBody::TextPositionError;
                return Ok(());
            };
            Section::Data {
                source,
                data: content,
            }
        }
        Section::Empty { source, record } => Section::Empty { source, record },
        Section::Unavailable { source, reason } => Section::Unavailable { source, reason },
        Section::Skipped { source, reason } => Section::Skipped { source, reason },
        Section::NotApplicable => Section::NotApplicable,
    };
    out.body = QueryBody::ArticleText(biodata::ArticleTextResult {
        pmcid: None,
        content,
    });
    Ok(())
}
