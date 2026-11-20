//! Preserve native SPL membership, order and offset paging.
use super::SOURCE;
use crate::{Ctx, Outcome};
use biomcp_adapters::openfda_label as source;
use biomcp_fetch::RequestError;
use biomcp_types::{
    DrugGet, DrugPage, DrugSearch, DrugSource, Paged, ReplyBody, Request, StepKind,
};
pub(super) fn apply(
    cx: &Ctx<'_>,
    query: &DrugSearch,
    window: biomcp_adapters::NativeWindow<biomcp_types::DrugHeader>,
    total: u64,
    out: &mut Outcome,
) {
    let next_offset = window.next_offset();
    let items = window.rows;
    let returned = items.len() as u32;
    crate::status::item(out, returned != 0, false, false);
    if let Some(source) = out.sources.first_mut() {
        source.count = Some(total);
        source.retrieved_count = Some(returned);
    }
    out.page = Some(crate::status::page_info(query.offset(), returned, total));
    nameless(&items, out);
    for item in &items {
        if let Some(get) = crate::next::checked(
            DrugGet::new(item.id.clone(), DrugSource::OpenfdaLabel),
            SOURCE,
            &mut out.problems,
        ) {
            out.next.push(crate::next::Step::run(
                StepKind::Disambiguate,
                &[SOURCE],
                Request::GetDrug(get),
                cx.fresh,
            ));
        }
    }
    if let Some(offset) = next_offset {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[SOURCE],
            Request::SearchDrug(query.at_offset(offset)),
            cx.fresh,
        ));
    }
    out.body = (DrugPage {
        items: items.into(),
        total: Some(total),
    })
    .with_query(query.clone());
}

pub(super) fn headers(
    rows: &[source::Record],
    query: &DrugSearch,
) -> Result<Vec<biomcp_types::DrugHeader>, RequestError> {
    rows.iter()
        .map(|record| {
            let mut header = record.header()?;
            header.matched_name = query.query().and_then(|name| record.name_match(name));
            Ok(header)
        })
        .collect()
}

/// Say plainly which rows have no name, since their labels state none (1060).
fn nameless(items: &[biomcp_types::DrugHeader], out: &mut Outcome) {
    let ids: Vec<String> = items
        .iter()
        .filter(|item| item.name.is_none())
        .map(|item| item.id.to_string())
        .collect();
    let Some(first) = ids.first() else {
        return;
    };
    let message = format!(
        "No drug name: openFDA gives none and the label's own text marks none for {}. Read one with: get drug {first} --source openfda-label",
        ids.join(", ")
    );
    if let Some(source) = out.sources.first_mut() {
        source
            .notices
            .push(biomcp_types::SourceNotice::RequestGuidance {
                data_date: None,
                message: message.into(),
                retry_after_seconds: None,
            });
    }
}
