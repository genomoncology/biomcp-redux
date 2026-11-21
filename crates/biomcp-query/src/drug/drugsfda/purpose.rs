//! Submission-owned letter evidence; never borrow indication wording from a product label.
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{drugsfda, pdf};
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    DrugFdaDocument, DrugFdaSubmission, DrugFdaSupplementPurpose, Source, SourceNotice,
};

pub(super) fn enrich(
    cx: &Ctx<'_>,
    records: &mut [DrugFdaSubmission],
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let leg = cx.fetch.leg(&drugsfda::DOCUMENTS, deadline).fresh(cx.fresh);
    let mut incomplete = false;
    let mut notices = Vec::new();
    for row in records.iter_mut().filter(|row| {
        row.event
            .kind
            .as_ref()
            .is_some_and(|v| v.as_str() == "SUPPL")
            && row
                .submission_class_code
                .as_ref()
                .is_some_and(|v| v.as_str() == "EFFICACY")
    }) {
        let purpose = read(&leg, row, &mut notices)?;
        incomplete |= matches!(purpose, DrugFdaSupplementPurpose::Unavailable { .. });
        row.supplement_purpose = Some(purpose);
    }
    let report = leg.finish();
    let document_failed = !report.failures.is_empty();
    let report = status::source(Source::Drugsfda, report, None);
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|s| s.source == Source::Drugsfda)
    {
        source.notices.extend(report.notices);
        for notice in notices {
            if !source.notices.contains(&notice) {
                source.notices.push(notice);
            }
        }
        if document_failed {
            source.notices.push(biomcp_types::SourceNotice::RequestGuidance {
            data_date: None,
                message: "Approval history was read; some supplement letters could not be read. Each affected row reports its document failure.".into(),
                retry_after_seconds: None,
            });
        }
    }
    if incomplete {
        status::item(out, true, false, true);
    }
    Ok(())
}

/// Why a letter in `state` gave no text to search, when it gave none.
fn unread(state: biomcp_types::PdfTextState) -> Option<&'static str> {
    Some(match state {
        biomcp_types::PdfTextState::Garbled => {
            "The linked FDA letter's text could not be decoded (it reads as gibberish); its supplement purpose is unavailable."
        }
        biomcp_types::PdfTextState::Refused => {
            "The linked FDA letter could not be read safely within BioMCP's memory or time limit; its supplement purpose is unavailable."
        }
        biomcp_types::PdfTextState::Damaged => {
            "The linked FDA letter's PDF is damaged and was not read; its supplement purpose is unavailable."
        }
        _ => return None,
    })
}

fn read(
    leg: &biomcp_fetch::Leg<'_>,
    row: &DrugFdaSubmission,
    notices: &mut Vec<SourceNotice>,
) -> Result<DrugFdaSupplementPurpose, RequestError> {
    let document = row
        .application_docs
        .as_deref()
        .unwrap_or_default()
        .iter()
        .filter(|doc| doc.r#type.as_ref().is_some_and(|v| v.as_str() == "Letter"))
        .max_by_key(|doc| {
            (
                doc.date.map(|date| (date.year(), date.month(), date.day())),
                doc.id.as_str(),
            )
        });
    let Some((document, url)) = document.and_then(|doc| doc.url.as_ref().map(|url| (doc, url)))
    else {
        return Ok(unavailable(
            "No FDA approval letter is linked to this supplement.",
            None,
            Some(biomcp_types::Reason::NotFound),
        ));
    };
    let extracted = match document.text.as_ref() {
        Some(text) => Ok(text.clone()),
        None => pdf::read(leg, &drugsfda::DOCUMENTS, url, 0),
    };
    let pdf = match extracted {
        Ok(pdf) => pdf,
        Err(error @ RequestError::Source(_)) => {
            return Ok(unavailable(
                "The linked FDA letter could not be read; its supplement purpose is unavailable.",
                Some(document),
                status::reason(error),
            ));
        }
        Err(RequestError::Input(
            biomcp_fetch::InputError::Host | biomcp_fetch::InputError::Path,
        )) => {
            return Ok(unavailable(
                "The linked FDA letter URL was rejected; its supplement purpose is unavailable.",
                Some(document),
                Some(biomcp_types::Reason::Rejected),
            ));
        }
        Err(error) => return Err(error),
    };
    if let Some(message) = unread(pdf.state) {
        return Ok(unavailable(message, Some(document), None));
    }
    let evidence = pdf.text.as_deref().and_then(|text| purpose(text, row));
    Ok(if let Some(text) = evidence {
        match biodata::Text::from_source(&text) {
            Ok(text) => {
                if pdf.private_symbol
                    && text.as_str().chars().any(pdf::is_private_use)
                    && let Some(notice) = pdf::missing_notice(&pdf)
                {
                    notices.push(notice);
                }
                DrugFdaSupplementPurpose::Stated {
                    text,
                    document_id: document.id.clone(),
                    url: url.clone(),
                }
            }
            Err(_) => unavailable(
                "The linked FDA letter purpose could not be represented as source text.",
                Some(document),
                Some(biomcp_types::Reason::BadReply),
            ),
        }
    } else {
        unavailable(
            "No complete supplement-purpose statement could be read from the linked FDA letter; no indication was inferred.",
            Some(document),
            Some(biomcp_types::Reason::NotStated),
        )
    })
}

fn unavailable(
    message: &'static str,
    document: Option<&DrugFdaDocument>,
    reason: Option<biomcp_types::Reason>,
) -> DrugFdaSupplementPurpose {
    DrugFdaSupplementPurpose::Unavailable {
        message: message.into(),
        document_id: document.map(|d| d.id.clone()),
        url: document.and_then(|d| d.url.clone()),
        reason,
    }
}

fn purpose(text: &str, row: &DrugFdaSubmission) -> Option<String> {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let header = format!(
        "{}/S-{:0>3}",
        row.event.record,
        row.submission_number.as_str()
    );
    let compact: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let tail = compact.split_once(&header)?.1;
    if tail.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    let provides = text.find(" provides for ")?;
    let start = text.get(..provides)?.rfind("This ")?;
    if !text.get(start..provides)?.contains("supplement") {
        return None;
    }
    let remainder = text.get(start..)?;
    let end = [
        " APPROVAL & LABELING",
        " APPROVAL AND LABELING",
        " We have completed",
    ]
    .iter()
    .filter_map(|marker| remainder.find(marker))
    .min()?;
    let stated = remainder.get(..end)?;
    if stated.len() <= provides - start + " provides for ".len() {
        return None;
    }
    Some(stated.to_owned())
}
