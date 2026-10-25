use super::{components, raw};
use biodata::{
    Attribution, Concept, Curie, DiseaseAssociation, DiseaseAssociationQualifier,
    DiseaseAssociationReference, Text,
};
use biomcp_fetch::{InputError, RequestError};
use biomcp_types::DiseaseAssociationItem;
pub(super) fn row(
    raw: raw::Row,
    requested: &Curie,
) -> Result<DiseaseAssociationItem, RequestError> {
    let mut qualifiers = raw
        .qualifiers
        .unwrap_or_default()
        .into_vec()
        .into_iter()
        .map(|id| qualifier("qualifiers", id, None))
        .collect::<Result<Vec<_>, _>>()?;
    for (field, id, label) in [
        (
            "frequency_qualifier",
            raw.frequency_qualifier,
            raw.frequency_qualifier_label,
        ),
        (
            "onset_qualifier",
            raw.onset_qualifier,
            raw.onset_qualifier_label,
        ),
        ("sex_qualifier", raw.sex_qualifier, raw.sex_qualifier_label),
        (
            "disease_context_qualifier",
            raw.disease_context_qualifier,
            raw.disease_context_qualifier_label,
        ),
        (
            "species_context_qualifier",
            raw.species_context_qualifier,
            raw.species_context_qualifier_label,
        ),
        (
            "stage_qualifier",
            raw.stage_qualifier,
            raw.stage_qualifier_label,
        ),
    ] {
        if let Some(id) = id {
            qualifiers.push(qualifier(field, id, label)?);
        }
    }
    Ok(DiseaseAssociationItem {
        association: DiseaseAssociation {
            requested_id: requested.clone(),
            subject: Concept {
                id: Some(raw.subject),
                label: raw.subject_label,
            },
            object: Concept {
                id: Some(raw.object),
                label: raw.object_label,
            },
            predicate: raw.predicate,
            original_subject: raw.original_subject,
            original_object: raw.original_object,
            attribution: Attribution {
                source: Text::new("monarch").map_err(|_| InputError::Serialization)?,
                record: Some(raw.id),
                url: raw.provided_by_link.map(|link| link.url),
                version: None,
                release: None,
                submitter: None,
                evaluated: None,
            },
            primary_knowledge_source: raw.primary_knowledge_source,
            aggregator_knowledge_source: raw.aggregator_knowledge_source,
            provided_by: raw.provided_by,
            evidence: references(raw.has_evidence, raw.has_evidence_links),
            publications: references(raw.publications, raw.publications_links),
            qualifiers: qualifiers.into(),
            original_predicate: raw.original_predicate,
            category: raw.category,
            agent_type: raw.agent_type,
            knowledge_level: raw.knowledge_level,
            evidence_count: raw.evidence_count,
            negated: raw.negated,
            has_count: raw.has_count,
            has_total: raw.has_total,
            has_percentage: raw.has_percentage,
            has_quotient: raw.has_quotient,
            subject_closure: raw.subject_closure.unwrap_or_default(),
            object_closure: raw.object_closure.unwrap_or_default(),
        },
        components: components()?,
    })
}
fn qualifier(
    field: &str,
    id: Curie,
    label: Option<Text>,
) -> Result<DiseaseAssociationQualifier, RequestError> {
    Ok(DiseaseAssociationQualifier {
        field: Text::new(field).map_err(|_| InputError::Serialization)?,
        id,
        label,
    })
}
fn references(
    ids: Option<Box<[Curie]>>,
    links: Option<Box<[raw::Reference]>>,
) -> Box<[DiseaseAssociationReference]> {
    let mut links = links.unwrap_or_default().into_vec();
    let mut references = Vec::new();
    for id in ids.unwrap_or_default() {
        let url = links
            .iter()
            .position(|link| link.id == id)
            .map(|index| links.remove(index).url);
        references.push(DiseaseAssociationReference {
            id,
            label: None,
            url,
        });
    }
    references.extend(links.into_iter().map(|link| DiseaseAssociationReference {
        id: link.id,
        label: None,
        url: Some(link.url),
    }));
    references.into()
}
