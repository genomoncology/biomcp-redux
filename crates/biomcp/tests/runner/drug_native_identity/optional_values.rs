use super::{Check, Environment, Stub, Value, fixture_root, fs, invoke, json};

pub(super) fn check() -> Check {
    let text = fs::read_to_string(fixture_root().join("opentargets-detail-D01-imatinib.json"))
        .map_err(|e| e.to_string())?;
    let mut changed: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    *changed
        .pointer_mut("/data/drug/name")
        .ok_or("native name")? = json!({"bad": true});
    let changed = changed.to_string();
    let mut positive = true;
    let stub = Stub::for_source(
        "api.platform.opentargets.org",
        "application/json",
        move |_| {
            if positive {
                positive = false;
                Ok(text.clone())
            } else {
                Ok(changed.clone())
            }
        },
    )?;
    let env = setup(&stub)?;
    let args = [
        "get",
        "drug",
        "CHEMBL941",
        "--source",
        "opentargets",
        "--fresh",
    ]
    .map(str::to_owned)
    .to_vec();
    let registry = super::super::schema::Registry::embedded()?;
    let control = invoke(&env, args.clone(), 0)?;
    registry.validate(&control, &args, true)?;
    if control.pointer("/result/header/id") != Some(&json!("CHEMBL941")) {
        return Err("positive Open Targets identity missing".into());
    }
    let answer = invoke(&env, args.clone(), 0)?;
    registry.validate(&answer, &args, true)?;
    if answer.pointer("/result/header/id") != Some(&json!("CHEMBL941"))
        || answer.pointer("/result/header/name").is_some()
        || answer
            .pointer("/problems")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(1)
        || answer.pointer("/problems/0/field") != Some(&json!("name"))
    {
        return Err("malformed Open Targets name lost identity or field disclosure".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("Open Targets name control did not use exactly two local replies".into());
    }
    Ok(())
}

fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}

pub(super) fn stages() -> Check {
    let text = fs::read_to_string(fixture_root().join("opentargets-detail-D01-imatinib.json"))
        .map_err(|e| e.to_string())?;
    let original: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let stages = [json!(""), json!(" \nPHASE_1\t "), json!({"bad":true})];
    let mut replies = stages.clone().into_iter().map(move |stage| {
        let mut body = original.clone();
        body["data"]["drug"]["indications"]["rows"][0]["maxClinicalStage"] = stage;
        body.to_string()
    });
    let stub = Stub::for_source(
        "api.platform.opentargets.org",
        "application/json",
        move |_| {
            replies
                .next()
                .ok_or_else(|| "unexpected stage request".into())
        },
    )?;
    let env = setup(&stub)?;
    let args = [
        "get",
        "drug",
        "CHEMBL941",
        "indications",
        "--source",
        "opentargets",
        "--fresh",
    ]
    .map(str::to_owned)
    .to_vec();
    let registry = super::super::schema::Registry::embedded()?;
    for stage in stages {
        let answer = invoke(&env, args.clone(), 0)?;
        registry.validate(&answer, &args, true)?;
        let value = answer.pointer("/result/indications/data/records/0/stage/value");
        if stage.is_string() {
            if value != Some(&stage) || answer.get("problems").is_some() {
                return Err("native stage string changed or gained typed-text validation".into());
            }
        } else if value.is_some()
            || answer.pointer("/problems/0/field") != Some(&json!("maxClinicalStage"))
            || answer["problems"].as_array().map(Vec::len) != Some(1)
        {
            return Err("malformed native stage was not disclosed independently".into());
        }
    }
    if stub.finish()?.len() != 3 {
        return Err("native stage checks changed their local request count".into());
    }
    Ok(())
}
