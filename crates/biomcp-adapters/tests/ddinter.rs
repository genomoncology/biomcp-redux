//! Feature drug-ddinter: source-local identity, pair order and duplicate provenance.
#![expect(
    clippy::unwrap_used,
    reason = "Malformed synthetic fixture setup must fail the contract"
)]
use biomcp_adapters::ddinter::{NativeRow, PartRow, partners};

fn row(part: &str, number: u64, a: &str, b: &str, name: &str) -> PartRow {
    PartRow {
        part: part.into(),
        row: number,
        value: serde_json::from_value(serde_json::json!({
            "DDInterID_A": a, "Drug_A": "Synthetic anchor",
            "DDInterID_B": b, "Drug_B": name, "Level": "Native test level"
        }))
        .unwrap(),
    }
}
#[test]
fn drug_ddinter_preserves_pair_order_orientation_and_all_provenances() {
    let rows = vec![
        row("B", 2, "DDInter1", "DDInter2", "Synthetic second"),
        row("L", 3, "DDInter2", "DDInter1", "Synthetic anchor"),
        row("L", 4, "DDInter1", "DDInter3", "Synthetic third"),
    ];
    let pairs = partners(rows, "DDInter1");
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].partner_id(), "DDInter2");
    assert_eq!(pairs[0].provenance.len(), 2);
    assert_eq!(pairs[0].provenance[0].part.as_ref(), "B");
    assert_eq!(pairs[0].provenance[1].part.as_ref(), "L");
    assert_eq!(pairs[1].partner_id(), "DDInter3");
    assert!(
        partners(
            vec![row("A", 2, "DDInter1", "DDInter2", "Other")],
            "DDInter9"
        )
        .is_empty()
    );
}
#[test]
fn drug_ddinter_checks_required_native_cells_before_publication() {
    let good = serde_json::json!({"DDInterID_A":"DDInter1", "Drug_A":"Synthetic A", "DDInterID_B":"DDInter2", "Drug_B":"Synthetic B", "Level":"Native test level"});
    assert!(serde_json::from_value::<NativeRow>(good.clone()).is_ok());
    for (key, value) in [
        ("DDInterID_A", "1"),
        ("DDInterID_B", "DDInter"),
        ("Drug_A", ""),
        ("Drug_B", ""),
        ("Level", ""),
    ] {
        let mut bad = good.clone();
        bad[key] = value.into();
        assert!(
            serde_json::from_value::<NativeRow>(bad).is_err(),
            "accepted {key}={value}"
        );
    }
}
#[test]
fn drug_ddinter_names_only_discover_separate_native_candidates() {
    let rows = vec![
        row("L", 4, "DDInter1", "DDInter3", "Other"),
        row("B", 2, "DDInter1", "DDInter2", "Synthetic second"),
    ];
    let items = biomcp_adapters::ddinter::candidates(rows, "Synthetic");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].id.to_string(), "DDInter1");
    assert_eq!(items[1].id.to_string(), "DDInter2");
    assert_eq!(items[0].identity_level, biodata::DrugIdentityLevel::Unknown);
    assert!(items[0].url.is_none());
}
