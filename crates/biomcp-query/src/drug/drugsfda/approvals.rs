use crate::{Ctx, Outcome};
use biomcp_adapters::{drugsfda as source, received_date};
use biomcp_fetch::RequestError;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DrugApprovals, DrugFdaSubmission, DrugGet, DrugRecords, PageInfo, Reason, Request, Section,
    Source, StepKind,
};
pub(super) fn select(
    cx: &Ctx<'_>,
    query: &DrugGet,
    row: &source::Application,
    metadata: (Option<i64>, Option<biodata::PartialDate>),
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let (fetched, updated) = metadata;
    let products: Box<[_]> = row
        .products()
        .into_vec()
        .into_iter()
        .filter(|p| {
            query
                .product_number()
                .is_none_or(|n| n == &p.product_number)
        })
        .collect();
    let approvals = if query.product_number().is_some() && products.is_empty() {
        crate::status::item(out, true, false, true);
        Section::unavailable(Source::Drugsfda, Reason::NotFound)
    } else if let Some(reason) = super::letter::refusal(query, row, out)? {
        crate::status::item(out, true, false, true);
        Section::unavailable(Source::Drugsfda, reason)
    } else if let Some(mut records) = super::letter::select(cx, query, row, deadline, out)? {
        super::purpose::enrich(cx, &mut records, deadline, out)?;
        Section::Data {
            source: Source::Drugsfda,
            data: DrugRecords {
                records,
                offset: 0,
                total: Some(1),
                truncated: None,
                version: None,
                retrieved: received_date(fetched),
            },
        }
    } else {
        Section::Data {
            source: Source::Drugsfda,
            data: history(cx, query, row, fetched, deadline, out)?,
        }
    };
    out.body = (DrugApprovals {
        identity: row.header(),
        products,
        source_updated: updated.map(Into::into),
        approvals,
        order: DrugApprovals::ORDER.into(),
    })
    .with_query(query.clone());
    Ok(())
}

fn history(
    cx: &Ctx<'_>,
    query: &DrugGet,
    row: &source::Application,
    fetched: Option<i64>,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) -> Result<DrugRecords<DrugFdaSubmission>, RequestError> {
    let offset = query.section_offset().unwrap_or(0);
    let window = query.decision_window();
    let limit = query
        .section_limit()
        .unwrap_or(biomcp_types::DEFAULT_DRUG_APPROVALS_LIMIT);
    let (mut records, total) = row.submissions(window, offset, limit)?;
    window_notice(window, total, row.submission_count(), out);
    if limit > LETTER_ROWS {
        guidance(LETTERS_SKIPPED.into(), out);
    } else {
        super::purpose::enrich(cx, &mut records, deadline, out)?;
    }
    let returned = records.len() as u32;
    out.page = Some(PageInfo {
        offset,
        returned,
        total,
        pool: None,
        truncated: None,
    });
    if let Some(next) = offset
        .checked_add(returned)
        .filter(|n| returned > 0 && u64::from(*n) < total)
        && let Some(request) = crate::next::checked(
            query.at_section_offset(next),
            biomcp_types::Source::Drugsfda,
            &mut out.problems,
        )
    {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[biomcp_types::Source::Drugsfda],
            Request::GetDrug(request),
            cx.fresh,
        ));
    }
    Ok(DrugRecords {
        records,
        offset,
        total: Some(total),
        truncated: None,
        version: None,
        retrieved: received_date(fetched),
    })
}

/// State the date window and how many of the application's submissions fall inside it.
fn window_notice(
    (from, to): (Option<biodata::PartialDate>, Option<biodata::PartialDate>),
    inside: u64,
    all: usize,
    out: &mut Outcome,
) {
    if from.is_none() && to.is_none() {
        return;
    }
    let bound = |date: Option<biodata::PartialDate>| date.map(|d| d.to_string());
    let message = format!(
        "{inside} of {all} submissions have a status date from {} to {}, inclusive; \
         undated submissions are left out.",
        bound(from).unwrap_or_else(|| "the first".into()),
        bound(to).unwrap_or_else(|| "the last".into()),
    );
    guidance(message, out);
}
/// The largest page that reads supplement letters: each efficacy supplement costs one FDA letter
/// download, so a page of more rows reads none.
const LETTER_ROWS: u32 = 50;
const LETTERS_SKIPPED: &str = "--section-limit above 50 reads no supplement letters, so rows carry \
    no supplement_purpose. Ask for 50 rows or fewer, or one --document-id, to read them.";
fn guidance(message: String, out: &mut Outcome) {
    if let Some(source) = out
        .sources
        .iter_mut()
        .find(|s| s.source == Source::Drugsfda)
    {
        source
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: message.into(),
                retry_after_seconds: None,
            });
    }
}
