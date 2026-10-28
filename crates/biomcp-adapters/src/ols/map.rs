use super::{Record, raw};
use crate::Mapped;
use crate::mapped::record as record_text;
use biodata::{
    Attribution, Concept, Curie, DiseaseEndpoint, DiseaseRelation, DiseaseSynonym, Text,
};
use biomcp_fetch::{InputError, RequestError};
use biomcp_types::{
    ComponentLabel, ComponentTerms, DiseaseCard, DiseaseOntology, DiseaseSection, Source,
};
pub(super) fn attribution(id: &Curie, iri: &Text, ontology: DiseaseOntology) -> Attribution {
    Attribution {
        source: Text::new(ontology.as_str()).unwrap_or_else(|_| iri.clone()),
        record: record_text(id),
        url: Some(iri.clone()),
        version: None,
        release: None,
        submitter: None,
        evaluated: None,
    }
}

/// Reviewed component terms remain separate from transport service terms.
pub fn components(ontology: DiseaseOntology) -> Result<Box<[ComponentLabel]>, RequestError> {
    let component = crate::disease_sources::components::get(Source::Ols, ontology.as_str())
        .ok_or(InputError::Serialization)?;
    Ok(vec![ComponentLabel {
        component: crate::mapped::text(component.name)
            .map_err(|_| RequestError::Input(InputError::Serialization))?,
        terms: ComponentTerms {
            usage: component.terms.usage,
            name: crate::mapped::text(component.terms.name)
                .map_err(|_| RequestError::Input(InputError::Serialization))?,
            url: crate::mapped::text(component.terms.url)
                .map_err(|_| RequestError::Input(InputError::Serialization))?,
        },
    }]
    .into())
}
pub(super) fn record(term: raw::Term, ontology: DiseaseOntology) -> Result<Record, RequestError> {
    let attribution = attribution(&term.obo_id, &term.iri, ontology);
    let synonyms = synonyms(&term, &attribution);
    let xrefs = xrefs(&term, &attribution);
    let mut sections = Vec::new();
    if !synonyms.value.is_empty() {
        sections.push(DiseaseSection::Synonyms);
    }
    if !xrefs.value.is_empty() {
        sections.push(DiseaseSection::Xrefs);
    }
    let mut problems = crate::mapped::Problems { source: Source::Ols, record: record_text(&term.obo_id), list: Vec::new() };
    let consider = ids(term.annotation.consider.as_deref().unwrap_or_default(), "annotation.consider", &mut problems);
    let replaced_by = term.term_replaced_by.as_ref().and_then(|values| ids(values, "term_replaced_by", &mut problems));
    let descriptions: Vec<_> = term.description.iter().filter_map(|value| match value {
        crate::raw::Field::Value(v) => Some(v.as_str()),
        crate::raw::Field::Invalid(_) => { problems.required::<()>("description", None); None }
    }).collect();
    let definition = if descriptions.is_empty() { None } else { problems.text("description", Some(&descriptions.join("\n\n"))) };
    Ok(Record {
        card: DiseaseCard {
            requested_id: term.obo_id.clone(),
            returned_id: term.obo_id.clone(),
            alias: None,
            concept: Concept {
                id: Some(term.obo_id),
                label: term.label,
            },
            definition,
            obsolete: term.is_obsolete,
            consider,
            replaced_by,
            attribution,
            sections: sections.into(),
            components: components(ontology)?,
        },
        synonyms,
        xrefs,
        problems: problems.list,
    })
}
fn ids(values: &[crate::raw::Field<Text>], field: &str, problems: &mut crate::mapped::Problems) -> Option<Box<[Curie]>> {
    if values.is_empty() { return None; }
    Some(values.iter().filter_map(|v| problems.required(field, match v {
        crate::raw::Field::Value(v) => iri_id(v.as_str()),
        crate::raw::Field::Invalid(_) => None,
    })).collect())
}
fn iri_id(value: &str) -> Option<Curie> {
    if let Some(native) = value.strip_prefix("http://purl.obolibrary.org/obo/") {
        let (prefix, local) = native.split_once('_')?;
        format!("{prefix}:{local}").parse().ok()
    } else {
        value.parse().ok()
    }
}

fn synonyms(term: &raw::Term, attribution: &Attribution) -> Mapped<Vec<DiseaseSynonym>> {
    let mut mapped = Mapped {
        value: Vec::new(),
        problems: Vec::new(),
    };
    for name in &term.synonyms {
        let crate::raw::Field::Value(name) = name else {
            crate::mapped::push_problem(&mut mapped.problems, Source::Ols, record_text(&term.obo_id), "synonyms");
            continue;
        };
        let native = term.obo_synonym.as_deref().unwrap_or_default().iter().find(|v| matches!(&v.name, crate::raw::Field::Value(v) if v == name));
        let mut problems = crate::mapped::Problems { source: Source::Ols, record: record_text(&term.obo_id), list: Vec::new() };
        let scope = problems.optional("synonym.scope", native.and_then(|s| s.scope.as_ref()).map(|s| match s {
            crate::raw::Field::Value(s) => format!("oboInOwl:{}", s.as_str()).parse().map_err(|_| ()),
            crate::raw::Field::Invalid(_) => Err(()),
        }));
        mapped.problems.extend(problems.list);
        mapped.value.push(DiseaseSynonym {
            value: name.clone(),
            scope,
            language: None,
            attribution: attribution.clone(),
        });
    }
    mapped
}
fn xrefs(term: &raw::Term, attribution: &Attribution) -> Mapped<Vec<DiseaseRelation>> {
    let mut mapped = Mapped {
        value: Vec::new(),
        problems: Vec::new(),
    };
    for native in term.obo_xref.as_deref().unwrap_or_default() {
        let mut problems = crate::mapped::Problems { source: Source::Ols, record: record_text(&term.obo_id), list: Vec::new() };
        let id = problems.required("xref.id", match (&native.database, &native.id) {
            (crate::raw::Field::Value(database), crate::raw::Field::Value(id)) => format!("{database}:{id}").parse().ok(),
            _ => None,
        });
        let description = problems.optional("xref.description", native.description.as_ref().map(|v| match v {
            crate::raw::Field::Value(v) => Ok::<_, ()>(v.as_str().into()),
            crate::raw::Field::Invalid(_) => Err(()),
        }));
        mapped.problems.extend(problems.list);
        let Some(id) = id else { continue; };
        let Ok(predicate) = "oboInOwl:hasDbXref".parse() else { continue; };
        mapped.value.push(DiseaseRelation {
            subject: DiseaseEndpoint {
                id: term.obo_id.clone(),
                label: Some(term.label.clone()),
            },
            predicate,
            object: DiseaseEndpoint { id, label: None },
            attribution: attribution.clone(),
            description,
        });
    }
    mapped
}
