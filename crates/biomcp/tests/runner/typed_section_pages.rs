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
                move || {
                    check(catalog)?;
                    candidate_pages(catalog)?;
                    drug_label_page(catalog).map_err(Into::into)
                },
            )
        })
        .collect()
}

fn check(catalog: &str) -> Result<(), String> {
    let (env, globals) = replay(catalog, "variant/typed-section-pages", &[])?;
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
        let last = follow(&env, &globals, step)?;
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

fn candidate_pages(catalog: &str) -> Result<(), String> {
    let (env, globals) = replay(catalog, "paging/variant", &[])?;
    let mut page = call(
        &env,
        &globals,
        json!({"name":"variant","arguments":{"action":"get","get":{
            "id":"rs113488022","section":"clinvar","candidate_limit":1
        }}}),
    )?;
    for offset in 0..3 {
        expect(&page, "/page/offset", &json!(offset))?;
        expect(&page, "/page/returned", &json!(1))?;
        expect(&page, "/query/section_offset", &json!(0))?;
        let pages = page_steps(&page);
        let next = pages
            .iter()
            .find(|step| step.pointer("/arguments/action").and_then(Value::as_str) == Some("get"));
        if offset < 2 {
            page = follow(
                &env,
                &globals,
                next.ok_or("missing candidate continuation")?,
            )?;
        } else if next.is_some() {
            return Err("last candidate page offered another candidate continuation".into());
        }
    }
    Ok(())
}

fn drug_label_page(catalog: &str) -> Result<(), String> {
    let (env, globals) = replay(
        catalog,
        "drug/label-discovery",
        &["--disable-sources", "mychem,chembl"],
    )?;
    let first = call(
        &env,
        &globals,
        json!({"name":"drug","arguments":{"action":"get","get":{
            "id":"98b197ea-bca4-427e-8737-97f70a3262bc","source":"openfda-label","section":"label","section_limit":1,
            "label_section":["spl_product_data_elements","package_label_principal_display_panel","warnings_and_cautions"]
        }}}),
    )?;
    let part = first
        .pointer("/result/parts/1/reply")
        .ok_or("second label part")?;
    expect(part, "/page/offset", &json!(0))?;
    let pages = page_steps(part);
    let [step] = pages.as_slice() else {
        return Err(format!("expected one label continuation, got {pages:?}"));
    };
    let last = follow(&env, &globals, step)?;
    expect(&last, "/page/offset", &json!(1))?;
    expect(
        &last,
        "/result/label_section",
        &json!("package_label_principal_display_panel"),
    )?;
    if !page_steps(&last).is_empty() {
        return Err("last label page offered another continuation".into());
    }
    Ok(())
}

fn replay(catalog: &str, path: &str, extra: &[&str]) -> Result<(Environment, Vec<String>), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut globals = env.globals(Some(&fixtures().join("replies").join(path)), false);
    globals.extend(["--tool-mode", "typed", "--catalog", catalog].map(str::to_owned));
    globals.extend(extra.iter().map(|arg| (*arg).to_owned()));
    Ok((env, globals))
}

fn follow(env: &Environment, globals: &[String], step: &Value) -> Result<Value, String> {
    let name = step.get("name").ok_or("continuation tool name")?;
    let arguments = step.get("arguments").ok_or("continuation arguments")?;
    call(env, globals, json!({"name":name,"arguments":arguments}))
}

fn page_steps(reply: &Value) -> Vec<&Value> {
    reply
        .get("next")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|step| step.get("kind").and_then(Value::as_str) == Some("page"))
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
