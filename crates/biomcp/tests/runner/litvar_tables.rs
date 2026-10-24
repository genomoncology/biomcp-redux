//! LitVar response boundaries through the existing isolated command runner.
#![expect(clippy::unwrap_used, clippy::indexing_slicing,
    reason = "Checked fixtures and response-only mutations fail directly on malformed shapes")]
use super::response_cases::{checked_case, checked_steps};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;
const KRAS: &[&str] = &["search", "article", "--gene", "KRAS", "--variant", "G12C", "--source", "litvar"];
const RSID: &[&str] = &["search", "article", "--variant", "rs121434592", "--source", "litvar"];
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("article/litvar/response-boundaries", || boundaries().map_err(|e| e.to_string().into())),
        Trial::test("article/litvar/source-failures", || failures().map_err(|e| e.to_string().into())),
    ]
}
fn publications(record: &Value) -> bool { record["url"].as_str().is_some_and(|url| url.ends_with("/publications")) }
fn boundaries() -> Check {
    let output = checked_case("article/litvar/kras-g12c", KRAS, |_, _| {}, json!({"/page/total":8915}), 0)?;
    assert!(output["result"]["items"].as_array().unwrap().iter().all(|item| item.get("found_by").is_none()));
    let prefixed = ["search", "article", "--gene", "KRAS", "--variant", "p.G12C", "--source", "litvar"];
    checked_case("article/litvar/kras-g12c", &prefixed, |_, _| {}, json!({"/sources/1/term":"rs121913530"}), 0)?;
    for (kind, expected, exit) in [
        ("empty",json!({"/status":"absent","/result/items":{"count":0}}),1),
        ("multiple",json!({"/status":"absent","/result/candidates":{"count":3}}),1),
        ("nearby",json!({"/status":"absent","/result/candidates":{"count":5},"/next":{"count":5}}),1),
        ("bad-id",json!({"/status":"failed","/sources/1/reason":"bad_reply"}),4),
        ("zero-pmid",json!({"/status":"failed","/sources/1/reason":"bad_reply"}),4),
        ("cap",json!({"/status":"complete","/page/total":9999,"/page/truncated":true,"/sources/1/count":12000}),0),
    ] {
        checked_case("article/litvar/kras-g12c", KRAS, |source, record| {
            if source != "litvar" { return; }
            for exchange in record["exchanges"].as_array_mut().unwrap() {
                let body = &mut exchange["body"]["value"];
                if let Some(rows) = body.as_array_mut() {
                    match kind {
                        "empty" => rows.clear(),
                        "multiple" => rows.push(rows[1].clone()),
                        "nearby" => { let row = rows[0].clone(); *rows = vec![row; 6]; },
                        "bad-id" => rows[1]["_id"] = json!("bad%23id"),
                        _ => {},
                    }
                } else if kind == "zero-pmid" { body["pmids"][0] = json!(0); }
                else if kind == "cap" {
                    let ids = body["pmids"].as_array_mut().unwrap();
                    ids.extend((1..=12000 - ids.len()).map(|id| json!(50_000_000 + id)));
                    body["pmids_count"] = json!(12000);
                }
            }
        }, expected, exit)?;
    }
    Ok(())
}
fn failures() -> Check {
    for (kind, reason) in [("429","rate_limited"),("500","unavailable"),("cut","bad_reply"),("error","bad_reply"),("deadline","deadline_exceeded")] {
        checked_steps("article/litvar/akt1-rsid-citations", |source, record| {
            if source != "litvar" || !publications(record) { return; }
            let mut exchange = record["exchanges"][0].clone();
            exchange["delay_ms"] = json!(0);
            match kind {
                "429" => exchange["status"] = json!(429),
                "500" => exchange["status"] = json!(500),
                "cut" => exchange["body"] = json!({"kind":"text","value":"{\"pmids\":["}),
                "error" => exchange["body"]["value"] = json!({"error":"synthetic failure"}),
                _ => exchange["delay_ms"] = json!(15001),
            }
            record["exchanges"] = json!(vec![exchange; if matches!(kind,"429"|"500") {3} else {1}]);
        }, json!([
            {"run":RSID,"exit":4,"expect":{"/status":"failed","/sources/0/reason":reason,"/next/0/kind":"fresh"}},
            {"run":RSID,"offline":true,"exit":4,"expect":{"/status":"failed","/sources/0/reason":"offline"}}
        ]))?;
    }
    Ok(())
}
