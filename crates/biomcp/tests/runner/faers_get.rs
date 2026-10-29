//! Feature adverse-event-get: synthetic exact identity and independent native sections.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        transcript_trial("adverse-event-get/age-unit-boundaries", || {
            ages().map_err(Into::into)
        }),
        transcript_trial("adverse-event-get/synthetic-sections-and-snapshot", || {
            sections().map_err(Into::into)
        }),
    ];
    trials.push(transcript_trial(
        "adverse-event-get/synthetic-complete-array-walks",
        || walks().map_err(Into::into),
    ));
    trials.push(transcript_trial(
        "adverse-event-get/synthetic-identifier-kinds",
        || identifier_kinds().map_err(Into::into),
    ));
    trials.push(transcript_trial(
        "adverse-event-get/synthetic-source-admission-before-retained-read",
        || source_policy().map_err(Into::into),
    ));
    for mode in [
        "wrong-id",
        "two-versions",
        "absent",
        "bad-drug-index",
        "missing-patient",
        "missing-drugs",
        "missing-reactions",
        "empty-collections",
    ] {
        trials.push(transcript_trial(
            format!("adverse-event-get/synthetic-{mode}"),
            move || identity(mode).map_err(Into::into),
        ));
    }
    trials
}
fn native() -> Value {
    json!(
        {"meta":{"last_updated":"2024-09-01","results":{"skip":0,"limit":5,"total":1}},"results":[{"safetyreportid":"100",
        "safetyreportversion":"1","receivedate":"20240101","receiptdate":"20240202","serious":"1","seriousnessdeath":"1",
        "seriousnesshospitalization":"1","primarysource":{"qualification":"1","reportercountry":"US"},"patient":{"patientonsetage":"42.5",
        "patientonsetageunit":"801",
        "patientsex":"2","drug":[{"medicinalproduct":"Synthetic A","drugcharacterization":"1","openfda":{"unii":["TESTA"],"rxcui":["1"],
        "product_ndc":["SYNTHETIC"],"spl_id":["SYNTHETIC-ID"],"spl_set_id":["SYNTHETIC-SET"],"application_number":["SYNTHETIC-APPLICATION"],
        "package_ndc":["A","B","C","D","E","F"]}},{"medicinalproduct":"Synthetic B","drugcharacterization":"2","openfda":{"unii":[]}},
        {"medicinalproduct":""}],"reaction":[{"reactionmeddrapt":"Synthetic reaction","reactionmeddraversionpt":"27.0",
        "reactionoutcome":"1"}]}}]}
    )
}
fn setup(mode: &str) -> Result<(Environment, Stub), String> {
    let mut body = native();
    age_input(&mut body, mode)?;
    if mode == "missing-patient" {
        body.pointer_mut("/results/0")
            .and_then(Value::as_object_mut)
            .ok_or("report")?
            .remove("patient");
    }
    for field in ["drug", "reaction"] {
        if mode == format!("missing-{field}s") || mode == "empty-collections" {
            let patient = body
                .pointer_mut("/results/0/patient")
                .and_then(Value::as_object_mut)
                .ok_or("patient")?;
            if mode == "empty-collections" {
                patient.insert(field.into(), json!([]));
            } else {
                patient.remove(field);
            }
        }
    }
    if mode == "walks" {
        let drugs: Vec<_> = (0..19)
            .map(|n| json!({"medicinalproduct":format!("Synthetic drug {n}"), "drugcharacterization":if n%2==0 {"1"}else{"2"}}))
            .collect();
        let reactions: Vec<_> = (0..22)
            .map(|n| json!({"reactionmeddrapt":format!("Synthetic reaction {n}"),"reactionoutcome":"1"}))
            .collect();
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
            if !request.starts_with(
                "GET /drug/event.json?search=safetyreportid%3A%22100%22&limit=5&skip=0&sort=receivedate%3Adesc \
                HTTP/1.1",
            ) {
                return Err(format!("wrong get: {request}"));
            }
            Ok(body.to_string())
        },
        move |_| status,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
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
    let full = serde_json::to_vec(&cli).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(&cli, "")?;
    budget(&cli, full, comparable)?;
    Ok(cli)
}
fn sections() -> Check {
    let (env, stub) = setup("valid")?;
    let card = both(&env, args(None), 0)?;
    if card.pointer("/result/drug_count") != Some(&json!(3))
        || card.pointer("/result/reaction_count") != Some(&json!(1))
        || card.pointer("/result/patient/age") != Some(&json!({"value":42.5,"unit":"801"}))
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
    let exit = if mode == "empty-collections" {
        0
    } else if mode == "absent" {
        1
    } else if mode == "bad-drug-index" {
        2
    } else {
        4
    };
    let value = both(&env, words, exit)?;
    if mode == "empty-collections" {
        if value.pointer("/result/drug_count") != Some(&json!(0))
            || value.pointer("/result/reaction_count") != Some(&json!(0))
        {
            return Err(format!("explicit empty arrays lost: {value}"));
        }
    } else if mode == "absent" {
        if value.get("status") != Some(&json!("absent")) {
            return Err(format!("absence: {value}"));
        }
    } else if mode != "bad-drug-index"
        && (value.get("result").is_some()
            || value.pointer("/sources/0/reason") != Some(&json!("bad_reply")))
    {
        return Err(format!("identity admitted: {value}"));
    }
    if matches!(mode, "wrong-id" | "two-versions")
        && value
            .get("problems")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty)
    {
        return Err("rejected identity lost its problem".into());
    }
    let expected = if matches!(mode, "bad-drug-index" | "empty-collections") {
        1
    } else {
        2
    };
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
        if expect_result && value.pointer("/result/unii/records").is_some() {
            return Err(format!("empty source list: {value}"));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("identifier sections reached HTTP".into());
    }
    Ok(())
}

fn source_policy() -> Check {
    let (env, stub) = setup("valid")?;
    let card = both(&env, args(None), 0)?;
    let snapshot = card
        .pointer("/result/snapshot")
        .and_then(Value::as_str)
        .ok_or("snapshot")?;
    for section in [None, Some("drugs"), Some("reactions"), Some("unii")] {
        let mut words = args(section);
        words.extend([
            "--snapshot".into(),
            snapshot.into(),
            "--disable-sources".into(),
            "faers".into(),
        ]);
        if section == Some("unii") {
            words.extend(["--drug-index".into(), "0".into()]);
        }
        let result = both(&env, words, 4)?;
        if result.get("result").is_some()
            || result.pointer("/sources/0/reason") != Some(&json!("disabled"))
        {
            return Err(format!("disabled source restored report: {result}"));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("disabled source reached HTTP".into());
    }
    Ok(())
}

fn budget(value: &Value, full: usize, comparable: usize) -> Check {
    let section = value
        .pointer("/query/section")
        .and_then(Value::as_str)
        .unwrap_or("card");
    let status = value.get("status").and_then(Value::as_str).unwrap_or("");
    // j13008: compact CLI/MCP maxima; every ceiling is measured bytes plus15%.
    let measured: (usize, usize) = match (section, status) {
        (_, "usage") => (143, 143),
        ("card", "complete") => (1215, 1100),
        ("card", "absent") => (298, 183),
        ("card", "failed") => (428, 313),
        ("drugs", "failed") => (419, 304),
        ("reactions", "failed") => (423, 308),
        ("unii", "failed") => (433, 318),
        ("unii", "absent") => (409, 294),
        ("drugs", "complete") => (1453, 1338),
        ("reactions", "complete") => (1403, 1288),
        ("unii", "complete") => (693, 578),
        ("rxcui", "complete") => (692, 577),
        ("product-ndc", "complete") => (718, 603),
        ("package-ndc", "complete") => (924, 809),
        ("spl-id", "complete") => (706, 591),
        ("spl-set-id", "complete") => (719, 604),
        ("application-number", "complete") => (751, 636),
        _ => return Err(format!("unmeasured FAERS envelope: {section}/{status}")),
    };
    let caps = (
        (measured.0 * 115).div_ceil(100),
        (measured.1 * 115).div_ceil(100),
    );
    if full > caps.0 || comparable > caps.1 {
        return Err(format!(
            "FAERS output exceeds measured +15% ceiling: {full}/{comparable} > {caps:?}"
        ));
    }
    Ok(())
}

fn ages() -> Check {
    legacy_age_snapshot()?;
    for code in [
        "800", "801", "802", "803", "804", "805", "absent", "999", "",
    ] {
        let (env, _stub) = setup(&format!("age-unit-{code}"))?;
        let output = both(&env, args(None), 0)?;
        let expected = match code {
            "absent" => Some(json!({"value":42.5})),
            "999" | "" => None,
            _ => Some(json!({"value":42.5,"unit":code})),
        };
        if output.pointer("/result/patient/age") != expected.as_ref() {
            return Err(format!("FAERS age changed its stated unit: {output}"));
        }
        if expected.is_none()
            && output
                .get("problems")
                .and_then(Value::as_array)
                .is_none_or(Vec::is_empty)
        {
            return Err("malformed FAERS unit lost its problem".into());
        }
    }
    let (env, _stub) = setup("age-invalid")?;
    let output = both(&env, args(None), 0)?;
    if output.pointer("/result/patient/age").is_some()
        || output
            .get("problems")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty)
    {
        return Err("nonfinite FAERS age escaped field rejection".into());
    }
    Ok(())
}

fn legacy_age_snapshot() -> Check {
    use sha2::{Digest, Sha256};
    let (env, _stub) = setup("valid")?;
    let cache = env.root().join("case-cache");
    let globals = vec![
        "--case-cache".to_owned(),
        cache.to_string_lossy().into_owned(),
    ];
    let mut initial = args(None);
    initial.extend(globals.clone());
    let card = invoke(&env, initial, 0)?;
    let snapshot = card
        .pointer("/result/snapshot")
        .and_then(Value::as_str)
        .ok_or("snapshot")?;
    let db = rusqlite::Connection::open_with_flags(
        cache.join("replay.sqlite"),
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
    )
    .map_err(|e| e.to_string())?;
    let key = Sha256::digest(b"biomcp.faers_snapshot/1\x00100").to_vec();
    let rows = db
        .execute(
            "UPDATE trial_snapshot SET request_key=?1,payload='obsolete age strings' WHERE id=?2",
            rusqlite::params![key, snapshot],
        )
        .map_err(|e| e.to_string())?;
    if rows != 1 {
        return Err("expected exactly one FAERS source snapshot".into());
    }
    let mut command = args(Some("drugs"));
    command.extend(["--snapshot".into(), snapshot.into()]);
    command.extend(globals);
    let output = invoke(&env, command, 3)?;
    if output.pointer("/next/0/kind") != Some(&json!("restart")) {
        return Err(format!(
            "old FAERS age snapshot did not restart safely: {output}"
        ));
    }
    Ok(())
}

fn age_input(body: &mut Value, mode: &str) -> Check {
    if let Some(unit) = mode.strip_prefix("age-unit-") {
        let patient = body
            .pointer_mut("/results/0/patient")
            .and_then(Value::as_object_mut)
            .ok_or("patient")?;
        if unit == "absent" {
            patient.remove("patientonsetageunit");
        } else {
            patient.insert("patientonsetageunit".into(), json!(unit));
        }
    }
    if mode == "age-invalid" {
        *body
            .pointer_mut("/results/0/patient/patientonsetage")
            .ok_or("age")? = json!("NaN");
    }
    Ok(())
}
