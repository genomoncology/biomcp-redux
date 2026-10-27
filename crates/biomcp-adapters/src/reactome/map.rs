use super::raw;
use biodata::{Pathway, PathwayId, PathwayLiteratureRef, PathwaySpecies, PathwaySummation, Text};
use biomcp_types::{PathwayCard, PathwayHeader, PathwaySection, Source};

fn identity(id: &str, name: &str, species: &str, class: &str) -> Option<PathwayHeader> {
    if species != "Homo sapiens" || !matches!(class, "Pathway" | "TopLevelPathway") {
        return None;
    }
    let id = id.parse::<biodata::ReactomeId>().ok()?;
    Some(PathwayHeader {
        id: PathwayId::Reactome(id),
        name: Text::new(name).ok()?,
        species: PathwaySpecies::Human,
        version: None,
        revision: None,
    })
}
pub(super) fn hit(row: &raw::Hit) -> Option<PathwayHeader> {
    if row.species.len() != 1 {
        return None;
    }
    // Reactome inserts only these highlight spans into the native display name.
    let name = row
        .name
        .replace("<span class=\"highlighting\" >", "")
        .replace("</span>", "");
    identity(&row.st_id, &name, row.species.first()?, &row.exact_type)
}
pub(super) fn header(row: &raw::Card) -> Option<PathwayHeader> {
    let mut header = identity(
        &row.st_id,
        &row.display_name,
        &row.species_name,
        &row.schema_class,
    )?;
    let prefix = format!("{}.", row.st_id);
    let version = row.st_id_version.strip_prefix(&prefix)?;
    if version.is_empty() || !version.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    header.version = Some(Text::new(&row.st_id_version).ok()?);
    Some(header)
}
pub(super) fn card(row: raw::Card, summary: bool) -> Option<PathwayCard> {
    let head = header(&row)?;
    let summations = if summary {
        row.summation.map(|rows| {
            rows.into_iter()
                .map(|r| PathwaySummation {
                    db_id: r.db_id,
                    text: r.text,
                })
                .collect()
        })
    } else {
        None
    };
    let references = if summary {
        match row.literature_reference {
            Some(rows) => Some(
                rows.into_iter()
                    .map(literature)
                    .collect::<Option<Box<[_]>>>()?,
            ),
            None => None,
        }
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
fn literature(row: raw::Literature) -> Option<PathwayLiteratureRef> {
    Some(PathwayLiteratureRef {
        db_id: row.db_id,
        pubmed: row
            .pub_med_identifier
            .map(biodata::Pmid::new)
            .transpose()
            .ok()?,
        title: row.title.as_deref().map(Text::new).transpose().ok()?,
        url: row.url.as_deref().map(Text::new).transpose().ok()?,
    })
}
