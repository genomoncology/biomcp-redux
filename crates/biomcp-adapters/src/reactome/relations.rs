use super::{SPEC, relation_raw as raw};
use crate::Mapped;
use biodata::{
    PathwayEventRef, PathwayMember, PathwayParticipant, PathwaySourceRef, ReactomeId, Text,
};
use biomcp_fetch::{Freshness, Leg, RequestError, SourceError};
use biomcp_types::{Reason, Source};
/// Complete source-reference membership with explicit occurrence and physical-entity counts.
#[derive(Debug)]
pub struct Members {
    /// Distinct namespaced reference rows in first occurrence order.
    pub items: Vec<PathwayMember>,
    /// Native reference occurrences before grouping.
    pub reference_occurrences: u64,
    /// Native physical entities, independent of reference and gene counts.
    pub physical_entities: u64,
}
fn bad_reply() -> RequestError {
    SourceError {
        source: Source::Reactome,
        reason: Reason::BadReply,
        status: Some(200),
        retry_after: None,
    }
    .into()
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
            name: crate::mapped::text(&row.display_name).map_err(|_| bad_reply())?,
            schema_class: crate::mapped::text(&row.schema_class).map_err(|_| bad_reply())?,
        };
        for reference in row.ref_entities {
            value.reference_occurrences += 1;
            let (namespace, identifier) = reference.st_id.split_once(':').ok_or_else(bad_reply)?;
            if let Some(existing) = value.items.iter_mut().find(|m| {
                m.reference.namespace.as_deref() == Some(namespace)
                    && m.reference.value.as_str() == identifier
            }) {
                let mut participants = existing
                    .participants
                    .take()
                    .map_or_else(Vec::new, |values| values.into_vec());
                if !participants.iter().any(|p| p.db_id == participant.db_id) {
                    participants.push(participant.clone());
                }
                existing.physical_context_count = participants.len() as u64;
                existing.participants = Some(participants.into());
            } else {
                value.items.push(PathwayMember {
                    node_id: None,
                    reference: PathwaySourceRef {
                        namespace: Some(
                            crate::mapped::text(namespace)
                                .map_err(|_| bad_reply())?
                                .as_str()
                                .into(),
                        ),
                        value: crate::mapped::text(identifier).map_err(|_| bad_reply())?,
                    },
                    name: reference
                        .display_name
                        .as_deref()
                        .map(|v| crate::mapped::text(v).map_err(|_| bad_reply()))
                        .transpose()?,
                    source_url: reference
                        .url
                        .as_deref()
                        .map(|v| crate::mapped::text(v).map_err(|_| bad_reply()))
                        .transpose()?,
                    gene: None,
                    schema_class: Some(
                        crate::mapped::text(&reference.schema_class).map_err(|_| bad_reply())?,
                    ),
                    physical_context_count: 1,
                    participants: Some(vec![participant.clone()].into()),
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
    let rows = leg.json_checked_cache::<raw::Events>(&endpoint, &[], |rows| {
        rows.0.iter().all(|row| match row {
            raw::Event::Database(_) => true,
            raw::Event::Object(v) => {
                v.species_name
                    .as_deref()
                    .is_none_or(|s| s == "Homo sapiens")
                    && v.st_id
                        .as_deref()
                        .is_none_or(|id| id.parse::<ReactomeId>().is_ok())
            }
        })
    })?;
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
                    mapped.problems.push(crate::mapped::problem(
                        Source::Reactome,
                        Text::new(id.as_str()).ok(),
                        "events.species",
                    ));
                    continue;
                }
                let native_id = v.st_id.as_deref().map(str::parse).transpose();
                let Ok(native_id) = native_id else {
                    mapped.problems.push(crate::mapped::problem(
                        Source::Reactome,
                        Text::new(id.as_str()).ok(),
                        "events.id",
                    ));
                    continue;
                };
                mapped.value.push(PathwayEventRef::Object {
                    db_id: v.db_id,
                    source_url: native_id
                        .as_ref()
                        .map(|id| {
                            crate::mapped::text(&format!(
                                "https://reactome.org/content/detail/{id}"
                            ))
                            .map_err(|_| bad_reply())
                        })
                        .transpose()?,
                    id: native_id,
                    version: v
                        .st_id_version
                        .as_deref()
                        .map(|v| crate::mapped::text(v).map_err(|_| bad_reply()))
                        .transpose()?,
                    name: v
                        .display_name
                        .as_deref()
                        .map(|v| crate::mapped::text(v).map_err(|_| bad_reply()))
                        .transpose()?,
                    schema_class: v
                        .schema_class
                        .as_deref()
                        .map(|v| crate::mapped::text(v).map_err(|_| bad_reply()))
                        .transpose()?,
                });
            }
        }
    }
    Ok(mapped)
}
