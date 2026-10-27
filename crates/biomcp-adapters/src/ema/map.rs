use super::raw;
use biodata::{Attribution, AuthorizationDate, AuthorizationEvent, DrugIdentityLevel, Text};
use biomcp_types::{DrugCard, DrugHeader, DrugSection, DrugSource};
pub(super) fn header(row: &raw::Medicine) -> DrugHeader {
    DrugHeader {
        matched_name: None,
        id: row.id.clone(),
        source: DrugSource::Ema,
        name: Some(row.name.clone()),
        version: None,
        // The feed's category describes Human/Veterinary, not identity granularity.
        identity_level: DrugIdentityLevel::Unknown,
        r#type: row.text("category"),
        url: row.text("medicine_url"),
    }
}
pub(super) fn card(row: &raw::Medicine) -> DrugCard {
    DrugCard {
        header: header(row),
        alternate_names: Box::default(),
        cross_references: Box::default(),
        parents: Box::default(),
        products: Box::default(),
        formulations: Box::default(),
        version: row.text("revision_number"),
        updated: row
            .fields
            .get("last_updated_date")
            .and_then(|s| raw::date(s).ok()),
        retrieved: None,
        available_sections: vec![DrugSection::Regulatory].into(),
        next: Box::default(),
    }
}
pub(super) fn event(row: &raw::Medicine) -> AuthorizationEvent {
    let dates = row
        .fields
        .iter()
        .filter(|(key, _)| key.ends_with("_date"))
        .filter_map(|(key, value)| {
            Some(AuthorizationDate {
                role: Text::new(key).ok()?,
                date: raw::date(value).ok()?,
            })
        })
        .collect();
    AuthorizationEvent {
        record: row.id.clone(),
        product_number: None,
        jurisdiction: None,
        status: row.text("medicine_status"),
        kind: row.text("opinion_status"),
        dates,
        indication: row.text("therapeutic_indication"),
        attribution: Attribution {
            source: row.source.clone(),
            record: Some(row.record.clone()),
            version: row.text("revision_number"),
            release: Some(row.meta.timestamp.clone()),
            submitter: row.text("marketing_authorisation_developer_applicant_holder"),
            evaluated: None,
            url: row.text("medicine_url"),
        },
    }
}
