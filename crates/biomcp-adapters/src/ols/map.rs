use super::{Record, raw};
use crate::Mapped;
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
        record: Text::new(&id.to_string()).ok(),
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
    let consider = ids(term.annotation.consider.as_deref().unwrap_or_default())?;
    let replaced_by = term
        .term_replaced_by
        .as_ref()
        .map(|values| ids(values))
        .transpose()?
        .flatten();
    let definition = if term.description.is_empty() {
        None
    } else {
        Some(
            crate::mapped::text(
                &term
                    .description
                    .iter()
                    .map(Text::as_str)
                    .collect::<Vec<_>>()
                    .join("\n\n"),
            )
            .map_err(|_| RequestError::Input(InputError::Serialization))?,
        )
    };
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
    })
}
fn ids(values: &[Text]) -> Result<Option<Box<[Curie]>>, RequestError> {
    if values.is_empty() {
        return Ok(None);
    }
    values
        .iter()
        .map(|v| iri_id(v.as_str()).ok_or_else(|| InputError::Serialization.into()))
        .collect::<Result<Vec<_>, _>>()
        .map(|v| Some(v.into()))
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
        let native = term
            .obo_synonym
            .as_deref()
            .unwrap_or_default()
            .iter()
            .find(|v| &v.name == name);
        let scope = native
            .and_then(|s| s.scope.as_ref())
            .and_then(|s| format!("oboInOwl:{}", s.as_str()).parse().ok());
        if native.is_some_and(|s| s.scope.is_some()) && scope.is_none() {
            mapped.problems.push(crate::mapped::problem(
                Source::Ols,
                Text::new(&&term.obo_id.to_string()).ok(),
                "synonym.scope",
            ));
        }
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
        let Ok(id) = format!("{}:{}", native.database, native.id).parse() else {
            mapped.problems.push(crate::mapped::problem(
                Source::Ols,
                Text::new(&&term.obo_id.to_string()).ok(),
                "xref.id",
            ));
            continue;
        };
        let Ok(predicate) = "oboInOwl:hasDbXref".parse() else {
            continue;
        };
        mapped.value.push(DiseaseRelation {
            subject: DiseaseEndpoint {
                id: term.obo_id.clone(),
                label: Some(term.label.clone()),
            },
            predicate,
            object: DiseaseEndpoint { id, label: None },
            attribution: attribution.clone(),
            description: native.description.clone(),
        });
    }
    mapped
}
