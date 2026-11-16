//! Made-up label: a name page keeps only what choosing among same-named labels reads.
use super::super::{Record, raw};

#[test]
fn name_pages_keep_only_what_the_label_choice_reads() {
    let label = serde_json::json!({
        "id": "0f3b2a3e-37a6-4a7c-9c43-6a4a7b0e2c11",
        "set_id": "fb832474-88d9-4e29-95cd-fbc446944cc4",
        "version": "12",
        "effective_time": "20240101",
        "openfda": {"brand_name": ["Glumetza"], "generic_name": ["METFORMIN HYDROCHLORIDE"],
            "application_number": ["NDA021748"],
            "spl_set_id": ["fb832474-88d9-4e29-95cd-fbc446944cc4"]},
        "indications_and_usage": ["1 INDICATIONS AND USAGE 1.1 Type 2 diabetes"],
        "adverse_reactions": ["6 ADVERSE REACTIONS ..."],
        "clinical_studies_table": ["<table>...</table>"]
    });
    let mut rows = vec![Record(
        serde_json::from_value::<raw::Record>(label).expect("label decodes"),
    )];
    let chosen = |row: &Record| format!("{:?} {:?}", row.header(), row.choice());
    let before = chosen(&rows[0]);
    super::choice_sections(&mut rows);
    assert!(!rows[0].has_heading("adverse_reactions"));
    assert!(!rows[0].has_heading("clinical_studies_table"));
    assert!(rows[0].has_heading("indications_and_usage"));
    assert_eq!(chosen(&rows[0]), before);
}
