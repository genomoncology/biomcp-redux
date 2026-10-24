//! Exact source request spelling crosses the real command and TLS transport.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant/missing-historical-fragment-is-empty", || {
            missing_component().map_err(Into::into)
        }),
        Trial::test("variant/historical-fragment-walk-is-warm", || {
            component_walk().map_err(Into::into)
        }),
        Trial::test("variant/assertions-source-wire", || {
            assertions_wire().map_err(Into::into)
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
    ]
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
    if !request.starts_with("GET /allele?hgvs=NM_004333.4%3Ac.1799T%3EA&fields=none%2B%40id%2BcommunityStandardTitle%2BgenomicAlleles%2BtranscriptAlleles.MANE%2BexternalRecords.dbSNP%2BexternalRecords.ClinVarVariations HTTP/1.1") {
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
    if !request.starts_with("GET /v1/variant/chr7%3Ag.140753336A%3ET?fields=_id%2Cdbsnp.rsid%2Cdbsnp.gene.symbol%2Cclinvar.variant_id&assembly=hg38 HTTP/1.1") { return Err(format!("exact annotation endpoint/build was not sent: {request}")); }
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
    Ok(json!({"@id":"http://reg.genome.network/allele/CA123643","genomicAlleles":[{"chromosome":"7","referenceGenome":"GRCh38","referenceSequence":"http://reg.genome.network/refseq/RS000055","hgvs":["NC_000007.14:g.140753336A>T"],"coordinates":[{"start":140753335,"end":140753336,"referenceAllele":"A","allele":"T"}]}]}).to_string())
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
    Ok(json!({"total":3,"hits":[{"_id":"chr7:g.140453136A>G","dbsnp":{"rsid":"rs113488022"}},{"_id":"chr7:g.140453136A>T","dbsnp":{"rsid":"rs113488022"}},{"_id":"chr7:g.140453136A>C","dbsnp":{"rsid":"rs113488022"}}]}).to_string())
}

fn source_environment(stub: &Stub) -> Result<Environment, String> {
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
    Ok(json!({"total":2,"hits":[{"_id":"chr7:g.140453136A>G","dbsnp":{"rsid":"rs1"}},{"_id":"chr7:g.140453136A>T","dbsnp":{"rsid":"rs113488022"}}]}).to_string())
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
    Ok(r#"{"flag":"gene_variant","metadata":{"variantvalidator_version":"synthetic"},"z":{"submitted_variant":"NM_004333.4:c.1799T>A","hgvs_transcript_variant":"NM_004333.4:c.2A>T"},"a":{"submitted_variant":"NM_004333.4:c.1799T>A","hgvs_transcript_variant":"NM_004333.4:c.1A>T"}}"#.into())
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
        if value.pointer(&format!("{base}/{name}/reason")) != Some(&json!("restricted_terms")) {
            return Err(format!("restricted component became an offer: {value}"));
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
        || requests.len() != 3
    {
        return Err(format!(
            "historical source values changed or extra source request: {value}; requests={requests:?}"
        ));
    }
    if requests
        .iter()
        .any(|(_, r)| r.contains("cadd") || r.contains("dbnsfp"))
    {
        return Err("restricted fields reached the wire".into());
    }
    Ok(())
}
fn component_body(request: &str) -> Result<String, String> {
    // Synthetic provider bodies test transport and association, not source truth.
    if request.starts_with("GET /v1/metadata") {
        return Ok(json!({"build_version":"synthetic-build","build_date":"2020-01-01T00:00:00Z","src":{"cosmic":{"version":"synthetic-68","license_url":"https://example.org/component-terms","upload_date":"2019-01-01T00:00:00Z","note":"synthetic historical snapshot"}}}).to_string());
    }
    if request.contains("fields=_id%2Ccosmic") {
        return Ok(json!({"_id":"chr7:g.140453136A>T","cosmic":{"cosmic_id":"COSM999999","ref":"T","alt":"A","chrom":"7","hg19":{"start":140453136,"end":140453136},"mut_nt":"T>A","mut_freq":0,"tumor_site":"synthetic_site"}}).to_string());
    }
    myvariant_body37(request)
}
fn myvariant_body37(_: &str) -> Result<String, String> {
    Ok(json!({"_id":"chr7:g.140453136A>T","dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},"clinvar":{"variant_id":13961}}).to_string())
}

fn assertions_wire() -> Result<(), String> {
    let stub = Stub::for_source("eutils.ncbi.nlm.nih.gov", "application/xml", clinvar_body)?;
    let environment = source_environment(&stub)?;
    let args = [
        "variant",
        "assertions",
        "clinvar:128042",
        "--source",
        "clinvar",
        "--assertion",
        "SCV000611258",
        "--version",
        "1",
    ]
    .iter()
    .map(|s| (*s).into())
    .collect::<Vec<_>>();
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if value.pointer("/result/assertions/0/statement/attribution/record")
        != Some(&json!("SCV000611258"))
        || value.pointer("/result/assertions/0/statement/classification/coding/code")
            != Some(&json!("pathogenic"))
    {
        return Err(format!("native assertion lost: {value}"));
    }
    let requests = stub.finish()?;
    if requests.len()!=1 || !requests.first().is_some_and(|(_, request)| request.starts_with("GET /entrez/eutils/efetch.fcgi?db=clinvar&id=128042&rettype=vcv&is_variationid=true&retmode=xml&tool=biomcp HTTP/1.1")) {return Err(format!("wrong exact ClinVar wire: {requests:?}"));}
    Ok(())
}
fn clinvar_body(_: &str) -> Result<String, String> {
    // Explicit synthetic XML uses clinically reviewed public fixture facts; this is not a source capture.
    Ok(r#"<ClinVarResult-Set><VariationArchive VariationID="128042" Accession="VCV000128042" Version="175"><ClassifiedRecord><Classifications><GermlineClassification><Description>Conflicting classifications of pathogenicity</Description></GermlineClassification></Classifications><ClinicalAssertionList><ClinicalAssertion><ClinVarAccession Accession="SCV000611258" Version="1" SubmitterName="Fulgent Genetics, Fulgent Genetics"/><Classification DateLastEvaluated="2017-05-18"><ReviewStatus>criteria provided, single submitter</ReviewStatus><GermlineClassification>Pathogenic</GermlineClassification></Classification><AttributeSet><Attribute Type="AssertionMethod">ACMG Guidelines, 2015</Attribute><Citation><ID Source="PubMed">25741868</ID></Citation></AttributeSet><ObservedInList><ObservedIn><Sample><Origin>unknown</Origin></Sample></ObservedIn></ObservedInList><TraitSet><Trait><XRef DB="OMIM" ID="114480"/></Trait><Trait><XRef DB="OMIM" ID="176807"/></Trait><Trait><XRef DB="OMIM" ID="259500"/></Trait><Trait><XRef DB="OMIM" ID="609265"/></Trait></TraitSet></ClinicalAssertion></ClinicalAssertionList></ClassifiedRecord></VariationArchive></ClinVarResult-Set>"#.into())
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
fn component_walk() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", component_page_body)?;
    let environment = source_environment(&stub)?;
    let mut ids = Vec::new();
    for offset in [0, 2] {
        let mut args = vec![
            "get".into(),
            "variant".into(),
            "chr7:g.140453136A>T".into(),
            "cosmic".into(),
            "--build".into(),
            "GRCh37".into(),
            "--section-limit".into(),
            "2".into(),
            "--section-offset".into(),
            offset.to_string(),
        ];
        if offset != 0 {
            args.push("--offline".into());
        }
        let output = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        let data = value
            .pointer("/result/candidates/0/sections/cosmic/data")
            .ok_or_else(|| format!("source page missing: {value}"))?;
        if data.get("total") != Some(&json!(4)) {
            return Err(format!("source total lost: {value}"));
        }
        if offset == 0 && data.pointer("/next/section_offset") != Some(&json!(2)) {
            return Err(format!(
                "typed section continuation lost its independent offset: {value}"
            ));
        }
        if offset != 0 && data.get("next").is_some() {
            return Err("extra historical page invented".into());
        }
        for row in data
            .get("records")
            .and_then(Value::as_array)
            .ok_or("rows missing")?
        {
            ids.push(row.get("cosmic_id").cloned().ok_or("native ID missing")?);
        }
    }
    if ids
        != [
            json!("COSM999991"),
            json!("COSM999992"),
            json!("COSM999993"),
            json!("COSM999994"),
        ]
        || stub.finish()?.len() != 3
    {
        return Err("source walk repeated, dropped, reordered or fetched a warm page".into());
    }
    Ok(())
}
fn component_page_body(request: &str) -> Result<String, String> {
    if !request.contains("fields=_id%2Ccosmic") {
        return component_body(request);
    }
    // Explicit synthetic rows keep source order and use no captured source content.
    Ok(json!({"_id":"chr7:g.140453136A>T","cosmic":[{"cosmic_id":"COSM999991"},{"cosmic_id":"COSM999992"},{"cosmic_id":"COSM999993"},{"cosmic_id":"COSM999994"}]}).to_string())
}
