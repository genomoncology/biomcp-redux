use super::raw;
use biodata::{Attribution, AuthorizationDate, AuthorizationEvent, DrugIdentityLevel, Text};
use biomcp_types::{DrugCard, DrugHeader, DrugSection, DrugSource};
pub(super) fn header(row: &raw::Medicine, problems: &mut crate::mapped::Problems) -> DrugHeader {
    DrugHeader {
        matched_name: None,
        id: row.id.clone(),
        source: DrugSource::Ema,
        name: Some(row.name.clone()),
        version: None,
        // The feed's category describes Human/Veterinary, not identity granularity.
        identity_level: DrugIdentityLevel::Unknown,
        r#type: problems.text("category", row.fields.get("category").map(String::as_str)),
        url: problems.text(
            "medicine_url",
            row.fields.get("medicine_url").map(String::as_str),
        ),
    }
}
pub(super) fn card(row: &raw::Medicine, problems: &mut crate::mapped::Problems) -> DrugCard {
    DrugCard {
        header: header(row, problems),
        alternate_names: Box::default(),
        cross_references: Box::default(),
        parents: Box::default(),
        products: Box::default(),
        formulations: Box::default(),
        version: problems.text(
            "revision_number",
            row.fields.get("revision_number").map(String::as_str),
        ),
        updated: problems.optional(
            "last_updated_date",
            row.fields
                .get("last_updated_date")
                .filter(|v| !v.is_empty())
                .map(|s| raw::date(s)),
        ),
        retrieved: None,
        available_sections: vec![DrugSection::Regulatory].into(),
        next: Box::default(),
    }
}
pub(super) fn event(
    row: &raw::Medicine,
    problems: &mut crate::mapped::Problems,
) -> AuthorizationEvent {
    let dates = row
        .fields
        .iter()
        .filter(|(key, _)| key.ends_with("_date"))
        .filter_map(|(key, value)| {
            Some(AuthorizationDate {
                role: Text::new(key).ok()?,
                date: problems.optional(key, (!value.is_empty()).then(|| raw::date(value)))?,
            })
        })
        .collect();
    AuthorizationEvent {
        record: row.id.clone(),
        product_number: None,
        jurisdiction: None,
        status: problems.text(
            "medicine_status",
            row.fields.get("medicine_status").map(String::as_str),
        ),
        kind: problems.text(
            "opinion_status",
            row.fields.get("opinion_status").map(String::as_str),
        ),
        dates,
        indication: problems.text(
            "therapeutic_indication",
            row.fields.get("therapeutic_indication").map(String::as_str),
        ),
        attribution: Attribution {
            source: row.source.clone(),
            record: Some(row.record.clone()),
            version: problems.text(
                "revision_number",
                row.fields.get("revision_number").map(String::as_str),
            ),
            release: Some(row.meta.timestamp.clone()),
            submitter: problems.text(
                "marketing_authorisation_developer_applicant_holder",
                row.fields
                    .get("marketing_authorisation_developer_applicant_holder")
                    .map(String::as_str),
            ),
            evaluated: None,
            url: problems.text(
                "medicine_url",
                row.fields.get("medicine_url").map(String::as_str),
            ),
        },
    }
}
