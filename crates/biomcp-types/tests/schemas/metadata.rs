//! Generated schema metadata stays attached to the owning public types.
use super::{generate, registry};
use biodata::PartialDate;
use biomcp_types::ErrorEnvelope;

#[test]
fn every_generated_schema_describes_its_root() {
    for contract in registry() {
        let schema = (contract.generate)();
        assert!(schema["description"].as_str().is_some_and(|text| !text.trim().is_empty()),
            "{} needs a root description", contract.name);
    }
}

#[test]
fn generated_metadata_preserves_partial_dates_and_describes_error_codes() {
    let date = generate::<PartialDate>();
    assert_eq!(date["pattern"], r"^[0-9]{4}(-[0-9]{2}(-[0-9]{2})?)?$");
    assert!(date.get("format").is_none());
    let error = generate::<ErrorEnvelope>();
    assert_eq!(error["$defs"]["ErrorCode"]["description"],
        "Stable error codes for command usage, local failures and expired results.");
}

#[test]
fn generated_links_expose_uri_metadata() {
    let schema = generate::<biomcp_types::TrialHeader>();
    assert_eq!(schema["properties"]["url"]["format"], "uri");
}
