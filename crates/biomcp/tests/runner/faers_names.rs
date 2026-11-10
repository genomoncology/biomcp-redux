//! Explicit FAERS aliases retain native counts and frozen, caller-authorized membership.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use serde_json::{Value, json};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
type Check = Result<(), String>;

pub(super) fn trials() -> Vec<libtest_mimic::Trial> {
    [
        ("faers/explicit-aliases-and-frozen-pages", complete as fn() -> Check),
        ("faers/alias-resolution-uncertainty", uncertainty),
        ("faers/alias-caller-controls-and-bounds", controls),
    ].into_iter().map(|(name, check)| super::transcript::trial(name, move || check().map_err(Into::into))).collect()
}

fn setup(mode: u8, extra: Value) -> Result<(Environment, Stub, Arc<AtomicBool>), String> {
    let no_lookup = Arc::new(AtomicBool::new(false));
    let stopped = no_lookup.clone();
    let stub = Stub::for_sources(&["mychem.info", "api.fda.gov"], |_| "application/json", move |request| {
        if request.contains("/v1/query?") {
            if stopped.load(Ordering::SeqCst) {
                return Err("a frozen continuation repeated name resolution".into());
            }
            return Ok(lookup(mode).to_string());
        }
        if !request.contains("/drug/event.json?search=") {
            return Err(format!("unexpected FAERS request: {request}"));
        }
        if request.contains("&count=") {
            return Ok(json!({"meta":{"last_updated":"2024-09-01"},"results":[{"term":"SYNTHETIC","count":2}]}).to_string());
        }
        let offset = u32::from(request.contains("skip=1"));
        Ok(json!({"meta":{"last_updated":"2024-09-01","results":{"skip":offset,"limit":1,"total":2}},
            "results":[{"safetyreportid":(100+offset).to_string(),"safetyreportversion":"1","receivedate":"20240102",
                "serious":"1","patient":{"drug":[],"reaction":[]}}]}).to_string())
    })?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    stub.retain(env.shared_root());
    let ca = env.root().join("source-ca.pem");
    std::fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca,
        "disable_sources":["chembl","openfda-label"]});
    config.as_object_mut().ok_or("config")?.extend(extra.as_object().ok_or("extra config")?.clone());
    env.config(&config).map_err(|e| e.to_string())?;
    Ok((env, stub, no_lookup))
}

fn lookup(mode: u8) -> Value {
    if mode == 3 { return json!({"total":"malformed","hits":[]}); }
    let row = |id: &str| json!({"_id":id,"drugcentral":{"name":"Typed drug","synonyms":["Typed drug","Brand name"]}});
    let hits = if mode == 2 {
        vec![row("KTUFNOKKBVMGRW-UHFFFAOYSA-N"), row("YLMAHDNUQAMNNX-UHFFFAOYSA-N")]
    } else { vec![row("KTUFNOKKBVMGRW-UHFFFAOYSA-N")] };
    json!({"total":if mode == 1 { 2 } else { hits.len() },"hits":hits})
}

fn args(expand: bool) -> Vec<String> {
    let mut words = ["search","adverse-event","Typed drug","--limit","1"].map(str::to_owned).to_vec();
    if expand { words.push("--expand-drug-names".into()); }
    words
}

fn query(value: &Value) -> Result<&str, String> {
    value.pointer("/result/source_query").and_then(Value::as_str).ok_or_else(|| format!("missing source query: {value}"))
}

fn complete() -> Check {
    let (env, stub, no_lookup) = setup(0, json!({}))?;
    let literal = invoke(&env, args(false), 0)?;
    if query(&literal)?.contains("Brand name") { return Err("literal default expanded names".into()); }
    let mut words = args(true);
    words.extend(["--reaction","Synthetic reaction","--serious","death","--received-from","2024-01","--received-to","2024-12"].map(str::to_owned));
    let first = invoke(&env, words.clone(), 0)?;
    check_expanded(&first)?;
    let mcp = super::mcp::call_with_globals(&env, &words, false, &[])?;
    if query(&mcp)? != query(&first)? || mcp.get("status") != first.get("status") { return Err("MCP expansion differs".into()); }
    let mut count = words;
    count.extend(["--aggregate".into(),"reactions".into()]);
    let counted = invoke(&env, count, 0)?;
    if query(&counted)? != query(&first)? || counted.pointer("/result/reaction_counts/items/0/count") != Some(&json!(2)) {
        return Err(format!("aggregate did not use the same native OR: {counted}"));
    }
    no_lookup.store(true, Ordering::SeqCst);
    let second = invoke(&env, super::action(&first, "page")?, 0)?;
    if query(&second)? != query(&first)? || second.pointer("/page/offset") != Some(&json!(1)) {
        return Err(format!("continuation changed resolved membership: {second}"));
    }
    let requests = stub.finish()?;
    if requests.iter().filter(|(_, line)| line.contains("/v1/query?")).count() != 1 {
        return Err("warm resolution was not retained".into());
    }
    Ok(())
}

fn check_expanded(value: &Value) -> Check {
    let expression = query(value)?;
    if value.get("status") != Some(&json!("complete")) || value.pointer("/page/total") != Some(&json!(2))
        || !expression.contains("Brand name") || !expression.contains("Typed drug")
        || !expression.contains("seriousnessdeath:1") || !expression.contains("receivedate:[20240101 TO 20241231]")
        || !value.to_string().contains("drug-name-search") || value.pointer("/query/name_snapshot").is_none()
    { return Err(format!("expanded query lost native facts or disclosure: {value}")); }
    Ok(())
}

fn uncertainty() -> Check {
    for mode in [1, 2, 3] {
        let (env, stub, _) = setup(mode, json!({}))?;
        let reply = invoke(&env, args(true), 0)?;
        if query(&reply)?.contains("Brand name") || !reply.to_string().contains("drug-name-search")
            || mode != 2 && reply.get("status") != Some(&json!("partial"))
        { return Err(format!("uncertain aliases claimed complete expansion: {reply}")); }
        let second = invoke(&env, super::action(&reply, "page")?, 0)?;
        if second.get("status") != reply.get("status") || query(&second)? != query(&reply)? {
            return Err("continuation lost incomplete or ambiguous resolution".into());
        }
        stub.finish()?;
    }
    Ok(())
}

fn controls() -> Check {
    let (env, stub, _) = setup(0, json!({"disable_sources":["mychem","chembl","openfda-label"]}))?;
    let reply = invoke(&env, args(true), 0)?;
    if query(&reply)?.contains("Brand name") || reply.get("status") != Some(&json!("partial")) {
        return Err("disabled lookup sources silently satisfied expansion".into());
    }
    if stub.finish()?.iter().any(|(_, line)| line.contains("/v1/query?")) { return Err("disabled name lookup ran".into()); }
    let (env, stub, _) = setup(0, json!({}))?;
    for source in ["vaers","maude"] {
        let mut words = args(true);
        words.extend(["--source".into(),source.into()]);
        invoke(&env, words, 2)?;
    }
    let first = invoke(&env, args(true), 0)?;
    let mut changed = super::action(&first, "page")?;
    changed.extend(["--reaction".into(),"Changed filter".into()]);
    let refused = invoke(&env, changed, 2)?;
    if refused.to_string().contains("Brand name") { return Err("snapshot escaped its selectors".into()); }
    stub.finish()?;
    Ok(())
}
