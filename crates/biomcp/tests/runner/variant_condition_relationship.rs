//! Native ClinVar disease relationships survive selected SCV detail through both callers.
use super::{network_stub::Stub, variant::source_environment};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "variant-identity/assertions-native-drug-response-disease-relationship",
        || relationship().map_err(Into::into),
    )]
}
fn relationship() -> Result<(), String> {
    for present in [true, false] {
        let stub = Stub::for_source_status(
            "eutils.ncbi.nlm.nih.gov",
            "application/xml",
            move |_| Ok(body(present)),
            |_| 200,
        )?;
        let env = source_environment(&stub)?;
        let args = [
            "variant",
            "assertions",
            "clinvar:666267",
            "--source",
            "clinvar",
            "--assertion",
            "SCV000965609",
            "--version",
            "1",
        ]
        .map(String::from);
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(format!("assertion command failed: {output:?}"));
        }
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        let row = value
            .pointer("/result/assertions/0")
            .ok_or("missing assertion")?;
        check(row, present)?;
        if super::mcp::call_with_globals(&env, &args, false, &[])? != value {
            return Err("disease relationship CLI/MCP mismatch".into());
        }
        if stub.finish()?.len() != 1 {
            return Err("SCV detail did not reuse its checked reply".into());
        }
    }
    Ok(())
}
fn check(row: &Value, present: bool) -> Result<(), String> {
    let mut expected = json!(
        { "kind":"scv","domain":"germline","statement_unmapped":true,"unmapped": {"classification":"drug response",
        "attribution":{ "source":"clinvar","record":"SCV000965609","version":"1","submitter":"Synthetic submitter","evaluated":"2019-05-21",
        "url":"https://www.ncbi.nlm.nih.gov/clinvar/?term=SCV000965609.1" },"conditions":[{"label":"Dabrafenib response"}]},
        "detail":{"conditions":[{"label":"Dabrafenib response"}],"origins":["somatic"]} }
    );
    if present {
        expected.get_mut("detail").and_then(Value::as_object_mut).ok_or("synthetic detail")?.insert(
            "condition_relationships".into(),
            json!(
                [{ "relationship_type":"DrugResponseAndDisease","source_conditions":[{"label":"Dabrafenib response"}],
                "related_conditions":[{"label":"Pancreatic Adenocarcinoma"}] }]
            ),
        );
    }
    if row != &expected {
        return Err(format!(
            "native SCV facts differ: expected={expected}; actual={row}"
        ));
    }
    Ok(())
}
fn body(present: bool) -> String {
    // Entirely synthetic XML, shaped after the clinically reviewed native path. No source body is copied.
    let relationship = if present {
        "<TraitRelationship Type=\"DrugResponseAndDisease\"><Name><ElementValue \
            Type=\"Preferred\">Pancreatic Adenocarcinoma</ElementValue></Name></TraitRelationship>"
    } else {
        ""
    };
    format!(
        "<ClinVarResult-Set><VariationArchive VariationID=\"666267\" Accession=\"VCV000666267\" \
            Version=\"1\"><ClassifiedRecord><ClinicalAssertionList><ClinicalAssertion \
            ID=\"1893254\"><ClinVarAccession Accession=\"SCV000965609\" Version=\"1\" \
            SubmitterName=\"Synthetic submitter\"/><Classification \
            DateLastEvaluated=\"2019-05-21\"><GermlineClassification>drug \
            response</GermlineClassification></Classification><ObservedInList><ObservedIn><Sample><Origin>somatic</Origin></Sample>\
                    </ObservedIn></ObservedInList><TraitSet \
            Type=\"DrugResponse\"><Trait><Name><ElementValue Type=\"Preferred\">Dabrafenib \
            response</ElementValue></Name>{relationship}</Trait></TraitSet></ClinicalAssertion></ClinicalAssertionList>\
                    </ClassifiedRecord></VariationArchive></ClinVarResult-Set>"
    )
}
