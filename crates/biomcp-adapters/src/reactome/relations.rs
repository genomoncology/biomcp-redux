use super::{SPEC, relation_raw as raw};
use crate::Mapped;
use biodata::{
    PathwayEventRef, PathwayMember, PathwayParticipant, PathwaySourceRef, ReactomeId, Text,
};
use biomcp_fetch::{BadReply, Freshness, Leg, RequestError};
use biomcp_types::{Problem, ProblemCode, Source};
/// Complete source-reference membership with explicit occurrence and physical-entity counts.
pub struct Members {
    /// Distinct namespaced reference rows in first occurrence order.
    pub items: Vec<PathwayMember>,
    /// Native reference occurrences before grouping.
    pub reference_occurrences: u64,
    /// Native physical entities, independent of reference and gene counts.
    pub physical_entities: u64,
}
fn text(value: &str) -> Result<Text, RequestError> {
    Text::new(value).map_err(|_| BadReply.into())
}
fn problem(id: &ReactomeId, field: &str) -> Problem {
    Problem {
        source: Source::Reactome,
        record: Text::new(id.as_str()).ok(),
        field: field.into(),
        code: ProblemCode::ValueUnknown,
    }
}
/// Read the bounded complete physical-participant relation without interpreting display names.
pub fn members(leg: &Leg<'_>, id: &ReactomeId) -> Result<Mapped<Members>, RequestError> {
    let endpoint = SPEC
        .get("/data/participants")
        .with_path(format!("/data/participants/{id}"))?
        .bytes(8 * 1024 * 1024)
        .fresh(Freshness::Days(7));
    let rows = leg.json::<raw::Participants>(&endpoint, &[])?;
    let mut value = Members {
        items: Vec::new(),
        reference_occurrences: 0,
        physical_entities: rows.0.len() as u64,
    };
    for row in rows.0 {
        let participant = PathwayParticipant {
            db_id: row.pe_db_id,
            name: text(&row.display_name)?,
            schema_class: text(&row.schema_class)?,
        };
        for reference in row.ref_entities {
            value.reference_occurrences += 1;
            let (namespace, identifier) = reference.st_id.split_once(':').ok_or(BadReply)?;
            if let Some(existing) = value.items.iter_mut().find(|m| {
                m.reference.namespace.as_str() == namespace
                    && m.reference.value.as_str() == identifier
            }) {
                if !existing
                    .participants
                    .iter()
                    .any(|p| p.db_id == participant.db_id)
                {
                    let mut participants = existing.participants.to_vec();
                    participants.push(participant.clone());
                    existing.participants = participants.into();
                }
            } else {
                value.items.push(PathwayMember {
                    reference: PathwaySourceRef {
                        namespace: text(namespace)?,
                        value: text(identifier)?,
                    },
                    name: reference.display_name.as_deref().map(text).transpose()?,
                    source_url: reference.url.as_deref().map(text).transpose()?,
                    gene: None,
                    schema_class: Some(text(&reference.schema_class)?),
                    participants: vec![participant.clone()].into(),
                });
            }
        }
    }
    Ok(Mapped {
        value,
        problems: Vec::new(),
    })
}
/// Read contained events while retaining every unresolved native integer reference.
pub fn events(
    leg: &Leg<'_>,
    id: &ReactomeId,
) -> Result<Mapped<Vec<PathwayEventRef>>, RequestError> {
    let endpoint = SPEC
        .get("/data/pathway/containedEvents")
        .with_path(format!("/data/pathway/{id}/containedEvents"))?
        .bytes(8 * 1024 * 1024)
        .fresh(Freshness::Days(7));
    let rows = leg.json::<raw::Events>(&endpoint, &[])?;
    let mut mapped = Mapped {
        value: Vec::new(),
        problems: Vec::new(),
    };
    for row in rows.0 {
        match row {
            raw::Event::Database(db_id) => mapped.value.push(PathwayEventRef::Database { db_id }),
            raw::Event::Object(v) => {
                if v.species_name
                    .as_deref()
                    .is_some_and(|s| s != "Homo sapiens")
                {
                    mapped.problems.push(problem(id, "events.species"));
                    continue;
                }
                let native_id = v.st_id.as_deref().map(str::parse).transpose();
                let Ok(native_id) = native_id else {
                    mapped.problems.push(problem(id, "events.id"));
                    continue;
                };
                mapped.value.push(PathwayEventRef::Object {
                    db_id: v.db_id,
                    id: native_id,
                    version: v.st_id_version.as_deref().map(text).transpose()?,
                    name: v.display_name.as_deref().map(text).transpose()?,
                    schema_class: v.schema_class.as_deref().map(text).transpose()?,
                });
            }
        }
    }
    Ok(mapped)
}
