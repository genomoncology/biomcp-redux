//! Canonical monarch publisher declaration.
use biomcp_fetch::{KeyUse, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// Publisher policy shared by health, admission and response outcomes.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: Source::Monarch,
    base: "https://api-v3.monarchinitiative.org",
    terms: Terms {
        name: "Monarch source-specific licensing",
        url: "https://monarch-app.monarchinitiative.org/Licensing/",
        usage: Use::Attribution,
    },
    quota: &crate::quotas::MONARCH,
    key: KeyUse::None,
    deadline: Duration::from_secs(15),
};

mod map;
mod presence;
mod raw;
mod receive;
use biodata::Curie;
use biomcp_fetch::{Arg, BadReply, Freshness, InputError, Leg, ReplyMetadata, RequestError};
use biomcp_types::{DiseaseAssociationItem, DiseaseGet, DiseaseSection};
/// Native association rows with their original retrieval metadata.
#[derive(Debug)]
pub struct Page {
    /// At least one native association identity was refused.
    pub dropped: bool,
    /// Native rows before rejecting malformed identities.
    pub received: u32,
    /// Rejected row identities and metadata.
    pub problems: Vec<biomcp_types::Problem>,
    /// Source ordered association rows, without gene deduplication.
    pub items: Vec<DiseaseAssociationItem>,
    /// Reviewed component terms, including an empty native page.
    pub components: Box<[biomcp_types::ComponentLabel]>,
    /// Publisher association count.
    pub total: u64,
    /// Original checked reply metadata.
    pub metadata: ReplyMetadata,
}
const ASSOCIATIONS: biomcp_fetch::Endpoint =
    SPEC.get("/v3/api/association").fresh(Freshness::Days(7));
/// Fetch exactly one selected native association page after component admission.
pub fn associations(leg: &Leg<'_>, query: &DiseaseGet) -> Result<Page, RequestError> {
    crate::disease_sources::components::monarch(leg)?;
    let id = query
        .id()
        .as_str()
        .parse::<Curie>()
        .map_err(|_| InputError::Path)?;
    let genes = match query.section() {
        Some(DiseaseSection::Genes) => true,
        Some(DiseaseSection::Phenotypes) => false,
        _ => return Err(InputError::Path.into()),
    };
    let args = [
        (
            if genes { "object" } else { "subject" },
            Arg::Text(query.id().as_str()),
        ),
        (
            if genes {
                "subject_category"
            } else {
                "object_category"
            },
            Arg::Text(if genes {
                "biolink:Gene"
            } else {
                "biolink:PhenotypicFeature"
            }),
        ),
        ("limit", Arg::Num(query.section_limit().into())),
        ("offset", Arg::Num(query.section_offset().into())),
    ];
    let (page, metadata) =
        leg.json_received_checked::<raw::Page>(&ASSOCIATIONS, &args, |page| {
            if page.limit != query.section_limit() || page.offset != query.section_offset() {
                return Err(BadReply);
            }
            // Native parent expansion may return a child endpoint; require its native closure proof.
            if page.items.iter().any(|row| {
                let (endpoint, closure) = if genes {
                    (&row.object, &row.object_closure)
                } else {
                    (&row.subject, &row.subject_closure)
                };
                endpoint.as_ref() != Some(&id)
                    && closure.as_ref().is_none_or(|values| !values.contains(&id))
            }) {
                return Err(BadReply);
            }
            Ok(())
        })?;
    let received = page.items.len() as u32;
    let mut problems = Vec::new();
    let mut items = Vec::new();
    for mut row in page.items {
        problems.append(&mut row.problems);
        if row.id.is_some()
            && row.subject.is_some()
            && row.object.is_some()
            && row.predicate.is_some()
        {
            let mapped = map::row(row, &id)?;
            problems.extend(mapped.problems);
            items.push(mapped.value);
        }
    }
    Ok(Page {
        dropped: items.len() < received as usize,
        received,
        problems,
        items,
        components: components()?,
        total: page.total,
        metadata,
    })
}
/// Reviewed terms for the carried components remain explicit on pages and rows.
pub fn components() -> Result<Box<[biomcp_types::ComponentLabel]>, RequestError> {
    ["hpo", "orphanet"]
        .into_iter()
        .map(|name| {
            let component = crate::disease_sources::components::get(Source::Monarch, name)
                .ok_or(InputError::Serialization)?;
            Ok(biomcp_types::ComponentLabel {
                component: biodata::Text::new(name).map_err(|_| InputError::Serialization)?,
                terms: biomcp_types::ComponentTerms {
                    usage: component.terms.usage,
                    name: biodata::Text::new(component.terms.name)
                        .map_err(|_| InputError::Serialization)?,
                    url: biodata::Text::new(component.terms.url)
                        .map_err(|_| InputError::Serialization)?,
                },
            })
        })
        .collect::<Result<Vec<_>, RequestError>>()
        .map(Into::into)
}
/// HP-native discovery and independent exact term cards.
pub mod phenotype;

#[cfg(test)]
mod native_contracts;
