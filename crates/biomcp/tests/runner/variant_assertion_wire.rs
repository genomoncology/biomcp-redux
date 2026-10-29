use super::network_stub::Stub;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("variant/assertions-source-wire", || {
            assertions_wire().map_err(Into::into)
        }),
        transcript_trial("variant/clinvar-parent-identities-stay-uncached", || {
            parent_identity().map_err(Into::into)
        }),
    ]
}
fn assertions_wire() -> Result<(), String> {
    let stub = Stub::for_source("eutils.ncbi.nlm.nih.gov", "application/xml", clinvar_body)?;
    let environment = super::variant::source_environment(&stub)?;
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
        || value.pointer("/result/assertions/0/statement/classification/codings/0/code")
            != Some(&json!("pathogenic"))
        || value.pointer("/result/assertions/0/statement/strength/codings/0")
            != Some(
                &json!({"system":"clinvar_review_status","code":"criteria_provided_single_submitter"}),
            )
        || value.pointer("/result/assertions/0/statement/allele_origin/codings/0")
            != Some(&json!({"system":"clinvar_allele_origin","code":"unknown"}))
    {
        return Err(format!("native assertion lost: {value}"));
    }
    let requests = stub.finish()?;
    if requests.len() != 1
        || !requests.first().is_some_and(|(_, request)| {
            request.starts_with(
                "GET \
        /entrez/eutils/efetch.fcgi?db=clinvar&id=128042&rettype=vcv&is_variationid=true&retmode=xml&tool=biomcp HTTP/1.1",
            )
        })
    {
        return Err(format!("wrong exact ClinVar wire: {requests:?}"));
    }
    Ok(())
}

pub(super) fn clinvar_body(_: &str) -> Result<String, String> {
    // Explicit synthetic XML uses clinically reviewed public fixture facts; this is not a source capture.
    Ok("<ClinVarResult-Set><VariationArchive VariationID=\"128042\" Accession=\"VCV000128042\" \
        Version=\"175\"><ClassifiedRecord><Classifications><GermlineClassification><Description>Pathogenic</\
        Description></GermlineClassification></Classifications><ClinicalAssertionList><ClinicalAssertion><ClinVarAccession \
                    Accession=\"SCV000611258\" Version=\"1\" SubmitterName=\"Fulgent Genetics, Fulgent \
        Genetics\"/><Classification DateLastEvaluated=\"2017-05-18\"><ReviewStatus>criteria provided, \
        single \
        submitter</ReviewStatus><GermlineClassification>Pathogenic</GermlineClassification></Classification>\
        <AttributeSet><Attribute Type=\"AssertionMethod\">ACMG Guidelines, 2015</Attribute><Citation><ID \
        Source=\"PubMed\">25741868</ID></Citation></AttributeSet><ObservedInList><ObservedIn><Sample><Origin\
        >unknown</Origin></Sample></ObservedIn></ObservedInList><TraitSet><Trait><XRef DB=\"OMIM\" \
        ID=\"114480\"/></Trait><Trait><XRef DB=\"OMIM\" ID=\"176807\"/></Trait><Trait><XRef DB=\"OMIM\" \
        ID=\"259500\"/></Trait><Trait><XRef DB=\"OMIM\" \
        ID=\"609265\"/></Trait></TraitSet></ClinicalAssertion></ClinicalAssertionList></ClassifiedRecord></VariationArchive></ClinVarResult-Set>"
        .into())
}

fn parent_identity() -> Result<(), String> {
    for (original, changed, field, selection) in [
        (
            "VariationID=\"128042\"",
            "VariationID=\"bad\"",
            "VariationID",
            false,
        ),
        (
            "VariationID=\"128042\"",
            "VariationID=\"999\"",
            "VariationID",
            false,
        ),
        ("VariationID=\"128042\"", "", "VariationID", false),
        (
            "Accession=\"VCV000128042\"",
            "Accession=\"VCV000000999\"",
            "Accession",
            true,
        ),
        ("Version=\"175\"", "Version=\"176\"", "Version", true),
    ] {
        let stub = Stub::for_source(
            "eutils.ncbi.nlm.nih.gov",
            "application/xml",
            move |request| Ok(clinvar_body(request)?.replace(original, changed)),
        )?;
        let env = super::variant::source_environment(&stub)?;
        let mut args = [
            "variant",
            "assertions",
            "clinvar:128042",
            "--source",
            "clinvar",
        ]
        .map(str::to_owned)
        .to_vec();
        if selection {
            args.extend(["--assertion", "VCV000128042", "--version", "175"].map(str::to_owned));
        }
        for _ in 0..2 {
            let output = env
                .command(env!("CARGO_BIN_EXE_biomcp"), &args)
                .output()
                .map_err(|e| e.to_string())?;
            let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
            let problems = value
                .get("problems")
                .and_then(Value::as_array)
                .ok_or("missing parent identity problem")?;
            if problems.len() != 1
                || problems.first().and_then(|p| p.get("field")) != Some(&json!(field))
                || value.pointer("/result/assertions").is_some()
                || value.pointer("/sources/0/state") != Some(&json!("ok"))
            {
                return Err(format!(
                    "bad parent identity admitted assertions or failed source: {value}"
                ));
            }
        }
        args.push("--offline".into());
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if value.pointer("/sources/0/reason") != Some(&json!("offline"))
            || stub.finish()?.len() != 2
        {
            return Err(format!("mismatched parent XML entered cache: {value}"));
        }
    }
    Ok(())
}
