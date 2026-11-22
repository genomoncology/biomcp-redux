use super::{SPEC, map, raw};
use crate::Mapped;
use biodata::Curie;
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, RequestError};
use biomcp_fetch::{Input, Read};
use biomcp_types::{DiseaseHeader, DiseaseOntology, DiseaseSearch};
/// A single native page without local ranking or identity selection.
#[derive(Debug)]
pub struct SearchPage {
    /// Rows retained without a usable identity.
    pub missing_ids: u32,
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
    let page = leg.call(
        &endpoint,
        Input::Params(&[
            ("q", Arg::Text(text)),
            ("rows", Arg::Num(u64::from(query.limit()))),
            ("start", Arg::Num(u64::from(query.offset()))),
            ("ontology", Arg::Text(ontology.as_str())),
            ("groupField", Arg::Text("iri")),
        ]),
        Read::<raw::Search>::json().check(&|page| {
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
        }),
    )?;
    map_page(page.value.response, ontology)
}

fn map_page(
    page: raw::SearchResponse,
    ontology: DiseaseOntology,
) -> Result<SearchPage, RequestError> {
    let mut problems = crate::mapped::Problems {
        source: biomcp_types::Source::Ols,
        record: None,
        list: Vec::new(),
    };
    let mut value = Vec::with_capacity(page.docs.len());
    let mut missing_ids = 0;
    for hit in page.docs {
        problems.record = None;
        let id = hit
            .obo_id
            .and_then(crate::raw::Field::value)
            .or_else(|| {
                hit.iri
                    .as_ref()
                    .and_then(crate::raw::Field::as_ref)
                    .and_then(|iri| search_iri_id(iri.as_str()))
            })
            .or_else(|| {
                let short_form = hit.short_form?.value()?;
                short_form.as_str().parse().ok().or_else(|| {
                    let id = map::short_form_id(short_form.as_str())?;
                    (DiseaseOntology::of(id.as_str()) == Some(ontology)).then_some(id)
                })
            });
        let id = problems.required("obo_id", id);
        if id.is_none() {
            missing_ids += 1;
        }
        problems.record = id.as_ref().and_then(crate::mapped::record);
        let iri = problems.field("iri", hit.iri);
        let label = problems.source_field("label", hit.label);
        value.push(DiseaseHeader {
            attribution: map::attribution(id.as_ref(), iri, ontology)?,
            concept: biomcp_types::DiseaseConcept { id, label },
            ontology,
            obsolete: problems.field("is_obsolete", hit.is_obsolete),
        });
    }
    Ok(SearchPage {
        missing_ids,
        total: page.total,
        rows: Mapped {
            refusals: if missing_ids == 0 {
                crate::Refusal::None
            } else {
                crate::Refusal::Refused
            },
            value,
            problems: problems.list,
        },
    })
}

// OLS may return imported gene terms with only a local number in its ID fields.
fn search_iri_id(iri: &str) -> Option<Curie> {
    let Some(path) = iri
        .strip_prefix("http://identifiers.org/")
        .or_else(|| iri.strip_prefix("https://identifiers.org/"))
    else {
        return map::iri_id(iri);
    };
    let Some((namespace, local)) = path.split_once('/') else {
        return path.parse().ok();
    };
    let authority = match namespace {
        "hgnc" => "HGNC",
        "ncbigene" => "NCBIGene",
        _ => return None,
    };
    if local.is_empty() || !local.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    format!("{authority}:{local}").parse().ok()
}
