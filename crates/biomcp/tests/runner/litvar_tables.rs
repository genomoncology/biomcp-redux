//! LitVar response boundaries through the existing isolated command runner.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Checked fixtures and response-only mutations fail directly on malformed shapes"
)]
use super::response_cases::{checked_case, checked_steps};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;
const KRAS: &[&str] = &[
    "search",
    "article",
    "--gene",
    "KRAS",
    "--variant",
    "G12C",
    "--source",
    "litvar",
];
const RSID: &[&str] = &[
    "search",
    "article",
    "--variant",
    "rs121434592",
    "--source",
    "litvar",
];
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("article/litvar/response-boundaries", || {
            boundaries().map_err(|e| e.to_string().into())
        }),
        transcript_trial("article/litvar/empty-recovery", || {
            empty_recovery().map_err(|e| e.to_string().into())
        }),
        transcript_trial("article/litvar/source-failures", || {
            failures().map_err(|e| e.to_string().into())
        }),
    ]
}
fn publications(record: &Value) -> bool {
    record["url"]
        .as_str()
        .is_some_and(|url| url.ends_with("/publications"))
}
fn boundaries() -> Check {
    let output = checked_case(
        "article/litvar/kras-g12c",
        KRAS,
        |_, _| {},
        json!({"/page/total":8915}),
        0,
    )?;
    assert!(
        output["result"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .all(|item| item.get("found_by").is_none()),
        "LitVar requests must stay inside the supported selector boundary"
    );
    let prefixed = [
        "search",
        "article",
        "--gene",
        "KRAS",
        "--variant",
        "p.G12C",
        "--source",
        "litvar",
    ];
    checked_case(
        "article/litvar/kras-g12c",
        &prefixed,
        |_, _| {},
        json!({"/sources/1/term":"rs121913530"}),
        0,
    )?;
    for (kind, expected, exit) in [
        ("empty", json!({"/status":"absent",}), 1),
        (
            "multiple",
            json!({"/status":"absent","/result/candidates":{"count":3}}),
            1,
        ),
        (
            "nearby",
            json!({"/status":"absent","/result/candidates":{"count":5},"/next":{"count":5}}),
            1,
        ),
        (
            "bad-id",
            json!({"/status":"absent","/result/candidates":{"count":1},"/problems/*/field":["identity"]}),
            1,
        ),
        (
            "bad-rsid",
            json!({"/status":"absent","/result/candidates":{"count":1},"/problems/*/field":["identity"]}),
            1,
        ),
        (
            "zero-pmid",
            json!({"/page/total":8914,"/result/items":{"count":10},"/problems/*/field":{"includes":"pmids"}}),
            0,
        ),
        (
            "cap",
            json!({"/status":"complete","/page/total":9999,"/page/truncated":true,"/sources/1/count":12000}),
            0,
        ),
    ] {
        checked_case(
            "article/litvar/kras-g12c",
            KRAS,
            |source, record| boundary_edit(source, record, kind),
            expected,
            exit,
        )?;
    }
    health()?;
    Ok(())
}
fn health() -> Check {
    let disabled = biomcp_adapters::registry::SOURCES
        .iter()
        .filter(|spec| spec.source != biomcp_types::Source::LitVar)
        .map(|spec| spec.source.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let output = checked_case(
        "article/litvar/akt1-rsid-citations",
        &["health", "--disable-sources", &disabled],
        |_, _| {},
        json!({"/status":"complete"}),
        0,
    )?;
    let row = output["result"]["sources"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["source"] == "litvar")
        .ok_or("LitVar missing from source health")?;
    assert_eq!(
        row["state"], "ok",
        "LitVar health must report source policy without fetching articles"
    );
    assert_eq!(
        row["use"], "open",
        "LitVar health must report source policy without fetching articles"
    );
    assert_eq!(
        row["terms_url"],
        biomcp_adapters::litvar::SPEC.terms.url,
        "LitVar health must report source policy without fetching articles"
    );
    Ok(())
}
fn failures() -> Check {
    for (kind, reason) in [
        ("429", "rate-limited"),
        ("500", "unavailable"),
        ("cut", "bad-reply"),
        ("error", "bad-reply"),
        ("deadline", "deadline-exceeded"),
    ] {
        let combined = [
            "search",
            "article",
            "--variant",
            "rs121434592",
            "--source",
            "pubmed,litvar",
        ];
        for (capture, args, partial) in [
            ("article/litvar/akt1-rsid-citations", RSID, false),
            ("article/litvar/combined-rsid", combined.as_slice(), true),
        ] {
            let source = if partial {
                "/sources/1/reason"
            } else {
                "/sources/0/reason"
            };
            let status = if partial { "partial" } else { "failed" };
            let exit = if partial { 0 } else { 4 };
            let mut expected = json!({"/status":status});
            expected[source] = json!(reason);
            expected[if partial {
                "/sources/1/term"
            } else {
                "/sources/0/term"
            }] = json!("rs121434592");
            if partial {
                expected["/result/items"] = json!({"count":3});
            }
            let mut offline = expected.clone();
            if !partial {
                offline[source] = json!("offline");
            }
            checked_steps(
                capture,
                |source, record| failure_edit(source, record, kind),
                json!([
                    {"run":args,"exit":exit,"expect":expected},
                    {"run":args,"offline":true,"exit":exit,"expect":offline}
                ]),
            )?;
        }
    }
    Ok(())
}
fn failure_edit(source: &str, record: &mut Value, kind: &str) {
    if source != "litvar" || !publications(record) {
        return;
    }
    let mut exchange = record["exchanges"][0].clone();
    exchange["delay_ms"] = json!(0);
    match kind {
        "429" => exchange["status"] = json!(429),
        "500" => exchange["status"] = json!(500),
        "cut" => exchange["body"] = json!({"kind":"text","value":"{\"pmids\":["}),
        "error" => exchange["body"]["value"] = json!({"error":"synthetic failure"}),
        _ => exchange["delay_ms"] = json!(15001),
    }
    record["exchanges"] = json!(vec![
        exchange;
        if matches!(kind, "429" | "500") { 3 } else { 1 }
    ]);
}

fn boundary_edit(source: &str, record: &mut Value, kind: &str) {
    if source != "litvar" {
        return;
    }
    for exchange in record["exchanges"].as_array_mut().unwrap() {
        let body = &mut exchange["body"]["value"];
        if let Some(rows) = body.as_array_mut() {
            match kind {
                "empty" => rows.clear(),
                "multiple" => rows.push(rows[1].clone()),
                "nearby" => {
                    let row = rows[0].clone();
                    *rows = vec![row; 6];
                }
                "bad-id" => rows[1]["_id"] = json!("bad%23id"),
                "bad-rsid" => rows[1]["rsid"] = json!("not-an-rsid"),
                _ => {}
            }
        } else if kind == "zero-pmid" {
            *body["pmids"].as_array_mut().unwrap().last_mut().unwrap() = json!(0);
        } else if kind == "cap" {
            let ids = body["pmids"].as_array_mut().unwrap();
            ids.extend((1..=12000 - ids.len()).map(|id| json!(50_000_000 + id)));
            body["pmids_count"] = json!(12000);
        }
    }
}

fn empty_recovery() -> Check {
    for disabled in [false, true] {
        let mut args = KRAS.to_vec();
        if disabled {
            args.extend(["--disable-sources", "europepmc"]);
        }
        let output = checked_steps(
            "article/litvar/kras-g12c",
            |source, record| boundary_edit(source, record, "empty"),
            json!([{
                "run": args, "mcp": args, "exit": 1,
                "expect": {"/status":"absent", }
            }]),
        )?;
        let steps = output["next"].as_array().ok_or("missing recovery steps")?;
        let expected = if disabled {
            vec!["pubmed"]
        } else {
            vec!["europepmc", "pubmed"]
        };
        assert_eq!(
            steps.len(),
            expected.len() + 1,
            "Empty results must keep recovery steps"
        );
        for (step, source) in steps.iter().zip(expected) {
            let mut args = vec!["search", "article", "--gene", "KRAS", "--variant", "G12C"];
            if source != "pubmed" {
                args.splice(4..4, ["--source", source]);
            }
            if disabled {
                args.splice(0..0, ["--disable-sources", "europepmc"]);
            }
            assert_eq!(
                step["args"],
                json!(args),
                "Empty results must keep recovery steps"
            );
        }
        let mut broader = vec!["search", "article", "--gene", "KRAS"];
        if disabled {
            broader.splice(0..0, ["--disable-sources", "europepmc"]);
        }
        assert_eq!(
            steps.last().ok_or("missing broader gene search")?["args"],
            json!(broader),
            "Empty results must keep recovery steps"
        );
        assert_eq!(
            output["sources"][1]["notices"][0]["kind"], "litvar-rsid-hint",
            "Empty results must keep recovery steps"
        );
        assert_eq!(
            output["sources"][1]["notices"][0]["message"],
            "If you have an rsID, look it up in LitVar with --variant RSID --source litvar.",
            "Empty results must keep recovery steps"
        );
    }
    markdown_recovery()
}

fn markdown_recovery() -> Check {
    let directory = tempfile::tempdir()?;
    super::response_cases::copy_capture(
        "article/litvar/kras-g12c",
        directory.path(),
        |source, record| boundary_edit(source, record, "empty"),
    )?;
    let environment = super::launch::Environment::new()?;
    let mut args = environment.globals(Some(directory.path()), false);
    args.extend(KRAS.iter().map(|arg| (*arg).to_owned()));
    args.extend(["--format".to_owned(), "md".to_owned()]);
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()?;
    assert_eq!(
        output.status.code(),
        Some(1),
        "Empty results must keep recovery steps"
    );
    let text = String::from_utf8(output.stdout)?;
    assert!(
        text.contains("If you have an rsID, look it up in LitVar"),
        "Empty results must keep recovery steps"
    );
    Ok(())
}
