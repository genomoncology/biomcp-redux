//! Synthetic faults cover each independently fetched GWAS resource.
use super::{
    Environment, Trial, Value, ancestry_body, associations, call, card, gwas_budgets, json, loci,
    seed, study,
};
use crate::runner::transcript::trial as transcript_trial;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = Vec::new();
    for surface in ["association", "study", "associations", "ancestries", "loci"] {
        for fault in [
            "429",
            "500",
            "timeout",
            "broken",
            "error-object",
            "missing-array",
        ] {
            trials.push(transcript_trial(
                format!("gwas-get/synthetic-{surface}-{fault}"),
                move || failure(surface, fault).map_err(Into::into),
            ));
        }
    }
    for fault in ["page", "duplicate-id", "study", "rsid", "self"] {
        trials.push(transcript_trial(
            format!("gwas-get/synthetic-association-page-{fault}"),
            move || admission(fault).map_err(Into::into),
        ));
    }
    trials
}
fn args(surface: &str) -> Vec<&'static str> {
    match surface {
        "association" => vec!["get", "gwas", "association:6114"],
        "study" => vec!["get", "gwas", "GCST000035"],
        "loci" => vec!["get", "gwas", "association:6114", "loci"],
        "associations" => vec!["get", "gwas", "GCST000035", "associations"],
        _ => vec!["get", "gwas", "GCST000035", "ancestries"],
    }
}
fn failure(surface: &str, fault: &str) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = args(surface);
    let text = if fault == "broken" {
        "{".to_owned()
    } else if fault == "error-object" {
        r#"{"error":"synthetic provider failure"}"#.to_owned()
    } else {
        missing_array(surface)?.to_string()
    };
    let replay = seed(
        &env,
        &args,
        text,
        fault.parse().unwrap_or(200),
        if fault == "timeout" { 16000 } else { 0 },
    )?;
    let value = call(
        &format!("gwas_get/failures/failure/{surface}/{fault}"),
        &env,
        &replay,
        &args,
        4,
    )?;
    failed(&value)?;
    let fresh = super::super::action(&value, "fresh")?;
    if !fresh.contains(&"--fresh".to_owned())
        || !args.iter().all(|a| fresh.contains(&(*a).to_owned()))
    {
        return Err(format!("fresh lost selectors: {value}"));
    }
    uncached(&env, &args)
}
fn failed(value: &Value) -> Result<(), String> {
    if value.get("status") != Some(&json!("failed"))
        || value.get("result").is_some()
        || value.pointer("/sources/0/source") != Some(&json!("gwas"))
        || value.pointer("/sources/0/state") != Some(&json!("failed"))
        || value.pointer("/sources/0/use") != Some(&json!("restricted"))
        || value.pointer("/sources/0/terms_url").is_none()
        || value.pointer("/sources/0/terms").is_none()
    {
        return Err(format!(
            "synthetic failure became zero or lost attribution: {value}"
        ));
    }
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    if bytes > 2500 || bytes.div_ceil(4) > 625 {
        return Err("failure exceeds ceiling".into());
    }
    Ok(())
}
fn uncached(env: &Environment, args: &[&str]) -> Result<(), String> {
    let mut offline = env.globals(None, true);
    offline.extend(args.iter().map(|a| (*a).to_owned()));
    let value = gwas_budgets::invoke("gwas_get/failures/uncached/1", env, offline, 4)?;
    if value.pointer("/sources/0/reason") != Some(&json!("offline")) {
        return Err(format!("rejected reply was cached: {value}"));
    }
    Ok(())
}
fn page_url() -> &'static str {
    "https://www.ebi.ac.uk/gwas/rest/api/v2/associations?accession_id=GCST000035&direction=asc&page=0&size=3&sort=p_value"
}
fn admission(fault: &str) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = args("associations");
    let path = page_url()
        .strip_prefix("https://www.ebi.ac.uk")
        .ok_or("URL prefix")?;
    let mut body = associations(path);
    let (pointer, replacement) = match fault {
        "page" => ("/page/number", json!(1)),
        "duplicate-id" => ("/_embedded/associations/1/association_id", json!(6110)),
        "study" => (
            "/_embedded/associations/0/accession_id",
            json!("GCST000037"),
        ),
        "rsid" => (
            "/_embedded/associations/0/snp_allele/0/rs_id",
            json!("wrong"),
        ),
        _ => (
            "/_links/self/href",
            json!(page_url().replace("GCST000035", "GCST000037")),
        ),
    };
    *body.pointer_mut(pointer).ok_or("synthetic fault pointer")? = replacement;
    let replay = seed(&env, &args, body.to_string(), 200, 0)?;
    let value = call(
        &format!("gwas_get/failures/admission/{fault}"),
        &env,
        &replay,
        &args,
        4,
    )?;
    failed(&value)?;
    if value
        .get("problems")
        .and_then(Value::as_array)
        .is_none_or(Vec::is_empty)
    {
        return Err(format!("rejected identity lost its problem: {value}"));
    }
    uncached(&env, &args)
}

fn missing_array(surface: &str) -> Result<Value, String> {
    let (mut value, owner, key) = match surface {
        "association" => (card(), "", "snp_allele"),
        "study" => (study(), "", "efo_traits"),
        "loci" => (loci(), "/_embedded", "loci"),
        "associations" => (associations("/synthetic"), "/_embedded", "associations"),
        _ => (ancestry_body(), "/_embedded", "ancestries"),
    };
    value
        .pointer_mut(owner)
        .and_then(Value::as_object_mut)
        .ok_or("array owner")?
        .remove(key)
        .ok_or("required array")?;
    Ok(value)
}
