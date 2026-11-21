//! Locate section rows in reply shapes outside the usual section page.
use super::rows::{Call, List, Step, items, list_items, objects, row_of, variants};
use biomcp_types::{ErrorMessage, Request};
use serde_json::Value;

fn requested(call: &Call, section: &str) -> bool {
    call.words
        .iter()
        .any(|word| word == section || word == "all")
}

/// A standalone section can carry rows inside its own result.
pub(super) fn standalone(schema: &Value, call: &Call, reply: &str) -> Vec<List> {
    if call.entity == "gwas" && reply == "gwas-page" && requested(call, "associations") {
        return items(schema);
    }
    let paths: &[(&[&str], bool)] = match (call.entity.as_str(), reply) {
        ("adverse-event", "adverse-event-drugs") if requested(call, "drugs") => {
            &[(&["drugs", "records"], true)]
        }
        ("adverse-event", "adverse-event-reactions") if requested(call, "reactions") => {
            &[(&["reactions", "records"], true)]
        }
        ("drug", "drug-markets") if requested(call, "markets") => &[(&["markets"], true)],
        ("article", "article-pdf") if requested(call, "pdf") => &[(&["file", "data"], false)],
        ("protein", "protein-sequence") if requested(call, "sequence") => &[(&["sequence"], false)],
        ("cell-line", "cell-line-identity") if requested(call, "identity") => IDENTITY,
        ("cell-line", "cell-line-xrefs") if requested(call, "xrefs") => CAPPED,
        ("cell-line", "cell-line-variants") if requested(call, "variants") => CAPPED,
        ("cell-line", "cell-line-chembl") if requested(call, "chembl") => CAPPED,
        ("variant", "variant-population") if requested(call, "population") => {
            &[(&["groups"], true)]
        }
        _ => &[],
    };
    schema
        .pointer("/properties/result")
        .map_or_else(Vec::new, |result| {
            rows(schema, &[Step::Key("result".into())], result, paths)
        })
}

const CAPPED: &[(&[&str], bool)] = &[(&["items", "items"], true)];
const IDENTITY: &[(&[&str], bool)] = &[
    (&["species", "items"], true),
    (&["diseases", "items"], true),
];

/// An object can be a single row. An object holding collections keeps its metadata whole.
pub(super) fn payload(schema: &Value, call: &Call, at: &[Step], section: &Value) -> Vec<List> {
    let Some(Step::Key(name)) = at.last() else {
        return Vec::new();
    };
    let paths: &[(&[&str], bool)] = match (call.entity.as_str(), name.as_str()) {
        ("gene", "civic" | "constraint" | "oncokb") | ("article", "tldr" | "citations" | "pdf") => {
            &[(&[], false)]
        }
        ("protein", "sequence") => &[(&["sequence"], false)],
        ("cell-line", "xrefs" | "variants" | "chembl") => CAPPED,
        ("cell-line", "identity") => IDENTITY,
        ("variant", "population") => &[
            (&["exome"], false),
            (&["genome"], false),
            (&["joint", "data"], false),
        ],
        _ => return Vec::new(),
    };
    let data_at = [at, &[Step::Key("data".into())]].concat();
    variants(section, schema)
        .into_iter()
        .filter_map(|branch| branch.pointer("/properties/data"))
        .flat_map(|data| rows(schema, &data_at, data, paths))
        .collect()
}

fn rows(schema: &Value, at: &[Step], node: &Value, paths: &[(&[&str], bool)]) -> Vec<List> {
    paths
        .iter()
        .filter_map(|(path, array)| {
            let node = path.iter().try_fold(node, |node, key| {
                variants(node, schema)
                    .into_iter()
                    .find_map(|branch| branch.get("properties")?.get(key))
            })?;
            let mut at = at.to_vec();
            at.extend(path.iter().map(|key| Step::Key((*key).into())));
            let row = if *array {
                at.push(Step::Each);
                list_items(node, schema)?
            } else {
                node
            };
            if !objects(row, schema) {
                return None;
            }
            let mut row = row_of(row, schema);
            // The sequencing stratum identifies a population observation.
            if row.names.contains("data_type") {
                row.kept.extend(
                    ["group_id", "ancestry_group", "sex"]
                        .into_iter()
                        .map(str::to_owned),
                );
            }
            row.kept.extend(
                ["reason", "record", "state"]
                    .into_iter()
                    .filter(|key| row.names.contains(*key))
                    .map(str::to_owned),
            );
            Some(List { at, row })
        })
        .collect()
}

pub(super) fn refusal(request: &Request) -> Option<ErrorMessage> {
    use biomcp_types::{CellLineSection, GeneSectionWord, PathwaySection};
    match request {
        Request::GetGene(q) if q.sections() == [GeneSectionWord::ClinVar] => {
            Some(ErrorMessage::FieldsClinvarCounts)
        }
        Request::GetCellLine(q) if q.section() == Some(CellLineSection::Names) => {
            Some(ErrorMessage::FieldsCellLineNames)
        }
        Request::GetPathway(q) if q.section() == Some(PathwaySection::Summary) => {
            Some(ErrorMessage::FieldsPathwaySummary)
        }
        _ => None,
    }
}
