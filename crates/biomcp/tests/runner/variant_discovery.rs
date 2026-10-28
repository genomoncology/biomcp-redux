//! Features variant-identity and variant-assertions expose source reads without fetching assertions.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("variant/card-discovers-clinvar-without-request", || {
            card().map_err(Into::into)
        }),
        Trial::test(
            "variant/clinvar-default-page-retains-total-and-continuation",
            || page().map_err(Into::into),
        ),
    ]
}
fn environment(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("synthetic-ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn call(env: &Environment, args: &[&str]) -> Result<Value, String> {
    let args = args.iter().map(|s| String::from(*s)).collect::<Vec<_>>();
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("{e}: {}", String::from_utf8_lossy(&output.stdout)))
}
fn card() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", |_| {
        Ok(json!({"_id":"chr7:g.140753336A>T","dbsnp":{"rsid":"rs113488022","gene":{"symbol":"BRAF"}},"clinvar":{"variant_id":13961}}).to_string())
    })?;
    let env = environment(&stub)?;
    let value = call(
        &env,
        &["get", "variant", "chr7:g.140753336A>T", "--build", "GRCh38"],
    )?;
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing steps")?;
    if !next.iter().any(|n| {
        n.get("kind") == Some(&json!("section"))
            && n.get("args")
                == Some(&json!([
                    "get",
                    "variant",
                    "chr7:g.140753336A>T",
                    "clinvar",
                    "--build",
                    "GRCh38"
                ]))
    }) {
        return Err(format!("card lacks ClinVar section step: {value}"));
    }
    if value.pointer("/result/candidates/0/sections").is_some()
        || value.to_string().contains("classification")
    {
        return Err(format!("card fetched or printed assertions: {value}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("card added a provider request".into());
    }
    Ok(())
}
fn page() -> Result<(), String> {
    let stub =
        Stub::for_source("eutils.ncbi.nlm.nih.gov", "application/xml", |request| {
            let xml = super::variant_assertion_wire::clinvar_body(request)?;
            let rows = (0..11).map(|i| format!(
            "<ClinicalAssertion><ClinVarAccession Accession=\"SCV{:09}\" Version=\"1\"/>\
            <Classification><GermlineClassification>Pathogenic</GermlineClassification>\
            </Classification></ClinicalAssertion>", 900000000+i,
        )).collect::<String>();
            Ok(xml.replace(
                "</ClinicalAssertionList>",
                &format!("{rows}</ClinicalAssertionList>"),
            ))
        })?;
    let env = environment(&stub)?;
    for args in [
        vec!["get", "variant", "clinvar:128042", "clinvar"],
        vec![
            "variant",
            "assertions",
            "clinvar:128042",
            "--source",
            "clinvar",
        ],
    ] {
        let value = call(&env, &args)?;
        let data = value
            .pointer("/result/candidates/0/sections/clinvar/data")
            .or_else(|| value.get("result"))
            .ok_or("missing section")?;
        if data.pointer("/page/total") != Some(&json!(13))
            || data.pointer("/page/returned") != Some(&json!(10))
            || data
                .get("assertions")
                .and_then(Value::as_array)
                .map(Vec::len)
                != Some(10)
        {
            return Err(format!("ClinVar default page lost rows or total: {value}"));
        }
        let next = value
            .get("next")
            .and_then(Value::as_array)
            .and_then(|n| n.iter().find(|n| n.get("kind") == Some(&json!("page"))))
            .and_then(|n| n.get("args"))
            .cloned()
            .ok_or("missing continuation")?;
        let next: Vec<String> = serde_json::from_value(next).map_err(|e| e.to_string())?;
        let next = next.iter().map(String::as_str).collect::<Vec<_>>();
        let remainder = call(&env, &next)?;
        if remainder.pointer("/result/page/offset") != Some(&json!(10))
            || remainder.pointer("/result/page/returned") != Some(&json!(3))
        {
            return Err(format!(
                "ClinVar continuation lost remaining rows: {remainder}"
            ));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("section made extra provider request".into());
    }
    Ok(())
}
