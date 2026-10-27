//! Ambiguous published IDs keep every source allele and executable population steps.
use super::{environment, execute};
use crate::runner::network_stub::Stub;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("variant/population-ambiguous-ids-retain-source-alleles", || {
        ambiguous().map_err(Into::into)
    })]
}
fn ambiguous() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", candidates)?;
    let env = environment(&stub)?;
    for input in ["rs7903146", "clinvar:7413"] {
        let output = execute(&env, &["variant", "population", input])?;
        let rows = output.pointer("/result/candidates").and_then(Value::as_array)
            .ok_or_else(|| format!("source candidates disappeared: {output}"))?;
        if rows.len() != 2 || rows[0].pointer("/record/id") != Some(&json!("chr10:g.114758349C>T"))
            || rows[1].pointer("/record/id") != Some(&json!("chr10:g.114758349C>G")) {
            return Err(format!("source allele order or identity changed: {output}"));
        }
        let next = output.get("next").and_then(Value::as_array)
            .ok_or_else(|| format!("candidate next steps absent: {output}"))?;
        if next.len() != 2 || output.get("status") == Some(&json!("failed")) {
            return Err(format!("ambiguity became an unexplained failure: {output}"));
        }
        for (row, step) in rows.iter().zip(next) {
            let args: Vec<String> = serde_json::from_value(step.get("args").cloned()
                .ok_or("missing candidate args")?).map_err(|e| e.to_string())?;
            if args.get(..2) != Some(&["variant".to_owned(), "population".to_owned()])
                || args.get(2).map(String::as_str) != row.pointer("/record/id").and_then(Value::as_str)
                || !args.iter().any(|s| s == "--build") {
                return Err(format!("candidate next step lost identity or assembly: {step}"));
            }
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("ambiguity made requests beyond the two source identity queries".into());
    }
    Ok(())
}
fn candidates(_: &str) -> Result<String, String> {
    // Synthetic reply-only multiallelic control; no claim about these public IDs.
    Ok(json!({"total":2,"hits":[
        {"_id":"chr10:g.114758349C>T","dbsnp":{"rsid":"rs7903146"},"clinvar":{"variant_id":7413}},
        {"_id":"chr10:g.114758349C>G","dbsnp":{"rsid":"rs7903146"},"clinvar":{"variant_id":7413}}
    ]}).to_string())
}
