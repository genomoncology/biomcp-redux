//! Bounded trial PDF reply parts, literal matches and version information.
use crate::{Outcome, pdf_find, status, text_find};
use biomcp_adapters::pdf::{self, Embedded};
use biomcp_types::{
    PdfText, ReplyBody, Request, Section, Source, StepKind, TrialDocument, TrialDocumentGet,
    TrialDocumentHit, TrialDocumentResult, TrialDocumentText,
};

pub(super) fn answer(
    out: &mut Outcome,
    query: &TrialDocumentGet,
    document: TrialDocument,
    mut text: PdfText,
    read: Option<Embedded>,
    fresh: bool,
) {
    let offset = query.section_offset.unwrap_or_default();
    let mut version = None;
    let mut hits = query.find.is_some().then(Box::default);
    let mut following = None;
    if let Some(read) = read {
        let cover_end = read.page_starts.get(1).copied().unwrap_or(read.text.len());
        version = read.text.get(..cover_end).and_then(cover_version);
        if let Some(find) = &query.find {
            let all = passages(&read, find.as_str());
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
    if let Some(next) = following {
        let mut next_query = query.clone();
        next_query.section_offset = Some(next);
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[Source::ClinicalTrialsGov],
            Request::TrialDocument(next_query),
            fresh,
        ));
    }
    status::pdf_item(out, Source::ClinicalTrialsGov, &text);
    out.body = TrialDocumentResult {
        content: Some(Section::Data {
            source: Source::ClinicalTrialsGov,
            data: TrialDocumentText {
                document,
                version,
                text,
                hits,
            },
        }),
        file: Section::NotApplicable,
    }
    .with_query(query.clone());
}

fn passages(read: &Embedded, find: &str) -> Vec<TrialDocumentHit> {
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
                page: (index + 1) as u32,
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
