//! Exact source request spelling crosses the real command and TLS transport.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
mod components;
mod labels;
mod roundtrip;
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = vec![
        Trial::test("variant/printed-repeat-identity-round-trips", || {
            roundtrip::check().map_err(Into::into)
        }),
        Trial::test("variant/unpaired-native-labels-remain-context", || {
            labels::check().map_err(Into::into)
        }),
        Trial::test("variant/malformed-score-is-not-missing-identity", || {
            malformed_score().map_err(Into::into)
        }),
        Trial::test("variant/disabled-clinvar-identity-never-queries", || {
            disabled_clinvar_identity().map_err(Into::into)
        }),
        Trial::test("variant/component-groups-guard-warm-data", || {
            component_groups().map_err(Into::into)
        }),
        Trial::test("variant/missing-historical-fragment-is-empty", || {
            missing_component().map_err(Into::into)
        }),
        Trial::test("variant/historical-fragment-walk-is-warm", || {
            components::component_walk().map_err(Into::into)
        }),
        Trial::test(
            "variant/component-rights-and-historical-association",
            || components().map_err(Into::into),
        ),
        Trial::test("variant/mixed-identity-page-never-enters-cache", || {
            mixed_page().map_err(Into::into)
        }),
        Trial::test(
            "variant/validator-keeps-native-order-and-transcript-only-forms",
            || validator_order().map_err(Into::into),
        ),
        Trial::test("variant/native-candidate-walk-is-warm", || {
            candidate_walk().map_err(Into::into)
        }),
        Trial::test("variant/car-projection-wire", || car().map_err(Into::into)),
        Trial::test("variant/myvariant-exact-build-wire", || {
            myvariant().map_err(Into::into)
        }),
    ];
    trials.extend(super::variant_assertion_wire::trials());
    trials.extend(super::variant_assertions::trials());
    trials.extend(super::variant_discovery::trials());
    trials.extend(super::variant_condition_relationship::trials());
    trials.extend(super::variant_population::trials());
    trials.extend(super::variant_population_release::trials());
    trials.extend(super::oncokb::trials());
    trials.extend(super::oncokb_context::trials());
    trials.extend(super::alphagenome::trials());
    trials.extend(super::alphagenome_tls::trials());
    trials.extend(super::variant_civic::trials());
    trials.extend(super::variant_hotspots::trials());
    trials.extend(super::recurrence::trials());
    trials.extend(super::recurrence_errors::trials());
    trials.extend(super::recurrence_metadata::trials());
    trials.extend(super::normalization::trials());
    trials.extend(super::variant_search::trials());
    trials.extend(super::next_navigation::trials());
    trials.extend(super::variant_filters::trials());
    trials
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
fn myvariant() -> Result<(), String> {
    let (output, requests) = call(
        "myvariant.info",
        myvariant_body,
        &["get", "variant", "chr7:g.140753336A>T", "--build", "GRCh38"],
    )?;
    let [(_, request)] = requests.as_slice() else {
        return Err("expected one exact MyVariant request".into());
    };
    if !request.starts_with(
        "GET \
        /v1/variant/chr7%3Ag.140753336A%3ET?fields=_id%2Cdbsnp.rsid%2Cdbsnp.gene.symbol%2Cclinvar.variant_id%2Csnpeff.ann\
        &assembly=hg38 HTTP/1.1",
    ) {
        return Err(format!("exact annotation endpoint/build was not sent: {request}"));
    }
    if output.pointer("/result/candidates/0/identity/representations/0/build")
        != Some(&json!("GRCh38"))
    {
        return Err(format!("source assembly lost: {output}"));
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
fn candidate_walk() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", candidates_body)?;
    let environment = source_environment(&stub)?;
    let mut args = ["get", "variant", "rs113488022", "--limit", "1"]
        .iter()
        .map(|s| (*s).into())
        .collect::<Vec<_>>();
    let mut ids = Vec::new();
    for offset in 0..3 {
        let output = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err("candidate page command failed".into());
        }
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if value.pointer("/page/offset") != Some(&json!(offset))
            || value.pointer("/page/returned") != Some(&json!(1))
            || value.pointer("/page/total") != Some(&json!(3))
        {
            return Err(format!("candidate page lost its bounds: {value}"));
        }
        ids.push(
            value
                .pointer("/result/candidates/0/record/id")
                .cloned()
                .ok_or("candidate ID absent")?,
        );
        let next = value
            .get("next")
            .and_then(Value::as_array)
            .ok_or("candidate selection absent")?;
        if !next
            .iter()
            .any(|n| n.get("kind").and_then(Value::as_str) == Some("disambiguate"))
        {
            return Err("candidate selection action absent".into());
        }
        if offset < 2 {
            let page = next
                .iter()
                .find(|n| n.get("kind").and_then(Value::as_str) == Some("page"))
                .ok_or("next candidate page absent")?;
            args = serde_json::from_value(page.get("args").cloned().ok_or("page args absent")?)
                .map_err(|e| e.to_string())?;
        } else if next
            .iter()
            .any(|n| n.get("kind").and_then(Value::as_str) == Some("page"))
        {
            return Err("unexpected fourth page".into());
        }
    }
    if ids
        != [
            json!("chr7:g.140453136A>G"),
            json!("chr7:g.140453136A>T"),
            json!("chr7:g.140453136A>C"),
        ]
    {
        return Err("candidate walk changed native order or dropped an allele".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("warm candidate pages made another HTTP request".into());
    }
    Ok(())
}
fn candidates_body(_: &str) -> Result<String, String> {
    Ok(json!(
        {"total":3,"hits":[{"_id":"chr7:g.140453136A>G","dbsnp":{"rsid":"rs113488022"}},{"_id":"chr7:g.140453136A>T",
        "dbsnp":{"rsid":"rs113488022"}},{"_id":"chr7:g.140453136A>C","dbsnp":{"rsid":"rs113488022"}}]}
    )
    .to_string())
}

pub(super) fn source_environment(stub: &Stub) -> Result<Environment, String> {
    let environment = Environment::new().map_err(|e| e.to_string())?;
    let ca = environment.root().join("synthetic-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    environment
        .config(&json!({"proxy":format!("http://{}", stub.address), "ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(environment)
}
fn mixed_page() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", mixed_body)?;
    let environment = source_environment(&stub)?;
    let args = ["get", "variant", "rs113488022"]
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();
    for _ in 0..2 {
        let output = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if value.pointer("/status") != Some(&json!("partial"))
            || value.pointer("/result/candidates/0/record/id")
                != Some(&json!("chr7:g.140453136A>T"))
            || value.pointer("/problems/0/field") != Some(&json!("identity"))
        {
            return Err(format!(
                "mixed page lost the valid source candidate or mismatch problem: {value}"
            ));
        }
    }
    let mut offline = args;
    offline.push("--offline".into());
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &offline)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if value.pointer("/sources/0/reason") != Some(&json!("offline")) || stub.finish()?.len() != 2 {
        return Err("mixed identity response entered or was reused from cache".into());
    }
    Ok(())
}
fn mixed_body(_: &str) -> Result<String, String> {
    Ok(json!(
        {"total":2,"hits":[{"_id":"chr7:g.140453136A>G","dbsnp":{"rsid":"rs1"}},{"_id":"chr7:g.140453136A>T",
        "dbsnp":{"rsid":"rs113488022"}}]}
    )
    .to_string())
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
        "{\"flag\":\"gene_variant\",\"metadata\":{\"variantvalidator_version\":\"synthetic\"},\"z\":{\"submit\
        ted_variant\":\"NM_004333.4:c.1799T>A\",\"hgvs_transcript_variant\":\"NM_004333.4:c.2A>T\"},\"a\":{\
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
            "upload_date":{"synthetic_part":"2020-01-01T00:00:00Z"}} }}
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

fn missing_component() -> Result<(), String> {
    let (value, requests) = call(
        "myvariant.info",
        myvariant_body37,
        &[
            "get",
            "variant",
            "chr7:g.140453136A>T",
            "cosmic",
            "--build",
            "GRCh37",
        ],
    )?;
    if value.pointer("/result/candidates/0/sections/cosmic/state") != Some(&json!("empty"))
        || requests.len() != 2
        || requests
            .iter()
            .any(|(_, r)| r.starts_with("GET /v1/metadata"))
    {
        return Err(format!(
            "missing source field became unavailable or metadata fetched unnecessarily: {value}"
        ));
    }
    Ok(())
}
fn component_groups() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", component_body)?;
    let environment = source_environment(&stub)?;
    let base = [
        "get",
        "variant",
        "chr7:g.140453136A>T",
        "predict",
        "conservation",
        "cosmic",
        "--build",
        "GRCh37",
    ];
    for group in [
        None,
        Some("noncommercial"),
        Some("restricted"),
        Some("myvariant"),
    ] {
        let mut args = base.iter().map(|s| (*s).into()).collect::<Vec<String>>();
        if let Some(group) = group {
            args.extend(["--disable-sources".into(), group.into()]);
        }
        let output = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if group == Some("myvariant") {
            if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
                return Err(format!("explicit source switch failed: {value}"));
            }
            continue;
        }
        if group == Some("restricted")
            && value
                .pointer("/result/candidates/0/identity/ids")
                .and_then(Value::as_array)
                .is_some_and(|ids| {
                    ids.iter()
                        .any(|id| id.as_str().is_some_and(|id| id.starts_with("clinvar:")))
                })
        {
            return Err(format!(
                "disabled ClinVar aliases leaked from an ignored projection: {value}"
            ));
        }
        let prefix = "/result/candidates/0/sections";
        for section in ["predict", "conservation", "cosmic"] {
            let off = group == Some("noncommercial") && section != "cosmic"
                || group == Some("restricted") && section == "cosmic";
            if value.pointer(&format!("{prefix}/{section}/state"))
                != Some(&json!(if off { "skipped" } else { "data" }))
            {
                return Err(format!(
                    "component group leaked or hid unaffected data: {value}"
                ));
            }
        }
    }
    configured_component_groups(&environment, &stub, &base)?;
    let requests = stub.finish()?;
    // A changed identity projection needs one new read; its ignored ClinVar response must not leak.
    if requests.len() != 6 {
        return Err(format!(
            "warm policy caused extra component reads: {}",
            requests.len()
        ));
    }
    Ok(())
}

fn disabled_clinvar_identity() -> Result<(), String> {
    let (value, requests) = call(
        "myvariant.info",
        component_body,
        &[
            "get",
            "variant",
            "clinvar:13961",
            "--disable-sources",
            "restricted",
        ],
    )?;
    if !requests.is_empty()
        || value.pointer("/result/resolution") != Some(&json!("unavailable"))
        || value.pointer("/sources/0/reason") != Some(&json!("disabled"))
    {
        return Err(format!(
            "disabled ClinVar identity became a request or false absence: {value}"
        ));
    }
    Ok(())
}

fn configured_component_groups(
    environment: &Environment,
    stub: &Stub,
    base: &[&str],
) -> Result<(), String> {
    environment
        .config(&json!({"proxy":format!("http://{}",
stub.address),
"ca_bundle":environment.root().join("synthetic-ca.pem"),
"disable_sources":["noncommercial"]}))
        .map_err(|e| e.to_string())?;
    for extra in [
        Vec::<String>::new(),
        vec!["--disable-sources".into(), "restricted".into()],
    ] {
        let mut args = base.iter().map(|s| (*s).into()).collect::<Vec<String>>();
        args.extend(extra.clone());
        let output = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if value.pointer("/result/candidates/0/sections/predict/state") != Some(&json!("skipped"))
            || value.pointer("/result/candidates/0/sections/conservation/state")
                != Some(&json!("skipped"))
            || value.pointer("/result/candidates/0/sections/cosmic/state")
                != Some(&json!(if extra.is_empty() { "data" } else { "skipped" }))
        {
            return Err(format!(
                "stored and command component policy did not combine: {value}"
            ));
        }
    }
    Ok(())
}

fn malformed_score() -> Result<(), String> {
    fn body(request: &str) -> Result<String, String> {
        if request.contains("cadd.phred") {
            Ok(
                json!({"_id":"chr7:g.140453136A>T","cadd":{"phred":"invalid_source_number"}})
                    .to_string(),
            )
        } else {
            myvariant_body37(request)
        }
    }
    let (value, requests) = call(
        "myvariant.info",
        body,
        &[
            "get",
            "variant",
            "chr7:g.140453136A>T",
            "predict",
            "--build",
            "GRCh37",
        ],
    )?;
    if value.pointer("/result/candidates/0/sections/predict/reason") != Some(&json!("bad_reply"))
        || value.pointer("/problems/0/field") != Some(&json!("cadd.phred"))
        || requests.len() != 2
    {
        return Err(format!(
            "invalid source number was treated as missing identity: {value}"
        ));
    }
    Ok(())
}
