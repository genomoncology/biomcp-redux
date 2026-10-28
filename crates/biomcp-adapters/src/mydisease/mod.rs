//! Canonical mydisease publisher declaration.
mod map;
mod native;
mod raw;
use biodata::Curie;
use biomcp_fetch::{Arg, BadReply, Freshness, InputError, Leg, PathSegment, RequestError};
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
pub use native::{Component, Record, Value};
/// Complete exact annotation before independently bounded response pages.
#[derive(Debug)]
pub struct Annotation {
    /// Exact identity with optional native alias proof.
    pub card: biomcp_types::DiseaseCard,
    /// Component-attributed native synonyms.
    pub synonyms: Vec<biodata::DiseaseSynonym>,
    /// Every native cross-reference with its original source field predicate.
    pub xrefs: Vec<biodata::DiseaseRelation>,
    /// Direct component parents, without ancestors or alternatives.
    pub parents: Vec<biodata::DiseaseRelation>,
}
/// Read and map one exact annotation without additional source requests.
pub fn annotation(leg: &Leg<'_>, id: &Curie) -> Result<crate::Mapped<Annotation>, RequestError> {
    map::annotation(get(leg, id)?)
}
use std::time::Duration;
/// Publisher policy shared by health, admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::MyDisease,
    base: "https://mydisease.info/v1",
    terms: Terms {
        name: "MyDisease source-specific terms",
        url: "https://docs.mydisease.info/en/latest/doc/data.html",
        usage: Use::Attribution,
    },
    quota: &crate::quotas::BIOTHINGS,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};
const GET: biomcp_fetch::Endpoint = SPEC.get("/disease").fresh(Freshness::Days(7));
const FIELDS: &str = "mondo,disease_ontology";
/// Whether an identifier belongs to the publisher's implemented exact namespaces.
pub fn supported_id(id: &str) -> bool {
    id.split_once(':').is_some_and(|(namespace, local)| {
        matches!(namespace, "MONDO" | "DOID")
            && !local.is_empty()
            && local.bytes().all(|v| v.is_ascii_digit())
    })
}

/// Read one exact MONDO or DOID annotation, with no search, fallback or fanout.
pub fn get(leg: &Leg<'_>, id: &Curie) -> Result<Record, RequestError> {
    let identifier = id.to_string();
    if !supported_id(&identifier) {
        return Err(InputError::Path.into());
    }
    let fields = crate::disease_sources::components::allowed_fields(leg, FIELDS)?;
    let endpoint = GET.clone().with_segments(&[PathSegment::new(&identifier)?]);
    let (raw, metadata) = leg.json_received_checked::<raw::Document>(
        &endpoint,
        &[("fields", Arg::Text(&fields))],
        |record| {
            if record.matches(id, &fields) {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )?;
    // Ignore any unrequested component even if a provider or old replay includes it.
    let mut components = Vec::new();
    if fields.split(',').any(|f| f == "mondo")
        && let Some(value) = raw.mondo
    {
        components.push(native::component("mondo", value));
    }
    if fields.split(',').any(|f| f == "disease_ontology")
        && let Some(value) = raw.disease_ontology
    {
        components.push(native::component("doid", value));
    }
    Ok(Record {
        requested_id: id.clone(),
        returned_id: raw.id,
        version: raw.version,
        components: components.into(),
        metadata,
    })
}
