use super::{Check, case, checked_case, tsv};
use serde_json::json;

pub(super) fn check() -> Check {
    case(
        "gene/main",
        &["get", "gene", "HGNC:1097"],
        |_, _| {},
        json!({"/result/items/0/answer/gene/hgnc_id": "HGNC:1097"}),
        0,
    )?;
    for (field, value) in [("name", " "), ("locus_group", "new group")] {
        let answer = checked_case(
            "gene/main",
            &["get", "gene", "HGNC:1097"],
            |source, record| {
                if source == "hgnc" {
                    tsv(record, "BRAF", &[(field, value)]);
                }
            },
            json!({"/result/items/0/answer/gene/hgnc_id": "HGNC:1097",
                "/problems": {"count": 1}, "/problems/*/field": [field]}),
            0,
        )?;
        if answer
            .pointer(&format!("/result/items/0/answer/gene/{field}"))
            .is_some()
        {
            return Err(format!("malformed HGNC {field} survived").into());
        }
    }
    Ok(())
}
