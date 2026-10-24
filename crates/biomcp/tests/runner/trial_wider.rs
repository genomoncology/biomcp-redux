//! Wider searches are explicit alternatives and never provider requests.
#![expect(clippy::indexing_slicing, reason = "Recorded response shapes fail directly")]
use super::response_cases::checked_steps;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("trial/wider-search/short-pages-only", || {
        short_pages().map_err(Into::into)
    })]
}
fn short_pages() -> Check {
    for (capture, args, wider) in [
        ("trial/biomarker", vec!["search", "trial", "--condition", "melanoma", "--variant", "BRAF V600E"], vec!["search", "trial", "--condition", "melanoma"]),
        ("trial/gene", vec!["search", "trial", "--condition", "melanoma", "--gene", "BRAF"], vec!["search", "trial", "--condition", "melanoma", "--criteria", "BRAF"]),
    ] {
        for count in [0, 2, 10] {
            let output = checked_steps(capture, |source, record| {
                if source == "ctgov" && record["url"].as_str().is_some_and(|url| url.contains("/studies?")) {
                    let body = &mut record["exchanges"][0]["body"]["value"];
                    body["totalCount"] = json!(count);
                    if let Some(studies) = body["studies"].as_array_mut() { studies.truncate(count); }
                    body.as_object_mut().map(|body| body.remove("nextPageToken"));
                }
            }, json!([{"run": args, "mcp": args, "expect": {"/status":"complete", "/result/items":{"count": count}}}]))?;
            let expected = if count < 10 { json!([{"kind":"disambiguate", "args":wider}]) } else { json!([]) };
            if output.get("next").unwrap_or(&Value::Null) != &expected {
                return Err(format!("{capture}/{count}: next {:?}, expected {expected}", output.get("next")).into());
            }
        }
    }
    Ok(())
}
