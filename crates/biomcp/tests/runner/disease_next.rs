//! Feature disease-mydisease: every step a refused disease read offers runs, for each ID namespace in
//! its printed and lowercase spelling, with and without each section.
use super::launch::{self, Environment};
use libtest_mimic::Trial;
use serde_json::Value;
use std::io::Write;

const IDS: [&str; 8] = [
    "MONDO:0007363",
    "DOID:0111595",
    "ORPHA:115",
    "Orphanet:115",
    "OMIM:121050",
    "NCIT:C34807",
    "ICD10CM:Q87.4",
    "HP:0001166",
];
const SECTIONS: [&str; 8] = [
    "synonyms",
    "xrefs",
    "hierarchy",
    "genes",
    "phenotypes",
    "incidence",
    "survival",
    "prevalence",
];

pub(super) fn trials() -> Vec<Trial> {
    vec![super::transcript::trial(
        "disease-mydisease/refusal-steps-run",
        || run().map_err(Into::into),
    )]
}

fn run() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = launch::fixtures().join("replies/disease/orphanet-id");
    let globals = env.globals(Some(&replay), false);
    let mut failures = Vec::new();
    let mut followed = 0;
    for id in IDS
        .iter()
        .flat_map(|id| [(*id).to_owned(), id.to_lowercase()])
    {
        let reads = std::iter::once(None).chain(SECTIONS.iter().map(Some));
        for section in reads {
            let mut args = vec!["get".to_owned(), "disease".to_owned(), id.clone()];
            args.extend(section.map(|word| (*word).to_owned()));
            let (_, reply) = cli(&env, &args, &globals)?;
            let refused = reply
                .get("schema")
                .and_then(Value::as_str)
                .is_some_and(|schema| schema.starts_with("biomcp.error"));
            for step in reply
                .get("next")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter(|_| refused)
            {
                let words: Vec<String> = step
                    .get("args")
                    .and_then(Value::as_array)
                    .ok_or("next step without args")?
                    .iter()
                    .filter_map(|word| word.as_str().map(str::to_owned))
                    .collect();
                followed += 1;
                let (code, next) = cli(&env, &words, &globals)?;
                if code != Some(0) {
                    let text: String = next.to_string().chars().take(400).collect();
                    failures.push(format!("{args:?} offered {words:?}: exit {code:?}: {text}"));
                }
            }
        }
    }
    writeln!(
        std::io::stdout(),
        "DISEASE_REFUSAL_STEPS followed={followed} failures={}",
        failures.len()
    )
    .map_err(|e| e.to_string())?;
    if followed == 0 {
        return Err("no refusal offered a step".into());
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}

fn cli(
    env: &Environment,
    args: &[String],
    globals: &[String],
) -> Result<(Option<i32>, Value), String> {
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &launch::arguments(args, globals),
        )
        .output()
        .map_err(|e| e.to_string())?;
    let reply = serde_json::from_slice(&output.stdout).unwrap_or(Value::Null);
    Ok((output.status.code(), reply))
}
