//! Select one native application document, without guessing a submission or product.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{drugsfda, pdf};
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{DrugFdaSubmission, DrugGet, PageInfo, PdfTextState, Request, Source, StepKind};
pub(super) fn select(
    cx: &Ctx<'_>,
    query: &DrugGet,
    row: &drugsfda::Application,
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<Option<Box<[DrugFdaSubmission]>>, RequestError> {
    let Some(id) = query.document_id() else {
        return Ok(None);
    };
    let mut selected = Vec::new();
    for mut submission in row.submissions(0, u32::MAX)? {
        let Some(docs) = submission.application_docs.take() else {
            continue;
        };
        for document in docs.into_vec().into_iter().filter(|d| &d.id == id) {
            let mut submission = submission.clone();
            submission.application_docs = Some(vec![document].into());
            selected.push(submission);
        }
    }
    if selected.len() != 1 {
        return Err(biomcp_fetch::bad_reply(Source::Drugsfda, Some(200)));
    }
    let document = selected
        .first_mut()
        .and_then(|s| s.application_docs.as_mut())
        .and_then(|docs| docs.first_mut())
        .ok_or_else(|| biomcp_fetch::bad_reply(Source::Drugsfda, Some(200)))?;
    if document
        .r#type
        .as_ref()
        .is_none_or(|kind| kind.as_str() != "Letter")
    {
        return Err(biomcp_fetch::bad_reply(Source::Drugsfda, Some(200)));
    }
    let url = document
        .url
        .as_ref()
        .ok_or_else(|| biomcp_fetch::bad_reply(Source::Drugsfda, Some(200)))?;
    let leg = cx.fetch.leg(&drugsfda::DOCUMENTS, deadline).fresh(cx.fresh);
    let offset = query.section_offset().unwrap_or(0);
    let result = pdf::read(&leg, &drugsfda::DOCUMENTS, url, offset);
    let error = result.as_ref().err().copied();
    let mut report = status::source(Source::Drugsfda, leg.finish(), error);
    refused(query, url, error, &mut report, out);
    // Both requests belong to Drugs@FDA; keep one source outcome and the document failure.
    if error.is_some()
        && let Some(source) = out
            .sources
            .iter_mut()
            .find(|source| source.source == Source::Drugsfda)
    {
        *source = report;
    }
    let text = result?;
    let returned = u32::from(text.text.is_some());
    status::item(out, true, false, text.state != PdfTextState::Text);
    out.page = Some(PageInfo {
        offset,
        returned,
        total: text.total.into(),
        pool: None,
        truncated: None,
    });
    if returned > 0
        && offset + returned < text.total
        && let Some(request) = crate::next::checked(
            query.at_section_offset(offset + returned),
            Source::Drugsfda,
            &mut out.problems,
        )
    {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[Source::Drugsfda],
            Request::GetDrug(request),
            cx.fresh,
        ));
    }
    document.text = Some(text);
    Ok(Some(selected.into()))
}

/// Say why a document ID cannot be read: the application lacks it, or it is not a letter.
/// Returns the section reason; a unique letter, or a duplicated ID, goes on to `select`.
pub(super) fn refusal(
    query: &DrugGet,
    row: &drugsfda::Application,
    out: &mut Outcome,
) -> Result<Option<biomcp_types::Reason>, RequestError> {
    let Some(id) = query.document_id() else {
        return Ok(None);
    };
    let mut kinds = Vec::new();
    let mut letters = Vec::new();
    for submission in row.submissions(0, u32::MAX)? {
        for document in submission
            .application_docs
            .iter()
            .flat_map(|docs| docs.iter())
        {
            let kind = document.r#type.as_ref().map(|kind| kind.as_str());
            if kind == Some("Letter") && !letters.contains(&document.id.as_str().to_owned()) {
                letters.push(document.id.as_str().to_owned());
            }
            if &document.id == id {
                kinds.push(kind.map(str::to_owned));
            }
        }
    }
    let (reason, opening) = match kinds.as_slice() {
        [] => (
            biomcp_types::Reason::NotFound,
            format!("This application lists no document {id}."),
        ),
        [Some(kind)] if kind == "Letter" => return Ok(None),
        [kind] => (
            biomcp_types::Reason::Unsupported,
            format!(
                "Document {id} is {}, not a Letter; document_id (--document-id on the command line) reads FDA letter text only.",
                kind.as_deref().map_or_else(
                    || "of no stated type".to_owned(),
                    |kind| format!("a {kind}")
                )
            ),
        ),
        _ => return Ok(None),
    };
    const SHOWN: usize = 10;
    let listed = match letters.len() {
        0 => "It lists no letter documents.".to_owned(),
        n if n <= SHOWN => format!("Its letter documents: {}.", letters.join(", ")),
        n => format!(
            "Its letter documents include {} ({n} in all; the approvals section lists each one).",
            letters
                .iter()
                .take(SHOWN)
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|source| source.source == Source::Drugsfda)
    {
        source
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: format!("{opening} {listed}").into(),
                retry_after_seconds: None,
            });
    }
    Ok(Some(reason))
}

/// Disclose an FDA letter refusal with its URL and a retry step.
fn refused(
    query: &DrugGet,
    url: &biodata::Text,
    error: Option<RequestError>,
    report: &mut biomcp_types::SourceOutcome,
    out: &mut Outcome,
) {
    if let Some(notice) = error.and_then(|error| pdf::refusal(url, &error)) {
        report.notices.push(notice);
        out.next.push(crate::next::Step::run(
            StepKind::Fresh,
            &[Source::Drugsfda],
            Request::GetDrug(query.clone()),
            true,
        ));
    }
}
