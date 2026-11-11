use super::raw;
use biodata::{Attribution, AuthorizationDate, AuthorizationEvent, DrugIdentityLevel, Text};
use biomcp_types::{DateRoleLabel, DrugCard, DrugHeader, DrugSection, DrugSource};
/// EMA's own words for its medicine date keys, from two EMA pages:
/// - "Download website data in JSON data format", section "Medicines through centralised procedure -
///   data included":
///   <https://www.ema.europa.eu/en/about-us/about-website/download-website-data-json-data-format>.
///   It names "first published date" and "last updated date".
/// - Each medicine's EPAR page, for example
///   <https://www.ema.europa.eu/en/medicines/human/EPAR/tecovirimat-siga>, shows the
///   `marketing_authorisation_date` value as "Marketing authorisation issued", the
///   `opinion_adopted_date` value as "Opinion adopted", and the `european_commission_decision_date`
///   value only beside "Latest procedure affecting product information" and its reference.
///
/// The `european_commission_decision_date` key is not the decision's date. For Tecovirimat SIGA
/// (read 2026-11-11) it holds 28/05/2026 beside REF/0000287477, while EMA's referral page and
/// referrals file give 29/05/2026 as the European Commission decision date for that procedure.
/// So its date is reported under [`LATEST_PROCEDURE_ROLE`] and no date carries the decision name.
///
/// Neither page labels the other date keys, such as `start_of_evaluation_date` or
/// `withdrawal_of_application_date`, so those keep their native name only.
fn date_label(role: &str) -> Option<Text> {
    Some(match role {
        LATEST_PROCEDURE_ROLE => biodata::text!("Latest procedure affecting product information"),
        "marketing_authorisation_date" => biodata::text!("Marketing authorisation issued"),
        "opinion_adopted_date" => biodata::text!("Opinion adopted"),
        "first_published_date" => biodata::text!("first published date"),
        "last_updated_date" => biodata::text!("last updated date"),
        _ => return None,
    })
}
/// EMA's native key whose value the EPAR page shows as the latest procedure's date.
const LATEST_PROCEDURE_KEY: &str = "european_commission_decision_date";
/// Role for that value, named for what EMA's product page shows beside it.
pub(super) const LATEST_PROCEDURE_ROLE: &str =
    "latest_procedure_affecting_product_information_date";
/// The role a native date key is reported under.
fn role(key: &str) -> &str {
    if key == LATEST_PROCEDURE_KEY {
        LATEST_PROCEDURE_ROLE
    } else {
        key
    }
}
/// Each date role in the event, labelled only with EMA's published words. The EPAR page shows
/// the latest procedure reference beside the latest procedure's date, so it is kept there.
pub(super) fn date_roles(
    row: &raw::Medicine,
    problems: &mut crate::mapped::Problems,
    event: &AuthorizationEvent,
) -> Box<[DateRoleLabel]> {
    event
        .dates
        .iter()
        .map(|date| DateRoleLabel {
            role: date.role.clone(),
            label: date_label(date.role.as_str()),
            procedure: (date.role.as_str() == LATEST_PROCEDURE_ROLE)
                .then(|| {
                    item(
                        row,
                        problems,
                        "latest_procedure_affecting_product_information",
                    )
                })
                .flatten(),
        })
        .collect()
}
pub(super) fn header(row: &raw::Medicine, problems: &mut crate::mapped::Problems) -> DrugHeader {
    problems.required(
        "_bundle_meta.timestamp",
        row.meta
            .timestamp
            .as_ref()
            .and_then(crate::raw::Field::as_ref),
    );
    DrugHeader {
        name_resolution: None,
        matched_name: None,
        id: row.id.clone(),
        source: DrugSource::Ema,
        name: {
            let name = item(row, problems, "name_of_medicine");
            problems.required("name_of_medicine", name)
        },
        version: None,
        // The feed's category describes Human/Veterinary, not identity granularity.
        identity_level: DrugIdentityLevel::Unknown,
        r#type: {
            let category = item(row, problems, "category");
            problems.required("category", category)
        },
        url: item(row, problems, "medicine_url"),
    }
}
pub(super) fn card(row: &raw::Medicine, problems: &mut crate::mapped::Problems) -> DrugCard {
    DrugCard {
        header: header(row, problems),
        alternate_names: Box::default(),
        official_names: Box::default(),
        cross_references: Box::default(),
        parents: Box::default(),
        products: Box::default(),
        formulations: Box::default(),
        version: item(row, problems, "revision_number"),
        updated: problems.optional(
            "last_updated_date",
            row.field("last_updated_date")
                .filter(|value| !matches!(value, Ok("")))
                .map(|value| value.and_then(raw::date)),
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
        .filter_map(|(key, _)| {
            Some(AuthorizationDate {
                role: problems.optional(key, Some(Text::new(role(key))))?,
                date: problems.optional(
                    key,
                    row.field(key)
                        .filter(|value| !matches!(value, Ok("")))
                        .map(|value| value.and_then(raw::date)),
                )?,
            })
        })
        .collect();
    AuthorizationEvent {
        record: row.id.clone(),
        product_number: None,
        jurisdiction: None,
        status: {
            let status = item(row, problems, "medicine_status");
            problems.required("medicine_status", status)
        },
        kind: item(row, problems, "opinion_status"),
        dates,
        indication: problems.optional(
            "therapeutic_indication",
            row.field("therapeutic_indication").map(|value| {
                value.and_then(|text| {
                    crate::mapped::source_text(&crate::source_text::decode_entities(text))
                })
            }),
        ),
        attribution: Attribution {
            approved: None,
            published: None,
            source: row.source.clone(),
            record: Some(row.record.clone()),
            version: item(row, problems, "revision_number"),
            release: problems.required(
                "_bundle_meta.timestamp",
                row.meta
                    .timestamp
                    .as_ref()
                    .and_then(crate::raw::Field::as_ref)
                    .cloned(),
            ),
            submitter: item(
                row,
                problems,
                "marketing_authorisation_developer_applicant_holder",
            ),
            evaluated: None,
            url: item(row, problems, "medicine_url"),
        },
    }
}

fn item(row: &raw::Medicine, problems: &mut crate::mapped::Problems, field: &str) -> Option<Text> {
    problems.optional(
        field,
        row.field(field)
            .map(|value| value.and_then(crate::mapped::source_text)),
    )
}
