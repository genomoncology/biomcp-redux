//! Response-boundary tables use the same binary, schema and isolation as YAML cases.
#![expect(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "Checked source fixtures and their response-only mutations must fail at the malformed shape"
)]
mod graphql;
mod hgnc;
mod metadata;
mod mygene;
mod rejections;

use super::response_cases::case;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::Value;

type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    [
        (
            "gene-identity/gene-commands/hgnc-boundaries",
            hgnc::check as fn() -> Check,
        ),
        (
            "gene-identity/gene-commands/mygene-boundaries",
            mygene::check,
        ),
        ("gene-sections/graphql-boundaries", graphql::check),
        ("gene-bundle/bundle-rejections", rejections::check),
        (
            "gene-sections/uniprot-metadata-regression",
            metadata::uniprot,
        ),
        (
            "pathway-reactome/reactome-metadata-regression",
            metadata::reactome,
        ),
        (
            "pathway-reactome/native-summation-keeps-source-text",
            metadata::native_summation,
        ),
    ]
    .into_iter()
    .map(|(name, check)| transcript_trial(name, move || check().map_err(|e| e.to_string().into())))
    .collect()
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
                    && let Some(cell) = cells.get_mut(i)
                {
                    *cell = (*value).into();
                }
            }
        }
        output.push(cells.join("\t"));
    }
    *body = format!("{}\n", output.join("\n")).into();
}
