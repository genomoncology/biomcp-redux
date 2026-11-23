//! Follow independent variant sections and native claim streams to their last pages.
use crate::runner::{
    launch::{Environment, fixtures},
    mcp::{exchange_with_globals, request},
    transcript::trial,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    ["compact", "full"]
        .into_iter()
        .map(|catalog| {
            trial(
                format!("mcp-parity/typed-section-pages-{catalog}"),
                move || check(catalog).map_err(Into::into),
            )
        })
        .collect()
}

fn check(catalog: &str) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut globals = env.globals(
        Some(&fixtures().join("replies/variant/typed-section-pages")),
        false,
    );
    globals.extend(["--tool-mode", "typed", "--catalog", catalog].map(str::to_owned));
    let first = call(
        &env,
        &globals,
        json!({"name":"variant","arguments":{"action":"get","get":{"id":"CA123643","section":["civic","hotspots"]}}}),
    )?;
    let sections = "/result/candidates/0/sections";
    let civic = format!("{sections}/civic/data");
    let hotspots = format!("{sections}/hotspots/data");
    expect(
        &first,
        &format!("{civic}/profiles/0/record/id"),
        &json!("120"),
    )?;
    expect(
        &first,
        &format!("{hotspots}/records/0/residue"),
        &json!("V600"),
    )?;
    let pages = page_steps(&first);
    if pages.len() != 4 {
        return Err(format!(
            "expected four independent continuations, got {pages:?}"
        ));
    }
    for (section, selector, position, row, expected) in [
        (
            "civic",
            "section_offset",
            json!(1),
            "profiles/0/record/id",
            json!("121"),
        ),
        (
            "hotspots",
            "section_offset",
            json!(3),
            "records/0/residue",
            json!("G469"),
        ),
        (
            "civic",
            "civic_kind",
            json!("evidence"),
            "profiles/0/statements/0/record/id",
            json!("EID1411"),
        ),
        (
            "civic",
            "civic_kind",
            json!("assertions"),
            "profiles/0/statements/0/record/id",
            json!("AID8"),
        ),
    ] {
        let step = pages
            .iter()
            .find(|step| step.pointer(&format!("/arguments/get/{selector}")) == Some(&position))
            .ok_or_else(|| {
                format!("missing {section} continuation with {selector}={position}: {pages:?}")
            })?;
        let last = call(
            &env,
            &globals,
            json!({"name":step["name"],"arguments":step["arguments"]}),
        )?;
        let base = format!("{sections}/{section}/data");
        expect(&last, &format!("{base}/{row}"), &expected)?;
        if selector == "section_offset" {
            expect(&last, &format!("{base}/page/offset"), &position)?;
        }
        let next = page_steps(&last);
        if !next.is_empty() {
            return Err(format!(
                "{section} last page offered further continuations: {next:?}"
            ));
        }
    }
    Ok(())
}

fn page_steps(reply: &Value) -> Vec<&Value> {
    reply["next"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|step| step["kind"] == "page")
        .collect()
}

fn call(env: &Environment, globals: &[String], params: Value) -> Result<Value, String> {
    let replies = exchange_with_globals(env, &[request(json!(1), "tools/call", params)], globals)?;
    replies
        .first()
        .and_then(|reply| reply.pointer("/result/structuredContent"))
        .cloned()
        .ok_or_else(|| format!("missing structured reply: {replies:?}"))
}

fn expect(reply: &Value, path: &str, expected: &Value) -> Result<(), String> {
    let actual = reply.pointer(path);
    if actual != Some(expected) {
        return Err(format!("{path} should be {expected}, got {actual:?}"));
    }
    Ok(())
}
