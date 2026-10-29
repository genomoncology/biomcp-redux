//! Feature: variant-clinvar-haplotype. Native compound subjects survive the CLI.
use super::super::network_stub::Stub;
use crate::runner::transcript::trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
const XML: &str = include_str!("../../../../../../tests/fixtures/clinvar/haplotype-441268.xml");
pub(super) fn trials() -> Vec<Trial> {
    vec![trial(
        "variant/clinvar-haplotype-preserves-native-members",
        || check().map_err(Into::into),
    )]
}
fn check() -> Result<(), String> {
    let stub = Stub::for_source("eutils.ncbi.nlm.nih.gov", "application/xml", |_| {
        Ok(XML.into())
    })?;
    let env = super::source_environment(&stub)?;
    let args = [
        "variant",
        "assertions",
        "clinvar:441268",
        "--source",
        "clinvar",
        "--assertion",
        "VCV000441268",
        "--version",
        "5",
    ]
    .map(str::to_owned)
    .to_vec();
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let expected = json!({"level":"haplotype","kind":"Haplotype","id":"clinvar:441268","accession":"VCV000441268","version":"5","number_of_chromosomes":"1","members":[{"id":"clinvar:17864","allele_id":"32903"},{"id":"clinvar:242765","allele_id":"32909"}]});
    let assertions = value
        .pointer("/result/assertions")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing haplotype assertions: {value}"))?;
    if assertions.len() != 1 || assertions[0].pointer("/statement/subject") != Some(&expected) {
        return Err(format!(
            "native haplotype grouping or members lost: {value}"
        ));
    }
    if stub.finish()?.len() != 1 {
        return Err("expected one native ClinVar request".into());
    }
    Ok(())
}
