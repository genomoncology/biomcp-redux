//! Bounded trial PDF reply parts, literal matches and version information.
use crate::{Outcome, pdf_find, status, text_find};
use biodata::TrialDocument;
use biomcp_adapters::pdf::{self, DownloadedText, Embedded};
use biomcp_types::{
    PdfText, PdfTextState, ReplyBody, Request, Section, Source, SourceNotice, StepKind,
    TrialDocumentGet, TrialDocumentHit, TrialDocumentReadResult, TrialDocumentText,
};
use std::num::NonZeroU32;

const SOURCE: Source = Source::ClinicalTrialsGov;

pub(super) fn answer(
    out: &mut Outcome,
    query: &TrialDocumentGet,
    document: TrialDocument,
    result: DownloadedText,
    fresh: bool,
) {
    let DownloadedText {
        status: mut text,
        read,
        can_save,
    } = result;
    let first = query.page_start.unwrap_or(NonZeroU32::MIN).get();
    let offset = query.section_offset.unwrap_or_default();
    let mut version = None;
    let mut hits = query.find.is_some().then(Box::default);
    let mut following = None;
    if let Some(read) = read {
        let cover_end = read.page_starts.get(1).copied().unwrap_or(read.text.len());
        if first == 1 {
            version = read.text.get(..cover_end).and_then(cover_version);
        }
        if let Some(find) = &query.find {
            let all = passages(&read, find.as_str(), first);
            if let Some(page) = text_find::page(&all, offset, pdf_find::FIND_BYTES, |hit| {
                Some(pdf::escaped_bytes(&hit.text) + 32)
            }) {
                hits = Some(page.hits.into());
                following = page.following;
                out.page = Some(page.info);
            } else {
                out.page = Some(status::page_info(offset, 0, all.len() as u64));
            }
        } else {
            pdf::part(&mut text, &read.text, offset);
            let returned = u32::from(text.text.is_some());
            let next = offset + returned;
            following = (returned != 0 && next < text.total).then_some(next);
            out.page = Some(status::page_info(offset, returned, text.total.into()));
        }
    }
    let next_window = window(out, query, &text, first);
    if let Some(next) = following {
        let mut next_query = query.clone();
        next_query.section_offset = Some(next);
        step(out, StepKind::Page, next_query, fresh);
    } else if let Some(next) = next_window {
        step(out, StepKind::Page, next, fresh);
    } else if !text.state.readable() {
        recovery(out, query, &text, can_save, fresh);
    }
    status::pdf_item(out, SOURCE, &text);
    out.body = TrialDocumentReadResult {
        content: Section::Data {
            source: SOURCE,
            data: TrialDocumentText {
                document,
                version,
                text,
                hits,
            },
        },
    }
    .with_query(query.clone());
}

fn window(
    out: &mut Outcome,
    query: &TrialDocumentGet,
    text: &PdfText,
    first: u32,
) -> Option<TrialDocumentGet> {
    let pages = text.pages?;
    let end = first.saturating_add(pdf::MAX_PAGES as u32 - 1).min(pages);
    if (first > 1 || end < pages)
        && first <= end
        && (text.state.readable() || text.state == PdfTextState::NoText)
    {
        notice(
            out,
            format!("This call covers PDF pages {first}–{end} of {pages}."),
        );
        if end < pages {
            let mut next = query.clone();
            next.page_start = NonZeroU32::new(end + 1);
            next.section_offset = None;
            return Some(next);
        }
    }
    None
}

fn recovery(
    out: &mut Outcome,
    query: &TrialDocumentGet,
    text: &PdfText,
    can_save: bool,
    fresh: bool,
) {
    if let (Some(first), Some(pages)) = (query.page_start, text.pages)
        && first.get() > pages
    {
        notice(
            out,
            format!("Page {first} is out of range for this {pages}-page PDF."),
        );
        let mut restart = query.clone();
        restart.page_start = None;
        restart.section_offset = None;
        step(out, StepKind::Configure, restart, fresh);
        return;
    }
    let reason = match text.state {
        PdfTextState::TooLarge if !can_save => {
            "The PDF exceeds the download limit. Open the source URL."
        }
        PdfTextState::TooLarge if text.pages.is_some() => {
            "The selected pages exceed the expanded-content limit."
        }
        PdfTextState::TooLarge => {
            "The PDF exceeds the structure limits, so its page count could not be read."
        }
        PdfTextState::NoText => "No embedded text was found in the selected pages.",
        PdfTextState::Garbled => "The embedded text could not be decoded.",
        _ => "",
    };
    if !reason.is_empty() {
        notice(out, reason.into());
    }
    if can_save {
        let mut save = query.clone();
        save.find = None;
        save.page_start = None;
        save.section_offset = None;
        save.save = Some(query.file.clone());
        let file = query.file.as_str();
        let id = query.input;
        notice(
            out,
            format!(
                "Save the original PDF in the CLI with `biomcp trial document --save={file} -- {id} {file}`."
            ),
        );
        step(out, StepKind::Configure, save, fresh);
    }
}

fn notice(out: &mut Outcome, message: String) {
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|source| source.source == SOURCE)
    {
        source.notices.push(SourceNotice::RequestGuidance {
            data_date: None,
            message: message.into(),
            retry_after_seconds: None,
        });
    }
}

fn step(out: &mut Outcome, kind: StepKind, query: TrialDocumentGet, fresh: bool) {
    out.next.push(crate::next::Step::run(
        kind,
        &[SOURCE],
        Request::TrialDocument(query),
        fresh,
    ));
}

fn passages(read: &Embedded, find: &str, first: u32) -> Vec<TrialDocumentHit> {
    let needles = [find.to_lowercase()];
    let ends = read
        .page_starts
        .iter()
        .copied()
        .skip(1)
        .chain([read.text.len()]);
    read.page_starts
        .iter()
        .copied()
        .zip(ends)
        .enumerate()
        .flat_map(|(index, (start, end))| {
            let page = read.text.get(start..end).unwrap_or_default();
            pdf_find::passages(page, &needles).map(move |passage| TrialDocumentHit {
                page: first + index as u32,
                text: passage.text.into(),
            })
        })
        .collect()
}

/// The registry manifest has a date but no version field. Use an explicit version on the
/// posted cover and never treat the registry's filename sequence as a revision number.
fn cover_version(cover: &str) -> Option<Box<str>> {
    cover.lines().find_map(|line| {
        let lower = line.trim().to_ascii_lowercase();
        let value = lower
            .strip_prefix("version")
            .or_else(|| lower.strip_prefix("document version"))
            .or_else(|| {
                let title = [
                    "statistical analysis plan",
                    "sap",
                    "study protocol",
                    "protocol",
                ]
                .iter()
                .any(|title| lower.contains(title));
                (title || lower.starts_with("document date"))
                    .then(|| {
                        lower
                            .split_once(" version ")
                            .or_else(|| lower.split_once(" v"))
                    })
                    .flatten()
                    .map(|(_, value)| value)
            })?;
        let value = value.trim_start_matches([' ', ':']);
        let value = value.strip_prefix("number").unwrap_or(value);
        let value = value.trim_start_matches([' ', ':']);
        let value = value.strip_prefix('v').unwrap_or(value);
        let version = value
            .split(|c: char| !c.is_ascii_digit() && c != '.')
            .next()?
            .trim_end_matches('.');
        (!version.is_empty() && version.split('.').all(|part| !part.is_empty()))
            .then(|| version.into())
    })
}
