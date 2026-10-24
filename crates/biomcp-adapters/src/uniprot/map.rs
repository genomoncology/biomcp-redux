use super::raw;
use crate::{Mapped, raw::Field};
use biodata::{Problem, ProblemCode, Protein, Source, Text, UniProtAcc};
pub(super) fn proteins(raw: raw::Proteins, ids: &[UniProtAcc]) -> Mapped<Vec<Protein>> {
    let mut problems = Vec::new();
    let mut values = Vec::new();
    let mut seen = Vec::new();
    for raw in raw.results {
        let Field::Value(raw) = raw else {
            problem(&mut problems, None, "protein");
            continue;
        };
        let Ok(id) = raw.primary_accession.parse::<UniProtAcc>() else {
            problem(&mut problems, None, "primaryAccession");
            continue;
        };
        if !ids.contains(&id) || seen.contains(&id) {
            problem(&mut problems, Some(&id), "primaryAccession");
            values.retain(|value: &Protein| value.accession != id);
            continue;
        }
        seen.push(id.clone());
        if raw.organism.taxon_id != 9606 || raw.entry_type != "UniProtKB reviewed (Swiss-Prot)" {
            problem(&mut problems, Some(&id), "reviewed_human");
            continue;
        }
        let Ok(name) = Text::new(&raw.protein_description.recommended_name.full_name.value) else {
            problem(&mut problems, Some(&id), "protein_name");
            continue;
        };
        if raw.sequence.length == 0 {
            problem(&mut problems, Some(&id), "length");
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
            problem(&mut problems, Some(id), "missing");
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
                    text(value.value, id, "function", problems, &mut function);
                }
            }
            "SUBCELLULAR LOCATION" => {
                for value in fields(
                    comment.subcellular_locations,
                    id,
                    "subcellular_locations",
                    problems,
                ) {
                    text(
                        value.location.value,
                        id,
                        "subcellular_locations",
                        problems,
                        &mut locations,
                    );
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
                    problem(problems, Some(id), field);
                    None
                }
            })
            .collect(),
        Some(_) => {
            problem(problems, Some(id), field);
            Vec::new()
        }
    }
}
fn text(
    value: String,
    id: &UniProtAcc,
    field: &str,
    problems: &mut Vec<Problem>,
    out: &mut Vec<Text>,
) {
    if let Ok(text) = Text::new(&value) {
        out.push(text);
    } else {
        problem(problems, Some(id), field);
    }
}
fn problem(problems: &mut Vec<Problem>, id: Option<&UniProtAcc>, field: &str) {
    problems.push(Problem {
        source: Source::UniProt,
        record: id.and_then(|id| Text::new(&id.to_string()).ok()),
        field: field.into(),
        code: ProblemCode::ValueUnknown,
    });
}
