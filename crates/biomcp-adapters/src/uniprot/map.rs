use super::raw;
use crate::mapped::push_problem;
use crate::mapped::record as record_text;
use crate::{Mapped, raw::Field};
use biodata::{Text, UniProtAcc};

use biomcp_types::{Problem, Protein, Source};
pub(super) fn proteins(raw: raw::Proteins, ids: &[UniProtAcc]) -> Mapped<Vec<Protein>> {
    let mut problems = Vec::new();
    let mut values = Vec::new();
    let mut seen = Vec::new();
    for raw in raw.results {
        let Field::Value(raw) = raw else {
            push_problem(&mut problems, Source::UniProt, None, "protein");
            continue;
        };
        let Ok(id) = raw.primary_accession.parse::<UniProtAcc>() else {
            push_problem(&mut problems, Source::UniProt, None, "primaryAccession");
            continue;
        };
        if !ids.contains(&id) || seen.contains(&id) {
            push_problem(
                &mut problems,
                Source::UniProt,
                record_text(&id),
                "primaryAccession",
            );
            values.retain(|value: &Protein| value.accession != id);
            continue;
        }
        seen.push(id.clone());
        if raw.organism.taxon_id != 9606 || raw.entry_type != "UniProtKB reviewed (Swiss-Prot)" {
            push_problem(
                &mut problems,
                Source::UniProt,
                record_text(&id),
                "reviewed_human",
            );
            continue;
        }
        let Ok(name) = Text::from_source(&raw.protein_description.recommended_name.full_name.value)
        else {
            push_problem(
                &mut problems,
                Source::UniProt,
                record_text(&id),
                "protein_name",
            );
            continue;
        };
        if raw.sequence.length == 0 {
            push_problem(&mut problems, Source::UniProt, record_text(&id), "length");
            continue;
        }
        let (function, subcellular_locations) = comments(raw.comments, &id, &mut problems);
        values.push(Protein {
            accession: id,
            name,
            length: raw.sequence.length,
            function: function.into(),
            subcellular_locations: subcellular_locations.into(),
        });
    }
    for id in ids {
        if !seen.contains(id) {
            push_problem(&mut problems, Source::UniProt, record_text(id), "missing");
        }
    }
    Mapped {
        value: values,
        problems,
    }
}
fn comments(
    raw: Option<Field<Vec<Field<raw::Comment>>>>,
    id: &UniProtAcc,
    problems: &mut Vec<Problem>,
) -> (Vec<Text>, Vec<Text>) {
    let mut function = Vec::new();
    let mut locations = Vec::new();
    for comment in fields(raw, id, "comments", problems) {
        match comment.comment_type.as_str() {
            "FUNCTION" => {
                for value in fields(comment.texts, id, "function", problems) {
                    function.extend(crate::mapped::field_text(
                        problems,
                        Source::UniProt,
                        record_text(id),
                        "function",
                        Some(Field::Value(value.value)),
                    ));
                }
            }
            "SUBCELLULAR LOCATION" => {
                for value in fields(
                    comment.subcellular_locations,
                    id,
                    "subcellular_locations",
                    problems,
                ) {
                    locations.extend(crate::mapped::field_text(
                        problems,
                        Source::UniProt,
                        record_text(id),
                        "subcellular_locations",
                        Some(Field::Value(value.location.value)),
                    ));
                }
            }
            _ => {}
        }
    }
    (function, locations)
}
fn fields<T>(
    raw: Option<Field<Vec<Field<T>>>>,
    id: &UniProtAcc,
    field: &str,
    problems: &mut Vec<Problem>,
) -> Vec<T> {
    match raw {
        None => Vec::new(),
        Some(Field::Value(values)) => values
            .into_iter()
            .filter_map(|value| {
                if let Field::Value(value) = value {
                    Some(value)
                } else {
                    push_problem(problems, Source::UniProt, record_text(id), field);
                    None
                }
            })
            .collect(),
        Some(_) => {
            push_problem(problems, Source::UniProt, record_text(id), field);
            Vec::new()
        }
    }
}
