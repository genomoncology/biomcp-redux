//! Native label parser contracts use synthetic prose, never licensed clinical answers.
use biomcp_adapters::openfda_label::Record;
use serde_json::json;

#[test]
fn native_label_parts_keep_every_character_and_missing_fields_stay_missing() {
    let record: Record = serde_json::from_value(json!({
        "id":"2b30c65b-bf6e-4a30-9754-585d500d59f3",
        "set_id":"211ef2da-2868-4a77-8055-1cb2cd78e24b",
        "version":"41", "effective_time":"20260713",
        "openfda":{"brand_name":["Synthetic brand"],"spl_set_id":["211ef2da-2868-4a77-8055-1cb2cd78e24b"]},
        "dosage_and_administration":["  α\nβ  ","\tSecond part.\n"]
    })).unwrap();
    assert_eq!(
        record.parts("dosage_and_administration").unwrap(),
        ["  α\nβ  ", "\tSecond part.\n"]
    );
    assert!(record.parts("boxed_warning").is_none());
    assert!(record.parts("not_a_heading").is_none());
    assert_eq!(record.header().unwrap().identity_level.as_str(), "spl_set");
    assert_eq!(
        record.header().unwrap().id.to_string(),
        "211ef2da-2868-4a77-8055-1cb2cd78e24b"
    );
    assert_eq!(
        record.reference().unwrap().revised.unwrap().to_string(),
        "2026-07-13"
    );
}
