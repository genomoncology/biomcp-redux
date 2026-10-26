use super::*;
pub(super) fn features() -> Result<(), String> {
    let stub = Stub::for_source("rest.uniprot.org", "application/json", |_| {
        Ok(entry().to_string())
    })?;
    let env = setup(&stub)?;
    let first = call(
        &env,
        None,
        &["protein", "features", "P00533", "--limit", "1"],
        0,
    )?;
    let pool = first
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or("feature pool")?;
    for (position, expected) in [(9, 0), (10, 1), (20, 1), (21, 0)] {
        let value = call(
            &env,
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
        if value.pointer("/page/total") != Some(&json!(expected)) {
            return Err(format!(
                "inclusive native coordinate or sequence identity changed: {value}"
            ));
        }
    }
    let next = super::super::action(&first, "page")?;
    let words: Vec<_> = next.iter().map(String::as_str).collect();
    let page = call(&env, None, &words, 0)?;
    if page.pointer("/result/items/0/feature/kind/label") != Some(&json!("Region"))
        || page.pointer("/page/pool/id").and_then(Value::as_str) != Some(pool)
    {
        return Err("feature continuation lost source order or retained identity".into());
    }
    let all = call(&env, None, &["get", "protein", "P00533", "all"], 0)?;
    if all.pointer("/result/items/0/sections/function/data/items/1/molecule")
        != Some(&json!("Isoform 2"))
        || all.pointer("/result/items/0/sections/isoforms/data/items/0/ids/0")
            != Some(&json!("Q8N726"))
        || all
            .pointer("/result/items/0/sections/structures/data/items/0/properties")
            .is_some()
    {
        return Err("native section facts were dropped or invented".into());
    }
    let past = call(
        &env,
        None,
        &["protein", "features", "P00533", "--position", "31"],
        4,
    )?;
    if past.pointer("/problems/0/field") != Some(&json!("position")) || past.get("result").is_some()
    {
        return Err("out-of-range position was admitted".into());
    }
    for args in [
        vec!["protein", "features", "P00533", "--position", "0"],
        vec!["protein", "features", "P00533-2", "--position", "10"],
    ] {
        call(&env, None, &args, 2)?;
    }
    if stub.finish()?.len() != 1 {
        return Err("retained entry sections repeated provider requests".into());
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
        super::super::response_cases::checked_steps(
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
