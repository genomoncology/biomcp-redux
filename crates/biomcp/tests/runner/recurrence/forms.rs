//! Each published identifier family reaches the same source-qualified cohort observation.
use crate::runner::{launch::Environment, mcp, network_stub::Stub};
use serde_json::{Value, json};

pub(super) fn check() -> Result<(), String> {
    let stub = Stub::for_sources(
        &[
            "www.cbioportal.org",
            "reg.genome.network",
            "storage.googleapis.com",
            "myvariant.info",
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
    let env = Environment::new().map_err(|e| e.to_string())?;
    super::config(&env, &stub)?;
    shared_forms(&env)?;
    rejections(&env)?;
    for input in [
        "rs113488022",
        "CA123643",
        "clinvar:13961",
        "VCV000013961",
        "NC_000007.13:g.140453136A>T",
        "chr7:g.140753336A>T",
        "7-140753336-A-T",
    ] {
        let mut args = super::args();
        args[2] = input.into();
        let cli = super::call(&env, &args, 0)?;
        observed(&cli, input)?;
        let single = mcp::call_with_globals(&env, &args, false, &[])?;
        observed(&single, input)?;
        let frame = mcp::request(
            json!(1),
            "tools/call",
            json!({"name":"variant_recurrence","arguments":{
                "input":input,"source":"cbioportal","gene":"BRAF",
                "study":"msk_impact_2017","profile":"msk_impact_2017_mutations",
                "sample_set":"msk_impact_2017_all","limit":3
            }}),
        );
        let replies =
            mcp::exchange_with_globals(&env, &[frame], &["--tool-mode".into(), "typed".into()])?;
        let typed = replies
            .first()
            .and_then(|r| r.pointer("/result/structuredContent"))
            .ok_or_else(|| format!("missing typed recurrence reply: {replies:?}"))?;
        observed(typed, input)?;
    }
    stub.finish()?;
    Ok(())
}
fn observed(value: &Value, input: &str) -> Result<(), String> {
    if value.pointer("/result/observed_sample_count") != Some(&json!(1))
        || value.pointer("/result/matching_granularity") != Some(&json!("exact-genomic-allele"))
    {
        return Err(format!(
            "{input} did not reach the exact cohort observation: {value}"
        ));
    }
    Ok(())
}
fn reply(request: &str) -> Result<String, String> {
    if request.starts_with("GET /v1/") {
        let hit = json!({
            "_id":"chr7:g.140753336A>T", "chrom":"7",
            "hg38":{"start":140753336,"end":140753336},
            "dbsnp":{"rsid":"rs113488022"},"clinvar":{"variant_id":13961},
            "clingen":{"caid":"CA123643"}
        });
        return Ok(if request.starts_with("GET /v1/query?") {
            json!({"total":1,"hits":[hit]})
        } else {
            hit
        }
        .to_string());
    }
    if request.starts_with("GET /allele?") {
        return super::car(request);
    }
    super::cbio(request)
}

fn shared_forms(env: &Environment) -> Result<(), String> {
    let mut definitions = Vec::new();
    for (_, _, id) in biomcp_types::command_paths()
        .into_iter()
        .filter(|(verb, entity, _)| {
            matches!(
                (*verb, *entity),
                ("variant", "recurrence" | "population" | "assertions") | ("get", "variant")
            )
        })
    {
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &["schema".into(), id.into()])
            .output()
            .map_err(|e| e.to_string())?;
        let schema: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if schema.pointer("/properties/input/$ref") != Some(&json!("#/$defs/VariantLookupInput")) {
            return Err(format!(
                "{id} no longer uses the shared variant input contract"
            ));
        }
        definitions.push(
            schema
                .pointer("/$defs/VariantLookupInput")
                .cloned()
                .ok_or("variant input schema")?,
        );
    }
    if definitions
        .iter()
        .any(|definition| definition != &definitions[0])
    {
        return Err("variant lookup commands advertise different identifier forms".into());
    }
    let validator = jsonschema::validator_for(&definitions[0]).map_err(|e| e.to_string())?;
    for input in UNSUPPORTED {
        if validator.is_valid(&json!(input)) {
            return Err(format!(
                "lookup schema advertises unrouted identifier {input}"
            ));
        }
    }
    Ok(())
}

const UNSUPPORTED: [&str; 3] = [
    "PA123",
    "COSV60000001",
    "ga4gh:VA.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
];
fn rejections(env: &Environment) -> Result<(), String> {
    for input in UNSUPPORTED {
        let mut recurrence = super::args();
        recurrence[2] = input.into();
        for args in [
            vec!["get".into(), "variant".into(), input.into()],
            vec!["variant".into(), "population".into(), input.into()],
            vec![
                "variant".into(),
                "assertions".into(),
                input.into(),
                "--source".into(),
                "clinvar".into(),
            ],
            recurrence,
        ] {
            let value = super::call(env, &args, 2)?;
            if value.pointer("/error/code") != Some(&json!("usage.arguments")) {
                return Err(format!(
                    "unrouted identifier did not fail command admission: {value}"
                ));
            }
        }
    }
    Ok(())
}
