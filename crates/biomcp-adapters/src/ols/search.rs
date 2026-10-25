use super::{SPEC, map, raw};
use crate::Mapped;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, RequestError};
use biomcp_types::{DiseaseHeader, DiseaseSearch};
/// A single native page without local ranking or identity selection.
#[derive(Debug)]
pub struct SearchPage {
    /// Publisher total for the literal query and one ontology.
    pub total: u64,
    /// Headers in publisher order and field-level problems.
    pub rows: Mapped<Vec<DiseaseHeader>>,
}
/// Fetch exactly one page from exactly one admitted ontology.
pub fn search(leg: &Leg<'_>, query: &DiseaseSearch) -> Result<SearchPage, RequestError> {
    let ontology = crate::disease_sources::components::ontology(
        leg,
        query
            .ontology()
            .ok_or(biomcp_fetch::InputError::Serialization)?,
    )?;
    let text = query
        .text()
        .ok_or(biomcp_fetch::InputError::Serialization)?;
    let endpoint = SPEC.get("/api/search").fresh(Freshness::Days(1));
    let (page, _) = leg.json_received_checked::<raw::Search>(
        &endpoint,
        &[
            ("q", Arg::Text(text.as_str())),
            ("rows", Arg::Num(u64::from(query.limit()))),
            ("start", Arg::Num(u64::from(query.offset()))),
            ("ontology", Arg::Text(ontology.as_str())),
            ("groupField", Arg::Text("iri")),
        ],
        |page| {
            let response = &page.response;
            if response.start != query.offset()
                || response.docs.len() > query.limit() as usize
                || response
                    .docs
                    .iter()
                    .any(|d| d.ontology_name.as_str() != ontology.as_str())
            {
                return Err(BadReply);
            }
            Ok(())
        },
    )?;
    Ok(SearchPage {
        total: page.response.total,
        rows: Mapped {
            value: page
                .response
                .docs
                .into_iter()
                .map(|hit| DiseaseHeader {
                    attribution: map::attribution(&hit.obo_id, &hit.iri, ontology),
                    concept: biodata::Concept {
                        id: Some(hit.obo_id),
                        label: hit.label,
                    },
                    ontology,
                    obsolete: hit.is_obsolete,
                })
                .collect(),
            problems: Vec::new(),
        },
    })
}
