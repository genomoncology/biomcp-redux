use super::{Environment, Value, body, call, gwas_budgets, json, pairs, seed, url};
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
            vec!["--pubmed", "17529967", "-l", "3"],
            vec![("pubmed_id", "17529967")],
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
        let args = [vec!["search", "gwas"], args].concat();
        let replay = seed(&env, &args, body(&url, 0, 6), 200, 0)?;
        let v = call("gwas_search/wire/selectors/1", &env, &replay, &args, 0)?;
        if v.pointer("/result/items/0/id") != Some(&json!("association:6110"))
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
    let replay = seed(
        &env,
        &["search", "gwas", "--pubmed", "17529967"],
        body(&first_url, 0, 6),
        200,
        0,
    )?;
    let mut p = pairs();
    p.retain(|(k, _)| *k != "page");
    p.push(("page", "1"));
    let second_url = url(&p);
    seed(
        &env,
        &["search", "gwas", "--pubmed", "17529967", "--offset", "3"],
        body(&second_url, 1, 6),
        200,
        0,
    )?;
    let first = call(
        "gwas_search/wire/paging/1",
        &env,
        &replay,
        &["search", "gwas", "--pubmed", "17529967"],
        0,
    )?;
    if first.pointer("/next/0/kind") != Some(&json!("page"))
        || first
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|steps| {
                steps.iter().any(|step| {
                    step.pointer("/args/1") == Some(&json!("gwas"))
                        && step.pointer("/args/0") == Some(&json!("get"))
                })
            })
    {
        return Err(format!("candidate/continuation ordering: {first}"));
    }
    let args = first
        .pointer("/next/0/args")
        .and_then(Value::as_array)
        .ok_or("continuation")?
        .iter()
        .map(|v| v.as_str().ok_or("arg"))
        .collect::<Result<Vec<_>, _>>()?;
    let second = call("gwas_search/wire/paging/2", &env, &replay, &args, 0)?;
    if second.pointer("/result/items/0/id") != Some(&json!("association:6113"))
        || second.pointer("/page/offset") != Some(&json!(3))
    {
        return Err(format!("native page: {second}"));
    }
    let between = call(
        "gwas_search/wire/paging/3",
        &env,
        &replay,
        &["search", "gwas", "--pubmed", "17529967", "--offset", "1"],
        0,
    )?;
    if between.pointer("/result/items/0/id") != Some(&json!("association:6111"))
        || between.pointer("/page/returned") != Some(&json!(2))
    {
        return Err("offset must skip within one native page".into());
    }
    Ok(())
}
pub(super) fn surfaces() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let url = url(&pairs());
    let replay = seed(
        &env,
        &["search", "gwas", "--pubmed", "17529967"],
        body(&url, 0, 6),
        200,
        0,
    )?;
    let args = ["search", "gwas", "--pubmed", "17529967"].map(str::to_owned);
    let value = super::super::mcp::call_with_globals(
        &env,
        &args,
        false,
        &env.globals(Some(&replay), false),
    )?;
    if value.pointer("/result/items/0/id") != Some(&json!("association:6110")) {
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
        || !text.contains("get variant rs2981582")
    {
        return Err(format!("Markdown lost source facts: {text}"));
    }
    let help = gwas_budgets::invoke(
        "gwas_search/wire/surfaces/1",
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

pub(super) fn warm() -> Result<(), String> {
    let stub = super::super::network_stub::Stub::for_source(
        "www.ebi.ac.uk",
        "application/json",
        live_body,
    )?;
    let env = super::super::variant::source_environment(&stub)?;
    let args = ["search", "gwas", "--pubmed", "17529967"]
        .map(str::to_owned)
        .to_vec();
    let first = gwas_budgets::invoke("gwas_search/wire/warm/1", &env, args.clone(), 0)?;
    let second = gwas_budgets::invoke("gwas_search/wire/warm/2", &env, args.clone(), 0)?;
    if first != second {
        return Err("warm page changed source assertions".into());
    }
    let mut fresh = args.clone();
    fresh.push("--fresh".into());
    gwas_budgets::invoke("gwas_search/wire/warm/3", &env, fresh, 0)?;
    for source in ["gwas", "restricted"] {
        let mut disabled = args.clone();
        disabled.extend(["--disable-sources".into(), source.into()]);
        let disabled = gwas_budgets::invoke("gwas_search/wire/warm/4", &env, disabled, 4)?;
        if disabled.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err("warm source policy was bypassed".into());
        }
    }
    let requests = stub.finish()?;
    if requests.len() != 2 {
        return Err(format!(
            "warm/fresh policy made {} requests, expected2",
            requests.len()
        ));
    }
    Ok(())
}
pub(super) fn live_body(request: &str) -> Result<String, String> {
    let path = request.split_whitespace().nth(1).ok_or("request path")?;
    let page = path
        .split(['?', '&'])
        .find_map(|pair| pair.strip_prefix("page="))
        .ok_or("request page")?
        .parse()
        .map_err(|_| "request page number")?;
    Ok(body(&format!("https://www.ebi.ac.uk{path}"), page, 6).to_string())
}
