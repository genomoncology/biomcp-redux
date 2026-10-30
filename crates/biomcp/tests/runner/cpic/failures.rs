//! Native row refusals stay distinct from protocol failures.
use super::{Check, Environment, Stub, Value, both, environment, invoke, json, rows};
fn fixture(name: &'static str) -> Result<(Vec<String>, Stub), String> {
    let (args, mut body) = rows::scenario(match name {
        "wrong-drug" => "table",
        "truncated-diplotype" => "diplotype",
        "truncated-card" => "card",
        _ => "pairs",
    })?;
    if name == "wrong-gene" {
        *body.pointer_mut("/0/genesymbol").ok_or("synthetic field")? = json!("TPMT");
    }
    if name == "wrong-drug" {
        *body.pointer_mut("/0/drugid").ok_or("synthetic field")? = json!("RxNorm:999");
    }
    if name == "unknown-field" {
        body.pointer_mut("/0")
            .and_then(Value::as_object_mut)
            .ok_or("synthetic row")?
            .insert("invented_extra".into(), json!("unreviewed"));
    }
    let stub = Stub::counted(
        move |request| {
            Ok(if name == "wrong-gene" && request.contains("offset=1") {
                let mut second = body.clone();
                *second.pointer_mut("/0/genesymbol").ok_or("second symbol")? = json!("DPYD");
                *second.pointer_mut("/0/pairid").ok_or("second identity")? = json!(2);
                second.to_string()
            } else if name == "status" {
                json!({"code":"SYNTHETIC","message":"Synthetic HTTP 200 error"}).to_string()
            } else if name == "broken" {
                "{".into()
            } else {
                body.to_string()
            })
        },
        move |request| match name {
            "wrong-gene" if request.contains("offset=1") => "Content-Range: 1-1/2\r\n".into(),
            "missing-range" => String::new(),
            "range" => "Content-Range: 4-4/5\r\n".into(),
            "count" => "Content-Range: 0-1/2\r\n".into(),
            _ => "Content-Range: 0-0/2\r\n".into(),
        },
        move |_| match name {
            "status" => 200,
            "429" => 429,
            "500" => 500,
            _ => 206,
        },
    )?;
    Ok((args, stub))
}
pub(super) fn failure(name: &'static str) -> Check {
    let (mut args, stub) = fixture(name)?;
    let env = environment(&stub)?;
    let record = env.root().join("record");
    args.splice(0..0, ["--record".into(), record.to_string_lossy().into()]);
    if matches!(name, "wrong-gene" | "wrong-drug" | "unknown-field") {
        return mapped(name, &env, stub, args);
    }
    let value = invoke(&env, args.clone(), 4)?;
    if value.get("result").is_some_and(|v| !v.is_null())
        || value.pointer("/sources/0/state") != Some(&json!("failed"))
        || value.pointer("/sources/0/terms").is_none()
    {
        return Err(format!("source failure hidden as success: {value}"));
    }
    if matches!(name, "wrong-gene" | "wrong-drug")
        && (value.pointer("/problems/0/source") != Some(&json!("cpic"))
            || value.pointer("/problems/0/field") != Some(&json!("record")))
    {
        return Err(format!("identity drop omitted its problem: {value}"));
    }
    crate::runner::pgx_retirements::bounded(&format!("CPIC {name}"), &value)?;
    let mut replay = args.clone();
    *replay.get_mut(0).ok_or("missing replay flag")? = "--replay".into();
    let replay_env = Environment::new().map_err(|e| e.to_string())?;
    let repeated = both(&replay_env, replay, 4)?;
    if repeated.get("status") != value.get("status")
        || repeated.get("sources") != value.get("sources")
    {
        return Err("recorded source failure changed".into());
    }
    let mut offline = args;
    offline.drain(0..2);
    offline.insert(0, "--offline".into());
    let unavailable = both(&env, offline, 4)?;
    if unavailable.pointer("/sources/0/reason") != Some(&json!("offline")) {
        return Err("rejected reply entered the checked cache".into());
    }
    let count = stub.finish()?.len();
    if count == 0 {
        return Err("failure never reached synthetic source".into());
    }
    Ok(())
}
fn mapped(name: &str, env: &Environment, stub: Stub, mut args: Vec<String>) -> Check {
    let value = invoke(env, args.clone(), 0)?;
    if value.pointer("/sources/0/state") != Some(&json!("ok")) {
        return Err(format!("one field rejected the native page: {value}"));
    }
    if name != "unknown-field"
        && (value.pointer("/problems/0/field") != Some(&json!("record"))
            || value.pointer("/result/items/0").is_some())
    {
        return Err(format!(
            "wrong identity leaked from the native page: {value}"
        ));
    }
    if name == "wrong-gene" {
        continue_page(env, &value)?;
    }
    *args.get_mut(0).ok_or("record switch")? = "--replay".into();
    let replay_env = Environment::new().map_err(|e| e.to_string())?;
    let repeated = both(&replay_env, args, 0)?;
    if repeated.get("result") != value.get("result")
        || repeated.get("problems") != value.get("problems")
    {
        return Err("native row filtering changed on replay".into());
    }
    stub.finish()?;
    Ok(())
}
fn continue_page(env: &Environment, value: &Value) -> Check {
    let command = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing steps")?
        .iter()
        .find(|step| step.get("kind") == Some(&json!("page")))
        .and_then(|step| step.get("args"))
        .and_then(Value::as_array)
        .ok_or("missing native page step")?
        .iter()
        .map(|arg| {
            arg.as_str()
                .map(str::to_owned)
                .ok_or("invalid page argument")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let second = invoke(env, command, 0)?;
    if second.pointer("/result/items/0/pairid/id") != Some(&json!(2))
        || second.pointer("/result/page/returned") != Some(&json!(1))
        || second.pointer("/result/page/offset") != Some(&json!(1))
    {
        return Err(format!(
            "native continuation skipped or repeated a row: {second}"
        ));
    }
    Ok(())
}
