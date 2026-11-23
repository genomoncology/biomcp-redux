//! Source-owned FDA device search and document access.
use super::{empty, next};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{fda_pma, pdf};
use biomcp_types::{
    DiagnosticGet, DiagnosticSummary, PageInfo, PdfText, PdfTextHit, ReplyBody, Request, StepKind,
};
pub(super) fn get(cx: &Ctx<'_>, q: &DiagnosticGet) -> Outcome {
    let mut out = empty(DiagnosticSummary::into_reply(q.clone(), None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&fda_pma::DOCUMENTS, deadline).fresh(cx.fresh);
    // FDA's legacy archive uses pdf/ (P010013, P970043); later folders are unpadded (P020014 -> pdf2/).
    // See the Summary links on those FDA PMA records; supplements have independent documents.
    let id = q.id().as_str();
    let year = id.get(1..3).unwrap_or_default();
    let directory = if ("02"..="69").contains(&year) {
        format!("pdf{}", year.trim_start_matches('0'))
    } else {
        "pdf".into()
    };
    let url: Result<biodata::Text, _> =
        format!("https://www.accessdata.fda.gov/cdrh_docs/{directory}/{id}B.pdf").parse();
    let result =
        url.as_ref()
            .map_err(|_| biomcp_fetch::bad_reply(q.source().source(), None))
            .and_then(|url| match q.find() {
                None => {
                    pdf::read(&leg, &fda_pma::DOCUMENTS, url, q.section_offset()).map(|d| (d, None))
                }
                Some(find) => pdf::whole(&leg, &fda_pma::DOCUMENTS, url)
                    .map(|(d, text)| (d, Some((find, text)))),
            });
    let error = result.as_ref().err().copied();
    out.local = error.and_then(status::local);
    let mut source = status::source(q.source().source(), leg.finish(), error);
    if let (Some(error), Ok(url)) = (error, url.as_ref())
        && let Some(notice) = pdf::refusal(url, &error)
    {
        source.notices.push(notice);
        next::push(
            &mut out,
            StepKind::Fresh,
            &[q.source().source()],
            Request::GetDiagnostic(q.clone()),
            true,
        );
    }
    out.sources.push(source);
    match result {
        Ok((document, Some((find, text)))) => {
            status::pdf_item(&mut out, q.source().source(), &document);
            let body = found(cx, q, document, find.as_str(), &text, &mut out);
            out.body = body;
        }
        Ok((document, None)) => {
            let returned = u32::from(document.text.is_some());
            status::pdf_item(&mut out, q.source().source(), &document);
            out.page = Some(PageInfo {
                offset: q.section_offset(),
                returned,
                total: document.total.into(),
                pool: None,
                truncated: None,
            });
            if returned > 0 && q.section_offset() + returned < document.total {
                next::push(
                    &mut out,
                    StepKind::Page,
                    &[q.source().source()],
                    Request::GetDiagnostic(q.at_section_offset(q.section_offset() + returned)),
                    cx.fresh,
                );
            }
            out.body = DiagnosticSummary {
                document,
                hits: None,
            }
            .with_query(q.clone());
        }
        Err(_) => status::item(&mut out, false, true, false),
    }
    command.finish(deadline.remaining());
    out
}

/// Answer a find with whole passages paged like article text's find.
fn found(
    cx: &Ctx<'_>,
    q: &DiagnosticGet,
    document: PdfText,
    find: &str,
    text: &str,
    out: &mut Outcome,
) -> biomcp_types::ReplyData {
    let all = passages(text, &[find.to_lowercase()]);
    let page = crate::text_find::page(
        &all,
        q.section_offset(),
        crate::pdf_find::FIND_BYTES,
        |hit| Some(fda_pma::escaped_bytes(&hit.text) + 32),
    );
    let (hits, info) = match page {
        Some(page) => {
            if let Some(following) = page.following {
                next::push(
                    out,
                    StepKind::Page,
                    &[q.source().source()],
                    Request::GetDiagnostic(q.at_section_offset(following)),
                    cx.fresh,
                );
            }
            (page.hits.to_vec(), page.info)
        }
        None => (
            Vec::new(),
            PageInfo {
                offset: q.section_offset(),
                returned: 0,
                total: all.len() as u64,
                pool: None,
                truncated: None,
            },
        ),
    };
    out.page = Some(info);
    DiagnosticSummary {
        document,
        hits: Some(hits.into()),
    }
    .with_query(q.clone())
}
/// Map shared PDF passages to the diagnostic reader's continuable fragments.
pub(super) fn passages(text: &str, needles: &[String]) -> Vec<PdfTextHit> {
    let starts = pdf::fragment_starts(text);
    crate::pdf_find::passages(text, needles)
        .map(|passage| PdfTextHit {
            fragment: starts
                .partition_point(|s| *s <= passage.start)
                .saturating_sub(1) as u32,
            text: passage.text.into(),
        })
        .collect()
}
