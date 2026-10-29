use super::raw;
use crate::{mapped::Problems, raw::Field};
use biodata::{Pathway, PathwayId, PathwayLiteratureRef, PathwaySpecies, PathwaySummation, Text};
use biomcp_types::{PathwayCard, PathwayHeader, PathwaySection, Source};

pub(super) fn fields(record: &str) -> Problems {
    Problems {
        source: Source::Reactome,
        record: crate::mapped::record(record),
        list: Vec::new(),
    }
}
pub(super) fn native_text(value: Option<&Field<String>>) -> Option<&str> {
    match value {
        Some(Field::Value(value)) => Some(value),
        _ => None,
    }
}
pub(super) fn metadata(
    problems: &mut Problems,
    field: &str,
    value: Option<&Field<String>>,
    required: bool,
) -> Option<Text> {
    if value.is_none() && !required {
        return None;
    }
    let admit = if matches!(field, "name" | "references.title" | "summations.text") {
        crate::mapped::source_text
    } else {
        crate::mapped::text
    };
    let value = problems.optional(field, native_text(value).map(admit));
    problems.required(field, value)
}
fn identity(id: &str, species: &str, class: &str) -> Option<PathwayId> {
    if species != "Homo sapiens" || !matches!(class, "Pathway" | "TopLevelPathway") {
        return None;
    }
    Some(PathwayId::Reactome(id.parse::<biodata::ReactomeId>().ok()?))
}
pub(super) fn hit(row: &raw::Hit, problems: &mut Problems) -> Option<PathwayHeader> {
    let id = identity(&row.st_id, row.species.first()?, &row.exact_type)?;
    if row.species.len() != 1 {
        return None;
    }
    // Reactome inserts only these highlight spans into the native display name.
    let name = native_text(row.name.as_ref()).map(|name| {
        name.replace("<span class=\"highlighting\" >", "")
            .replace("</span>", "")
    });
    let name = problems.optional("name", name.as_deref().map(crate::mapped::source_text));
    let name = problems.required("name", name)?;
    Some(PathwayHeader {
        id,
        name,
        species: PathwaySpecies::Human,
        version: None,
        revision: None,
    })
}
pub(super) fn header(row: &raw::Card, problems: &mut Problems) -> Option<PathwayHeader> {
    let id = identity(&row.st_id, &row.species_name, &row.schema_class)?;
    let name = metadata(problems, "name", row.display_name.as_ref(), true)?;
    let prefix = format!("{}.", row.st_id);
    let version = native_text(row.st_id_version.as_ref()).filter(|v| {
        v.strip_prefix(&prefix)
            .is_some_and(|v| !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()))
    });
    let version = problems.required("version", version.and_then(|v| crate::mapped::text(v).ok()));
    Some(PathwayHeader {
        id,
        name,
        species: PathwaySpecies::Human,
        version,
        revision: None,
    })
}
pub(super) fn card(row: raw::Card, summary: bool, problems: &mut Problems) -> Option<PathwayCard> {
    let head = header(&row, problems)?;
    let summations = if summary {
        problems
            .optional("summations", row.summation.map(|v| v.value().ok_or(())))
            .map(|rows| {
                rows.into_iter()
                    .filter_map(|row| {
                        let row = problems.required("summations.identity", row.value())?;
                        let text = problems.required(
                            "summations.text",
                            row.text
                                .and_then(Field::value)
                                .filter(|v| !v.trim().is_empty()),
                        )?;
                        Some(PathwaySummation {
                            db_id: row.db_id,
                            text: text.into(),
                        })
                    })
                    .collect()
            })
    } else {
        None
    };
    let references = if summary {
        problems
            .optional(
                "references",
                row.literature_reference.map(|v| v.value().ok_or(())),
            )
            .map(|rows| {
                rows.into_iter()
                    .filter_map(|row| {
                        let row = problems.required("references.identity", row.value())?;
                        Some(literature(row, problems))
                    })
                    .collect()
            })
    } else {
        None
    };
    Some(PathwayCard {
        pathway: Pathway {
            id: head.id,
            name: head.name,
            species: head.species,
            description: None,
            summations,
            references,
            cited_in: None,
            version: head.version,
            revision: None,
        },
        release: None,
        description_truncated: None,
        sections: vec![
            PathwaySection::Summary,
            PathwaySection::Genes,
            PathwaySection::Events,
        ]
        .into(),
        genes: None,
        events: None,
    })
}
fn literature(row: raw::Literature, problems: &mut Problems) -> PathwayLiteratureRef {
    PathwayLiteratureRef {
        db_id: row.db_id,
        pubmed: problems.optional(
            "references.pubmed",
            row.pub_med_identifier
                .map(|v| v.value().and_then(|v| biodata::Pmid::new(v).ok()).ok_or(())),
        ),
        title: metadata(problems, "references.title", row.title.as_ref(), false),
        url: metadata(problems, "references.url", row.url.as_ref(), false),
    }
}
