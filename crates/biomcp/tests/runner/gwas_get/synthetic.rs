use super::*;
pub(super) fn identity() -> Result<(), String> {
    for (args, mut value, key, replacement) in [
        (
            vec!["get", "gwas", "association:6114"],
            card(),
            "association_id",
            json!(6063),
        ),
        (
            vec!["get", "gwas", "GCST000035"],
            study(),
            "accession_id",
            json!("GCST000037"),
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        *value.get_mut(key).ok_or("identity field")? = replacement;
        let replay = seed(&env, &args, value.to_string(), 200, 0)?;
        let failed = call(&env, &replay, &args, 4)?;
        if failed.get("result").is_some()
            || failed.pointer("/sources/0/reason") != Some(&json!("bad_reply"))
            || failed
                .get("problems")
                .and_then(Value::as_array)
                .is_none_or(Vec::is_empty)
        {
            return Err(format!("wrong identity admitted: {failed}"));
        }
        let mut offline = env.globals(None, true);
        offline.extend(args.iter().map(|s| (*s).to_owned()));
        let value = invoke(&env, offline, 4)?;
        if value.pointer("/sources/0/reason") != Some(&json!("offline")) {
            return Err("wrong identity cached".into());
        }
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["get", "gwas", "association:6114"];
    let replay = seed(&env, &args, card().to_string(), 200, 0)?;
    let result = call(&env, &replay, &args, 0)?;
    let steps = result.get("next").and_then(Value::as_array).ok_or("next")?;
    if steps.iter().any(|s| {
        s.pointer("/args/2") == Some(&json!("association:6114"))
            && s.pointer("/args/3") != Some(&json!("loci"))
    }) || !steps
        .iter()
        .any(|s| s.pointer("/args/2") == Some(&json!("GCST000035")))
    {
        return Err(format!("association navigation: {result}"));
    }
    Ok(())
}
pub(super) fn ancestry() -> Result<(), String> {
    let args = [
        "get",
        "gwas",
        "GCST000035",
        "ancestries",
        "--section-limit",
        "1",
    ];
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, &args, ancestry_body().to_string(), 200, 0)?;
    let mut value = call(&env, &replay, &args, 0)?;
    fs::remove_dir_all(&replay).map_err(|e| e.to_string())?;
    let mut counts = vec![
        value
            .pointer("/result/items/0/number_of_individuals")
            .cloned()
            .ok_or("first row")?,
    ];
    for _ in 0..2 {
        let args = super::super::action(&value, "page")?;
        let mut command = env.globals(None, false);
        command.extend(args);
        value = invoke(&env, command, 0)?;
        counts.push(
            value
                .pointer("/result/items/0/number_of_individuals")
                .cloned()
                .ok_or("next row")?,
        );
    }
    if counts != vec![json!(10), json!(11), json!(12)] {
        return Err(format!("ancestry walk: {counts:?}"));
    }
    let mut disabled = env.globals(None, false);
    disabled.extend(
        [
            "get",
            "gwas",
            "GCST000035",
            "ancestries",
            "--disable-sources",
            "restricted",
        ]
        .map(str::to_owned),
    );
    let value = invoke(&env, disabled, 4)?;
    if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
        return Err("cached disabled ancestry".into());
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut rows = ancestry_body();
    *rows
        .pointer_mut("/_embedded/ancestries/1/_links/self/href")
        .ok_or("row link")? = json!(format!("{BASE}/studies/GCST000037/ancestries/1"));
    let replay = seed(&env, &args, rows.to_string(), 200, 0)?;
    let value = call(
        &env,
        &replay,
        &["get", "gwas", "GCST000035", "ancestries"],
        0,
    )?;
    if value.pointer("/status") != Some(&json!("partial"))
        || value.pointer("/page/total") != Some(&json!(2))
    {
        return Err(format!("foreign ancestry admitted: {value}"));
    }
    Ok(())
}
pub(super) fn failures() -> Result<(), String> {
    for (body, status, delay, reason) in [
        ("{}", 429, 0, "rate_limited"),
        ("{}", 500, 0, "unavailable"),
        ("broken", 200, 0, "bad_reply"),
        ("{}", 200, 16000, "deadline_exceeded"),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let args = ["get", "gwas", "association:6114"];
        let replay = seed(&env, &args, body.into(), status, delay)?;
        let value = call(&env, &replay, &args, 4)?;
        if value.pointer("/status") != Some(&json!("failed"))
            || value.get("result").is_some()
            || value.pointer("/sources/0/reason") != Some(&json!(reason))
            || value.pointer("/sources/0/terms_url").is_none()
        {
            return Err(format!("failure {reason}: {value}"));
        }
    }
    for args in [
        vec!["get", "gwas", "association:0"],
        vec!["get", "gwas", "GCST000035", "loci"],
        vec!["get", "gwas", "association:6114", "ancestries"],
        vec!["get", "gwas", "GCST000035", "--section-limit", "0"],
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let mut command = env.globals(None, false);
        command.extend(args.iter().map(|s| (*s).to_owned()));
        invoke(&env, command, 2)?;
    }
    Ok(())
}

pub(super) fn contexts() -> Result<(), String> {
    let stub=Stub::for_source("www.ebi.ac.uk","application/json",source)?;
    let live=super::super::variant::source_environment(&stub)?;
    let value=invoke(&live,vec!["get".into(),"gwas".into(),"association:6114".into(),"loci".into()],0)?;
    for id in ["ENSG00000066468","2263"] {
        if !value.get("next").and_then(Value::as_array).is_some_and(|s|s.iter().any(|s|s.pointer("/args/2")==Some(&json!(id)))) {return Err(format!("missing exact gene {id}: {value}"))}
    }
    if stub.finish()?.len()!=1 {return Err("loci performed a gene request".into())}
    let stub=Stub::for_source("www.ebi.ac.uk","application/json",source)?;
    let live=super::super::variant::source_environment(&stub)?;
    let first=invoke(&live,["get","gwas","GCST000035","associations"].map(str::to_owned).into(),0)?;
    let next=super::super::action(&first,"page")?;
    let last=invoke(&live,next,0)?;
    if first.pointer("/result/items/0/id")!=Some(&json!(6110)) || last.pointer("/result/items/0/id")!=Some(&json!(6113)) || last.pointer("/query/query/id")!=Some(&json!("GCST000035")) || last.pointer("/page/total")!=Some(&json!(6)) {return Err(format!("study source walk: {first} {last}"))}
    if stub.finish()?.len()!=2 {return Err("study walk request count".into())}
    Ok(())
}
