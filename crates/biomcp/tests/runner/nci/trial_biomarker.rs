//! Feature trial-nci: exact concept entries keep source positions and native values.
use super::super::{launch::Environment, nci::invoke, response_cases::copy_capture};
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("trial/nci-biomarker-entry-boundaries", || {
            boundaries().map_err(Into::into)
        }),
        Trial::test("trial/nci-biomarker-input-and-next", || {
            inputs().map_err(Into::into)
        }),
    ]
}
fn arguments() -> Vec<String> {
    [
        "search",
        "trial",
        "--source",
        "nci",
        "--biomarker-code",
        "C96022",
        "--biomarker-use",
        "inclusion",
        "--limit",
        "2",
    ]
    .map(str::to_owned)
    .to_vec()
}
type Boundary = (
    Value,
    &'static str,
    Option<&'static str>,
    Option<&'static str>,
);
fn boundary_cases() -> [Boundary; 8] {
    let entry = json!({"nci_thesaurus_concept_id":"C96022", "eligibility_criterion":"exclusion", "inclusion_indicator":"TRIAL"});
    [
        (
            Value::Null,
            "biomarkers.nci_thesaurus_concept_id",
            None,
            None,
        ),
        (json!(false), "biomarkers", None, None),
        (
            json!([false, entry.clone()]),
            "biomarkers",
            Some("exclusion"),
            Some("TRIAL"),
        ),
        (
            json!([{"nci_thesaurus_concept_id":17}]),
            "biomarkers.nci_thesaurus_concept_id",
            None,
            None,
        ),
        (
            json!([{"nci_thesaurus_concept_id":"C96022", "eligibility_criterion":17}]),
            "biomarkers.eligibility_criterion",
            None,
            None,
        ),
        (
            json!([{"nci_thesaurus_concept_id":"C96022", "eligibility_criterion":"inclusion", "inclusion_indicator":17}]),
            "biomarkers.inclusion_indicator",
            Some("inclusion"),
            None,
        ),
        (
            json!([entry, {"nci_thesaurus_concept_id":"C1", "eligibility_criterion":"inclusion", "inclusion_indicator":"TREE"}]),
            "biomarkers.eligibility_criterion",
            Some("exclusion"),
            Some("TRIAL"),
        ),
        (
            json!([{"nci_thesaurus_concept_id":"C96022", "eligibility_criterion":" inclusion ", "inclusion_indicator":" TREE "}]),
            "biomarkers.eligibility_criterion",
            Some(" inclusion "),
            Some(" TREE "),
        ),
    ]
}
fn boundaries() -> Check {
    for (biomarkers, field, criterion, indicator) in boundary_cases() {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = env.root().join("synthetic-reply");
        copy_capture("trial/nci/biomarker/native", &replay, |_, record| {
            if let Some(rows) = record
                .pointer_mut("/exchanges/0/body/value/data")
                .and_then(Value::as_array_mut)
            {
                for row in rows {
                    row["biomarkers"] = biomarkers.clone();
                }
            }
        })
        .map_err(|e| e.to_string())?;
        let mut args = arguments();
        args.extend(env.globals(Some(&replay), false));
        let answer = invoke(&env, &args, 0)?;
        if answer.pointer("/page/total") != Some(&json!(318))
            || answer.pointer("/page/returned") != Some(&json!(2))
            || answer.pointer("/result/items/0/nct_id") != Some(&json!("NCT05677490"))
            || answer.pointer("/result/items/1/nct_id") != Some(&json!("NCT06203600"))
            || !answer
                .get("problems")
                .and_then(Value::as_array)
                .is_some_and(|rows| rows.iter().any(|p| p.get("field") == Some(&json!(field))))
        {
            return Err(format!(
                "entry boundary lost position, total or field problem: {field}"
            ));
        }
        if answer
            .pointer("/result/items/0/matched_biomarkers/0/eligibility_criterion")
            .and_then(Value::as_str)
            != criterion
        {
            return Err("matched code acquired another entry's criterion".into());
        }
        if answer
            .pointer("/result/items/0/matched_biomarkers/0/inclusion_indicator")
            .and_then(Value::as_str)
            != indicator
        {
            return Err("matched code lost its native inclusion indicator".into());
        }
    }
    Ok(())
}
fn inputs() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for args in [
        vec!["search", "trial", "--biomarker-code", "C96022"],
        vec![
            "search",
            "trial",
            "--source",
            "nci",
            "--biomarker-use",
            "inclusion",
        ],
        vec![
            "search",
            "trial",
            "--source",
            "nci",
            "--biomarker-code",
            "96022",
        ],
        vec![
            "search",
            "trial",
            "--source",
            "nci",
            "--biomarker-code",
            "C96022",
            "--biomarker-use",
            "positive",
        ],
    ] {
        let args: Vec<_> = args.into_iter().map(str::to_owned).collect();
        let answer = invoke(&env, &args, 2)?;
        if answer.get("sources").is_some() {
            return Err("invalid biomarker filter reached a source".into());
        }
    }
    let replay = super::super::launch::fixtures().join("replies/biomarker/native");
    for args in [
        vec!["get", "biomarker", "C96022"],
        vec!["search", "biomarker", "--source", "nci", "--name", "PD-L1"],
    ] {
        let mut args: Vec<_> = args.into_iter().map(str::to_owned).collect();
        args.extend(env.globals(Some(&replay), false));
        let answer = invoke(&env, &args, 0)?;
        let next = answer
            .get("next")
            .and_then(Value::as_array)
            .ok_or("missing next steps")?;
        if !next.iter().any(|step| {
            step.get("args")
                .and_then(Value::as_array)
                .is_some_and(|args| {
                    args.iter().any(|v| v == "--biomarker-code")
                        && args.iter().any(|v| v == "C96022")
                })
        }) {
            return Err("biomarker header omitted exact-code trial next step".into());
        }
    }
    Ok(())
}
