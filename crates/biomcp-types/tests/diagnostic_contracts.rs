//! Feature diagnostic-types: checked source-owned records before adapters exist.
use biodata::*;
use biomcp_types::*;
use serde_json::{Value, json};

fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(value: Value) -> Value {
    serde_json::to_value(serde_json::from_value::<T>(value).unwrap()).unwrap()
}

#[test]
fn native_id_grammars_refuse_wrappers_and_incomplete_identities() {
    for value in ["GTR000006692.3", "GTR000511179.13"] {
        assert_eq!(roundtrip::<GtrTestId>(json!(value)), value);
    }
    for value in [
        "GTR000006692",
        "gtr:GTR000006692.3",
        "GTR6692.3",
        "GTR000006692.0",
        "GTR000006692.03",
    ] {
        assert!(serde_json::from_value::<GtrTestId>(json!(value)).is_err());
    }
    assert_eq!(
        roundtrip::<WhoProductId>(json!("0466-046-00")),
        "0466-046-00"
    );
    assert_eq!(roundtrip::<PmaNumber>(json!("P150013")), "P150013");
    assert_eq!(roundtrip::<PmaSupplement>(json!("S020")), "S020");
    assert_eq!(
        roundtrip::<FdaDrugApplication>(json!("NDA021335")),
        "NDA021335"
    );
    for value in ["021335", "NDA21335", "drugsfda:NDA021335", "nda021335"] {
        assert!(serde_json::from_value::<FdaDrugApplication>(json!(value)).is_err());
    }
    for value in ["P150013", "P150013/S020", "K173492", "DEN170019"] {
        assert_eq!(roundtrip::<DiagnosticDeviceId>(json!(value)), value);
    }
    for value in [
        "fda:P150013",
        "P150013S020",
        "P150013/S000",
        "DEN17",
        "K173492/S001",
    ] {
        assert!(serde_json::from_value::<DiagnosticDeviceId>(json!(value)).is_err());
    }
}

#[test]
fn checked_search_rejects_cross_source_selectors_before_io() {
    for query in [
        json!({"source":"gtr","gene_id":"672"}),
        json!({"source":"gtr","gene":"BRCA1","type":"Clinical"}),
        json!({"source":"gtr","condition_id":"MedGen:C2676676"}),
        json!({"source":"fda-cdx","device_id":"P150013/S020"}),
        json!({"source":"fda-cdx","drug_application":"BLA125514"}),
        json!({"source":"fda-cdx","biomarker":"KRAS"}),
    ] {
        let checked: DiagnosticSearch = serde_json::from_value(query.clone()).unwrap();
        let mut expected = roundtrip::<DiagnosticSearch>(query);
        assert_eq!(expected["limit"], 5);
        expected["offset"] = json!(5);
        assert_eq!(
            serde_json::to_value(checked.at_offset(5)).unwrap(),
            expected
        );
    }
    for query in [
        json!({"gene":"BRCA1"}),
        json!({"source":"all"}),
        json!({"source":"gtr","gene":"BRCA1","gene_id":"672"}),
        json!({"source":"gtr","condition_id":"MONDO:0007254"}),
        json!({"source":"gtr","condition_id":"MedGen:2676676"}),
        json!({"source":"gtr","device_id":"P150013"}),
        json!({"source":"fda-cdx","gene_id":"672"}),
        json!({"source":"fda-cdx","condition_id":"MedGen:C2676676"}),
        json!({"source":"fda-cdx","type":"Clinical"}),
        json!({"source":"fda-cdx","device_id":"K173492"}),
        json!({"source":"gtr","limit":0}),
        json!({"source":"fda-cdx","drug_cell":"Keytruda"}),
    ] {
        assert!(
            serde_json::from_value::<DiagnosticSearch>(query.clone()).is_err(),
            "{query}"
        );
    }
}

#[test]
fn checked_get_keeps_base_supplement_and_section_scope() {
    for query in [
        json!({"source":"gtr","id":"GTR000006692.3","section":"genes"}),
        json!({"source":"gtr","id":"GTR000006692.3","section":"conditions"}),
        json!({"source":"gtr","id":"GTR000006692.3","section":"methods"}),
        json!({"source":"who-ivd","id":"0466-046-00"}),
        json!({"source":"fda-pma","id":"P150013","section":"supplements"}),
        json!({"source":"fda-pma","id":"P150013","supplement":"S020","section":"regulatory"}),
    ] {
        let get: DiagnosticGet = serde_json::from_value(query.clone()).unwrap();
        let mut expected = roundtrip::<DiagnosticGet>(query);
        expected["section_offset"] = json!(5);
        assert_eq!(
            serde_json::to_value(get.at_section_offset(5)).unwrap(),
            expected
        );
    }
    for query in [
        json!({"id":"GTR000006692.3"}),
        json!({"source":"gtr","id":"GTR000006692"}),
        json!({"source":"gtr","id":"GTR000006692.3","section":"all"}),
        json!({"source":"gtr","id":"GTR000006692.3","supplement":"S020"}),
        json!({"source":"who-ivd","id":"0466-046-00","section":"genes"}),
        json!({"source":"fda-pma","id":"P150013","section":"genes"}),
        json!({"source":"fda-pma","id":"P150013","supplement":"S020","section":"supplements"}),
        json!({"source":"fda-pma","id":"P150013/S020"}),
        json!({"source":"fda-pma","id":"K173492"}),
        json!({"source":"fda-cdx","id":"P150013"}),
        json!({"source":"gtr","id":"GTR000006692.3","full":true}),
    ] {
        assert!(
            serde_json::from_value::<DiagnosticGet>(query.clone()).is_err(),
            "{query}"
        );
    }
}
