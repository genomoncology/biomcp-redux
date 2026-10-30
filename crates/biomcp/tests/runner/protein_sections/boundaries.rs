use super::{Environment, Stub, Value, call, entry, json, record, setup};
pub(super) fn features() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["rest.uniprot.org", "www.ebi.ac.uk"],
        |_| "application/json",
        |request| {
            if request.contains("/interpro/") {
                Ok(json!({"count":0,"next":null,"results":[]}).to_string())
            } else if request.contains("/complex-ws/") {
                Ok(json!({"size":0,"totalNumberOfResults":0,"elements":[]}).to_string())
            } else {
                Ok(entry().to_string())
            }
        },
    )?;
    let env = setup(&stub)?;
    let first = call(
        &env,
        None,
        &["protein", "features", "P00533", "--limit", "1"],
        0,
    )?;
    super::super::protein_budgets::check(&first, "section-boundary/first", "cli")?;
    coordinates(&env)?;
    retained(&env, &first)?;
    section_context(&env)?;
    usage(&env)?;
    if stub.finish()?.len() != 3 {
        return Err("retained entry sections repeated provider requests".into());
    }
    Ok(())
}
fn coordinates(env: &Environment) -> Result<(), String> {
    for (position, expected) in [(9, 0), (10, 1), (20, 1), (21, 0)] {
        let value = call(
            env,
            None,
            &[
                "protein",
                "features",
                "P00533",
                "--type",
                "Domain",
                "--position",
                &position.to_string(),
            ],
            0,
        )?;
        super::super::protein_budgets::check(
            &value,
            &format!("section-boundary/position-{position}"),
            "cli",
        )?;
        if value.pointer("/page/total") != Some(&json!(expected)) {
            return Err(format!(
                "inclusive native coordinate or sequence identity changed: {value}"
            ));
        }
    }
    let uncertain = call(
        env,
        None,
        &[
            "protein",
            "features",
            "P00533",
            "--type",
            "Region",
            "--position",
            "15",
        ],
        0,
    )?;
    super::super::protein_budgets::check(&uncertain, "section-boundary/uncertain", "cli")?;
    if uncertain.pointer("/page/total") != Some(&json!(1)) {
        return Err("uncertain containment was guessed or a proved interval was lost".into());
    }
    Ok(())
}
fn retained(env: &Environment, first: &Value) -> Result<(), String> {
    let pool = first
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or("feature pool")?;
    let mismatch = call(
        env,
        None,
        &[
            "protein",
            "structures",
            "P00533",
            "--limit",
            "1",
            "--pool",
            pool,
        ],
        2,
    )?;
    super::super::protein_budgets::check(&mismatch, "section-boundary/mismatched-pool", "cli")?;
    if mismatch.pointer("/error/code") != Some(&json!("usage.pool_mismatch")) {
        return Err("section pool accepted another selector".into());
    }
    call(
        env,
        None,
        &[
            "protein", "features", "P00533", "--limit", "1", "--pool", pool, "--offset", "100",
        ],
        3,
    )?;
    let next = super::super::action(first, "page")?;
    let words: Vec<_> = next.iter().map(String::as_str).collect();
    let page = call(env, None, &words, 0)?;
    super::super::protein_budgets::check(&page, "section-boundary/next", "cli")?;
    if page.pointer("/result/items/0/feature/kind/label") != Some(&json!("Region"))
        || page.pointer("/page/pool/id").and_then(Value::as_str) != Some(pool)
    {
        return Err("feature continuation lost source order or retained identity".into());
    }
    Ok(())
}
fn section_context(env: &Environment) -> Result<(), String> {
    let all = call(env, None, &["get", "protein", "P00533", "all"], 0)?;
    super::super::protein_budgets::check(&all, "section-boundary/all", "cli")?;
    if all.pointer("/result/items/0/sections/structures/data/items/0/id") != Some(&json!("1ABC"))
        || all.pointer("/result/items/0/sections/function/data/items/1/molecule")
            != Some(&json!("Isoform 2"))
        || all.pointer("/result/items/0/sections/isoforms/data/items/0/ids/0")
            != Some(&json!("Q8N726"))
        || all
            .pointer("/result/items/0/sections/structures/data/items/0/properties")
            .is_some()
    {
        return Err("native section facts were dropped or invented".into());
    }
    Ok(())
}
fn usage(env: &Environment) -> Result<(), String> {
    let past = call(
        env,
        None,
        &["protein", "features", "P00533", "--position", "31"],
        4,
    )?;
    super::super::protein_budgets::check(&past, "section-boundary/past-length", "cli")?;
    if past.pointer("/problems/0/field") != Some(&json!("position")) || past.get("result").is_some()
    {
        return Err("out-of-range position was admitted".into());
    }
    for args in [
        vec!["protein", "features", "P00533", "--position", "0"],
        vec!["protein", "features", "P00533-2", "--position", "10"],
    ] {
        call(env, None, &args, 2)?;
    }
    Ok(())
}
pub(super) fn sequence() -> Result<(), String> {
    let stub = Stub::for_source("rest.uniprot.org", "text/plain", |_| {
        Ok(
            ">sp|P38398-2|SYNTHETIC_HUMAN Synthetic OS=Homo sapiens OX=9606 GN=SYN\nACDEFG\n"
                .into(),
        )
    })?;
    let env = setup(&stub)?;
    let directory = env.root().join("sequence-recording");
    let value = record(&env, &["protein", "sequence", "P38398-2"], &directory, 0)?;
    super::super::protein_budgets::check(&value, "sequence-admission/valid", "cli")?;
    if value.pointer("/result/sequence/length") != Some(&json!(6))
        || value.pointer("/result/sequence/audit") != Some(&json!({}))
    {
        return Err("alternative sequence inherited displayed audit or residues".into());
    }
    stub.finish()?;
    for replacement in [
        ">sp|P38398|SYNTHETIC_HUMAN Synthetic OX=9606 GN=SYN\nACDEFG\n",
        ">sp|P38398-2|SYNTHETIC_HUMAN Synthetic OX=9606 GN=SYN\nACD!\n",
    ] {
        super::super::protein_budgets::steps(
            if replacement.contains("ACD!") {
                "sequence-admission/invalid-alphabet"
            } else {
                "sequence-admission/wrong-accession"
            },
            directory.to_str().ok_or("capture directory")?,
            |_, recording| {
                if let Some(exchange) = recording.pointer_mut("/exchanges/0/body") {
                    *exchange = json!({"kind":"text","value":replacement});
                }
            },
            json!([{"run":["protein","sequence","P38398-2"],"exit":4,
            "expect":{"/status":"failed","/problems/0/code":"value.unknown"}},
            {"run":["protein","sequence","P38398-2"],"offline":true,"exit":4,
            "expect":{"/sources/0/reason":"offline"}}]),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

tool_only! {
pub(super) fn feature_id() -> Result<(), String> {
    let stub = Stub::for_source("rest.uniprot.org", "application/json", |_| {
        let mut value = entry();
        value
            .pointer_mut("/features/0")
            .and_then(Value::as_object_mut)
            .ok_or("feature row")?
            .insert("featureId".into(), json!(""));
        Ok(value.to_string())
    })?;
    let env = setup(&stub)?;
    let answer = call(&env, None, &["protein", "features", "P00533"], 0)?;
    if answer.pointer("/result/items/0/feature/protein") != Some(&json!("P00533"))
        || answer.pointer("/result/items/0/feature/id").is_some()
        || answer.pointer("/problems/0/field") != Some(&json!("features.featureId"))
        || answer
            .get("problems")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(1)
    {
        return Err("bad feature ID lost the coordinate identity or its field problem".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("feature ID disclosure repeated the local request".into());
    }
    Ok(())
}
}
