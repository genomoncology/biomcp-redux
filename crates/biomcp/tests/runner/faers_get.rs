//! Feature adverse-event-get: synthetic exact identity and independent native sections.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![Trial::test(
        "adverse-event-get/synthetic-sections-and-snapshot",
        || sections().map_err(Into::into),
    )];
    trials.push(Trial::test(
        "adverse-event-get/synthetic-complete-array-walks",
        || walks().map_err(Into::into),
    ));
    trials.push(Trial::test(
        "adverse-event-get/synthetic-identifier-kinds",
        || identifier_kinds().map_err(Into::into),
    ));
    for mode in ["wrong-id", "two-versions", "absent", "bad-drug-index"] {
        trials.push(Trial::test(
            format!("adverse-event-get/synthetic-{mode}"),
            move || identity(mode).map_err(Into::into),
        ));
    }
    trials
}
fn native() -> Value {
    json!({"meta":{"last_updated":"2024-09-01","results":{"skip":0,"limit":5,"total":1}},"results":[{"safetyreportid":"100","safetyreportversion":"1","receivedate":"20240101","receiptdate":"20240202","serious":"1","seriousnessdeath":"1","seriousnesshospitalization":"1","primarysource":{"qualification":"1","reportercountry":"US"},"patient":{"patientonsetage":"","patientsex":"2","drug":[{"medicinalproduct":"Synthetic A","drugcharacterization":"1","openfda":{"unii":["TESTA"],"rxcui":["1"],"product_ndc":["SYNTHETIC"],"spl_id":["SYNTHETIC-ID"],"spl_set_id":["SYNTHETIC-SET"],"application_number":["SYNTHETIC-APPLICATION"],"package_ndc":["A","B","C","D","E","F"]}},{"medicinalproduct":"Synthetic B","drugcharacterization":"2","openfda":{"unii":[]}},{"medicinalproduct":""}],"reaction":[{"reactionmeddrapt":"Synthetic reaction","reactionmeddraversionpt":"27.0","reactionoutcome":"1"}]}}]})
}
fn setup(mode: &str) -> Result<(Environment, Stub), String> {
    let mut body = native();
    if mode == "walks" {
        let drugs: Vec<_> = (0..19).map(|n| json!({"medicinalproduct":format!("Synthetic drug {n}"),"drugcharacterization":if n%2==0 {"1"}else{"2"}})).collect();
        let reactions: Vec<_> = (0..22).map(|n| json!({"reactionmeddrapt":format!("Synthetic reaction {n}"),"reactionoutcome":"1"})).collect();
        *body.pointer_mut("/results/0/patient/drug").ok_or("drugs")? = json!(drugs);
        *body
            .pointer_mut("/results/0/patient/reaction")
            .ok_or("reactions")? = json!(reactions);
    }
    if mode == "wrong-id" {
        *body
            .pointer_mut("/results/0/safetyreportid")
            .ok_or("report ID")? = json!("101");
    }
    if mode == "two-versions" {
        let mut second = body.pointer("/results/0").ok_or("first report")?.clone();
        *second.get_mut("safetyreportversion").ok_or("version")? = json!("2");
        body.get_mut("results")
            .and_then(Value::as_array_mut)
            .ok_or("results")?
            .push(second);
        *body.pointer_mut("/meta/results/total").ok_or("total")? = json!(2);
    }
    if mode == "absent" {
        body = json!({"error":{"code":"NOT_FOUND","message":"No matches found!"}});
    }
    let status = if mode == "absent" { 404 } else { 200 };
    let stub = Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if !request.starts_with("GET /drug/event.json?search=safetyreportid%3A%22100%22&limit=5&skip=0&sort=receivedate%3Adesc HTTP/1.1") {return Err(format!("wrong get: {request}"));}
            Ok(body.to_string())
        },
        move |_| status,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":ca,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok((env, stub))
}
fn args(section: Option<&str>) -> Vec<String> {
    let mut args = vec!["get".into(), "adverse-event".into(), "100".into()];
    if let Some(section) = section {
        args.push(section.into());
    }
    args
}
fn both(env: &Environment, args: Vec<String>, exit: i32) -> Result<Value, String> {
    let cli = invoke(env, args.clone(), exit)?;
    let mcp = super::mcp::call_with_globals(env, &args, exit >= 2, &[])?;
    if cli != mcp {
        return Err(format!("CLI/MCP differ: {cli} {mcp}"));
    }
    super::schema::Registry::embedded()?.validate(&cli, &args, true)?;
    Ok(cli)
}
fn sections() -> Check {
    let (env, stub) = setup("valid")?;
    let card = both(&env, args(None), 0)?;
    if card.pointer("/result/drug_count") != Some(&json!(3))
        || card.pointer("/result/reaction_count") != Some(&json!(1))
        || card.pointer("/result/patient/age") != Some(&json!(""))
        || card.pointer("/result/patient/weight").is_some()
        || card.pointer("/result/latest_received") != Some(&json!("20240202"))
    {
        return Err(format!("card facts: {card}"));
    }
    let snapshot = card
        .pointer("/result/snapshot")
        .and_then(Value::as_str)
        .ok_or("snapshot")?;
    let mut drugs = args(Some("drugs"));
    drugs.extend(["--snapshot".into(), snapshot.into()]);
    let rows = both(&env, drugs, 0)?;
    if rows.pointer("/result/drugs/records/0/role") != Some(&json!("1"))
        || rows.pointer("/result/drugs/records/1/role") != Some(&json!("2"))
        || rows.pointer("/result/drugs/records/2/role").is_some()
        || rows.pointer("/result/drugs/records/1/identity_counts/unii") != Some(&json!(0))
        || rows
            .pointer("/result/drugs/records/2/identity_counts/unii")
            .is_some()
    {
        return Err(format!("independent drug fields: {rows}"));
    }
    let mut ids = args(Some("package-ndc"));
    ids.extend([
        "--drug-index".into(),
        "0".into(),
        "--snapshot".into(),
        snapshot.into(),
    ]);
    let first = both(&env, ids, 0)?;
    let next = first
        .get("next")
        .and_then(Value::as_array)
        .and_then(|v| v.iter().find(|s| s.get("kind") == Some(&json!("page"))))
        .and_then(|s| s.get("args"))
        .ok_or("identifier continuation")?;
    let next: Vec<String> = serde_json::from_value(next.clone()).map_err(|e| e.to_string())?;
    let last = both(&env, next, 0)?;
    if last.pointer("/result/package_ndc/records") != Some(&json!(["F"]))
        || last.pointer("/result/package_ndc/offset") != Some(&json!(5))
    {
        return Err(format!("identifier page: {last}"));
    }
    let mut reaction = args(Some("reactions"));
    reaction.extend(["--snapshot".into(), snapshot.into()]);
    let result = both(&env, reaction.clone(), 0)?;
    if result.pointer("/result/reactions/records/0/term/label")
        != Some(&json!("Synthetic reaction"))
        || result
            .pointer("/result/reactions/records/0/term/id")
            .is_some()
        || result.pointer("/result/reactions/records/0/outcome") != Some(&json!("1"))
    {
        return Err(format!("reaction facts: {result}"));
    }
    reaction.extend(["--disable-sources".into(), "meddra".into()]);
    let denied = invoke(&env, reaction, 4)?;
    if denied.get("result").is_some()
        || denied.pointer("/sources/0/reason") != Some(&json!("disabled"))
    {
        return Err(format!("component retained read leaked: {denied}"));
    }
    let mut expired = args(Some("drugs"));
    expired.extend(["--snapshot".into(), "0".repeat(64)]);
    let expired = invoke(&env, expired, 3)?;
    if expired.pointer("/next/0/kind") != Some(&json!("restart")) {
        return Err(format!("expired snapshot: {expired}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("retained section paging reached HTTP".into());
    }
    Ok(())
}
fn identity(mode: &str) -> Check {
    let (env, stub) = setup(mode)?;
    let mut words = args(None);
    if mode == "bad-drug-index" {
        words = args(Some("unii"));
        words.extend(["--drug-index".into(), "99".into()]);
    }
    let exit = if mode == "absent" {
        1
    } else if mode == "bad-drug-index" {
        2
    } else {
        4
    };
    let value = both(&env, words, exit)?;
    if mode == "absent" {
        if value.get("status") != Some(&json!("absent")) {
            return Err(format!("absence: {value}"));
        }
    } else if mode != "bad-drug-index"
        && (value.get("result").is_some()
            || value.pointer("/sources/0/reason") != Some(&json!("bad_reply")))
    {
        return Err(format!("identity admitted: {value}"));
    }
    let expected = if mode == "bad-drug-index" { 1 } else { 2 };
    if stub.finish()?.len() != expected {
        return Err("invalid identity entered cache".into());
    }
    Ok(())
}

fn walks() -> Check {
    let (env, stub) = setup("walks")?;
    let card = both(&env, args(None), 0)?;
    let snapshot = card
        .pointer("/result/snapshot")
        .and_then(Value::as_str)
        .ok_or("snapshot")?;
    for (section, total) in [("drugs", 19), ("reactions", 22)] {
        let mut words = args(Some(section));
        words.extend(["--snapshot".into(), snapshot.into()]);
        let mut seen = Vec::new();
        loop {
            let value = both(&env, words, 0)?;
            let rows = value
                .pointer(&format!("/result/{section}/records"))
                .and_then(Value::as_array)
                .ok_or("rows")?;
            seen.extend(
                rows.iter()
                    .map(|r| r.get("index").cloned().unwrap_or(Value::Null)),
            );
            let next = value
                .get("next")
                .and_then(Value::as_array)
                .and_then(|v| v.iter().find(|r| r.get("kind") == Some(&json!("page"))))
                .and_then(|r| r.get("args"));
            let Some(next) = next else { break };
            words = serde_json::from_value(next.clone()).map_err(|e| e.to_string())?;
            if seen.len() > total {
                return Err("array walk repeated".into());
            }
        }
        let expected: Vec<_> = (0..total).map(|n| json!(n)).collect();
        if seen != expected {
            return Err(format!("{section} native index gap or duplicate: {seen:?}"));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("array continuation reached HTTP".into());
    }
    Ok(())
}
fn identifier_kinds() -> Check {
    let (env, stub) = setup("valid")?;
    let card = both(&env, args(None), 0)?;
    let snapshot = card
        .pointer("/result/snapshot")
        .and_then(Value::as_str)
        .ok_or("snapshot")?;
    for (section, field, expected) in [
        ("unii", "unii", "TESTA"),
        ("rxcui", "rxcui", "1"),
        ("product-ndc", "product_ndc", "SYNTHETIC"),
        ("spl-id", "spl_id", "SYNTHETIC-ID"),
        ("spl-set-id", "spl_set_id", "SYNTHETIC-SET"),
        (
            "application-number",
            "application_number",
            "SYNTHETIC-APPLICATION",
        ),
    ] {
        let mut words = args(Some(section));
        words.extend([
            "--drug-index".into(),
            "0".into(),
            "--snapshot".into(),
            snapshot.into(),
        ]);
        let value = both(&env, words, 0)?;
        if value.pointer(&format!("/result/{field}/records")) != Some(&json!([expected]))
            || value.get("next").is_some()
        {
            return Err(format!("identifier rewritten or acquired pivot: {value}"));
        }
    }
    for (index, exit, expect_result) in [(1, 0, true), (2, 1, false)] {
        let mut words = args(Some("unii"));
        words.extend([
            "--drug-index".into(),
            index.to_string(),
            "--snapshot".into(),
            snapshot.into(),
        ]);
        let value = both(&env, words, exit)?;
        if value.get("result").is_some() != expect_result {
            return Err(format!("missing and empty list collapsed: {value}"));
        }
        if expect_result && value.pointer("/result/unii/records") != Some(&json!([])) {
            return Err(format!("empty source list: {value}"));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("identifier sections reached HTTP".into());
    }
    Ok(())
}
