//! Public checked query boundaries used by CLI and MCP.
use biodata::{GeneGet, GeneInput, GeneSearch, Text};
use serde_json::json;
#[test]
fn get_rejects_empty_and_oversized_batches_at_constructor_and_json_boundary() {
    for count in [0, 1, 20, 21] {
        let inputs = vec!["BRAF".parse::<GeneInput>().unwrap(); count];
        assert_eq!(
            GeneGet::new(inputs.clone().into()).is_ok(),
            (1..=20).contains(&count)
        );
        assert_eq!(
            serde_json::from_value::<GeneGet>(json!({"input":inputs})).is_ok(),
            (1..=20).contains(&count)
        );
    }
    for sections in [json!([]), json!(["constraint", "all", "constraint"])] {
        let query: GeneGet = serde_json::from_value(json!({"input":["BRAF"],"section":sections})).unwrap();
        assert_eq!(query.sections().len(), sections.as_array().unwrap().len());
    }
    for sections in [json!(["Constraint"]), json!(["unknown"]), json!(null), json!("constraint")] {
        assert!(serde_json::from_value::<GeneGet>(json!({"input":["BRAF"],"section":sections})).is_err());
    }
}
#[test]
fn search_checks_selectors_and_page_bounds_at_both_entry_points() {
    for limit in [0, 1, 10, 50, 51] {
        assert_eq!(
            GeneSearch::new(Some(Text::new("BRAF").unwrap()), None, limit, u32::MAX).is_ok(),
            (1..=50).contains(&limit)
        );
        assert_eq!(
            serde_json::from_value::<GeneSearch>(json!({"query":"BRAF", "limit":limit})).is_ok(),
            (1..=50).contains(&limit)
        );
    }
    assert!(GeneSearch::new(None, None, 10, 0).is_err());
    assert!(serde_json::from_value::<GeneSearch>(json!({})).is_err());
    let query: GeneSearch = serde_json::from_value(json!({"band":"9p21.3"})).unwrap();
    assert_eq!(query.limit(), 10);
    assert_eq!(query.offset(), 0);
    assert_eq!(query.at_offset(50).offset(), 50);
}
