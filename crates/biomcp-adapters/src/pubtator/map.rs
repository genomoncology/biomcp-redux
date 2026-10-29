use super::raw::{Annotation, Document};
use crate::mapped::push_problem;
use crate::mapped::record as record_text;
use crate::{Mapped, raw::Field};
use biodata::{Pmid, Text};

use biomcp_types::{
    AnnotatedEntity, AnnotationKind, Annotations, Capped, Problem, Section, Source,
};
const LIMIT: usize = 20;
pub(super) fn document(id: Pmid, document: Option<&Document>) -> Mapped<Section<Annotations>> {
    let mut problems = Vec::new();
    let mut entities: Vec<(String, AnnotatedEntity)> = Vec::new();
    for annotation in document
        .into_iter()
        .flat_map(|d| &d.passages)
        .flat_map(|p| p.annotations.iter().flatten())
    {
        let Some((key, entity)) = entity(id, annotation, &mut problems) else {
            continue;
        };
        if let Some((_, existing)) = entities
            .iter_mut()
            .find(|(k, e)| k == &key && e.kind == entity.kind)
        {
            existing.count += 1;
        } else {
            entities.push((key, entity));
        }
    }
    entities.sort_by(|(ka, a), (kb, b)| {
        b.count
            .cmp(&a.count)
            .then_with(|| order(a.kind).cmp(&order(b.kind)))
            .then_with(|| a.name.as_str().cmp(b.name.as_str()))
            .then_with(|| ka.cmp(kb))
    });
    let total = u64::try_from(entities.len()).unwrap_or(u64::MAX);
    let value = if entities.is_empty() {
        Section::Empty {
            source: Source::PubTator,
            record: None,
        }
    } else {
        Section::Data {
            source: Source::PubTator,
            data: Capped {
                items: entities.into_iter().take(LIMIT).map(|(_, e)| e).collect(),
                total,
            },
        }
    };
    Mapped { value, problems }
}
fn order(kind: AnnotationKind) -> u8 {
    match kind {
        AnnotationKind::Gene => 0,
        AnnotationKind::Variant => 1,
        AnnotationKind::Disease => 2,
        AnnotationKind::Chemical => 3,
    }
}

fn optional(
    id: Pmid,
    field: &str,
    raw: Option<&Field<String>>,
    problems: &mut Vec<Problem>,
) -> Option<Text> {
    match raw {
        Some(Field::Value(s)) if s != "-" && !s.is_empty() => match Text::new(s) {
            Ok(text) => Some(text),
            Err(_) => {
                push_problem(problems, Source::PubTator, record_text(id), field);
                None
            }
        },
        Some(Field::Invalid(_)) => {
            push_problem(problems, Source::PubTator, record_text(id), field);
            None
        }
        _ => None,
    }
}
fn entity(
    id: Pmid,
    annotation: &Annotation,
    problems: &mut Vec<Problem>,
) -> Option<(String, AnnotatedEntity)> {
    let raw = &annotation.infons;
    let kind = match raw.kind.as_str() {
        "Gene" => AnnotationKind::Gene,
        "Variant" => AnnotationKind::Variant,
        "Disease" => AnnotationKind::Disease,
        "Chemical" => AnnotationKind::Chemical,
        _ => return None,
    };
    let identifier = optional(id, "identifier", raw.identifier.as_ref(), problems);
    let name = optional(id, "name", raw.name.as_ref(), problems)
        .or_else(|| annotation.text.as_ref().and_then(|s| Text::new(s).ok()))?;
    let mut value = AnnotatedEntity {
        kind,
        name,
        count: 1,
        entrez_id: None,
        mesh: None,
        hgvs: None,
        rsid: None,
    };
    match kind {
        AnnotationKind::Gene => {
            value.entrez_id = identifier.as_ref().and_then(|s| s.as_str().parse().ok());
            if identifier.is_some() && value.entrez_id.is_none() {
                push_problem(problems, Source::PubTator, record_text(id), "entrez_id");
            }
        }
        AnnotationKind::Variant => {
            value.hgvs = optional(id, "hgvs", raw.hgvs.as_ref(), problems);
            value.rsid = optional(id, "rsid", raw.rsid.as_ref(), problems);
            if value.hgvs.is_none() && value.rsid.is_none() {
                push_problem(problems, Source::PubTator, record_text(id), "variant");
                return None;
            }
            value.entrez_id = match &raw.gene_id {
                Some(Field::Value(n)) => biodata::NcbiGeneId::new(*n).ok(),
                _ => None,
            };
            if raw.gene_id.is_some() && value.entrez_id.is_none() {
                push_problem(problems, Source::PubTator, record_text(id), "gene_id");
            }
        }
        AnnotationKind::Disease | AnnotationKind::Chemical => value.mesh = identifier.clone(),
    }
    let checked_identifier = match kind {
        AnnotationKind::Gene => value.entrez_id.map(|id| id.to_string()),
        _ => identifier.map(|s| s.to_string()),
    };
    let key = checked_identifier
        .map(|id| format!("id:{id}"))
        .unwrap_or_else(|| {
            if value.hgvs.is_some() || value.rsid.is_some() {
                format!(
                    "variant:{:?}/{:?}/{:?}",
                    value.entrez_id, value.hgvs, value.rsid
                )
            } else {
                format!("name:{}", value.name)
            }
        });
    Some((key, value))
}
