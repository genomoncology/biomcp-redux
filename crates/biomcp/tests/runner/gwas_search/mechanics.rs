use super::{
    Environment, Path, Value, body, call, gwas_budgets, json, pairs, seed, seed_text, url,
};
pub(super) fn admission() -> Result<(), String> {
    for fault in [
        "self",
        "duplicate-parameter",
        "study",
        "publication",
        "variant",
        "ontology",
        "page",
        "duplicate-id",
        "order",
        "parts",
        "missing-array",
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let mut p = pairs();
        p.extend([
            ("accession_id", "GCST000035"),
            ("rs_id", "rs2981582"),
            ("efo_id", "MONDO_0004989"),
            ("show_child_trait", "false"),
        ]);
        let url = url(&p);
        let mut response = body(&url, 0, 6);
        alter(&mut response, fault, &url)?;
        let args = [
            "search",
            "gwas",
            "--study",
            "GCST000035",
            "--pubmed",
            "17529967",
            "--variant",
            "rs2981582",
            "--trait-id",
            "MONDO_0004989",
        ];
        let replay = seed(&env, &args, response, 200, 0)?;
        let failed = call(
            &format!("gwas_search/mechanics/admission/{fault}"),
            &env,
            &replay,
            &args,
            if fault == "parts" { 0 } else { 4 },
        )?;
        if fault == "parts" {
            gwas_budgets::optional_page(
                &failed,
                "pvalue_mantissa",
                "/result/items/0/exact_p_value",
            )?;
            continue;
        }
        if failed.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
            || failed.get("result").is_some()
            || failed
                .get("problems")
                .and_then(Value::as_array)
                .is_none_or(Vec::is_empty)
        {
            return Err(format!("admitted {fault}: {failed}"));
        }
        let mut offline = env.globals(None, true);
        offline.extend(args.map(str::to_owned));
        let rejected = gwas_budgets::invoke(
            &format!("gwas_search/mechanics/uncached/{fault}"),
            &env,
            offline,
            4,
        )?;
        if rejected.pointer("/sources/0/reason") != Some(&json!("offline")) {
            return Err(format!("cached {fault}: {rejected}"));
        }
    }
    Ok(())
}
fn alter(value: &mut Value, fault: &str, url: &str) -> Result<(), String> {
    let (pointer, replacement) = match fault {
        "self" => (
            "/_links/self/href",
            json!(url.replace("17529967", "17529974")),
        ),
        "duplicate-parameter" => ("/_links/self/href", json!(format!("{url}&page=0"))),
        "study" => (
            "/_embedded/associations/0/accession_id",
            json!("GCST000037"),
        ),
        "publication" => ("/_embedded/associations/0/pubmed_id", json!("17529974")),
        "variant" => (
            "/_embedded/associations/0/snp_allele/0/rs_id",
            json!("rs3803662"),
        ),
        "ontology" => (
            "/_embedded/associations/0/efo_traits/0/efo_id",
            json!("EFO_0000305"),
        ),
        "page" => ("/page/number", json!(1)),
        "duplicate-id" => ("/_embedded/associations/1/association_id", json!(6110)),
        "order" => ("/_embedded/associations/0/pvalue_exponent", json!(-90)),
        "parts" => ("/_embedded/associations/0/pvalue_mantissa", Value::Null),
        "missing-array" => ("/_embedded/associations/0/snp_allele", Value::Null),
        _ => return Err("unknown fault".into()),
    };
    *value.pointer_mut(pointer).ok_or("fault pointer")? = replacement;
    Ok(())
}
pub(super) fn failures() -> Result<(), String> {
    for fault in [
        "429",
        "500",
        "timeout",
        "broken",
        "error-object",
        "missing-array",
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let url = url(&pairs());
        let text = if fault == "broken" {
            "{".into()
        } else if fault == "error-object" {
            r#"{"error":"synthetic provider failure"}"#.into()
        } else if fault == "missing-array" {
            let mut body = body(&url, 0, 6);
            body.get_mut("_embedded")
                .and_then(Value::as_object_mut)
                .ok_or("array owner")?
                .remove("associations")
                .ok_or("required array")?;
            body.to_string()
        } else {
            body(&url, 0, 6).to_string()
        };
        let replay = seed_text(
            &env,
            &["search", "gwas", "--pubmed", "17529967"],
            text,
            fault.parse().unwrap_or(200),
            if fault == "timeout" { 16000 } else { 0 },
        )?;
        let v = call(
            &format!("gwas_search/mechanics/failure/{fault}"),
            &env,
            &replay,
            &["search", "gwas", "--pubmed", "17529967"],
            4,
        )?;
        if serde_json::to_vec(&v).map_err(|e| e.to_string())?.len() > 2500 {
            return Err("synthetic failure exceeds ceiling".into());
        }
        if v.get("status") != Some(&json!("failed"))
            || v.get("result").is_some()
            || v.pointer("/sources/0/use") != Some(&json!("restricted"))
            || v.pointer("/sources/0/terms").is_none()
            || v.pointer("/next/0/kind") != Some(&json!("fresh"))
        {
            return Err(format!("failed request became empty: {fault}: {v}"));
        }
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    let url = url(&pairs());
    let replay = seed(
        &env,
        &["search", "gwas", "--pubmed", "17529967"],
        body(&url, 0, 0),
        200,
        0,
    )?;
    let v = call(
        "gwas_search/mechanics/failures/2",
        &env,
        &replay,
        &["search", "gwas", "--pubmed", "17529967"],
        0,
    )?;
    if v.pointer("/page/total") != Some(&json!(0)) || v.pointer("/result/items").is_some() {
        return Err(format!("source zero lost: {v}"));
    }
    invalid(&env, &replay)
}
fn invalid(env: &Environment, replay: &Path) -> Result<(), String> {
    for tail in [
        vec![],
        vec!["--variant", "RS2981582"],
        vec!["--study", "GCST000000"],
        vec!["--trait-id", "invalid"],
        vec!["--pubmed", "0"],
        vec!["--gene", "FGFR2", "--limit", "0"],
        vec!["--gene", "FGFR2", "--offset", "-1"],
        vec!["--gene", "FGFR2", "--include-child-traits"],
        vec!["--pubmed", "17529967", "--extended-geneset"],
        vec!["--gene", "FGFR2", "--p-value", "0"],
        vec!["--gene", "FGFR2", "--sort", "p_value"],
        vec!["--gene", "FGFR2", "--p-value", "1e-5", "--sort", "other"],
    ] {
        let args = [vec!["search", "gwas"], tail].concat();
        let value = call("gwas_search/mechanics/invalid/1", env, replay, &args, 2)?;
        if args.contains(&"--p-value")
            && args.last() == Some(&"0")
            && value.pointer("/error/message")
                != Some(&json!(
                    "--p-value threshold must be a positive probability in scientific notation"
                ))
        {
            return Err(format!("missing positive threshold explanation: {value}"));
        }
        if value.get("sources").is_some() {
            return Err("invalid selector reached source".into());
        }
    }
    Ok(())
}
