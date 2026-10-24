//! Drug public API contracts, before any command implementation exists.
use biodata::{DrugId, DrugIdentityLevel};
use biomcp_types::{DrugGet, DrugSearch};
use serde_json::json;

#[test]
fn published_ids_roundtrip_without_source_prefixes() {
    for value in ["CHEMBL941", "EMEA/H/C/000406", "NDA021335", "741ff3e3-dc1a-45a6-84e5-2481b27131aa", "DDInter1744", "5", "KTUFNOKKBVMGRW-UHFFFAOYSA-N"] {
        let id: DrugId = value.parse().expect("published ID");
        assert_eq!(id.to_string(), value);
        assert_eq!(serde_json::from_value::<DrugId>(json!(value)).expect("serde ID"), id);
    }
    for value in ["chembl:CHEMBL941", "ema:EMEA/H/C/000406", "openfda-label:741ff3e3-dc1a-45a6-84e5-2481b27131aa", "", "Gleevec tablets", " CHEMBL941"] {
        assert!(value.parse::<DrugId>().is_err(), "{value}");
        assert!(serde_json::from_value::<DrugId>(json!(value)).is_err());
    }
}

#[test]
fn unknown_identity_remains_unknown() {
    let value: DrugIdentityLevel = "unknown".parse().expect("unknown level");
    assert_eq!(value.to_string(), "unknown");
    assert!("chemical_guess".parse::<DrugIdentityLevel>().is_err());
}

#[test]
fn search_defaults_and_required_get_source_are_explicit() {
    let query: DrugSearch = serde_json::from_value(json!({"query":"Gleevec"})).expect("search");
    assert_eq!(serde_json::to_value(query).expect("search JSON")["source"], "mychem");
    assert!(serde_json::from_value::<DrugGet>(json!({"input":"CHEMBL941"})).is_err());
    assert!(serde_json::from_value::<DrugGet>(json!({"input":"chembl:CHEMBL941", "source":"chembl"})).is_err());
}
