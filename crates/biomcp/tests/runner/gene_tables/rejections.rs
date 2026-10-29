use super::super::response_cases::checked_steps;
use super::Check;
use serde_json::json;

pub(super) fn check() -> Check {
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
                "/result/rows": 176, "/problems": {"count": 4},
                "/problems/*/field": {"includes": "hgnc row 173: no usable query key"},
                "/problems/*/code": {"includes": "value.unknown"}
            }},
            {"run": ["sync", "hgnc"], "expect": {
                "/result/rows": 176, "/problems": {"count": 4},
                "/problems/*/field": {"includes": "hgnc row 174: wrong number of columns"}
            }},
            {"run": ["get", "gene", "BRAF"], "expect": {
                "/result/items/0/answer/gene/hgnc_id": "HGNC:1097"
            }}
        ]),
    )?;
    Ok(())
}
