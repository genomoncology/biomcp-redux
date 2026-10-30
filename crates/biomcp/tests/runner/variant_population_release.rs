//! Synthetic published-shape responses establish release provenance through TLS.
use super::{
    network_stub::Stub,
    variant_population::{environment, population_body},
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "variant-population/population-release-from-same-response",
        || release().map_err(Into::into),
    )]
}

fn release() -> Result<(), String> {
    for (reply, problem) in [
        (published as fn(&str) -> Result<String, String>, None),
        (missing_release, Some("dataset_version")),
        (wrong_counts, Some("va.exome.counts")),
        (wrong_denominator, Some("va.exome.counts")),
        (duplicate_total, Some("va.exome.cohort")),
    ] {
        let stub = Stub::for_sources(
            &["reg.genome.network", "gnomad.broadinstitute.org"],
            |_| "application/json",
            reply,
        )?;
        let env = environment(&stub)?;
        let record = env.root().join("record");
        let args = vec![
            "variant".into(),
            "population".into(),
            "CA123643".into(),
            "--detail".into(),
            "--record".into(),
            record.to_string_lossy().into(),
        ];
        let out = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
        check(&value, problem)?;
        if !out.status.success() {
            return Err(format!("population release command failed: {value}"));
        }
        let requests = stub.finish()?;
        let [_, (_, request)] = requests.as_slice() else {
            return Err(format!(
                "expected one identity and one population request: {requests:?}"
            ));
        };
        if !request.contains(
            "va{exome{derivedFrom{version} focusAlleleCount locusAlleleCount cohort{id}} \
            genome{derivedFrom{version} focusAlleleCount locusAlleleCount cohort{id}}}",
        ) {
            return Err(format!(
                "same-response versions and count checks not requested: {request}"
            ));
        }
        if problem.is_none() {
            let replay = env
                .command(
                    env!("CARGO_BIN_EXE_biomcp"),
                    &[
                        "variant".into(),
                        "population".into(),
                        "CA123643".into(),
                        "--detail".into(),
                        "--replay".into(),
                        record.to_string_lossy().into(),
                    ],
                )
                .output()
                .map_err(|e| e.to_string())?;
            let replayed: Value =
                serde_json::from_slice(&replay.stdout).map_err(|e| e.to_string())?;
            check(&replayed, None)?;
        }
    }
    Ok(())
}

fn check(value: &Value, problem: Option<&str>) -> Result<(), String> {
    let data = value
        .pointer("/result/population/data")
        .ok_or_else(|| format!("missing native counts: {value}"))?;
    if data.pointer("/exome/ac") != Some(&json!(2))
        || data.pointer("/exome/an") != Some(&json!(1460618))
        || data.pointer("/genome/ac") != Some(&json!(0))
        || data.pointer("/genome/frequency/dataset_version") != Some(&json!("synthetic-genome-9.2"))
        || data.pointer("/genome/frequency/allele_count") != Some(&json!(0))
    {
        return Err(format!(
            "native counts or independent genome provenance lost: {value}"
        ));
    }
    if let Some(field) = problem {
        if data.pointer("/exome/frequency").is_some()
            || !value["problems"]
                .as_array()
                .is_some_and(|rows| rows.iter().any(|p| p["field"] == field))
        {
            return Err(format!(
                "unproved release/counts emitted typed frequency: {value}"
            ));
        }
    } else if data.pointer("/exome/frequency/dataset_version")
        != Some(&json!("synthetic-exome-8.7"))
        || data.pointer("/exome/frequency/allele_count") != Some(&json!(2))
        || data.pointer("/exome/frequency/dataset") != Some(&json!("gnomad"))
        || data.pointer("/exome/frequency/allele_number") != Some(&json!(1460618))
        || data.pointer("/exome/frequency/subject/id") != Some(&json!("CA123643"))
        || data.pointer("/exome/frequency/allele_frequency").is_some()
        || value.get("problems").is_some()
    {
        return Err(format!(
            "same-response typed frequency provenance lost: {value}"
        ));
    }
    Ok(())
}

fn published(request: &str) -> Result<String, String> {
    let mut body: Value =
        serde_json::from_str(&population_body(request)?).map_err(|e| e.to_string())?;
    if !request.starts_with("GET /allele/") {
        *body
            .pointer_mut("/data/variant/genome")
            .ok_or("missing synthetic genome")? =
            json!({"ac":0,"an":50,"filters":[],"populations":[]});
        // ALL is identified by its cohort, never by array order or a matching count.
        let variant = body
            .pointer_mut("/data/variant")
            .and_then(Value::as_object_mut)
            .ok_or("missing synthetic variant")?;
        variant.insert(
            "va".into(),
            json!({
                "exome":[{"derivedFrom":{"version":"synthetic-subgroup"},"focusAlleleCount":2,"locusAlleleCount":1460618,"cohort":{"id":"nfe"}},
                    {"derivedFrom":{"version":"synthetic-exome-8.7"},"focusAlleleCount":2,"locusAlleleCount":1460618,"cohort":{"id":"ALL"}}],
                "genome":[{"derivedFrom":{"version":"synthetic-genome-9.2"},"focusAlleleCount":0,"locusAlleleCount":50,"cohort":{"id":"ALL"}}]
            }),
        );
    }
    Ok(body.to_string())
}
fn edited(request: &str, field: &str, value: Value) -> Result<String, String> {
    let mut body: Value = serde_json::from_str(&published(request)?).map_err(|e| e.to_string())?;
    if !request.starts_with("GET /allele/") {
        *body
            .pointer_mut(field)
            .ok_or("missing synthetic VA field")? = value;
    }
    Ok(body.to_string())
}
fn missing_release(request: &str) -> Result<String, String> {
    edited(
        request,
        "/data/variant/va/exome/1/derivedFrom/version",
        Value::Null,
    )
}
fn wrong_counts(request: &str) -> Result<String, String> {
    edited(
        request,
        "/data/variant/va/exome/1/focusAlleleCount",
        json!(3),
    )
}
fn wrong_denominator(request: &str) -> Result<String, String> {
    edited(
        request,
        "/data/variant/va/exome/1/locusAlleleCount",
        json!(1460619),
    )
}
fn duplicate_total(request: &str) -> Result<String, String> {
    edited(request, "/data/variant/va/exome/0/cohort/id", json!("ALL"))
}
