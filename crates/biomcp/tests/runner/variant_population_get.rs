//! Ordinary get reuses the reviewed population identity path through local TLS replies.
use super::{environment, execute, ids, population_body};
use crate::runner::network_stub::Stub;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant/population-get-rsid-source-ca", || sourced(false).map_err(Into::into)),
        Trial::test("variant/population-get-grch38-hgvs", || sourced(true).map_err(Into::into)),
        Trial::test("variant/population-get-ambiguous-no-selection", || ambiguous().map_err(Into::into)),
        Trial::test("variant/population-get-missing-identity-recovery", || recovery().map_err(Into::into)),
    ]
}

fn sourced(hgvs: bool) -> Result<(), String> {
    let stub = Stub::for_sources(
        &["myvariant.info", "reg.genome.network", "gnomad.broadinstitute.org"],
        |_| "application/json",
        |request| {
            if request.starts_with("GET /allele?") {
                // Synthetic query envelope reuses the existing reviewed CAR identity.
                population_body("GET /allele/CA123643?")
            } else {
                ids::source_ca(request)
            }
        },
    )?;
    let env = environment(&stub)?;
    let input = if hgvs { "NC_000007.14:g.140753336A>T" } else { "rs113488022" };
    let output = execute(&env, &["get", "variant", input, "population"])?;
    if output.pointer("/result/candidates/0/sections/population/data/variant") != Some(&json!("7-140753336-A-T"))
        || output.pointer("/result/candidates/0/sections/population/data/exome/ac") != Some(&json!(2))
    {
        return Err(format!("ordinary population get lost source join: {output}"));
    }
    let requests = stub.finish()?;
    if requests.len() != if hgvs { 2 } else { 3 } {
        return Err(format!("unexpected identity requests: {requests:?}"));
    }
    Ok(())
}

fn ambiguous() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", |_| {
        // Synthetic multiallelic GRCh38 records can each join, but the shared rsID cannot.
        let mut body: Value = serde_json::from_str(&ids::candidates("")?).map_err(|e| e.to_string())?;
        for row in body["hits"].as_array_mut().ok_or("missing hits")? {
            let row = row.as_object_mut().ok_or("missing candidate")?;
            let coordinates = row.remove("hg19").ok_or("missing source coordinates")?;
            row.insert("hg38".into(), coordinates);
            row.insert("clingen".into(), json!({"caid":"CA123643"}));
        }
        Ok(body.to_string())
    })?;
    let env = environment(&stub)?;
    for limit in ["10", "1"] {
        // Paging to one displayed row must not turn a multiallelic ID into one allele.
        let output = execute(&env, &["get", "variant", "rs7903146", "population", "--limit", limit])?;
        let rows = output.pointer("/result/candidates").and_then(Value::as_array).ok_or("missing candidates")?;
        if output.pointer("/result/resolution") != Some(&json!("candidates"))
            || output.pointer("/page/total") != Some(&json!(2))
            || rows.len() != if limit == "1" { 1 } else { 2 }
            || rows.iter().any(|row| row.pointer("/sections/population/data").is_some())
            || output.get("next").and_then(Value::as_array).is_none_or(|steps| steps.len() < 2)
        {
            return Err(format!("ordinary get selected an ambiguous source allele: {output}"));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("ambiguous source identity triggered a downstream request".into());
    }
    Ok(())
}

fn recovery() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", |_| {
        // Synthetic unknown-build source record has no CA cross-reference.
        Ok(json!({"total":1,"hits":[{"_id":"chr7:g.140753336A>T","dbsnp":{"rsid":"rs113488022"}}]}).to_string())
    })?;
    let env = environment(&stub)?;
    let output = execute(&env, &["get", "variant", "rs113488022", "population"])?;
    if output.pointer("/result/candidates/0/sections/population/reason") != Some(&json!("no_join_id"))
        || !output.get("problems").and_then(Value::as_array).is_some_and(|p| p.iter().any(|p| p["field"] == "GRCh38_allele"))
        || !output.get("next").and_then(Value::as_array).is_some_and(|steps| steps.iter().any(|s| {
            s["args"].as_array().is_some_and(|args| args.first() == Some(&json!("search")) && args.iter().any(|a| a.as_str().is_some_and(|s| s.contains("_id:") && s.contains("140753336"))))
        }))
        || stub.finish()?.len() != 1
    {
        return Err(format!("missing identity lacks exact source-record recovery: {output}"));
    }
    Ok(())
}
