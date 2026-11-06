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
        args.extend(["--build".into(), "GRCh38".into()]);
        let cli = super::call(&env, &args, 0)?;
        observed(&cli, input)?;
        let single = mcp::call_with_globals(&env, &args, false, &[])?;
        observed(&single, input)?;
        let frame = mcp::request(
            json!(1),
            "tools/call",
            json!({"name":"variant_recurrence","arguments":{
                "input":input,"build":"GRCh38","source":"cbioportal","gene":"BRAF",
                "study":"msk_impact_2017","profile":"msk_impact_2017_mutations",
                "sample_set":"msk_impact_2017_all","limit":3
            }}),
        );
        let replies = mcp::exchange_with_globals(
            &env,
            &[frame],
            &["--tool-mode".into(), "typed".into()],
        )?;
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
        return Err(format!("{input} did not reach the exact cohort observation: {value}"));
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
        }.to_string());
    }
    if request.starts_with("GET /allele?") {
        return super::car(request);
    }
    super::cbio(request)
}
