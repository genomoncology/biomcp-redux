use super::*;
pub(super) fn complete() -> Check {
    let (env, stub) = setup(0, json!({}))?;
    let literal = invoke(&env, args(false), 0)?;
    if literal.pointer("/page/total") != Some(&json!(1)) {
        return Err(format!("literal changed {literal}"));
    }
    let mut words = args(true);
    words.extend(["--indication".into(), "Synthetic indication".into()]);
    let first = invoke(&env, words.clone(), 0)?;
    let mcp = crate::runner::mcp::call_with_globals(&env, &words, false, &[])?;
    for value in [&first, &mcp] {
        if value.pointer("/page/total") != Some(&json!(2))
            || !selector(value)?.contains("Brand name")
            || !selector(value)?.contains("indications_and_usage:\"Synthetic indication\"")
            || value.pointer("/query/name_snapshot").is_none()
            || !value.to_string().contains("drug-name-search")
        {
            return Err(format!(
                "expanded native union or disclosure missing {value}"
            ));
        }
    }
    let second = invoke(&env, crate::runner::action(&first, "page")?, 0)?;
    if selector(&first)? != selector(&second)?
        || second.pointer("/page/offset") != Some(&json!(1))
        || first.pointer("/result/items/0/id") == second.pointer("/result/items/0/id")
        || second
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|steps| steps.iter().any(|s| s.get("kind") == Some(&json!("page"))))
    {
        return Err(format!(
            "native identity or exhausted page changed {second}"
        ));
    }
    snapshots(&env, &first)?;
    stub.finish()?;
    Ok(())
}
fn snapshots(env: &Environment, first: &Value) -> Check {
    let next = crate::runner::action(first, "page")?;
    let index = next
        .iter()
        .position(|s| s == "--name-snapshot")
        .ok_or("snapshot")?;
    let mut missing = next.clone();
    missing.drain(index..=index + 1);
    invoke(env, missing, 2)?;
    let mut bad = next.clone();
    *bad.get_mut(index + 1).ok_or("value")? = "synthetic-missing".into();
    let failed = invoke(env, bad, 3)?;
    crate::runner::action(&failed, "restart")?;
    for disabled in ["mychem", "drugcentral", "openfda-label"] {
        let mut bad = next.clone();
        bad.extend(["--disable-sources".into(), disabled.into()]);
        let failed = invoke(env, bad, 3)?;
        crate::runner::action(&failed, "restart")?;
    }
    let mut changed = next;
    changed.extend(["--indication".into(), "Changed".into()]);
    let failed = invoke(env, changed, 3)?;
    crate::runner::action(&failed, "restart")?;
    for input in ["NDA202155", "095a08ac-cf0e-497e-a682-ddef38d6b29c"] {
        let mut native = args(true);
        *native.get_mut(2).ok_or("input")? = input.into();
        invoke(env, native, 2)?;
    }
    let mut bad = args(false);
    bad.extend(["--name-snapshot".into(), "synthetic-missing".into()]);
    invoke(env, bad, 2)?;
    for source in ["mychem", "drugsfda"] {
        let mut bad = args(true);
        bad.extend(["--source".into(), source.into()]);
        invoke(env, bad, 2)?;
    }
    Ok(())
}
pub(super) fn controls() -> Check {
    for mode in [1, 2, 3, 4, 5, 6] {
        let (env, stub) = setup(mode, json!({}))?;
        let value = invoke(&env, args(true), if mode == 6 { 4 } else { 0 })?;
        if mode == 6 && value.get("status") != Some(&json!("failed")) {
            return Err("lookup hid primary failure".into());
        }
        if mode != 6 {
            let expression = selector(&value)?;
            if expression.len() > 4096
                || mode == 4 && expression.contains("Long alias")
                || [1, 2, 3].contains(&mode) && expression.contains("Brand name")
                || [1, 3].contains(&mode) && value.get("status") != Some(&json!("partial"))
                || mode == 5 && !expression.contains(r#"Brand \"quoted\" \\name"#)
            {
                return Err(format!("uncertainty or bound dishonest {value}"));
            }
        }
        stub.finish()?;
    }
    let (env, stub) = setup(0, json!({"drug_search_name_limit":1}))?;
    let value = invoke(&env, args(true), 0)?;
    if selector(&value)?.contains("Brand name") {
        return Err("configured name cap ignored".into());
    }
    stub.finish()?;
    let (env, stub) = setup(0, json!({"disable_sources":["mychem","chembl"]}))?;
    let value = invoke(&env, args(true), 0)?;
    if selector(&value)?.contains("Brand name") {
        return Err("disabled discovery supplied names".into());
    }
    stub.finish()?;
    Ok(())
}
pub(super) fn tier() -> Check {
    let (env, stub) = setup(7, json!({}))?;
    let first = invoke(&env, args(true), 0)?;
    if selector(&first)?.contains(".exact") {
        return Err("empty exact tier did not advance".into());
    }
    let mut next = crate::runner::action(&first, "page")?;
    next.push("--fresh".into());
    let second = invoke(&env, next, 0)?;
    if selector(&first)? != selector(&second)? {
        return Err("selected native tier changed".into());
    }
    let requests = stub.finish()?;
    if requests
        .iter()
        .filter(|(_, s)| s.contains("/v1/query?"))
        .count()
        != 1
    {
        return Err("continuation re-resolved names".into());
    }
    Ok(())
}
