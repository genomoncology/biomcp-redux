//! Cohort identity, missing source fields and rejection cache boundaries.
use crate::runner::transcript::trial as transcript_trial;
use super::{
    launch::Environment,
    network_stub::Stub,
    recurrence::{args, call, cbio, config, row},
};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial(
            "variant-recurrence/unsupported-identity-stops-before-cohort",
            || unsupported().map_err(Into::into),
        ),
        transcript_trial("variant-recurrence/empty-page-is-an-observed-zero", || {
            empty().map_err(Into::into)
        }),
        transcript_trial(
            "variant-recurrence/missing-fields-never-become-denominators",
            || missing().map_err(Into::into),
        ),
        transcript_trial("variant-recurrence/wrong-cohort-never-enters-cache", || {
            mismatch().map_err(Into::into)
        }),
        transcript_trial(
            "variant-recurrence/provider-rejections-never-enter-cache",
            || rejected().map_err(Into::into),
        ),
    ]
}
fn peer(reply: fn(&str) -> Result<String, String>) -> Result<(Environment, Stub), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_sources(
        &[
            "www.cbioportal.org",
            "reg.genome.network",
            "storage.googleapis.com",
        ],
        |r| {
            if r.contains("hgnc_complete_set") {
                "text/tab-separated-values"
            } else {
                "application/json"
            }
        },
        reply,
    )?;
    config(&env, &stub)?;
    Ok((env, stub))
}
fn missing() -> Result<(), String> {
    let (env, stub) = peer(missing_reply)?;
    let value = call(&env, &args(), 0)?;
    if value.pointer("/result/selected_sample_count").is_some()
        || value.pointer("/result/denominator").is_some()
        || value.pointer("/result/observed_sample_count") != Some(&json!(1))
        || value.pointer("/status") != Some(&json!("partial"))
        || !value
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|p| {
                p.iter()
                    .any(|v| v.get("field") == Some(&json!("mutation.exact_allele")))
            })
    {
        return Err(format!(
            "missing assembly/count was invented or uncertain allele joined: {value}"
        ));
    }
    stub.finish()?;
    Ok(())
}
fn missing_reply(request: &str) -> Result<String, String> {
    if !request.starts_with("GET /api/") {
        return cbio(request);
    }
    let mut body: Value = serde_json::from_str(&cbio(request)?).map_err(|e| e.to_string())?;
    if request.contains("/sample-lists?") {
        body = json!([{"studyId":"msk_impact_2017","sampleListId":"msk_impact_2017_all"}]);
    }
    if request.contains("/mutations?") {
        let mut missing = row("missing-build", "T");
        missing.as_object_mut().ok_or("row")?.remove("ncbiBuild");
        let mut wrong = row("wrong-build", "T");
        *wrong.get_mut("ncbiBuild").ok_or("missing build")? = json!("GRCh38");
        body = json!([missing, wrong, row("exact", "T")]);
    }
    Ok(body.to_string())
}
fn mismatch() -> Result<(), String> {
    let (env, stub) = peer(mismatch_reply)?;
    for _ in 0..2 {
        let value = call(&env, &args(), 4)?;
        if value.pointer("/sources/2/reason") != Some(&json!("bad_reply"))
            || value.pointer("/result/observed_sample_count").is_some()
        {
            return Err(format!("wrong cohort was accepted: {value}"));
        }
    }
    let mut offline = args();
    offline.push("--offline".into());
    let value = call(&env, &offline, 4)?;
    if value.pointer("/sources/2/reason") != Some(&json!("offline")) {
        return Err(format!("wrong cohort entered cache: {value}"));
    }
    if stub
        .finish()?
        .iter()
        .filter(|(_, r)| r.contains("/mutations?"))
        .count()
        != 2
    {
        return Err("rejected cohort response was reused".into());
    }
    Ok(())
}
fn mismatch_reply(request: &str) -> Result<String, String> {
    if request.contains("/mutations?") {
        let mut bad = row("s1", "T");
        *bad.get_mut("studyId").ok_or("missing study")? = json!("wrong-study");
        return Ok(json!([bad]).to_string());
    }
    cbio(request)
}
fn rejected() -> Result<(), String> {
    let (env, stub) = peer(rejected_reply)?;
    for _ in 0..2 {
        let value = call(&env, &args(), 4)?;
        if value.pointer("/sources/0/reason") != Some(&json!("rejected"))
            || value.pointer("/result/observed_sample_count").is_some()
        {
            return Err(format!("provider rejection lost: {value}"));
        }
    }
    let mut offline = args();
    offline.push("--offline".into());
    let value = call(&env, &offline, 4)?;
    if value.pointer("/sources/0/reason") != Some(&json!("offline")) || stub.finish()?.len() != 2 {
        return Err("rejection entered identity cache or downstream source ran".into());
    }
    Ok(())
}
fn rejected_reply(_: &str) -> Result<String, String> {
    Ok(json!({"errorType":"IncorrectReferenceAllele"}).to_string())
}

fn unsupported() -> Result<(), String> {
    for reply in [
        insertion as fn(&str) -> Result<String, String>,
        deletion,
        nonchromosomal,
    ] {
        let (env, stub) = peer(reply)?;
        let value = call(&env, &args(), 4)?;
        if value.pointer("/sources/0/reason") != Some(&json!("rejected"))
            || value.pointer("/result/observed_sample_count").is_some()
            || stub.finish()?.len() != 1
        {
            return Err(format!(
                "unsupported identity reached dependent sources: {value}"
            ));
        }
    }
    Ok(())
}
fn insertion(_: &str) -> Result<String, String> {
    Ok(json!(
        {"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"referenceGenome":"GRCh37",
        "hgvs":["NC_000007.13:g.140453135_140453136insT"],"coordinates":[{"start":140453135,"end":140453135,"referenceAllele":"",
        "allele":"T"}]}]}
    )
    .to_string())
}
fn deletion(_: &str) -> Result<String, String> {
    Ok(json!(
        {"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"referenceGenome":"GRCh37",
        "hgvs":["NC_000007.13:g.140453136del"],"coordinates":[{"start":140453135,"end":140453136,"referenceAllele":"A","allele":""}]}]}
    )
    .to_string())
}

fn nonchromosomal(_: &str) -> Result<String, String> {
    Ok(json!(
        {"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"referenceGenome":"GRCh37","hgvs":["NG_007873.3:g.2A>T"],
        "coordinates":[{"start":1,"end":2,"referenceAllele":"A","allele":"T"}]}]}
    )
    .to_string())
}
fn empty() -> Result<(), String> {
    let (env, stub) = peer(empty_reply)?;
    let value = call(&env, &args(), 0)?;
    if value.pointer("/result/observed_sample_count") != Some(&json!(0))
        || value.pointer("/result/all_pages_read") != Some(&json!(true))
        || value.pointer("/result/more") != Some(&json!(false))
    {
        return Err(format!(
            "valid empty page became failure or incomplete: {value}"
        ));
    }
    let mut next = args();
    next.extend(["--page".into(), "1".into()]);
    let value = call(&env, &next, 0)?;
    if value.pointer("/result/observed_sample_count") != Some(&json!(0))
        || value.pointer("/result/all_pages_read") != Some(&json!(false))
    {
        return Err("later empty page incorrectly claims the whole query was read".into());
    }
    stub.finish()?;
    Ok(())
}
fn empty_reply(request: &str) -> Result<String, String> {
    if request.contains("/mutations?") {
        return Ok("[]".into());
    }
    cbio(request)
}
