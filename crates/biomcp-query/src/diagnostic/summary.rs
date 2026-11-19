//! Source-owned FDA device search and document access.
use super::{empty, next};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{fda_pma, pdf};
use biomcp_types::{
    DiagnosticGet, DiagnosticSummary, PageInfo, PdfText, PdfTextHit, ReplyBody, Request, StepKind,
};
/// Escaped bytes of one found passage, half a fragment: enough to reach a numbered table set
/// below the heading that matched (P150047's Table 15 ends 1,741 bytes after its heading).
const PASSAGE_BYTES: usize = 3072;
/// Escaped bytes of text kept before a match.
const LEAD_BYTES: usize = 512;
/// Escaped bytes of one find page, the article text find page size.
const FIND_BYTES: usize = 12_000;
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
    let error = match result {
        Ok((document, Some((find, text)))) => {
            status::pdf_item(&mut out, q.source().source(), &document);
            let body = found(cx, q, document, find.as_str(), &text, &mut out);
            out.body = body;
            None
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
            None
        }
        Err(error) => Some(error),
    };
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
    if error.is_some() {
        status::item(&mut out, false, true, false);
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
    let page = crate::text_find::page(&all, q.section_offset(), FIND_BYTES, |hit| {
        Some(fda_pma::escaped_bytes(&hit.text) + 32)
    });
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
/// Each match starts a passage a short lead before it, running through the following text up to
/// the passage size and ending at a line break after the match when one fits; a match inside an
/// earlier passage adds none. Fragment numbers are those of continuable fragment reading.
pub(super) fn passages(text: &str, needles: &[String]) -> Vec<PdfTextHit> {
    let starts = pdf::fragment_starts(text);
    let mut hits = Vec::new();
    let mut covered = 0;
    while let Some(at) = next_match(text, covered, needles) {
        let begin = lead(text, at).max(covered);
        let end = passage_end(text, begin, at);
        // Search on after the passage, and always past the match itself.
        let past = text
            .get(at..)
            .and_then(|rest| rest.chars().next())
            .map_or(text.len(), |c| at + c.len_utf8());
        covered = end.max(past);
        hits.push(PdfTextHit {
            fragment: starts.partition_point(|s| *s <= begin).saturating_sub(1) as u32,
            text: text[begin..end].trim().into(),
        });
    }
    hits
}
/// The first match at or after `from`, which may be mid-line after an earlier passage.
fn next_match(text: &str, from: usize, needles: &[String]) -> Option<usize> {
    let mut here = from;
    let mut lines = text.get(from..)?.split_inclusive('\n').peekable();
    while let Some(line) = lines.next() {
        let found = first_match(line, needles).or_else(|| {
            // A phrase may wrap onto the next extracted line; it then starts near this line's end.
            let next = lines.peek()?;
            let kept = line.trim_end();
            let joined = format!("{kept} {}", next.trim_start());
            first_match(&joined, needles).filter(|at| *at < kept.len())
        });
        if let Some(at) = found {
            return Some(here + at);
        }
        here += line.len();
    }
    None
}
/// Byte position of the first case-insensitive match in the original spelling.
fn first_match(line: &str, needles: &[String]) -> Option<usize> {
    let lower = line.to_lowercase();
    let start = needles
        .iter()
        .filter_map(|needle| lower.find(needle.as_str()))
        .min()?;
    let mut lower_at = 0;
    line.char_indices().find_map(|(byte, c)| {
        let width: usize = c.to_lowercase().map(char::len_utf8).sum();
        let found = (start < lower_at + width).then_some(byte);
        lower_at += width;
        found
    })
}
/// Start up to the lead before the match, after a line break or space when one is there.
fn lead(text: &str, at: usize) -> usize {
    let mut begin = at.saturating_sub(LEAD_BYTES);
    while !text.is_char_boundary(begin) {
        begin += 1;
    }
    text[begin..at]
        .rfind('\n')
        .or_else(|| text[begin..at].find(' '))
        .map_or(begin, |gap| begin + gap + 1)
}
/// End within the passage size, at the last line break after the match when there is one.
fn passage_end(text: &str, begin: usize, at: usize) -> usize {
    let mut size = 0;
    let mut end = begin;
    let mut line_end = None;
    for (offset, c) in text[begin..].char_indices() {
        size += fda_pma::escaped_bytes(c.encode_utf8(&mut [0; 4]));
        if size > PASSAGE_BYTES {
            break;
        }
        end = begin + offset + c.len_utf8();
        if c == '\n' && end > at {
            line_end = Some(end);
        }
    }
    if end < text.len() {
        line_end.unwrap_or(end)
    } else {
        end
    }
}
