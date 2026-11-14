//! One HP term's ancestor list from OLS, addressed by CURIE so no IRI path is built.
use super::SPEC;
use crate::native_text::NativeText;
use crate::raw::{Field, OneOrMany};
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
    /// Ancestor rows; an empty list is a root or obsolete term.
    pub items: Vec<PhenotypeAncestor>,
    /// Whether an ancestor OLS listed was left out because its identity was missing.
    pub refusals: crate::Refusal,
    /// Whether OLS marks the term obsolete.
    pub obsolete: bool,
    /// Terms OLS names as replacing an obsolete term.
    pub replaced_by: Vec<PhenotypeAncestor>,
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
    #[serde(rename = "isObsolete")]
    obsolete: Option<Field<bool>>,
    /// IAO:0100001 "term replaced by", one CURIE or several.
    #[serde(rename = "http://purl.obolibrary.org/obo/IAO_0100001")]
    replaced_by: Option<Field<OneOrMany<Field<Curie>>>>,
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
        // OLS hides obsolete terms unless asked, which would read like an unknown ID.
        Input::Params(&[
            ("curie", Arg::Text(id.as_str())),
            ("includeObsoleteEntities", Arg::Text("true")),
        ]),
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
    let obsolete = problems.field("isObsolete", class.obsolete) == Some(true);
    let replacements: Vec<Curie> = problems
        .field("IAO_0100001", class.replaced_by)
        .map(OneOrMany::values)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|curie| problems.field("IAO_0100001", Some(curie)))
        .collect();
    let mut refusals = crate::Refusal::None;
    let mut items = Vec::new();
    for iri in iris {
        let entity = linked.remove(&iri);
        match term(&mut problems, entity, None) {
            Some(row) => items.push(row),
            // A missing ancestor would read as "not under it", so the page is refused, not shortened.
            None => refusals = crate::Refusal::Refused,
        }
    }
    let replaced_by = replacements
        .into_iter()
        .filter_map(|curie| {
            let entity = linked.remove(&curie.to_string());
            term(&mut problems, entity, Some(curie))
        })
        .collect();
    let text = |value: &str| {
        crate::mapped::text(value).map_err(|_| RequestError::Input(InputError::Serialization))
    };
    Ok(Some(Ancestors {
        items,
        refusals,
        obsolete,
        replaced_by,
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
/// One linked term; `known` is a CURIE the class itself already gave.
fn term(
    problems: &mut crate::mapped::Problems,
    entity: Option<Field<Linked>>,
    known: Option<Curie>,
) -> Option<PhenotypeAncestor> {
    let entity = entity.and_then(|entity| problems.field("linkedEntities", Some(entity)));
    let (curie, label) = entity.map_or((None, None), |e| (e.curie, e.label));
    let curie = match known {
        Some(curie) => curie,
        None => problems.required_field("linkedEntities.curie", curie)?,
    };
    let labels = problems.optional_fields("linkedEntities.label", label);
    let label = match labels.as_deref() {
        Some([label]) => Some(Box::<str>::from(label.as_str())),
        Some(_) => problems.required::<Box<str>>("linkedEntities.label", None),
        None => None,
    };
    Some(PhenotypeAncestor { id: curie, label })
}
