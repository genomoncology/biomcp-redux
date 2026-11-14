//! One HP term's ancestor list from OLS, addressed by CURIE so no IRI path is built.
use super::SPEC;
use crate::native_text::NativeText;
use crate::raw::Field;
use biodata::{Curie, HpId};
use biomcp_fetch::{
    Arg, BadReply, Freshness, Input, InputError, Leg, Read, Reply, ReplyMetadata, RequestError,
    SourceError,
};
use biomcp_types::{ComponentLabel, ComponentTerms, PhenotypeAncestor, Source};
use serde::Deserialize;
use std::collections::BTreeMap;
/// Every ancestor OLS lists for one term, in OLS order.
#[derive(Debug)]
pub struct Ancestors {
    /// Ancestor rows; an empty list is a root term.
    pub items: Vec<PhenotypeAncestor>,
    /// Rejected ancestor fields.
    pub problems: Vec<biomcp_types::Problem>,
    /// The HPO component terms carried by OLS.
    pub components: Box<[ComponentLabel]>,
    /// Checked retrieval metadata.
    pub metadata: ReplyMetadata,
}
#[derive(Deserialize)]
struct Classes {
    elements: Vec<Class>,
}
#[derive(Deserialize)]
struct Class {
    curie: String,
    #[serde(rename = "ontologyId")]
    ontology: String,
    /// OLS's is-a closure, the same set its v1 `ancestors` endpoint lists.
    #[serde(rename = "directAncestor")]
    ancestors: Option<Field<Vec<Field<String>>>>,
    #[serde(rename = "linkedEntities")]
    linked: Option<Field<BTreeMap<String, Field<Linked>>>>,
}
#[derive(Deserialize)]
struct Linked {
    curie: Option<Field<Curie>>,
    label: Option<Field<Vec<Field<NativeText>>>>,
}
impl Reply for Classes {
    fn check(&self) -> Result<(), BadReply> {
        if self.elements.len() > 1 {
            return Err(BadReply);
        }
        Ok(())
    }
}
/// Read one HP term's ancestors; `None` is a term OLS does not hold.
pub fn ancestors(leg: &Leg<'_>, id: &HpId) -> Result<Option<Ancestors>, RequestError> {
    let component = crate::disease_sources::components::get(Source::Ols, "hp")
        .ok_or(InputError::Serialization)?;
    if component.disabled_on_leg(leg) {
        return Err(SourceError::disabled(Source::Ols).into());
    }
    let endpoint = SPEC
        .get("/api/v2/ontologies/hp/classes")
        .fresh(Freshness::Days(1));
    let received = leg.call(
        &endpoint,
        Input::Params(&[("curie", Arg::Text(id.as_str()))]),
        Read::<Classes>::json().check(&|reply| {
            if reply
                .elements
                .iter()
                .any(|class| class.curie != id.as_str() || class.ontology != "hp")
            {
                return Err(BadReply);
            }
            Ok(())
        }),
    )?;
    let Some(class) = received.value.elements.into_iter().next() else {
        return Ok(None);
    };
    let mut problems = crate::mapped::Problems {
        source: Source::Ols,
        record: crate::mapped::record(id),
        list: Vec::new(),
    };
    let iris: Vec<String> = problems
        .optional_fields("directAncestor", class.ancestors)
        .unwrap_or_default();
    let mut linked = problems
        .field("linkedEntities", class.linked)
        .unwrap_or_default();
    let mut items = Vec::new();
    for iri in iris {
        let entity = linked
            .remove(&iri)
            .and_then(|entity| problems.field("linkedEntities", Some(entity)));
        let (curie, label) = entity.map_or((None, None), |e| (e.curie, e.label));
        let Some(curie) = problems.required_field("linkedEntities.curie", curie) else {
            continue;
        };
        let labels = problems.optional_fields("linkedEntities.label", label);
        let label = match labels.as_deref() {
            Some([label]) => Some(Box::<str>::from(label.as_str())),
            Some(_) => problems.required::<Box<str>>("linkedEntities.label", None),
            None => None,
        };
        items.push(PhenotypeAncestor { id: curie, label });
    }
    let text = |value: &str| {
        crate::mapped::text(value).map_err(|_| RequestError::Input(InputError::Serialization))
    };
    Ok(Some(Ancestors {
        items,
        problems: problems.list,
        components: vec![ComponentLabel {
            component: text(component.name)?,
            terms: ComponentTerms {
                usage: component.terms.usage,
                name: text(component.terms.name)?,
                url: text(component.terms.url)?,
            },
        }]
        .into(),
        metadata: received.meta,
    }))
}
