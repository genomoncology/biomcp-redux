use super::super::response_cases::checked_steps;
use super::Check;
use serde_json::json;

pub(super) fn check() -> Check {
    let problems = json!([
        {"source": "hgnc", "field": "hgnc.rows.173.no_usable_query_key", "code": "value.unknown"},
        {"source": "hgnc", "field": "hgnc.rows.174.column_count", "code": "value.unknown"},
        {"source": "hgnc", "field": "hgnc-withdrawn.rows.7.no_usable_query_key", "code": "value.unknown"},
        {"source": "hgnc", "field": "hgnc-withdrawn.rows.8.column_count", "code": "value.unknown"}
    ]);
    checked_steps(
        "gene/main",
        |source, record| {
            if source != "hgnc" {
                return;
            }
            let Some(body) = record.pointer_mut("/exchanges/0/body/value") else {
                return;
            };
            let Some(text) = body.as_str() else {
                return;
            };
            let Some(header) = text.lines().next() else {
                return;
            };
            // Append one unkeyable row and one malformed row to each identity file.
            let empty = vec![""; header.split('\t').count()].join("\t");
            *body = format!(
                "{}\n{empty}\nprivate rejected row contents\n",
                text.trim_end_matches('\n')
            )
            .into();
        },
        json!([
            {"run": ["sync", "hgnc"], "expect": {
                "/result/rows": 176, "/problems": problems.clone()
            }},
            {"run": ["sync", "hgnc"], "expect": {
                "/result/rows": 176, "/problems": problems
            }},
            {"run": ["get", "gene", "BRAF"], "expect": {
                "/result/items/0/answer/gene/hgnc_id": "HGNC:1097"
            }}
        ]),
    )?;
    Ok(())
}
