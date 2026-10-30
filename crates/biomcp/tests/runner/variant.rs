//! Exact source request spelling crosses the real command and TLS transport.
use super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

mod components;

pub(super) fn trials() -> Vec<Trial> {
    // Keep dynamic state, source bindings or wire checks that YAML cannot express.
    vec![transcript_trial(
        crate::runner::required_contract::VALIDATOR_RESOLVER.0,
        || validator_order().map_err(Into::into),
    )]
}
fn car() -> Result<(), String> {
    let (output, requests) = call(
        "reg.genome.network",
        car_body,
        &[
            "get",
            "variant",
            "NM_004333.4:c.1799T>A",
            "--disable-sources",
            "myvariant",
        ],
    )?;
    let [(_, request)] = requests.as_slice() else {
        return Err("expected one CAR request".into());
    };
    if !request.starts_with(
        "GET \
        /allele?hgvs=NM_004333.4%3Ac.1799T%3EA&fields=none%2B%40id%2BcommunityStandardTitle%2BgenomicAlleles\
        %2BtranscriptAlleles.MANE%2BexternalRecords.dbSNP%2BexternalRecords.ClinVarVariations HTTP/1.1",
    ) {
        return Err(format!("CAR projection must preserve literal plus modifiers: {request}"));
    }
    if output.pointer("/result/candidates/0/identity/ids/0") != Some(&json!("CA123643")) {
        return Err(format!("source identity lost: {output}"));
    }
    Ok(())
}
type Requests = Vec<(String, String)>;
fn call(
    host: &'static str,
    body: fn(&str) -> Result<String, String>,
    args: &[&str],
) -> Result<(Value, Requests), String> {
    let stub = Stub::for_source(host, "application/json", body)?;
    let environment = source_environment(&stub)?;
    let args = args.iter().map(|s| (*s).into()).collect::<Vec<_>>();
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    Ok((value, stub.finish()?))
}
fn car_body(_: &str) -> Result<String, String> {
    Ok(json!(
        {"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"chromosome":"7","referenceGenome":"GRCh38",
        "referenceSequence":"http://reg.genome.network/refseq/RS000055","hgvs":["NC_000007.14:g.140753336A>T"],
        "coordinates":[{"start":140753335,"end":140753336,"referenceAllele":"A","allele":"T"}]}]}
    )
    .to_string())
}
fn myvariant_body(_: &str) -> Result<String, String> {
    Ok(json!({"_id":"chr7:g.140753336A>T","dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},"clinvar":{"variant_id":13961}}).to_string())
}

pub(super) fn source_environment(stub: &Stub) -> Result<Environment, String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let ca = environment.root().join("synthetic-ca.pem");
    stub.retain(&environment);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment
        .config(&json!({"proxy":format!("http://{}", stub.address), "ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(environment)
}

fn validator_order() -> Result<(), String> {
    let (value, requests) = call(
        "rest.variantvalidator.org",
        validator_body,
        &[
            "get",
            "variant",
            "NM_004333.4:c.1799T>A",
            "--resolver",
            "variantvalidator",
            "--build",
            "GRCh38",
        ],
    )?;
    if requests.len() != 1
        || value.pointer("/result/candidates/0/record/id") != Some(&json!("NM_004333.4:c.2A>T"))
        || value.pointer("/result/candidates/1/record/id") != Some(&json!("NM_004333.4:c.1A>T"))
        || value.pointer("/result/candidates/0/identity/representations/0/expression/value")
            != Some(&json!("NM_004333.4:c.2A>T"))
    {
        return Err(format!(
            "validator changed object order or lost transcript-only source forms: {value}"
        ));
    }
    Ok(())
}
fn validator_body(_: &str) -> Result<String, String> {
    Ok(
        "{\"flag\":\"gene_variant\",\"metadata\":{\"variantvalidator_version\":\"synthetic\"},\
                    \"z\":{\"submitted_variant\":\"NM_004333.4:c.1799T>A\",\"hgvs_transcript_variant\":\"NM_004333.4:c.2A>T\"},\"a\":{\
        \"submitted_variant\":\"NM_004333.4:c.1799T>A\",\"hgvs_transcript_variant\":\"NM_004333.4:c.1A>T\"}}"
            .into(),
    )
}

fn components() -> Result<(), String> {
    let (value, requests) = call(
        "myvariant.info",
        component_body,
        &[
            "get",
            "variant",
            "chr7:g.140453136A>T",
            "predict",
            "conservation",
            "cosmic",
            "--build",
            "GRCh37",
        ],
    )?;
    let base = "/result/candidates/0/sections";
    for name in ["predict", "conservation"] {
        if value.pointer(&format!("{base}/{name}/state")) != Some(&json!("data")) {
            return Err(format!("default component data missing: {value}"));
        }
    }
    let cosmic = value
        .pointer(&format!("{base}/cosmic/data"))
        .ok_or("historical fragment missing")?;
    if cosmic.pointer("/records/0/myvariant_record") != Some(&json!("chr7:g.140453136A>T"))
        || cosmic.pointer("/records/0/identity") != Some(&json!("unverified"))
        || cosmic.pointer("/records/0/reference") != Some(&json!("T"))
        || cosmic.pointer("/records/0/alternate") != Some(&json!("A"))
        || cosmic
            .pointer("/records/0/mut_freq")
            .and_then(Value::as_f64)
            != Some(0.0)
        || cosmic.pointer("/components/0/version") != Some(&json!("synthetic-68"))
        || value
            .pointer(&format!("{base}/predict/data/components/0/version"))
            .is_some()
        || requests.len() != 5
    {
        return Err(format!(
            "historical source values changed or extra source request: {value}; requests={requests:?}"
        ));
    }
    if value
        .pointer(&format!("{base}/predict/data/scores/0/score"))
        .and_then(Value::as_f64)
        != Some(1.5)
        || value.pointer(&format!("{base}/predict/data/components/0/terms/usage"))
            != Some(&json!("noncommercial"))
    {
        return Err(format!(
            "native scores or independent terms changed: {value}"
        ));
    }
    if value.pointer(&format!(
        "{base}/conservation/data/components/0/uploads/0/date"
    )) != Some(&json!({"value":"2020-01-01T00:00:00.123456Z", "timezone":"in-value"}))
    {
        return Err(format!(
            "component upload timestamp lost its precision or timezone: {value}"
        ));
    }
    Ok(())
}
fn component_body(request: &str) -> Result<String, String> {
    // Synthetic provider bodies test transport and association, not source truth.
    if request.starts_with("GET /v1/metadata") {
        return Ok(json!(
            {"build_version":"synthetic-build","build_date":"2020-01-01T00:00:00Z","src":{ "cosmic":{"version":"synthetic-68",
            "license_url":"https://example.org/component-terms","upload_date":"2019-01-01T00:00:00Z","note":"synthetic historical snapshot"},
            "cadd":{"version":null,"license_url":"https://example.org/cadd-terms","upload_date":"2019-01-01T00:00:00Z"},
            "dbnsfp":{"version":"synthetic-4.8a","license_url":"https://example.org/dbnsfp-terms",
            "upload_date":{"synthetic_part":"2020-01-01T00:00:00.123456Z"}} }}
        )
        .to_string());
    }
    if request.contains("cadd.phred") || request.contains("dbnsfp.gerp") {
        return Ok(json!({"_id":"chr7:g.140453136A>T",
            "cadd":{"phred":1.5,"rawscore":0.25,"sift":{"val":0,"cat":"synthetic_native_label"}},
            "dbnsfp":{"hgvsp":["p.SourceOne","p.SourceTwo"],"ensembl":{"transcriptid":["source_transcript_one","source_transcript_two"]},
                "polyphen2":{"hdiv":{"score":0.4,"pred":"D"},"hvar":{"score":0.3,"pred":"B"}},"revel":{"score":0.2},"mutationtaster":{"score":0.1,"pred":"D"},
                "gerp++":{"nr":1.5,"rs":1.2,"rs_rankscore":0.1},"phastcons":{"100way_vertebrate":{"score":0.8,"rankscore":0.6}},
                "siphy_29way":{"logodds_score":1.4,"logodds_rankscore":0.2,"pi":{"a":1,"c":0,"g":0,"t":0}}}
        })
        .to_string());
    }
    if request.contains("fields=_id%2Ccosmic") {
        return Ok(json!(
            {"_id":"chr7:g.140453136A>T","cosmic":{"cosmic_id":"COSM999999","ref":"T","alt":"A","chrom":"7","hg19":{"start":140453136,
            "end":140453136},"mut_nt":"T>A","mut_freq":0,"tumor_site":"synthetic_site"}}
        )
        .to_string());
    }
    myvariant_body37(request)
}
fn myvariant_body37(_: &str) -> Result<String, String> {
    Ok(json!({"_id":"chr7:g.140453136A>T","dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},"clinvar":{"variant_id":13961}}).to_string())
}
