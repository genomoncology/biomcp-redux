//! Native trial malformed-value tables for the outside-in caller checks.
use serde_json::{Value, json};
type Case = (&'static str, Value, &'static str);
pub(super) fn detail() -> Vec<Case> {
    vec![
        ("identificationModule/briefTitle", json!(42), "briefTitle"),
        (
            "statusModule/startDateStruct/type",
            json!("FUTURE"),
            "startDateStruct.type",
        ),
        (
            "eligibilityModule/healthyVolunteers",
            json!(42),
            "healthyVolunteers",
        ),
        (
            "eligibilityModule/eligibilityCriteria",
            json!(42),
            "eligibilityCriteria",
        ),
        (
            "conditionsModule/conditions",
            json!(["valid", 42]),
            "conditions",
        ),
        (
            "armsInterventionsModule/interventions/0/type",
            json!("FUTURE"),
            "interventions.type",
        ),
        (
            "armsInterventionsModule/interventions/0/name",
            json!(42),
            "interventions.name",
        ),
        (
            "contactsLocationsModule/locations/0/status",
            json!("FUTURE"),
            "locations.status",
        ),
        (
            "contactsLocationsModule/locations/0/geoPoint",
            json!({"lat":999,"lon":3}),
            "geoPoint",
        ),
        (
            "referencesModule/references/0/type",
            json!("FUTURE"),
            "references.kind",
        ),
        (
            "documentSection/largeDocumentModule/largeDocs/0/uploadDate",
            json!("2023-02-29T12:00"),
            "largeDocs.uploadDate",
        ),
        (
            "referencesModule/references/0/pmid",
            json!("bad"),
            "references.pmid",
        ),
        (
            "documentSection/largeDocumentModule/largeDocs/0/date",
            json!("2026-99-99"),
            "largeDocs.date",
        ),
        (
            "documentSection/largeDocumentModule/largeDocs/0/size",
            json!("bad"),
            "largeDocs.size",
        ),
        (
            "documentSection/largeDocumentModule/largeDocs/0/filename",
            json!("../evil.pdf"),
            "largeDocs.filename",
        ),
        ("statusModule", json!(42), "statusModule"),
        ("designModule", json!(42), "designModule"),
    ]
}
pub(super) fn text() -> Vec<Case> {
    vec![
        (
            "sponsorCollaboratorsModule/leadSponsor/name",
            json!(" \t "),
            "leadSponsor.name",
        ),
        (
            "outcomesModule/primaryOutcomes/0/measure",
            json!(" \t "),
            "outcomes.measure",
        ),
        (
            "armsInterventionsModule/interventions/0/name",
            json!(" \t "),
            "interventions.name",
        ),
        (
            "contactsLocationsModule/locations/0/city",
            json!(" \t "),
            "city",
        ),
        (
            "referencesModule/references/0/citation",
            json!(" \t "),
            "references.citation",
        ),
        (
            "referencesModule/references/0/type",
            json!(" \t "),
            "references.kind",
        ),
        (
            "documentSection/largeDocumentModule/largeDocs/0/label",
            json!(" \t "),
            "largeDocs.label",
        ),
        (
            "documentSection/largeDocumentModule/largeDocs/0/uploadDate",
            json!(" \t "),
            "largeDocs.uploadDate",
        ),
    ]
}
