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
            let mut header = if let Some(mechanism) = query.mechanism_of_action() {
                let mut header = record.mechanism_header()?;
                header.matched_sentence = record.sentence(MECHANISM, mechanism.as_str())?;
                header
            } else {
                record.header()?
            };
            header.matched_name = query.query().and_then(|name| record.name_match(name));
            Ok(header)
        })
        .collect()
}

/// openFDA's Mechanism of Action field.
const MECHANISM: &str = "mechanism_of_action";

/// Describe the normalized mechanism search and its coverage even when no labels are returned.
pub(super) fn mechanism(query: &DrugSearch, out: &mut Outcome) {
    let (Some(mechanism), Some(source)) = (query.mechanism_of_action(), out.sources.first_mut())
    else {
        return;
    };
    if query.query().is_none() {
        match source::scope(query).map(|scope| scope.parse()).transpose() {
            Ok(term) => source.term = term,
            // The selector holds the checked mechanism words, so only a local fault reaches this.
            Err(_) => out.local = crate::status::local(biomcp_fetch::bad_reply(SOURCE, Some(200))),
        }
    }
    let words = biomcp_types::openfda_words(mechanism.as_str());
    let message = format!(
        "The search uses \"{words}\" in the {MECHANISM} field only. Labels without that field or with different wording are excluded. \
         No synonyms or drug class inference are used. Totals count matching labels. When present, matched_sentence quotes the label."
    );
    source
        .notices
        .push(biomcp_types::SourceNotice::RequestGuidance {
            data_date: None,
            message: message.into(),
            retry_after_seconds: None,
        });
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
