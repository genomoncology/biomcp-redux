//! Source failures use synthetic HTTP and recorded deadlines, never live faults.
use super::{
    Check, Environment, Stub, Trial, Value, args, both, environment, invoke, json, native,
};
use crate::runner::transcript::trial as transcript_trial;
pub(super) fn trials() -> Vec<Trial> {
    let mut out = vec![];
    for name in [
        "429",
        "500",
        "broken",
        "status",
        "wrong-accession",
        "wrong-gene",
        "unknown-field",
        "unknown-nested-field",
    ] {
        out.push(transcript_trial(
            format!("clinpgx-annotations/reject-{name}"),
            move || failure(name).map_err(Into::into),
        ));
    }
    out.push(transcript_trial(
        "clinpgx-annotations/recorded-timeout",
        || timeout().map_err(Into::into),
    ));
    out.push(transcript_trial(
        "clinpgx-annotations/exclusive-selectors",
        || selectors().map_err(Into::into),
    ));
    out.push(transcript_trial(
        "clinpgx-annotations/missing-snapshot-no-http",
        || missing_snapshot().map_err(Into::into),
    ));
    out
}
fn failure(name: &'static str) -> Check {
    let shape = if name == "wrong-accession" {
        "annotation"
    } else {
        "clinical"
    };
    let mut body = native(shape)?;
    if name == "wrong-accession" {
        *body
            .pointer_mut("/data/accessionId")
            .ok_or("synthetic identity")? = json!("PA999999");
    }
    if name == "wrong-gene" {
        *body
            .pointer_mut("/data/0/location/genes/0/symbol")
            .ok_or("synthetic symbol")? = json!("TP53");
    }
    if name == "status" {
        *body.get_mut("status").ok_or("status")? = json!("error");
    }
    if matches!(name, "unknown-field" | "unknown-nested-field") {
        let path = if name == "unknown-nested-field" {
            "/data/0/location/genes/0"
        } else {
            "/data/0"
        };
        body.pointer_mut(path)
            .and_then(Value::as_object_mut)
            .ok_or("row")?
            .insert("unknown_field".into(), json!(true));
    }
    let stub = Stub::for_source_status(
        "api.clinpgx.org",
        "application/json",
        move |_| {
            Ok(if name == "broken" {
                "{".into()
            } else {
                body.to_string()
            })
        },
        move |_| match name {
            "429" => 429,
            "500" => 500,
            _ => 200,
        },
    )?;
    let env = environment(&stub)?;
    let record = env.root().join("record");
    let mut command = vec!["--record".into(), record.to_string_lossy().into()];
    command.extend(args(shape));
    let value = invoke(&env, command.clone(), 4)?;
    if value.get("result").is_some_and(|v| !v.is_null())
        || value.pointer("/sources/0/state") != Some(&json!("failed"))
        || value.pointer("/sources/0/terms").is_none()
    {
        return Err("source failure became a negative fact".into());
    }
    if matches!(name, "wrong-accession" | "wrong-gene")
        && (value.pointer("/problems/0/source") != Some(&json!("clinpgx"))
            || value.pointer("/problems/0/field") != Some(&json!("record")))
    {
        return Err(format!("identity drop omitted its problem: {value}"));
    }
    super::super::pgx_retirements::bounded(&format!("ClinPGx {name}"), &value)?;
    *command.get_mut(0).ok_or("record switch")? = "--replay".into();
    let fresh = Environment::new().map_err(|e| e.to_string())?;
    let replay = both(&fresh, command, 4)?;
    if replay.get("sources") != value.get("sources") {
        return Err("recorded failure changed".into());
    }
    let mut offline = vec!["--offline".into()];
    offline.extend(args(shape));
    if both(&env, offline, 4)?.pointer("/sources/0/reason") != Some(&json!("offline")) {
        return Err("rejected response entered cache".into());
    }
    stub.finish()?;
    Ok(())
}

fn selectors() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for extra in [
        vec!["--gene", "DPYD"],
        vec!["annotations"],
        vec!["--kind", "clinical"],
        vec!["--section-offset", "1"],
        vec!["--source", "cpic"],
    ] {
        let mut command = args("annotation");
        command.extend(extra.into_iter().map(str::to_owned));
        both(&env, command, 2)?;
    }
    Ok(())
}
fn timeout() -> Check {
    let body = native("annotation")?;
    let stub = Stub::for_sources(
        &["api.clinpgx.org"],
        |_| "application/json",
        move |_| Ok(body.to_string()),
    )?;
    let env = environment(&stub)?;
    let record = env.root().join("record");
    let mut command = vec!["--record".into(), record.to_string_lossy().into()];
    command.extend(args("annotation"));
    invoke(&env, command, 0)?;
    for entry in std::fs::read_dir(record.join("clinpgx")).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }
        let mut recording: Value =
            serde_json::from_slice(&std::fs::read(&path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        *recording
            .pointer_mut("/exchanges/0/delay_ms")
            .ok_or("recorded delay")? = json!(16000);
        std::fs::write(
            path,
            serde_json::to_vec(&recording).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    let fresh = Environment::new().map_err(|e| e.to_string())?;
    let mut replay = vec!["--replay".into(), record.to_string_lossy().into()];
    replay.extend(args("annotation"));
    let v = both(&fresh, replay, 4)?;
    if v.pointer("/sources/0/reason") != Some(&json!("deadline_exceeded"))
        || v.get("result").is_some_and(|v| !v.is_null())
    {
        return Err("timeout became negative fact".into());
    }
    super::super::pgx_retirements::bounded("ClinPGx timeout", &v)?;
    if stub.finish()?.len() != 1 {
        return Err("recorded timeout reached provider".into());
    }
    Ok(())
}

fn missing_snapshot() -> Check {
    let stub = Stub::for_sources(
        &["api.clinpgx.org"],
        |_| "application/json",
        |_| Err("snapshot continuation reached HTTP".into()),
    )?;
    let env = environment(&stub)?;
    let mut command = args("clinical");
    command.extend(["--snapshot".into(), "0".repeat(64)]);
    let v = both(&env, command, 3)?;
    if v.pointer("/error/code") != Some(&json!("pool.expired"))
        || v.pointer("/next/0/kind") != Some(&json!("restart"))
    {
        return Err("missing snapshot did not offer explicit restart".into());
    }
    if !stub.finish()?.is_empty() {
        return Err("missing snapshot reached HTTP".into());
    }
    Ok(())
}
