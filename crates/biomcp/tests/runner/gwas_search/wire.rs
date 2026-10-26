use super::*;
pub(super) fn selectors() -> Result<(), String> {
    for (args, extra) in [
        (vec!["--variant", "rs2981582"], vec![("rs_id", "rs2981582")]),
        (
            vec!["--trait", "breast carcinoma"],
            vec![("efo_trait", "breast carcinoma")],
        ),
        (
            vec!["--trait-id", "MONDO_0004989"],
            vec![("efo_id", "MONDO_0004989"), ("show_child_trait", "false")],
        ),
        (
            vec!["--trait-id", "MONDO_0004989", "--include-child-traits"],
            vec![("efo_id", "MONDO_0004989"), ("show_child_trait", "true")],
        ),
        (
            vec!["--gene", "FGFR2"],
            vec![("mapped_gene", "FGFR2"), ("extended_geneset", "false")],
        ),
        (
            vec!["-g", "FGFR2"],
            vec![("mapped_gene", "FGFR2"), ("extended_geneset", "false")],
        ),
        (
            vec!["FGFR2"],
            vec![("mapped_gene", "FGFR2"), ("extended_geneset", "false")],
        ),
        (
            vec!["--gene", "FGFR2", "--extended-geneset"],
            vec![("mapped_gene", "FGFR2"), ("extended_geneset", "true")],
        ),
        (
            vec!["--study", "GCST000035"],
            vec![("accession_id", "GCST000035")],
        ),
        (
            vec!["--pubmed", "17529967"],
            vec![("pubmed_id", "17529967")],
        ),
        (
            vec!["--gene", "FGFR2", "--trait-id", "MONDO_0004989"],
            vec![
                ("mapped_gene", "FGFR2"),
                ("extended_geneset", "false"),
                ("efo_id", "MONDO_0004989"),
                ("show_child_trait", "false"),
            ],
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let mut p = pairs();
        p.retain(|(key, _)| *key != "pubmed_id");
        p.extend(extra);
        let url = url(&p);
        let replay = seed(&env, &url, body(&url, 0, 6), 200, 0)?;
        let args = [vec!["search", "gwas"], args].concat();
        let v = call(&env, &replay, &args, 0)?;
        if v.pointer("/result/items/0/id") != Some(&json!(6110))
            || v.pointer("/query/query/order") != Some(&json!("p_value,asc"))
        {
            return Err(format!("selector lost: {v}"));
        }
    }
    Ok(())
}
pub(super) fn paging() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let first_url = url(&pairs());
    let replay = seed(&env, &first_url, body(&first_url, 0, 6), 200, 0)?;
    let mut p = pairs();
    p.retain(|(k, _)| *k != "page");
    p.push(("page", "1"));
    let second_url = url(&p);
    seed(&env, &second_url, body(&second_url, 1, 6), 200, 0)?;
    let first = call(
        &env,
        &replay,
        &["search", "gwas", "--pubmed", "17529967"],
        0,
    )?;
    if first.pointer("/next/0/args/2") != Some(&json!("association:6110"))
        || first.pointer("/next/3/kind") != Some(&json!("page"))
    {
        return Err(format!("candidate/continuation ordering: {first}"));
    }
    let args = first
        .pointer("/next/3/args")
        .and_then(Value::as_array)
        .ok_or("continuation")?
        .iter()
        .map(|v| v.as_str().ok_or("arg"))
        .collect::<Result<Vec<_>, _>>()?;
    let second = call(&env, &replay, &args, 0)?;
    if second.pointer("/result/items/0/id") != Some(&json!(6113))
        || second.pointer("/page/offset") != Some(&json!(3))
    {
        return Err(format!("native page: {second}"));
    }
    let between = call(
        &env,
        &replay,
        &["search", "gwas", "--pubmed", "17529967", "--offset", "1"],
        0,
    )?;
    if between.pointer("/result/items/0/id") != Some(&json!(6111))
        || between.pointer("/page/returned") != Some(&json!(2))
    {
        return Err("offset must skip within one native page".into());
    }
    fs::remove_dir_all(replay.join("gwas")).map_err(|e| e.to_string())?;
    call(
        &env,
        &replay,
        &["search", "gwas", "--pubmed", "17529967"],
        0,
    )?;
    call(
        &env,
        &replay,
        &["search", "gwas", "--pubmed", "17529967", "--fresh"],
        4,
    )?;
    Ok(())
}
pub(super) fn surfaces() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let url = url(&pairs());
    let replay = seed(&env, &url, body(&url, 0, 6), 200, 0)?;
    let args = ["search", "gwas", "--pubmed", "17529967"].map(str::to_owned);
    let value = super::super::mcp::call_with_globals(
        &env,
        &args,
        false,
        &env.globals(Some(&replay), false),
    )?;
    if value.pointer("/result/items/0/id") != Some(&json!(6110)) {
        return Err("MCP argument mapping".into());
    }
    let mut md = env.globals(Some(&replay), false);
    md.extend(args);
    md.extend(["--format".into(), "md".into()]);
    let out = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &md)
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8(out.stdout).map_err(|e| e.to_string())?;
    if !out.status.success()
        || !text.contains("6110")
        || !text.contains("GWAS Catalog terms")
        || !text.contains("get gwas association:6110")
    {
        return Err(format!("Markdown lost source facts: {text}"));
    }
    let help = invoke(
        &env,
        ["help", "search", "gwas"].map(str::to_owned).to_vec(),
        0,
    )?;
    let text = help.to_string();
    if !text.contains("--trait-id")
        || !text.contains("reported_trait")
        || !text.contains("--extended-geneset")
        || text.contains("--sort")
    {
        return Err("GWAS executable help drift".into());
    }
    Ok(())
}
