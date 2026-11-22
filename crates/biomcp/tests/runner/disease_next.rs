//! Feature disease-mydisease: every step a refused disease read offers runs, for each ID namespace in
//! its printed and lowercase spelling, with and without each section.
//! Feature disease-hpo-release: pages stay pinned and an old filtered cache is refreshed.
use super::launch::{self, Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, io::Write};

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
    vec![
        super::transcript::trial("disease-mydisease/refusal-steps-run", || {
            run().map_err(Into::into)
        }),
        super::transcript::trial("disease-hpo-release/current-release-advances", || {
            hpo_release_advances().map_err(Into::into)
        }),
        super::transcript::trial("disease-hpo-release/refreshes-filtered-cache", || {
            refreshes_filtered_cache().map_err(Into::into)
        }),
    ]
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

fn hpo_release_advances() -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let replay = env.root().join("replay");
    let input = launch::fixtures().join("replies/disease/hpo-release/hpo");
    let target = replay.join("hpo");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    let mut latest = None;
    for entry in fs::read_dir(input).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let record = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        let path = target.join(entry.file_name());
        if record.contains("/releases/latest") {
            fs::write(&path, record.replace("v2099-02-01", "v2099-01-01"))
                .map_err(|error| error.to_string())?;
            latest = Some((path, record));
        } else {
            fs::write(path, record).map_err(|error| error.to_string())?;
        }
    }
    let (path, new_release) = latest.ok_or("missing latest release recording")?;
    let first_args = [
        "search",
        "disease",
        "--source",
        "hpo",
        "--phenotype",
        "HP:0001083",
        "--phenotype",
        "HP:0000098",
        "--phenotype",
        "HP:0001634",
        "--limit",
        "1",
    ]
    .map(str::to_owned);
    let globals = env.globals(Some(&replay), false);
    let (code, first) = cli(&env, &first_args, &globals)?;
    if code != Some(0) || first.pointer("/result/release") != Some(&json!("v2099-01-01")) {
        return Err(format!(
            "first search missed the publisher's original release: {first}"
        ));
    }
    fs::write(path, new_release).map_err(|error| error.to_string())?;
    let (code, new) = cli(&env, &first_args, &globals)?;
    if code != Some(0)
        || new.pointer("/result/items/0/gene_links/0/symbol") != Some(&json!("SYNTH3"))
    {
        return Err(format!(
            "a new search did not pick up the new release: {new}"
        ));
    }
    let (code, next) = cli(&env, &super::action(&first, "page")?, &globals)?;
    if code != Some(0)
        || next.pointer("/result/release") != Some(&json!("v2099-01-01"))
        || next.pointer("/result/items/0/gene_links_missing") != Some(&json!(true))
    {
        return Err(format!(
            "continuation changed release when the publisher updated it: {next}"
        ));
    }
    Ok(())
}

fn refreshes_filtered_cache() -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let replay = env.root().join("replay");
    super::response_cases::cache_capture(
        "disease/hpo-release",
        &replay,
        super::response_cases::CACHE_ACQUIRED,
    )?;
    let args = [
        "disease",
        "search",
        "--hpo-release",
        "v2099-01-01",
        "--phenotype",
        "HP:0001083,HP:0000098,HP:0001634",
    ]
    .map(str::to_owned);
    let globals = env.globals(Some(&replay), false);
    let (code, first) = cli(&env, &args, &globals)?;
    if code != Some(0) {
        return Err(format!("could not populate the HPO store: {first}"));
    }
    restore_filtered_annotation(&env).map_err(|error| error.to_string())?;
    let (code, refreshed) = cli(&env, &args, &globals)?;
    if code != Some(0) || refreshed.pointer("/result/items/1/id") != Some(&json!("OMIM:900007")) {
        return Err(format!(
            "did not refresh the old filtered annotation: {refreshed}"
        ));
    }
    let (code, offline) = cli(&env, &args, &env.globals(None, true))?;
    if code != Some(0) || offline.get("result") != refreshed.get("result") {
        return Err(format!(
            "the refreshed HPO file was not reusable offline: {offline}"
        ));
    }
    Ok(())
}

fn restore_filtered_annotation(env: &Environment) -> Result<(), Box<dyn std::error::Error>> {
    use sha2::{Digest, Sha256};
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))?;
    let (url, body): (String, Vec<u8>) = db.query_row(
        "SELECT url, body FROM reply WHERE source='hpo' AND url LIKE '%/phenotype.hpoa'",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    let filtered: String = std::str::from_utf8(&body)?
        .split_inclusive('\n')
        .filter(|line| line.split('\t').nth(5) != Some("IEA"))
        .collect();
    let legacy_key = Sha256::digest(format!("GET\0{url}").as_bytes()).to_vec();
    db.execute(
        "UPDATE reply SET key=?1,body=?2 WHERE url=?3",
        rusqlite::params![legacy_key, filtered.as_bytes(), url],
    )?;
    Ok(())
}
