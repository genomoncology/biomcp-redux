use super::{SPEC, map, raw};
use crate::Mapped;
use crate::raw::Field;
use biodata::{Curie, Error};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, RequestError};
use biomcp_fetch::{Input, Read};
use biomcp_types::{DiseaseHeader, DiseaseOntology, DiseaseSearch};
/// A single native page without local ranking or identity selection.
#[derive(Debug)]
pub struct SearchPage {
    unidentified: Vec<(usize, DiseaseHeader)>,
    /// Native window size before identity refusals.
    pub received: u32,
    /// Publisher total for the literal query and one ontology.
    pub total: u64,
    /// Identified headers in publisher order and field-level problems.
    pub rows: Mapped<Vec<DiseaseHeader>>,
}

impl SearchPage {
    /// Number of source rows whose identities were refused.
    pub fn missing_ids(&self) -> usize {
        self.unidentified.len()
    }

    /// Keep the unidentified rows in their original positions in the display page.
    pub fn into_display_rows(self) -> Mapped<Vec<DiseaseHeader>> {
        let mut rows = self.rows;
        for (index, row) in self.unidentified {
            rows.value.insert(index, row);
        }
        rows
    }
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
    let received = page.docs.len();
    let mut value = Vec::with_capacity(received);
    let mut unidentified = Vec::new();
    for (index, hit) in page.docs.into_iter().enumerate() {
        problems.record = None;
        let id = hit
            .obo_id
            .and_then(Field::value)
            .ok_or(Error::CurieInvalid)
            .or_else(|_| {
                let iri = hit
                    .iri
                    .as_ref()
                    .and_then(Field::as_ref)
                    .ok_or(Error::CurieInvalid)?;
                search_iri_id(iri.as_str())
            })
            .or_else(|_| {
                let short_form = hit
                    .short_form
                    .and_then(Field::value)
                    .ok_or(Error::CurieInvalid)?;
                search_short_form_id(short_form.as_str(), ontology)
            });
        let id = problems.optional("obo_id", Some(id));
        problems.record = id.as_ref().and_then(crate::mapped::record);
        let iri = problems.field("iri", hit.iri);
        let label = problems.source_field("label", hit.label);
        let row = DiseaseHeader {
            attribution: map::attribution(id.as_ref(), iri, ontology)?,
            concept: biomcp_types::DiseaseConcept { id, label },
            ontology,
            obsolete: problems.field("is_obsolete", hit.is_obsolete),
        };
        if row.concept.id.is_some() {
            value.push(row);
        } else {
            unidentified.push((index, row));
        }
    }
    let refusals = if unidentified.is_empty() {
        crate::Refusal::None
    } else {
        crate::Refusal::Refused
    };
    Ok(SearchPage {
        unidentified,
        received: received as u32,
        total: page.total,
        rows: Mapped {
            refusals,
            value,
            problems: problems.list,
        },
    })
}

// OLS may return imported gene terms with only a local number in its ID fields.
fn search_iri_id(iri: &str) -> Result<Curie, Error> {
    let Some(path) = iri
        .strip_prefix("http://identifiers.org/")
        .or_else(|| iri.strip_prefix("https://identifiers.org/"))
    else {
        return map::iri_id(iri);
    };
    let Some((namespace, local)) = path.split_once('/') else {
        return path.parse();
    };
    let authority = match namespace {
        "hgnc" => "HGNC",
        "ncbigene" => "NCBIGene",
        _ => return Err(Error::CurieInvalid),
    };
    if local.is_empty() || !local.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(Error::CurieInvalid);
    }
    format!("{authority}:{local}").parse()
}

fn search_short_form_id(value: &str, ontology: DiseaseOntology) -> Result<Curie, Error> {
    value.parse().or_else(|_| {
        let id = map::short_form_id(value)?;
        if DiseaseOntology::of(id.as_str()) == Some(ontology) {
            Ok(id)
        } else {
            Err(Error::CurieInvalid)
        }
    })
}
