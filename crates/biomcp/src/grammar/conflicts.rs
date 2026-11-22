//! Recover source-specific sub-questions from conflicting selectors.
use biomcp_types::{DrugSource, ErrorMessage, LabelHistorySection, QueryInvalid, Rewrite, Step};
use serde_json::{Map, Value, json};

pub(super) fn next(
    via: Rewrite,
    path: (&str, &str),
    value: &Value,
    cause: &QueryInvalid,
    projection: &[String],
) -> Option<(ErrorMessage, Vec<Step>)> {
    let mut fields = value.as_object()?.clone();
    let (message, examples): (_, &[&str]) = match path {
        ("get", "drug") => (drug(&mut fields, cause)?, &[]),
        ("get", "diagnostic") if exact(cause, &["supplement", "section"]) => {
            remove(
                &mut fields,
                &["section", "find", "section_limit", "section_offset"],
            );
            (ErrorMessage::PmaSupplementRead, &[])
        }
        ("search", "drug") if empty_withdrawals(&fields, cause) => (
            ErrorMessage::DrugWithdrawalQuery,
            &["2026", "tazemetostat", "epithelioid sarcoma"],
        ),
        _ => return None,
    };
    let step = |fields| {
        let request = super::rewrite::request(via, path.0, path.1, Value::Object(fields)).ok()?;
        biomcp_query::Action::Run {
            request: request.clone(),
            fresh: false,
        }
        .prepare()
        .ok()?;
        let mut step = super::recovery::run(&request)?;
        crate::fields::carry_to_correction(&request, projection, &mut step);
        Some(step)
    };
    if examples.is_empty() {
        return Some((message, vec![step(fields)?]));
    }
    let steps = examples
        .iter()
        .filter_map(|example| {
            fields.insert("query".into(), json!(example));
            step(fields.clone())
        })
        .collect::<Vec<_>>();
    (!steps.is_empty()).then_some((message, steps))
}

fn drug(fields: &mut Map<String, Value>, cause: &QueryInvalid) -> Option<ErrorMessage> {
    match fields.get("section")?.as_array()?.as_slice() {
        [section] if section == "label-history" => history(fields, cause),
        [section] if section == "label" => current_label(fields, cause),
        _ => None,
    }
}

fn history(fields: &mut Map<String, Value>, cause: &QueryInvalid) -> Option<ErrorMessage> {
    if exact(
        cause,
        &[
            "decision_from",
            "decision_to",
            "source",
            "section",
            "document_id",
        ],
    ) || exact(cause, &["document_id", "source", "section"])
        || exact(cause, &["section", "find", "label_section"])
            && !present(fields, "find")
            && !present(fields, "label_section")
        || exact(cause, &["section_limit", "section"])
            && !present(fields, "find")
            && !present(fields, "label_section")
    {
        if present(fields, "document_id")
            && (present(fields, "decision_from") || present(fields, "decision_to"))
        {
            return None;
        }
        remove(
            fields,
            &["find", "label_section", "every_version", "section_offset"],
        );
        fields.insert("section".into(), json!(["approvals"]));
        fields.insert("source".into(), json!(DrugSource::Drugsfda));
        return Some(ErrorMessage::DrugHistoryApprovals);
    }
    if exact(cause, &["section_limit", "section"]) {
        fields.remove("section_limit");
        if let Some(heading) = heading(fields)
            && heading.parse::<LabelHistorySection>().is_ok()
            && present(fields, "find")
        {
            fields.insert("label_section".into(), json!(heading));
            return Some(ErrorMessage::ArgumentCombination);
        }
        // The same call may also need a selector; recover the question below.
    } else if cause.message() != Some(ErrorMessage::LabelHistorySectionUnknown)
        && !exact(cause, &["section", "find", "label_section"])
    {
        return None;
    }
    let heading = heading(fields);
    if let Some(heading) = heading
        && heading.parse::<LabelHistorySection>().is_ok()
        && present(fields, "find")
    {
        fields.insert("label_section".into(), json!(heading));
        return Some(ErrorMessage::DrugHistoryHeading);
    }
    remove(fields, &["every_version", "section_offset"]);
    fields.insert("section".into(), json!(["label"]));
    fields.insert("source".into(), json!(DrugSource::OpenfdaLabel));
    match heading {
        Some(heading) => {
            fields.insert("label_section".into(), json!(heading));
        }
        None if !present(fields, "label_section") && present(fields, "find") => {
            safety_headings(fields);
        }
        None => return None,
    }
    // Preserve the established message for a heading known only to current labels.
    Some(
        cause
            .message()
            .unwrap_or(ErrorMessage::DrugHistoryCurrentLabel),
    )
}

fn current_label(fields: &mut Map<String, Value>, cause: &QueryInvalid) -> Option<ErrorMessage> {
    if exact(cause, &["find", "section_limit", "section_offset"]) {
        remove(fields, &["section_limit", "section_offset"]);
        if present(fields, "label_section") {
            return Some(ErrorMessage::DrugLabelFindPaging);
        }
    } else if !exact(cause, &["find", "label_section"]) {
        return None;
    }
    safety_headings(fields);
    Some(ErrorMessage::DrugLabelFindHeading)
}

fn safety_headings(fields: &mut Map<String, Value>) {
    fields.insert(
        "label_section".into(),
        json!("warnings_and_cautions,adverse_reactions"),
    );
}

fn heading(fields: &Map<String, Value>) -> Option<&'static str> {
    biomcp_types::openfda_label_field(fields.get("label_section")?.as_str()?)
}

fn empty_withdrawals(fields: &Map<String, Value>, cause: &QueryInvalid) -> bool {
    fields.get("source").and_then(Value::as_str) == Some(DrugSource::FdaWithdrawnApprovals.as_str())
        && !present(fields, "query")
        && exact(
            cause,
            &[
                "source",
                "query",
                "firm",
                "classification",
                "status",
                "reason",
                "initiated_from",
                "initiated_to",
            ],
        )
}

fn present(fields: &Map<String, Value>, name: &str) -> bool {
    match fields.get(name) {
        None | Some(Value::Null | Value::Bool(false)) => false,
        Some(Value::Array(items)) => !items.is_empty(),
        _ => true,
    }
}

fn remove(fields: &mut Map<String, Value>, names: &[&str]) {
    for name in names {
        fields.remove(*name);
    }
}

fn exact(cause: &QueryInvalid, names: &[&str]) -> bool {
    cause
        .arguments()
        .iter()
        .map(AsRef::as_ref)
        .eq(names.iter().copied())
}
