//! Stored cursor and hit positions through real command record/replay boundaries.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Checked test envelopes fail at the malformed boundary"
)]
use super::{launch::Environment, response_cases::checked_steps};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("article/evidence/independent-source-outcomes", || {
            independence().map_err(|e| e.to_string().into())
        }),
        Trial::test("article/evidence/fifth-page-continuation", || {
            cursor().map_err(Into::into)
        }),
        Trial::test("article/evidence/overflow-and-reference-identity", || {
            overflow().map_err(|e| e.to_string().into())
        }),
        Trial::test(
            "article/evidence/resolver-policy-before-stored-text",
            || resolver_policy().map_err(|e| e.to_string().into()),
        ),
        Trial::test("article/evidence/exact-context-fragments", || {
            exact_context().map_err(|e| e.to_string().into())
        }),
        Trial::test("article/evidence/citing-doi-fulltext", || {
            citing_doi().map_err(|e| e.to_string().into())
        }),
    ]
}
fn citing_doi() -> Result<(), Box<dyn std::error::Error>> {
    for mode in ["joined", "no-join", "no-pmc", "mismatch", "disabled"] {
        let env = Environment::new()?;
        let recording = env.root().join("recordings");
        citing_doi_capture(mode, &recording)?;
        let args = vec![
            "article".into(),
            "citation-evidence".into(),
            "10.3389/fcell.2020.573599".into(),
            "10.1038/nature05933".into(),
            "--fulltext".into(),
            "--disable-sources".into(),
            if mode == "disabled" {
                "semantic-scholar,pubmed".into()
            } else {
                "semantic-scholar".into()
            },
        ];
        let output = command(&env, &args, &env.globals(Some(&recording), false))?;
        let state = output.pointer("/result/fulltext/state");
        let valid = match mode {
            "joined" => output.pointer("/result/fulltext/data/paragraphs/total") == Some(&json!(7)),
            "no-pmc" => state == Some(&json!("not_applicable")),
            "disabled" => {
                state == Some(&json!("skipped"))
                    && output.pointer("/result/fulltext/reason") == Some(&json!("disabled"))
            }
            _ => {
                state == Some(&json!("unavailable"))
                    && output.pointer("/result/fulltext/reason") == Some(&json!("no_join_id"))
            }
        };
        if !valid {
            return Err(format!("citing DOI {mode} outcome is wrong: {output}").into());
        }
    }
    Ok(())
}
fn citing_doi_capture(
    mode: &str,
    recording: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    super::response_cases::copy_capture("article/evidence/source", recording, |source, record| {
        if source == "pubmed"
            && record.pointer("/exchanges/0/body/value/result/uids") == Some(&json!(["33123537"]))
        {
            let ids = record["exchanges"][0]["body"]["value"]["result"]["33123537"]["articleids"]
                .as_array_mut()
                .unwrap();
            if mode == "no-pmc" {
                ids.retain(|id| id["idtype"] != "pmc");
            }
            if mode == "mismatch" {
                for id in ids {
                    if id["idtype"] == "doi" {
                        id["value"] = json!("10.0000/unrelated");
                    }
                }
            }
        }
    })?;
    super::response_cases::copy_capture(
        "article/annotations/synthetic-expanded-doi",
        recording,
        |_, record| {
            if record["url"]
                .as_str()
                .is_some_and(|url| url.contains("esearch.fcgi"))
            {
                record["exchanges"][0]["body"]["value"]["esearchresult"] = if mode == "no-join" {
                    json!({"count":"0","idlist":[]})
                } else {
                    json!({"count":"1","idlist":["33123537"]})
                };
            }
        },
    )?;
    Ok(())
}
fn exact_context() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    let recording = env.root().join("recordings");
    let original = format!("a{}b", " ".repeat(7000));
    super::response_cases::copy_capture(
        "article/evidence/source",
        &recording,
        |source, record| {
            if source == "semantic-scholar" {
                for row in record["exchanges"][0]["body"]["value"]["data"]
                    .as_array_mut()
                    .unwrap()
                {
                    if row.pointer("/citedPaper/externalIds/PubMed") == Some(&json!("17611497")) {
                        row["contexts"] = json!([original]);
                    }
                }
            }
        },
    )?;
    let globals = env.globals(Some(&recording), false);
    let mut args = vec![
        "article".into(),
        "citation-evidence".into(),
        "33123537".into(),
        "17611497".into(),
    ];
    let mut joined = String::new();
    for _ in 0..4 {
        let output = command(&env, &args, &globals)?;
        let items = output
            .pointer("/result/semantic_scholar/data/contexts/items")
            .and_then(Value::as_array)
            .ok_or("source context was lost")?;
        for item in items {
            if item["character_offset"] != json!(joined.chars().count()) {
                return Err("context fragment skipped or repeated characters".into());
            }
            joined.push_str(item["text"].as_str().ok_or("missing fragment text")?);
        }
        match super::action(&output, "page") {
            Ok(next) => args = next,
            Err(_) => break,
        }
    }
    if joined != original {
        return Err("context fragments did not preserve every source character".into());
    }
    Ok(())
}
fn resolver_policy() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    let recording = env.root().join("recordings");
    super::response_cases::copy_capture(
        "article/evidence/source",
        &recording,
        |source, record| {
            if source == "semantic-scholar" {
                for row in record["exchanges"][0]["body"]["value"]["data"]
                    .as_array_mut()
                    .unwrap()
                {
                    if row.pointer("/citedPaper/externalIds/PubMed") == Some(&json!("17611497")) {
                        row["contexts"] = json!(["a".repeat(10000)]);
                    }
                }
            }
        },
    )?;
    let globals = env.globals(Some(&recording), false);
    let first = command(
        &env,
        &[
            "article".into(),
            "citation-evidence".into(),
            "33123537".into(),
            "10.1038/nature05933".into(),
            "--fulltext".into(),
        ],
        &globals,
    )?;
    if first.pointer("/result/fulltext/data/paragraphs/total") != Some(&json!(7)) {
        return Err(format!("initial resolver-backed evidence failed: {first}").into());
    }
    for source in ["pubmed", "pmc-oa"] {
        let mut next = super::action(&first, "page")?;
        next.extend(["--disable-sources".into(), source.into()]);
        let blocked = command(&env, &next, &globals)?;
        if blocked.pointer("/result/fulltext/state") == Some(&json!("data")) {
            return Err(format!("disabled {source} restored its stored text: {blocked}").into());
        }
        if blocked.get("next").is_some() {
            return Err(format!(
                "disabled {source} still offers an evidence continuation: {blocked}"
            )
            .into());
        }
    }
    Ok(())
}
fn cursor() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = super::network_stub::Stub::for_source(
        "api.semanticscholar.org",
        "application/json",
        window,
    )?;
    let root = env.root().canonicalize().map_err(|e| e.to_string())?;
    let certificate = root.join("ca.pem");
    stub.retain(&env);
    fs::write(&certificate, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":certificate,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let initial = vec![
        "article".into(),
        "citation-evidence".into(),
        "33123537".into(),
        "10.1038/nature05933".into(),
    ];
    let first = command(
        &env,
        &initial,
        &[
            "--record".into(),
            root.join("first").to_string_lossy().into(),
        ],
    )?;
    if first.pointer("/result/semantic_scholar/data/complete") != Some(&json!(false))
        || first.pointer("/result/semantic_scholar/data/source_cursor") != Some(&json!(5))
        || first
            .pointer("/result/semantic_scholar/data/notice")
            .is_some()
    {
        return Err(format!("capped scan falsely claimed absence: {first}"));
    }
    let next = super::action(&first, "page")?;
    let second = command(
        &env,
        &next,
        &[
            "--record".into(),
            root.join("second").to_string_lossy().into(),
        ],
    )?;
    if second.pointer("/result/semantic_scholar/data/complete") != Some(&json!(true))
        || second.pointer("/result/semantic_scholar/data/contexts/items/0/text")
            != Some(&json!("last page context"))
    {
        return Err(format!("continuation skipped the source cursor: {second}"));
    }
    let blocked = command(
        &env,
        &next,
        &["--disable-sources".into(), "semantic-scholar".into()],
    )?;
    if blocked.pointer("/result/semantic_scholar/state") != Some(&json!("skipped")) {
        return Err("disabled source restored stored evidence".into());
    }
    if stub.finish()?.len() != 6 {
        return Err("graph scan exceeded its bounded requests".into());
    }
    let capture = combine(&root)?;
    let replay = Environment::new().map_err(|e| e.to_string())?;
    let globals = replay.globals(Some(&capture), false);
    let first = command(&replay, &initial, &globals)?;
    let next = super::action(&first, "page")?;
    let second = command(&replay, &next, &globals)?;
    if second.pointer("/result/semantic_scholar/data/contexts/items/0/text")
        != Some(&json!("last page context"))
    {
        return Err("source cursor failed keyless replay".into());
    }
    Ok(())
}
fn command(env: &Environment, args: &[String], globals: &[String]) -> Result<Value, String> {
    let mut args = args.to_vec();
    args.extend_from_slice(globals);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.stderr.is_empty() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn window(request: &str) -> Result<String, String> {
    let offset = request
        .split("offset=")
        .nth(1)
        .and_then(|value| value.split(|ch: char| !ch.is_ascii_digit()).next())
        .unwrap_or("0")
        .parse::<u32>()
        .map_err(|e| e.to_string())?;
    let data = if offset == 5 {
        json!([{"citedPaper":{"externalIds":{"DOI":"10.1038/NATURE05933"}},"contexts":["last page context"],"intents":["background"],"isInfluential":false}])
    } else {
        json!([])
    };
    let mut reply = json!({"data":data});
    if offset < 5 {
        reply["next"] = json!(offset + 1)
    }
    Ok(reply.to_string())
}
fn overflow() -> Result<(), Box<dyn std::error::Error>> {
    checked_steps(
        "article/evidence/source",
        |source, record| {
            if source == "pmc-oa"
                && record["url"]
                    .as_str()
                    .is_some_and(|url| url.contains(".xml?"))
            {
                let body = record["exchanges"][0]["body"]["value"].as_str().unwrap();
                let start = body.find("<body>").unwrap() + 6;
                let end = body.find("</body>").unwrap();
                let replacement = format!(
                    "<sec><title>Evidence</title><p><xref ref-type=\"bibr\" rid=\"B17\">exact</xref>a{}b</p></sec>",
                    " ".repeat(7000)
                );
                record["exchanges"][0]["body"]["value"] =
                    json!(format!("{}{}{}", &body[..start], replacement, &body[end..]));
            }
        },
        json!([{"run":["article","citation-evidence","33123537","17611497","--fulltext"],"expect":{
            "/result/fulltext/data/paragraphs/total":3,
            "/result/fulltext/data/paragraphs/items/1/character_offset":3500,
            "/result/fulltext/data/paragraphs/items/1/markdown":" ".repeat(3500)
        }}]),
    )?;
    checked_steps(
        "article/evidence/source",
        |source, record| {
            if source == "semantic-scholar" {
                let rows = record["exchanges"][0]["body"]["value"]["data"]
                    .as_array_mut()
                    .unwrap();
                for row in rows {
                    if row.pointer("/citedPaper/externalIds/PubMed") == Some(&json!("17611497")) {
                        row["contexts"] = json!(["a".repeat(10000), "last context"]);
                    }
                }
            }
        },
        json!(
            [ {"run":["article","citation-evidence","33123537","17611497"],"expect":{"/result/semantic_scholar/data/contexts/total":4,
            "/result/semantic_scholar/data/contexts/items/0/character_offset":0}},{"follow":"page",
            "expect":{"/result/semantic_scholar/data/contexts/items/0/character_offset":3500}},{"follow":"page",
            "expect":{"/result/semantic_scholar/data/contexts/items/0/character_offset":7000,
            "/result/semantic_scholar/data/contexts/items/1/text":"last context"}} ]
        ),
    )?;
    checked_steps(
        "article/evidence/source",
        |source, record| {
            if source == "pmc-oa"
                && record["url"]
                    .as_str()
                    .is_some_and(|url| url.contains(".xml?"))
            {
                let body = record["exchanges"][0]["body"]["value"].as_str().unwrap();
                let start = body.find("<body>").unwrap() + 6;
                let end = body.find("</body>").unwrap();
                let replacement = format!(
                    "<sec><title>Evidence</title><list><list-item><p><xref ref-type=\"bibr\" rid=\"B17 \
                        B99\">exact</xref>{}</p></list-item></list><p><xref ref-type=\"bibr\" \
                        rid=\"B170\">decoy</xref></p></sec>",
                    "z".repeat(10000)
                );
                record["exchanges"][0]["body"]["value"] =
                    json!(format!("{}{}{}", &body[..start], replacement, &body[end..]));
            }
        },
        json!(
            [ {"run":["article","citation-evidence","33123537","17611497","--fulltext"],"expect":{"/result/fulltext/data/paragraphs/total":3,
            "/result/fulltext/data/paragraphs/items/0/position":1,"/result/fulltext/data/paragraphs/items/0/part":1}},{"follow":"page",
            "expect":{"/result/fulltext/data/paragraphs/items/0/position":1,"/result/fulltext/data/paragraphs/items/0/character_offset":7000}} ]
        ),
    )?;
    Ok(())
}

fn combine(root: &std::path::Path) -> Result<std::path::PathBuf, String> {
    let capture = root.join("all");
    fs::create_dir_all(capture.join("semantic-scholar")).map_err(|e| e.to_string())?;
    for part in ["first", "second"] {
        for entry in
            fs::read_dir(root.join(part).join("semantic-scholar")).map_err(|e| e.to_string())?
        {
            let entry = entry.map_err(|e| e.to_string())?;
            fs::copy(
                entry.path(),
                capture.join("semantic-scholar").join(entry.file_name()),
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(capture)
}

fn independence() -> Result<(), Box<dyn std::error::Error>> {
    for (mode, expect) in [
        (
            "empty",
            json!({"/status":"complete","/result/semantic_scholar/data/notice":"not_in_references"}),
        ),
        (
            "graph-failure",
            json!(
                {"/status":"partial","/result/semantic_scholar/state":"unavailable","/result/fulltext/data/paragraphs/total":7,
                "/next/0/args":["help","key","semantic-scholar"]}
            ),
        ),
        (
            "summary-failure",
            json!({"/status":"partial","/result/semantic_scholar/data/contexts/total":6,"/result/fulltext/data/paragraphs/total":7}),
        ),
        (
            "text-deadline",
            json!({"/status":"partial",
"/result/semantic_scholar/data/contexts/total":6,
"/result/fulltext/state":"unavailable",
"/result/fulltext/reason":"deadline_exceeded"}),
        ),
    ] {
        let command = if mode == "empty" {
            json!(["article", "citation-evidence", "33123537", "17611497"])
        } else {
            json!([
                "article",
                "citation-evidence",
                "33123537",
                "17611497",
                "--fulltext"
            ])
        };
        checked_steps(
            "article/evidence/source",
            |source, record| {
                if mode == "text-deadline" {
                    if source == "semantic-scholar" {
                        record["exchanges"][0]["delay_ms"] = json!(10000);
                    } else if source == "pmc-oa" {
                        record["exchanges"][0]["delay_ms"] = json!(30000);
                    }
                }
                if mode == "empty" && source == "semantic-scholar" {
                    record["exchanges"][0]["body"]["value"]["data"] = json!([]);
                }
                let fail = mode == "graph-failure" && source == "semantic-scholar"
                    || mode == "summary-failure"
                        && record.pointer("/exchanges/0/body/value/result/uids")
                            == Some(&json!(["17611497"]));
                if fail {
                    let mut reply = record["exchanges"][0].clone();
                    reply["status"] = json!(if mode == "graph-failure" { 429 } else { 500 });
                    reply["body"] = json!({"kind":"json","value":{"error":"Synthetic failure"}});
                    record["exchanges"] = json!([reply.clone(), reply.clone(), reply]);
                }
            },
            json!([{"run":command,"expect":expect}]),
        )?;
    }
    Ok(())
}
