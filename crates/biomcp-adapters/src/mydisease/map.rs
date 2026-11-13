use super::{Annotation, native};
use crate::Mapped;
use biodata::{Attribution, Curie, DiseaseEndpoint, DiseaseRelation, DiseaseSynonym};
use biomcp_fetch::{InputError, RequestError};
use biomcp_types::{
    ComponentLabel, ComponentTerms, DiseaseCard, DiseaseConcept, DiseaseSection, Source,
};

fn attribution(
    id: &Curie,
    source: &str,
    version: Option<u64>,
) -> Result<Attribution, RequestError> {
    Ok(Attribution {
        approved: None,
        published: None,
        source: crate::mapped::text(source)
            .map_err(|_| RequestError::Input(InputError::Serialization))?,
        record: Some(
            crate::mapped::text(&id.to_string())
                .map_err(|_| RequestError::Input(InputError::Serialization))?,
        ),
        version: version
            .map(|v| {
                crate::mapped::text(&v.to_string())
                    .map_err(|_| RequestError::Input(InputError::Serialization))
            })
            .transpose()?,
        release: None,
        submitter: None,
        evaluated: None,
        url: Some(
            crate::mapped::text(&format!("https://mydisease.info/v1/disease/{id}"))
                .map_err(|_| RequestError::Input(InputError::Serialization))?,
        ),
    })
}
fn terms(components: &[native::Component]) -> Result<Box<[ComponentLabel]>, RequestError> {
    components
        .iter()
        .map(|value| {
            let component = crate::disease_sources::components::get(Source::MyDisease, value.name)
                .ok_or(InputError::Serialization)?;
            Ok(ComponentLabel {
                component: crate::mapped::text(component.name)
                    .map_err(|_| RequestError::Input(InputError::Serialization))?,
                terms: ComponentTerms {
                    usage: component.terms.usage,
                    name: crate::mapped::text(component.terms.name)
                        .map_err(|_| RequestError::Input(InputError::Serialization))?,
                    url: crate::mapped::text(component.terms.url)
                        .map_err(|_| RequestError::Input(InputError::Serialization))?,
                },
            })
        })
        .collect()
}
fn component_id(component: &native::Component) -> &str {
    if component.name == "doid" {
        "disease_ontology"
    } else {
        component.name
    }
}
fn predicate(component: &native::Component, field: &str) -> Result<Curie, RequestError> {
    format!("mydisease:{}.{field}", component_id(component))
        .parse()
        .map_err(|_| InputError::Serialization.into())
}
fn endpoint(component: &native::Component) -> DiseaseEndpoint {
    DiseaseEndpoint {
        id: component.id.clone(),
        label: component.label.clone(),
    }
}
fn xref_id(value: &native::Reference) -> Option<Curie> {
    let prefix = value.field.to_ascii_uppercase();
    let native = value.value.as_str();
    if native
        .split_once(':')
        .is_some_and(|(p, _)| p.eq_ignore_ascii_case(&prefix))
    {
        native.parse().ok()
    } else {
        format!("{prefix}:{native}").parse().ok()
    }
}
pub(super) fn annotation(record: native::Record) -> Result<Mapped<Annotation>, RequestError> {
    let primary = record
        .components
        .iter()
        .find(|c| c.id == record.returned_id)
        .ok_or(InputError::Serialization)?;
    let mut mapped = Mapped {
        refusals: crate::Refusal::None,
        value: Annotation {
            card: DiseaseCard {
                requested_id: record.requested_id.clone(),
                returned_id: record.returned_id.clone(),
                alias: None,
                concept: DiseaseConcept {
                    id: Some(primary.id.clone()),
                    label: primary.label.clone(),
                },
                definition: primary.definition.clone(),
                obsolete: primary.obsolete,
                consider: primary.consider.clone(),
                replaced_by: primary.replaced_by.clone(),
                attribution: attribution(&record.returned_id, "mydisease", record.version)?,
                sections: Box::default(),
                components: terms(&record.components)?,
            },
            synonyms: Vec::new(),
            xrefs: Vec::new(),
            parents: Vec::new(),
        },
        problems: record.problems,
    };
    for component in &record.components {
        mapped.problems.extend(component.problems.iter().cloned());
        rows(component, &mut mapped)?;
    }
    let value = &mut mapped.value;
    if record.requested_id != record.returned_id {
        value.card.alias = value
            .xrefs
            .iter()
            .find(|r| {
                r.subject.id == record.returned_id
                    && r.object.id == record.requested_id
                    && r.predicate.to_string() == "mydisease:mondo.xrefs.doid"
            })
            .cloned();
        if value.card.alias.is_none() {
            return Err(InputError::Serialization.into());
        }
    }
    value.card.sections = [
        (!value.synonyms.is_empty(), DiseaseSection::Synonyms),
        (!value.xrefs.is_empty(), DiseaseSection::Xrefs),
        (!value.parents.is_empty(), DiseaseSection::Hierarchy),
    ]
    .into_iter()
    .filter_map(|(present, section)| present.then_some(section))
    .collect();
    Ok(mapped)
}
fn rows(
    component: &native::Component,
    mapped: &mut Mapped<Annotation>,
) -> Result<(), RequestError> {
    let attribution = attribution(&component.id, component.name, None)?;
    let synonym_field = if component.name == "doid" {
        "synonyms"
    } else {
        "synonym"
    };
    let mut problems = crate::mapped::Problems {
        source: Source::MyDisease,
        record: crate::mapped::record(&component.id),
        list: Vec::new(),
    };
    for value in &component.synonyms {
        let field = format!("{synonym_field}.{}", value.field);
        let scope = problems.optional(
            &format!("{}.{}", component_id(component), field),
            Some(predicate(component, &field)),
        );
        let row = DiseaseSynonym {
            value: value.value.clone(),
            scope,
            language: None,
            attribution: attribution.clone(),
        };
        // One row per distinct text; the first row keeps it and later rows are listed with it.
        let text = row.value.as_str().to_lowercase();
        match mapped
            .value
            .synonyms
            .iter_mut()
            .find(|item| item.synonym.value.as_str().to_lowercase() == text)
        {
            Some(item) => {
                let mut also = std::mem::take(&mut item.also_stated_by).into_vec();
                also.push(row);
                item.also_stated_by = also.into();
            }
            None => mapped.value.synonyms.push(row.into()),
        }
    }
    mapped.problems.extend(problems.list);
    for value in &component.xrefs {
        let Some(id) = xref_id(value) else {
            crate::mapped::push_problem(
                &mut mapped.problems,
                Source::MyDisease,
                crate::mapped::record(&component.id),
                &format!("{}.xrefs.{}", component_id(component), value.field),
            );
            continue;
        };
        mapped.value.xrefs.push(DiseaseRelation {
            subject: endpoint(component),
            predicate: predicate(component, &format!("xrefs.{}", value.field))?,
            object: DiseaseEndpoint { id, label: None },
            attribution: attribution.clone(),
            description: None,
        });
    }
    for id in &component.parents {
        mapped.value.parents.push(DiseaseRelation {
            subject: endpoint(component),
            predicate: predicate(component, "parents")?,
            object: DiseaseEndpoint {
                id: id.clone(),
                label: None,
            },
            attribution: attribution.clone(),
            description: None,
        });
    }
    Ok(())
}

/// Keep a parent label only when the parent's own component record states it.
pub(super) fn labels(
    hits: super::raw::LabelHits,
    ids: &[Curie],
) -> Mapped<std::collections::HashMap<String, biodata::Text>> {
    let mut problems = Vec::new();
    let mut value = std::collections::HashMap::new();
    for id in ids {
        let key = id.to_string();
        let doid = key.starts_with("DOID:");
        let root = if doid { "disease_ontology" } else { "mondo" };
        let field = format!("{root}.{}", if doid { "name" } else { "label" });
        let mut problem = crate::mapped::Problems {
            source: Source::MyDisease,
            record: crate::mapped::record(&key),
            list: Vec::new(),
        };
        let mut found: Vec<biodata::Text> = Vec::new();
        for hit in hits.0.iter().filter(|hit| hit.query == key) {
            let component = if doid {
                hit.disease_ontology.as_ref()
            } else {
                hit.mondo.as_ref()
            };
            let Some(component) = component.and_then(crate::raw::Field::as_ref) else {
                continue;
            };
            if component.id.as_ref().and_then(crate::raw::Field::as_ref) != Some(id) {
                continue;
            }
            let label = match &component.label {
                Some(crate::raw::Field::Value(label)) => Some(biodata::Text::from(label.clone())),
                Some(crate::raw::Field::Invalid(_)) => problem.required(&field, None),
                None => None,
            };
            if let Some(label) = label
                && !found.contains(&label)
            {
                found.push(label);
            }
        }
        match found.as_slice() {
            [label] => {
                value.insert(key, label.clone());
            }
            [] => {}
            _ => {
                problem.required::<()>(&field, None);
            }
        }
        problems.extend(problem.list);
    }
    Mapped {
        value,
        problems,
        refusals: crate::Refusal::None,
    }
}
