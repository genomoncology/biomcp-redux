//! Wider searches are explicit alternatives and never provider requests.
use crate::runner::transcript::trial as transcript_trial;
#![expect(
    clippy::indexing_slicing,
    reason = "Recorded response shapes fail directly"
)]
use super::response_cases::checked_steps;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("trial/wider-search/short-pages-only", || {
            short_pages().map_err(Into::into)
        }),
        transcript_trial("trial/wider-search/active-policy", || {
            policy().map_err(Into::into)
        }),
        transcript_trial("trial/wider-search/typed-option-contract", || {
            options().map_err(Into::into)
        }),
    ]
}
fn short_pages() -> Check {
    for (capture, args, wider) in [
        (
            "trial/biomarker",
            vec![
                "search",
                "trial",
                "--condition",
                "melanoma",
                "--variant",
                "BRAF V600E",
            ],
            vec!["search", "trial", "--condition", "melanoma"],
        ),
        (
            "trial/gene",
            vec![
                "search",
                "trial",
                "--condition",
                "melanoma",
                "--gene",
                "BRAF",
            ],
            vec![
                "search",
                "trial",
                "--condition",
                "melanoma",
                "--criteria",
                "BRAF",
            ],
        ),
    ] {
        for count in [0, 2, 10] {
            let output = checked_steps(
                capture,
                |source, record| {
                    if source == "ctgov"
                        && record["url"]
                            .as_str()
                            .is_some_and(|url| url.contains("/studies?"))
                    {
                        let body = &mut record["exchanges"][0]["body"]["value"];
                        body["totalCount"] = json!(count);
                        if let Some(studies) = body["studies"].as_array_mut() {
                            studies.truncate(count);
                        }
                        if let Some(body) = body.as_object_mut() {
                            body.remove("nextPageToken");
                        }
                    }
                },
                json!([{"run": args, "mcp": args, "expect": {"/status":"complete", "/page/returned": count}}]),
            )?;
            let expected = if count < 10 {
                json!([{"kind":"disambiguate", "args":wider}])
            } else {
                Value::Null
            };
            if output.get("next").unwrap_or(&Value::Null) != &expected {
                return Err(format!(
                    "{capture}/{count}: next {:?}, expected {expected}",
                    output.get("next")
                )
                .into());
            }
        }
    }
    Ok(())
}

// The public typed-query boundary protects all option combinations independently
// of provider request availability; wire execution is covered by the case above.
fn options() -> Check {
    let original = json!({
        "source":"ctgov", "query":"literal AND (text)", "condition":["basal cell carcinoma"],
        "intervention":["literal drug"], "gene":"HGNC:9585", "variant":"p.L576P",
        "status":"RECRUITING", "phase":"1/2", "study_type":"INTERVENTIONAL",
        "sex":"ALL", "age":18, "lat":"40", "lon":"-75", "distance":"50mi",
        "sponsor":"literal sponsor", "sponsor_type":"nih", "updated_since":"2020",
        "updated_until":"2026", "id":"NCT02032823", "count_only":true,
        "has_results":true, "limit":100
    });
    for (existing, cursor) in [(false, false), (true, false), (false, true)] {
        let mut value = original.clone();
        if existing {
            value["criteria"] = json!("retain literal criteria");
        }
        if cursor {
            value["next_page"] = json!("opaque-token");
        }
        let query: biomcp_types::TrialSearch = serde_json::from_value(value)?;
        let before = serde_json::to_value(&query)?;
        let alternatives = query.wider_searches(Some("PTCH1".parse()?));
        if cursor {
            if !alternatives.is_empty() {
                return Err("cursor search was widened".into());
            }
            continue;
        }
        let mut without_variant = before.clone();
        without_variant
            .as_object_mut()
            .ok_or("query object")?
            .remove("variant");
        let mut expected = vec![without_variant];
        if !existing {
            let mut eligibility = before;
            eligibility
                .as_object_mut()
                .ok_or("query object")?
                .remove("gene");
            eligibility["criteria"] = json!("PTCH1");
            expected.push(eligibility);
        }
        let actual = alternatives
            .iter()
            .map(serde_json::to_value)
            .collect::<Result<Vec<_>, _>>()?;
        if actual != expected {
            return Err("wider search changed another option or order".into());
        }
    }
    Ok(())
}

fn policy() -> Check {
    let args = [
        "--disable-sources",
        "pubmed",
        "search",
        "trial",
        "--condition",
        "melanoma",
        "--gene",
        "BRAF",
        "--fresh",
    ];
    checked_steps(
        "trial/gene",
        |source, record| {
            if source == "ctgov"
                && record["url"]
                    .as_str()
                    .is_some_and(|url| url.contains("/studies?"))
            {
                let body = &mut record["exchanges"][0]["body"]["value"];
                body["totalCount"] = json!(0);
                body["studies"] = json!([]);
                if let Some(body) = body.as_object_mut() {
                    body.remove("nextPageToken");
                }
            }
        },
        json!([{"run":args, "mcp":args,"expect":{
            "/status":"complete",  "/next":[{
                "kind":"disambiguate", "args":["--disable-sources","pubmed","search","trial","--condition","melanoma","--criteria","BRAF","--fresh"]
            }]
        }}]),
    )?;
    Ok(())
}
