//! Canonical mydisease publisher declaration.
mod native;
mod raw;
pub use native::{Component, Record, Value};
use biodata::Curie;
use biomcp_fetch::{Arg, BadReply, Freshness, InputError, Leg, PathSegment, RequestError};
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// Publisher policy shared by health, admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
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

/// Read one exact MONDO or DOID annotation, with no search, fallback or fanout.
pub fn get(leg: &Leg<'_>, id: &Curie) -> Result<Record, RequestError> {
    let (namespace, local) = id.as_str().split_once(':').ok_or(InputError::Path)?;
    if !matches!(namespace, "MONDO" | "DOID") || !local.bytes().all(|v| v.is_ascii_digit()) {
        return Err(InputError::Path.into());
    }
    let fields = crate::disease_sources::components::allowed_fields(leg, FIELDS)?;
    let endpoint = GET.clone().with_segments(&[PathSegment::new(id.as_str())?]);
    let (raw, metadata) = leg.json_received_checked::<raw::Document>(
        &endpoint, &[("fields", Arg::Text(&fields))],
        |record| if record.matches(id, &fields) { Ok(()) } else { Err(BadReply) },
    )?;
    // Ignore any unrequested component even if a provider or old replay includes it.
    let mut components = Vec::new();
    if fields.split(',').any(|f| f == "mondo") && let Some(value) = raw.mondo {
        components.push(native::component("mondo", value));
    }
    if fields.split(',').any(|f| f == "disease_ontology") && let Some(value) = raw.disease_ontology {
        components.push(native::component("doid", value));
    }
    Ok(Record { requested_id: id.clone(), returned_id: raw.id, version: raw.version,
        components: components.into(), metadata })
}
