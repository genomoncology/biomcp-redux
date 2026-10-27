use super::raw;
use crate::{Mapped, raw::Field};
use biodata::{Text, UniProtAcc};

use biomcp_types::{Problem, Protein, Source};
pub(super) fn proteins(raw: raw::Proteins, ids: &[UniProtAcc]) -> Mapped<Vec<Protein>> {
    let mut problems = Vec::new();
    let mut values = Vec::new();
    let mut seen = Vec::new();
    for raw in raw.results {
        let Field::Value(raw) = raw else {
            crate::mapped::push_problem(
                &mut problems,
                Source::UniProt,
                None.and_then(|id: &UniProtAcc| Text::new(&id.to_string()).ok()),
                "protein",
            );
            continue;
        };
        let Ok(id) = raw.primary_accession.parse::<UniProtAcc>() else {
            crate::mapped::push_problem(
                &mut problems,
                Source::UniProt,
                None.and_then(|id: &UniProtAcc| Text::new(&id.to_string()).ok()),
                "primaryAccession",
            );
            continue;
        };
        if !ids.contains(&id) || seen.contains(&id) {
            crate::mapped::push_problem(
                &mut problems,
                Source::UniProt,
                Some(&id).and_then(|id: &UniProtAcc| Text::new(&id.to_string()).ok()),
                "primaryAccession",
            );
            values.retain(|value: &Protein| value.accession != id);
            continue;
        }
        seen.push(id.clone());
        if raw.organism.taxon_id != 9606 || raw.entry_type != "UniProtKB reviewed (Swiss-Prot)" {
            crate::mapped::push_problem(
                &mut problems,
                Source::UniProt,
                Some(&id).and_then(|id: &UniProtAcc| Text::new(&id.to_string()).ok()),
                "reviewed_human",
            );
            continue;
        }
        let Ok(name) = Text::new(&raw.protein_description.recommended_name.full_name.value) else {
            crate::mapped::push_problem(
                &mut problems,
                Source::UniProt,
                Some(&id).and_then(|id: &UniProtAcc| Text::new(&id.to_string()).ok()),
                "protein_name",
            );
            continue;
        };
        if raw.sequence.length == 0 {
            crate::mapped::push_problem(
                &mut problems,
                Source::UniProt,
                Some(&id).and_then(|id: &UniProtAcc| Text::new(&id.to_string()).ok()),
                "length",
            );
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
            crate::mapped::push_problem(
                &mut problems,
                Source::UniProt,
                Some(id).and_then(|id: &UniProtAcc| Text::new(&id.to_string()).ok()),
                "missing",
            );
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
                    &mut function.extend({
                        let mut fields = crate::mapped::Problems {
                            source: Source::UniProt,
                            record: Text::new(&id.to_string()).ok(),
                            list: std::mem::take(problems),
                        };
                        let value = fields.text("function", Some(&value.value));
                        *problems = fields.list;
                        value
                    });
                }
            }
            "SUBCELLULAR LOCATION" => {
                for value in fields(
                    comment.subcellular_locations,
                    id,
                    "subcellular_locations",
                    problems,
                ) {
                    &mut locations.extend({
                        let mut fields = crate::mapped::Problems {
                            source: Source::UniProt,
                            record: Text::new(&id.to_string()).ok(),
                            list: std::mem::take(problems),
                        };
                        let value =
                            fields.text("subcellular_locations", Some(&value.location.value));
                        *problems = fields.list;
                        value
                    });
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
                    crate::mapped::push_problem(
                        problems,
                        Source::UniProt,
                        Some(id).and_then(|id: &UniProtAcc| Text::new(&id.to_string()).ok()),
                        field,
                    );
                    None
                }
            })
            .collect(),
        Some(_) => {
            crate::mapped::push_problem(
                problems,
                Source::UniProt,
                Some(id).and_then(|id: &UniProtAcc| Text::new(&id.to_string()).ok()),
                field,
            );
            Vec::new()
        }
    }
}
