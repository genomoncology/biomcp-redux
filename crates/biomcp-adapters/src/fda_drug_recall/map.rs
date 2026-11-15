use super::raw;
use crate::{
    mapped::{Mapped, field_text, push_problem, record},
    raw::Field,
};
use biodata::{DrugRecallNumber, PartialDate, Text};
use biomcp_types::{DrugRecallCard, DrugRecallHeader, Problem, Source};
const SOURCE: Source = Source::FdaDrugRecall;
/// A native openFDA enforcement YYYYMMDD day; another spelling is a field problem.
pub(crate) fn date(
    source: Source,
    value: Option<Field<String>>,
    field: &str,
    id: Option<Text>,
    problems: &mut Vec<Problem>,
) -> Option<PartialDate> {
    let value = value?;
    let date = value.value().and_then(|v| {
        if v.len() != 8 || !v.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let year = v.get(..4)?.parse().ok()?;
        let month = v.get(4..6)?.parse().ok()?;
        let day = v.get(6..)?.parse().ok()?;
        PartialDate::new(year, Some(month), Some(day)).ok()
    });
    if date.is_none() {
        push_problem(problems, source, id, field);
    }
    date
}
/// A record without a well-formed recall number has no identity, so it is declined with a
/// `record.declined` problem naming its raw recall number, or its openFDA `event_id` when the
/// number is blank or `N/A`, instead of failing its page.
pub(super) fn mapped(row: raw::Record) -> Mapped<Option<DrugRecallCard>> {
    let mut problems = Vec::new();
    let number = row.recall_number.as_ref().and_then(Field::as_ref);
    let recall_number = match number.map(|v| v.parse::<DrugRecallNumber>()) {
        Some(Ok(recall_number)) => recall_number,
        _ => {
            return Mapped {
                refusals: crate::Refusal::Refused,
                value: None,
                problems: vec![crate::mapped::declined(
                    SOURCE,
                    "recall_number",
                    row.recall_number.as_ref(),
                    "event_id",
                    row.event_id.as_ref(),
                )],
            };
        }
    };
    let id = record(&recall_number);
    let initiated = date(
        SOURCE,
        row.recall_initiation_date,
        "recall_initiation_date",
        id.clone(),
        &mut problems,
    );
    let reported = date(
        SOURCE,
        row.report_date,
        "report_date",
        id.clone(),
        &mut problems,
    );
    let mut text = |field, value| {
        let admit = if matches!(
            field,
            "classification" | "status" | "code_info" | "voluntary_mandated"
        ) {
            crate::mapped::field_code_text
        } else {
            field_text
        };
        admit(&mut problems, SOURCE, id.clone(), field, value)
    };
    let header = DrugRecallHeader {
        recall_number,
        classification: text("classification", row.classification),
        status: text("status", row.status),
        firm: text("recalling_firm", row.recalling_firm),
        product_description: text("product_description", row.product_description),
        initiated,
        reported,
    };
    let value = DrugRecallCard {
        header,
        reason: text("reason_for_recall", row.reason_for_recall),
        distribution_pattern: text("distribution_pattern", row.distribution_pattern),
        code_info: text("code_info", row.code_info),
        voluntary_mandated: text("voluntary_mandated", row.voluntary_mandated),
    };
    Mapped {
        refusals: crate::Refusal::None,
        value: Some(value),
        problems,
    }
}
