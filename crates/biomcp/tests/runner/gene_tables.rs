//! Response-boundary tables use the same binary, schema and isolation as YAML cases.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Checked source fixtures and their response-only mutations must fail at the malformed shape"
)]
mod graphql;
mod hgnc;
mod mygene;

use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};

type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    [
        (
            "gene-commands/hgnc-boundaries",
            hgnc::check as fn() -> Check,
        ),
        ("gene-commands/mygene-boundaries", mygene::check),
        ("gene-sections/graphql-boundaries", graphql::check),
    ]
    .into_iter()
    .map(|(name, check)| Trial::test(name, move || check().map_err(|e| e.to_string().into())))
    .collect()
}

fn case(
    capture: &str,
    args: &[&str],
    mut change: impl FnMut(&str, &mut Value),
    expect: Value,
    exit: i32,
) -> Check {
    let directory = tempfile::tempdir()?;
    let recordings = directory.path().join("recordings");
    let original = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/replies")
        .join(capture);
    for source in fs::read_dir(original)? {
        let source = source?;
        if !source.file_type()?.is_dir() {
            continue;
        }
        let name = source.file_name().to_string_lossy().into_owned();
        fs::create_dir_all(recordings.join(&name))?;
        for file in fs::read_dir(source.path())? {
            let file = file?;
            if !file.file_type()?.is_file() || file.path().extension().is_none_or(|v| v != "json") {
                continue;
            }
            let mut record = serde_json::from_slice(&fs::read(file.path())?)?;
            change(&name, &mut record);
            fs::write(
                recordings.join(&name).join(file.file_name()),
                serde_json::to_vec(&record)?,
            )?;
        }
    }
    let path = directory.path().join("case.yaml");
    fs::write(
        &path,
        serde_json::to_vec(&json!({
            "feature": "gene-commands",
            "story": "A checked command keeps valid source facts and reports malformed siblings",
            "replay": recordings,
            "steps": [{"run": args, "expect": expect, "exit": exit}]
        }))?,
    )?;
    super::run_case(&path, &super::schema::Registry::embedded()?)?;
    Ok(())
}

fn tsv(record: &mut Value, symbol: &str, edits: &[(&str, &str)]) {
    let Some(body) = record.pointer_mut("/exchanges/0/body/value") else {
        return;
    };
    let Some(text) = body.as_str() else { return };
    let mut lines = text.lines();
    let Some(header) = lines.next() else { return };
    let fields: Vec<_> = header.split('\t').collect();
    let Some(index) = fields
        .iter()
        .position(|v| matches!(*v, "symbol" | "WITHDRAWN_SYMBOL"))
    else {
        return;
    };
    let mut output = vec![header.to_owned()];
    for line in lines {
        let mut cells: Vec<_> = line.split('\t').map(str::to_owned).collect();
        if cells.get(index).is_some_and(|v| v == symbol) {
            for (field, value) in edits {
                if let Some(i) = fields.iter().position(|v| v == field)
                    && let Some(cell) = cells.get_mut(i) {
                    *cell = (*value).into();
                }
            }
        }
        output.push(cells.join("\t"));
    }
    *body = format!("{}\n", output.join("\n")).into();
}
