//! Named provider boundaries through the shared isolated command runner.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Checked captures and response-only edits fail directly on malformed fixture shapes"
)]
use super::response_cases::{checked_case, checked_steps};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::json;
type Check = Result<(), Box<dyn std::error::Error>>;
const WORDS: &str = "next-generation sequencing recommendations advanced cancer ESMO";
const NONSENSE: &str = "xqzvw zzyzxq kinase";
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("article-named-search/named-search/failures", || {
            failures().map_err(|e| e.to_string().into())
        }),
        transcript_trial("article-named-search/named-search/paging-and-join", || {
            paging().map_err(|e| e.to_string().into())
        }),
        transcript_trial(
            "article-named-search/named-search/sentences-and-policies",
            || sentences().map_err(|e| e.to_string().into()),
        ),
    ]
}
fn failures() -> Check {
    for (source, capture, words, deadline) in [
        ("litsense", "article/litsense/combined", NONSENSE, 15001),
        ("pubtator", "article/pubtator/combined", WORDS, 8001),
    ] {
        for (kind, reason) in [
            ("429", "rate-limited"),
            ("500", "unavailable"),
            ("cut", "bad-reply"),
            ("error", "bad-reply"),
            ("deadline", "deadline-exceeded"),
        ] {
            let args = [
                "search",
                "article",
                words,
                "--source",
                &format!("pubmed,{source}"),
            ];
            let expect = json!(
                {"/status":"partial","/sources/1/source":source,"/sources/1/reason":reason,"/result/items":{"count":10},
                "/next/*/kind":{"includes":"fresh"}}
            );
            checked_steps(
                capture,
                |name, record| {
                    if name != source {
                        return;
                    }
                    let mut ex = record["exchanges"][0].clone();
                    ex["delay_ms"] = json!(0);
                    match kind {
                        "429" => ex["status"] = json!(429),
                        "500" => ex["status"] = json!(500),
                        "cut" => ex["body"] = json!({"kind":"text","value":"{\"results\":["}),
                        "error" => ex["body"]["value"] = json!({"error":"synthetic failure"}),
                        _ => ex["delay_ms"] = json!(deadline),
                    }
                    record["exchanges"] = json!(vec![ex; if matches!(kind, "429" | "500") { 3 } else { 1 }]);
                },
                json!([ {"run":args,"expect":expect}, {"run":args,"offline":true,"expect":expect} ]),
            )
            .map_err(|e| format!("{source}/{kind}: {e}"))?;
        }
    }
    Ok(())
}
fn paging() -> Check {
    let args = ["search", "article", WORDS, "--source", "pubtator"];
    for kind in ["cursor", "repeat", "later-failure", "terminal"] {
        let first = checked_case(
            "article/pubtator/pubtator-words",
            &args,
            |_, _| {},
            json!({"/sources/0/retrieved_count":100}),
            0,
        )?;
        let ids: Vec<_> = first["result"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| json!({"pmid":r["pmid"].as_str().unwrap().parse::<u32>().unwrap()}))
            .collect();
        let output = checked_case(
            "article/pubtator/pubtator-words",
            &args,
            |source, record| {
                if source != "pubtator" {
                    return;
                }
                let ex = &mut record["exchanges"][0];
                let page = ex["body"]["value"]["current"].as_u64().unwrap();
                if kind == "terminal" && page == 1 {
                    ex["body"]["value"]["total_pages"] = json!(1);
                }
                if page != 2 {
                    return;
                }
                match kind {
                    "cursor" => ex["body"]["value"]["current"] = json!(1),
                    "repeat" => ex["body"]["value"]["results"] = json!(ids),
                    "later-failure" => {
                        ex["status"] = json!(503);
                        let ex = ex.clone();
                        record["exchanges"] = json!([ex.clone(), ex.clone(), ex]);
                    }
                    _ => {}
                }
            },
            json!({"/sources/0/retrieved_count":10,"/page/total":10,"/page/truncated":true,"/result/items":{"count":10}}),
            0,
        )?;
        assert_eq!(
            output["status"],
            if kind == "terminal" {
                "complete"
            } else {
                "partial"
            },
            "Named searches must preserve snapshot paging and continuations"
        );
        if kind != "terminal" {
            assert_eq!(
                output["sources"][0]["reason"],
                if kind == "later-failure" {
                    "unavailable"
                } else {
                    "bad-reply"
                },
                "Named searches must preserve snapshot paging and continuations"
            );
        }
    }
    native_duplicates()?;
    join()?;
    Ok(())
}
fn join() -> Check {
    let args = [
        "search",
        "article",
        "--gene",
        "AKT1",
        "--variant",
        "E17K",
        "--source",
        "pubtator",
    ];
    checked_case(
        "article/pubtator/pubtator-gene",
        &args,
        |source, record| {
            if source == "pubtator" {
                record["exchanges"] = json!([]);
            }
            if source != "hgnc" {
                return;
            }
            let body = &mut record["exchanges"][0]["body"]["value"];
            let text = body.as_str().unwrap();
            let mut lines = text.lines();
            let header = lines.next().unwrap();
            let column = header
                .split('\t')
                .position(|name| name == "entrez_id")
                .unwrap();
            let rows = lines
                .map(|line| {
                    let mut fields: Vec<_> = line.split('\t').collect();
                    fields[column] = "";
                    fields.join("\t")
                })
                .collect::<Vec<_>>();
            *body = json!(format!("{header}\n{}\n", rows.join("\n")));
        },
        json!({"/status":"failed","/sources/1/reason":"no-join-id"}),
        4,
    )?;
    Ok(())
}
fn sentences() -> Check {
    let args = ["search", "article", NONSENSE, "--source", "litsense"];
    let first = checked_case(
        "article/litsense/licensed-subset",
        &args,
        |_, _| {},
        json!({"/status":"complete","/page/total":7}),
        0,
    )?;
    assert!(
        first["result"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row.get("found_by").is_none()),
        "Research search results must retain citable native identity"
    );
    let output = checked_case(
        "article/litsense/licensed-subset",
        &args,
        |source, record| {
            if source != "litsense" {
                return;
            }
            let rows = record["exchanges"][0]["body"]["value"]
                .as_array_mut()
                .unwrap();
            rows[0]["section"] = json!(42);
            rows[0]["score"] = json!(-1000);
            let mut duplicate = rows[0].clone();
            duplicate["score"] = json!(1000);
            duplicate["text"] =
                json!("Synthetic duplicate must not replace the first provider sentence.");
            rows.push(duplicate);
        },
        json!({"/status":"complete","/page/total":7}),
        0,
    )?;
    assert_eq!(
        output["result"]["items"][0]["sentence"]["text"],
        first["result"]["items"][0]["sentence"]["text"],
        "Research search results must retain citable native identity"
    );
    assert!(
        output["result"]["items"][0]["sentence"]
            .get("section")
            .is_none(),
        "Research search results must retain citable native identity"
    );
    invalid_sentences(&args)?;
    let folded = checked_case(
        "article/litsense/combined",
        &["search", "article", NONSENSE, "--source", "pubmed,litsense"],
        |_, _| {},
        json!({"/status":"complete","/result/items/1/sentence/section":"METHODS","/result/items/1/found_by/litsense":1}),
        0,
    )?;
    assert_eq!(
        folded["result"]["items"][1]["sentence"]["text"],
        first["result"]["items"][0]["sentence"]["text"],
        "Research search results must retain citable native identity"
    );
    merged_sentence()?;
    policies()?;
    Ok(())
}
fn policies() -> Check {
    let help = checked_case(
        "article/litsense/licensed-subset",
        &["help", "search", "article"],
        |_, _| {},
        json!({"/status":"complete"}),
        0,
    )?;
    let source = help["result"]["commands"][0]["arguments"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == "--source")
        .ok_or("source help absent")?;
    let text = source["description"].as_str().unwrap();
    assert!(
        text.contains("resemblance, not a match")
            && text.contains("unrelated")
            && text.contains("right entity ID"),
        "Research search must respect the selected source policy"
    );
    for name in ["litsense", "pubtator"] {
        let args = [
            "search",
            "article",
            "AKT1",
            "--source",
            name,
            "--disable-sources",
            name,
        ];
        checked_case(
            "article/litsense/licensed-subset",
            &args,
            |_, _| {},
            json!({"/status":"failed","/sources/0/state":"skipped","/sources/0/reason":"disabled"}),
            4,
        )?;
    }
    Ok(())
}

fn invalid_sentences(args: &[&str]) -> Check {
    for kind in ["oversize", "zero-pmid", "bad-text"] {
        // Row admission is independent of PubMed hydration, whose original
        // capture covers the complete seven-PMID request. Other proofs above
        // retain the real mixed-source join and its exact request scope.
        let mut selected = args.to_vec();
        let (expect, exit) = if kind == "oversize" {
            (
                json!({"/status":"failed","/sources/0/reason":"bad-reply"}),
                4,
            )
        } else {
            selected.extend(["--disable-sources", "pubmed"]);
            (sentence_admission(kind), 0)
        };
        checked_case(
            "article/litsense/licensed-subset",
            &selected,
            |source, record| {
                if source != "litsense" {
                    return;
                }
                let rows = record["exchanges"][0]["body"]["value"]
                    .as_array_mut()
                    .unwrap();
                match kind {
                    "oversize" => {
                        let row = rows[0].clone();
                        *rows = vec![row; 101];
                    }
                    "zero-pmid" => rows[0]["pmid"] = json!(0),
                    _ => rows[0]["text"] = json!(""),
                }
            },
            expect,
            exit,
        )?;
    }
    Ok(())
}
fn sentence_admission(kind: &str) -> serde_json::Value {
    let mut ids = vec![
        "34718584", "30363995", "28017606", "19112497", "34647959", "36032122", "35656107",
    ];
    let problem = if kind == "zero-pmid" {
        ids.remove(0);
        json!({"source":"litsense","field":"pmid","code":"value.unknown"})
    } else {
        json!({"source":"litsense","record":"34718584","field":"text","code":"value.unknown"})
    };
    let mut expect = json!({
        "/status":"partial", "/result/items/*/pmid":ids,
        "/page/total":ids.len(), "/page/returned":ids.len(),
        "/sources/0/source":"litsense", "/sources/0/state":"ok",
        "/sources/0/count":ids.len(), "/sources/0/retrieved_count":ids.len(),
        "/sources/1/source":"pubmed", "/sources/1/reason":"disabled",
        "/problems":{"equals":[problem]}
    });
    if kind == "bad-text" {
        expect["/result/items/0/sentence"] = json!({"omits":["text"]});
    }
    expect
}

fn merged_sentence() -> Check {
    checked_case(
        "article/litsense/combined",
        &["search", "article", NONSENSE, "--source", "pubmed,litsense"],
        |source, record| {
            if source != "pubmed" || !record["url"].as_str().unwrap().contains("esearch") {
                return;
            }
            let ids = record["exchanges"][0]["body"]["value"]["esearchresult"]["idlist"]
                .as_array_mut()
                .unwrap();
            ids.retain(|id| id != "34718584");
            ids.insert(0, json!("34718584"));
            if ids.len() > 9999 {
                ids.pop();
            }
        },
        json!(
            {"/status":"complete","/result/items/0/pmid":"34718584","/result/items/0/sentence/section":"METHODS",
            "/result/items/0/found_by/pubmed":1,"/result/items/0/found_by/litsense":1}
        ),
        0,
    )?;
    Ok(())
}
fn native_duplicates() -> Check {
    checked_case(
        "article/pubtator/pubtator-words",
        &[
            "search",
            "article",
            WORDS,
            "--source",
            "pubmed,pubtator",
            "--disable-sources",
            "pubmed",
        ],
        |source, record| {
            if source != "pubtator" {
                return;
            }
            let body = &mut record["exchanges"][0]["body"]["value"];
            if body["current"] == 1 {
                body["results"][1] = body["results"][0].clone();
            }
        },
        json!({
            "/status":"partial", "/page/total":99, "/sources/0/retrieved_count":100,
            "/result/items/0/found_by/pubtator":1, "/result/items/1/found_by/pubtator":3
        }),
        0,
    )?;
    Ok(())
}
