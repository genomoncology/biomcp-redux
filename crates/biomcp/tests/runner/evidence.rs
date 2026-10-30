//! Stored cursor and hit positions through real command record/replay boundaries.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Checked test envelopes fail at the malformed boundary"
)]
use super::{launch::Environment, response_cases::checked_steps};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    // Keep dynamic state, source bindings or wire checks that YAML cannot express.
    vec![transcript_trial(
        crate::runner::pool_contract::EVIDENCE_FRAGMENTS.0,
        || overflow().map_err(|e| e.to_string().into()),
    )]
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
