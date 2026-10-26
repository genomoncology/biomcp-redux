//! Feature: phenotype-types. Public checked request and neutral record contracts.
use biodata::HpId;
use biomcp_types::{PhenotypeGet, PhenotypeSearch, PhenotypeSource};
use serde_json::json;

#[test]
fn hp_identity_round_trips_and_rejects_other_namespaces() {
    for value in ["HP:0001250", "HP:0000000", "HP:9999999"] {
        let id: HpId = value.parse().unwrap();
        assert_eq!(id.to_string(), value);
        assert_eq!(serde_json::from_value::<HpId>(json!(value)).unwrap(), id);
    }
    for value in ["HP:1250", "HP:00001250", "hp:0001250", "MONDO:0001250", "HGNC:1", " HP:0001250", "HP:00012a0"] {
        assert!(value.parse::<HpId>().is_err(), "{value}");
    }
}

#[test]
fn source_and_section_pairings_validate_before_any_adapter() {
    let search: PhenotypeSearch = serde_json::from_value(json!({"text":"seizure"})).unwrap();
    assert_eq!(search.source(), PhenotypeSource::Monarch);
    assert_eq!(search.limit(), 5);
    for section in [None, Some("synonyms"), Some("xrefs"), Some("hierarchy"), Some("diseases"), Some("genes")] {
        let mut value = json!({"id":"HP:0001250"});
        if let Some(section) = section { value.as_object_mut().unwrap().insert("section".into(), json!(section)); }
        let get: PhenotypeGet = serde_json::from_value(value.clone()).unwrap();
        let association = matches!(section, Some("diseases" | "genes"));
        let monarch = association || section == Some("hierarchy");
        assert_eq!(get.source(), if monarch {PhenotypeSource::Monarch} else {PhenotypeSource::Hpo});
        assert_eq!(get.section_limit(), if association {1} else {5});
        for source in ["hpo", "monarch"] {
            value.as_object_mut().unwrap().insert("source".into(), json!(source));
            assert_eq!(serde_json::from_value::<PhenotypeGet>(value.clone()).is_ok(), source == "monarch" || !monarch);
        }
    }
    for bad in [json!({"text":"x","source":"hpo"}), json!({"text":" "}), json!({"text":"x","limit":0}), json!({"text":"x","limit":51}), json!({"text":"x","offset":-1})] {
        assert!(serde_json::from_value::<PhenotypeSearch>(bad).is_err());
    }
    for bad in [json!({"id":"MONDO:1"}), json!({"id":"HP:0001250","section":"similarity"}), json!({"id":"HP:0001250","section_limit":0}), json!({"id":"HP:0001250","source":null})] {
        assert!(serde_json::from_value::<PhenotypeGet>(bad).is_err());
    }
}
