//! Population source requests cross the public command and real TLS transport.
use super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
#[path = "variant_population_get.rs"]
mod get;
#[path = "variant_population_ids.rs"]
mod ids;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        transcript_trial("variant/population-source-given-grch38-snv", || {
            population().map_err(Into::into)
        }),
        transcript_trial(
            "variant/population-mismatched-id-never-enters-cache",
            || population_mismatch().map_err(Into::into),
        ),
        transcript_trial("variant/population-unknown-group-remains-native", || {
            unknown_group().map_err(Into::into)
        }),
        transcript_trial("variant/population-source-chromosome-disagreement", || {
            chromosome_mismatch().map_err(Into::into)
        }),
        transcript_trial("variant/population-unpadded-indel-is-unjoinable", || {
            unjoinable().map_err(Into::into)
        }),
        transcript_trial("variant/gwas-native-pages-preserve-effects", || {
            gwas_walk("rs113488022", 2).map_err(Into::into)
        }),
        transcript_trial("variant/gwas-resolved-id-pins-source-continuation", || {
            gwas_walk("CA123643", 3).map_err(Into::into)
        }),
        transcript_trial("variant/gwas-v2-valid-zero", || {
            gwas_zero().map_err(Into::into)
        }),
    ];
    trials.extend(ids::trials());
    trials.extend(get::trials());
    trials
}
fn gwas_zero() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", empty_gwas)?;
    let env = environment(&stub)?;
    let args = ["get", "variant", "rs113488022", "gwas"].map(Into::into);
    let out = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    if !out.status.success()
        || value.pointer("/result/candidates/0/sections/gwas/state") != Some(&json!("empty"))
    {
        return Err(format!(
            "valid v2 catalog zero was not source-local empty: {value}"
        ));
    }
    let requests = stub.finish()?;
    let [(_, request)] = requests.as_slice() else {
        return Err(format!("expected one catalog request: {requests:?}"));
    };
    if !request
        .starts_with("GET /gwas/rest/api/v2/associations?rs_id=rs113488022&page=0&size=3 HTTP/1.1")
    {
        return Err(format!("expected exact rsID v2 request only: {requests:?}"));
    }
    Ok(())
}
fn empty_gwas(_: &str) -> Result<String, String> {
    Ok(json!(
        {"_links":{"self":{"href":"https://www.ebi.ac.uk/gwas/rest/api/v2/associations?rs_id=rs113488022&page=0&size=3"}},"page":{"size":3,
        "totalElements":0,"totalPages":0,"number":0}}
    )
    .to_string())
}

fn population() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["reg.genome.network", "gnomad.broadinstitute.org"],
        |_| "application/json",
        population_body,
    )?;
    let env = environment(&stub)?;
    let args = [
        "variant",
        "population",
        "CA123643",
        "--detail",
        "--limit",
        "1",
    ]
    .map(Into::into);
    let out = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    if !out.status.success()
        || value.pointer("/result/population/data/exome/ac") != Some(&json!(2))
        || value.pointer("/result/population/data/exome/an") != Some(&json!(1460618))
        || value.pointer("/problems/0/field") != Some(&json!("dataset_version"))
        || value
            .pointer("/result/population/data/exome/frequency")
            .is_some()
        || value.pointer("/result/population/data/genome").is_some()
        || value
            .pointer("/result/population/data/exome/allele_frequency")
            .is_some()
    {
        return Err(format!(
            "source counts/null dataset/no-derived-frequency lost: {value}"
        ));
    }
    let page_args: Vec<String> = serde_json::from_value(
        value
            .pointer("/next/0/args")
            .cloned()
            .ok_or("missing ancestry continuation")?,
    )
    .map_err(|e| e.to_string())?;
    let out = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &page_args)
        .output()
        .map_err(|e| e.to_string())?;
    let second: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    if value.pointer("/result/groups/0/ancestry_group") != Some(&json!("nfe"))
        || second.pointer("/result/groups/0/ancestry_group") != Some(&json!("sas"))
        || second.pointer("/page/offset") != Some(&json!(1))
        || second.pointer("/page/total") != Some(&json!(2))
        || second.pointer("/next").is_some()
    {
        return Err(format!(
            "ancestry continuation lost source rows/bounds: {value} {second}"
        ));
    }
    let requests = stub.finish()?;
    let [_, (_, request)] = requests.as_slice() else {
        return Err(format!(
            "expected one CAR and one gnomAD request across warm pages: {requests:?}"
        ));
    };
    if !request.starts_with("POST /api HTTP/1.1")
        || !request.contains("gnomad_r4")
        || !request.contains("7-140753336-A-T")
        || requests.iter().any(|(_, r)| r.contains("myvariant"))
    {
        return Err(format!(
            "source-given exact SNV wire projection was not used: {requests:?}"
        ));
    }
    Ok(())
}
pub(super) fn population_body(request: &str) -> Result<String, String> {
    if request.starts_with("GET /allele/") {
        return Ok(json!(
            {"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"chromosome":"7","referenceGenome":"GRCh38",
            "referenceSequence":"http://reg.genome.network/refseq/RS000055","hgvs":["NC_000007.14:g.140753336A>T"],
            "coordinates":[{"start":140753335,"end":140753336,"referenceAllele":"A","allele":"T"}]}]}
        )
        .to_string());
    }
    Ok(json!(
        {"data":{"variant":{"variant_id":"7-140753336-A-T","exome":{"ac":2,"an":1460618,"filters":[],"populations":[{"id":"nfe","ac":1,
        "an":1111098},{"id":"sas","ac":1,"an":86244}]},"genome":null}}}
    )
    .to_string())
}

pub(super) fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn execute(env: &Environment, args: &[&str]) -> Result<Value, String> {
    let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
    let out = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}
fn population_mismatch() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["reg.genome.network", "gnomad.broadinstitute.org"],
        |_| "application/json",
        mismatched_body,
    )?;
    let env = environment(&stub)?;
    for _ in 0..2 {
        let value = execute(&env, &["variant", "population", "CA123643", "--detail"])?;
        if value.pointer("/result/population/state") != Some(&json!("empty"))
            || value.pointer("/problems/0/field") != Some(&json!("variant_id"))
            || value.pointer("/status") != Some(&json!("partial"))
            || value.pointer("/result/population/data").is_some()
        {
            return Err(format!("wrong source allele leaked observations: {value}"));
        }
    }
    let offline = execute(
        &env,
        &["variant", "population", "CA123643", "--detail", "--offline"],
    )?;
    if offline.pointer("/result/population/reason") != Some(&json!("offline"))
        || stub.finish()?.len() != 3
    {
        return Err(format!(
            "mismatched GraphQL allele entered cache: {offline}"
        ));
    }
    Ok(())
}
fn mismatched_body(request: &str) -> Result<String, String> {
    let mut body: Value =
        serde_json::from_str(&population_body(request)?).map_err(|e| e.to_string())?;
    if !request.starts_with("GET /allele/") {
        *body
            .pointer_mut("/data/variant/variant_id")
            .ok_or("missing synthetic variant ID")? = json!("7-140753336-A-C");
    }
    Ok(body.to_string())
}
fn unjoinable() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["reg.genome.network", "gnomad.broadinstitute.org"],
        |_| "application/json",
        indel_body,
    )?;
    let env = environment(&stub)?;
    let value = execute(&env, &["variant", "population", "CA123643", "--detail"])?;
    if value.pointer("/result/population/reason") != Some(&json!("no-join-id"))
        || value.pointer("/problems/0/field") != Some(&json!("GRCh38_allele"))
        || stub.finish()?.len() != 1
    {
        return Err(format!(
            "unpadded indel should not produce a gnomAD request: {value}"
        ));
    }
    Ok(())
}
fn indel_body(request: &str) -> Result<String, String> {
    // Synthetic response-only insertion control; it makes no claim about this real CAid.
    let mut body: Value =
        serde_json::from_str(&population_body(request)?).map_err(|e| e.to_string())?;
    *body
        .pointer_mut("/genomicAlleles/0/hgvs")
        .ok_or("missing synthetic source HGVS")? =
        json!(["NC_000007.14:g.140753336_140753337insT"]);
    *body
        .pointer_mut("/genomicAlleles/0/coordinates")
        .ok_or("missing synthetic source coordinates")? =
        json!([{"start":140753336,"end":140753336,"referenceAllele":"","allele":"T"}]);
    Ok(body.to_string())
}

fn gwas_walk(input: &str, requests: usize) -> Result<(), String> {
    let stub = Stub::for_sources(
        &["www.ebi.ac.uk", "reg.genome.network"],
        |_| "application/json",
        gwas_page,
    )?;
    let env = environment(&stub)?;
    let mut args = ["get", "variant", input, "gwas", "--section-limit", "2"]
        .map(String::from)
        .to_vec();
    let mut ids = Vec::new();
    for page in 0..2 {
        let out = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
        let data = value
            .pointer("/result/candidates/0/sections/gwas/data")
            .ok_or_else(|| format!("catalog data absent: {value}"))?;
        if data.pointer("/page/offset") != Some(&json!(page * 2))
            || data.pointer("/page/total") != Some(&json!(3))
        {
            return Err(format!("source catalog bounds lost: {value}"));
        }
        for row in data
            .get("associations")
            .and_then(Value::as_array)
            .ok_or("associations absent")?
        {
            if row.get("beta") != Some(&json!("0.036833 kg decrease"))
                || row.pointer("/alleles/0/allele") != Some(&json!("T"))
                || row.pointer("/ontology_traits/0/id") != Some(&json!("EFO_0004344"))
                || row.pointer("/exact_p_value/exponent") != Some(&json!(-28539))
                || row.pointer("/exact_p_value/mantissa") != Some(&json!("2"))
            {
                return Err(format!("source association effect/trait changed: {row}"));
            }
            ids.push(
                row.get("id")
                    .cloned()
                    .ok_or("source association ID absent")?,
            );
        }
        if page == 0 {
            args = serde_json::from_value(
                value
                    .pointer("/next/0/args")
                    .cloned()
                    .ok_or("catalog next command absent")?,
            )
            .map_err(|e| e.to_string())?;
        } else if value.get("next").is_some() {
            return Err("catalog walk created a third page".into());
        }
    }
    args.push("--offline".into());
    let warm = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&warm.stdout).map_err(|e| e.to_string())?;
    if !warm.status.success()
        || value.pointer("/result/candidates/0/sections/gwas/data/associations/0/id")
            != Some(&json!(103))
    {
        return Err(format!(
            "warm native catalog page was not available offline: {value}"
        ));
    }
    if ids != [json!(101), json!(102), json!(103)] || stub.finish()?.len() != requests {
        return Err(
            "catalog walk duplicated/dropped source rows or made extra source requests".into(),
        );
    }
    Ok(())
}
fn gwas_page(request: &str) -> Result<String, String> {
    if request.starts_with("GET /allele/") {
        let mut body: Value =
            serde_json::from_str(&population_body(request)?).map_err(|e| e.to_string())?;
        body.as_object_mut().ok_or("CAR object required")?.insert(
            "externalRecords".into(),
            json!({"dbSNP":[{"rs":113488022}]}),
        );
        return Ok(body.to_string());
    }
    // Synthetic response-only associations exercise the source v2 schema and native page order.
    let second = request.contains("&page=1&");
    let ids = if second { vec![103] } else { vec![101, 102] };
    let rows: Vec<_> = ids
        .into_iter()
        .map(|id| {
            json!(
                {"association_id":id,"accession_id":"GCST90996374","reported_trait":["Synthetic trait"],"efo_traits":[{"efo_id":"EFO_0004344",
                "efo_trait":"Synthetic ontology label"}],"snp_allele":[{"rs_id":"rs113488022","effect_allele":"T"}],"p_value":0.0,
                "pvalue_mantissa":2,"pvalue_exponent":-28539,"beta":"0.036833 kg decrease","risk_frequency":"NR","range":"[0.025-0.048]",
                "pubmed_id":"40210729"}
            )
        })
        .collect();
    let number = if second { 1 } else { 0 };
    Ok(json!(
        {"_embedded":{"associations":rows},
        "_links":{"self":{"href":format!("https://www.ebi.ac.uk/gwas/rest/api/v2/associations?rs_id=rs113488022&page={number}&size=2")}},
        "page":{"size":2,"totalElements":3,"totalPages":2,"number":number}}
    )
    .to_string())
}

fn chromosome_mismatch() -> Result<(), String> {
    let stub = Stub::for_source("reg.genome.network", "application/json", chromosome_body)?;
    let env = environment(&stub)?;
    let value = execute(&env, &["variant", "population", "CA123643", "--detail"])?;
    let problems = value
        .get("problems")
        .and_then(Value::as_array)
        .ok_or("missing problems")?;
    if value.pointer("/result/population/reason") != Some(&json!("no-join-id"))
        || !problems
            .iter()
            .any(|p| p.get("field") == Some(&json!("representations.chromosome")))
        || stub.finish()?.len() != 1
    {
        return Err(format!(
            "contradictory source chromosome produced a join: {value}"
        ));
    }
    Ok(())
}
fn chromosome_body(request: &str) -> Result<String, String> {
    let mut body: Value =
        serde_json::from_str(&population_body(request)?).map_err(|e| e.to_string())?;
    *body
        .pointer_mut("/genomicAlleles/0/chromosome")
        .ok_or("missing synthetic chromosome")? = json!("8");
    Ok(body.to_string())
}

fn unknown_group() -> Result<(), String> {
    let stub = Stub::for_sources(
        &["reg.genome.network", "gnomad.broadinstitute.org"],
        |_| "application/json",
        unknown_group_body,
    )?;
    let env = environment(&stub)?;
    let value = execute(&env, &["variant", "population", "CA123643", "--detail"])?;
    let row = value
        .pointer("/result/groups/0")
        .ok_or("missing native row")?;
    if row.get("group_id") != Some(&json!("unknown_XX"))
        || row.get("ancestry_group").is_some()
        || row.get("sex").is_some()
        || row.get("ac") != Some(&json!(1))
        || value.pointer("/result/groups/1/ancestry_group") != Some(&json!("nfe_swe"))
        || value.pointer("/result/groups/1/sex") != Some(&json!("xx"))
        || !value
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|rows| {
                rows.iter()
                    .any(|p| p.get("field") == Some(&json!("population_group")))
            })
        || stub.finish()?.len() != 2
    {
        return Err(format!(
            "unknown group was dropped or assigned invented strata: {value}"
        ));
    }
    Ok(())
}
fn unknown_group_body(request: &str) -> Result<String, String> {
    let mut body: Value =
        serde_json::from_str(&population_body(request)?).map_err(|e| e.to_string())?;
    if !request.starts_with("GET /allele/") {
        *body
            .pointer_mut("/data/variant/exome/populations/0/id")
            .ok_or("missing synthetic group")? = json!("unknown_XX");
        *body
            .pointer_mut("/data/variant/exome/populations/1/id")
            .ok_or("missing second synthetic group")? = json!("nfe_swe_XX");
    }
    Ok(body.to_string())
}
