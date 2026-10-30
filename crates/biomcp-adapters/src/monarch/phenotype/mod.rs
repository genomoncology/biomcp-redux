//! Literal HP discovery and independent exact Monarch term cards.
pub mod associations;
mod counts;
mod raw;
mod sections;
use biodata::{Attribution, PhenotypeCard, PhenotypeHeader, Text};
use biomcp_fetch::{Arg, BadReply, Freshness, InputError, Leg, ReplyMetadata, RequestError};
use biomcp_types::{PhenotypeGet, PhenotypeSearch, PhenotypeSection, PhenotypeTermCard};
/// Native ordered HP page and retrieval metadata.
#[derive(Debug)]
pub struct Page {
    /// One or more native row identities were refused.
    pub dropped: bool,
    /// Native row count before refusing invalid row identities.
    pub received: u32,
    /// Rejected native metadata on returned rows.
    pub problems: Vec<biomcp_types::Problem>,
    /// Publisher order without reranking or deduplication.
    pub items: Vec<PhenotypeHeader>,
    /// Source total, without local extrapolation.
    pub total: u64,
    /// Original retrieval metadata.
    pub metadata: ReplyMetadata,
}
/// Exact term card and original retrieval metadata.
#[derive(Debug)]
pub struct Record {
    /// Rejected native metadata retained for section snapshots.
    pub problems: Vec<biomcp_types::Problem>,
    /// Complete independent source card.
    pub card: PhenotypeTermCard,
    /// Native source synonyms with their original classes.
    pub synonyms: Option<Box<[biodata::PhenotypeSynonym]>>,
    /// Native references without an invented predicate.
    pub xrefs: Option<Box<[biodata::PhenotypeXref]>>,
    /// Direct parents followed by direct children, preserving each native array.
    pub hierarchy: Option<Box<[biodata::PhenotypeHierarchyLink]>>,
    /// Original retrieval metadata.
    pub metadata: ReplyMetadata,
}
/// Send the caller's literal query with the proved native HP/category selectors.
pub fn search(leg: &Leg<'_>, q: &PhenotypeSearch) -> Result<Page, RequestError> {
    crate::disease_sources::components::monarch(leg)?;
    let endpoint = super::SPEC.get("/v3/api/search").fresh(Freshness::Days(1));
    let args = [
        ("q", Arg::Text(q.text())),
        ("category", Arg::Text("biolink:PhenotypicFeature")),
        ("namespace", Arg::Text("HP")),
        ("limit", Arg::Num(q.limit().into())),
        ("offset", Arg::Num(q.offset().into())),
    ];
    let (page, metadata) = leg.json_received_checked::<raw::Page>(&endpoint, &args, |page| {
        if page.limit != q.limit() || page.offset != q.offset() {
            Err(BadReply)
        } else {
            Ok(())
        }
    })?;
    let received = page.items.len() as u32;
    let mut problems = Vec::new();
    let items = page
        .items
        .into_iter()
        .filter_map(|node| {
            problems.extend(node.problems);
            let id = node.id?;
            Some(attribution(&id).map(|attribution| PhenotypeHeader {
                attribution,
                id,
                label: node.name,
                match_score: node.score,
                matched_field: node.matched_field,
                match_type: node.match_type,
            }))
        })
        .collect::<Result<Vec<_>, RequestError>>()?;
    Ok(Page {
        dropped: items.len() < received as usize,
        received,
        problems,
        items,
        total: page.total,
        metadata,
    })
}
/// Reject a mismatched publisher identity before cache admission.
pub fn get(leg: &Leg<'_>, q: &PhenotypeGet) -> Result<Record, RequestError> {
    crate::disease_sources::components::monarch(leg)?;
    let endpoint = super::SPEC
        .get("/v3/api/entity")
        .with_segments(&[biomcp_fetch::PathSegment::new(q.id().as_str())?])
        .fresh(Freshness::Days(1));
    let (mut node, metadata) = leg.json_received_checked::<raw::Node>(&endpoint, &[], |node| {
        if node.id.as_ref() == Some(q.id()) {
            Ok(())
        } else {
            Err(BadReply)
        }
    })?;
    let id = node.id.as_ref().ok_or(InputError::Serialization)?.clone();
    let sections = sections(&node);
    let attribution = attribution(&id)?;
    let mut problems = crate::mapped::Problems {
        source: biomcp_types::Source::Monarch,
        record: crate::mapped::record(&id),
        list: std::mem::take(&mut node.problems),
    };
    let (synonyms, xrefs, hierarchy) = sections::rows(&mut node, &attribution, &mut problems);
    Ok(Record {
        problems: problems.list,
        synonyms,
        xrefs,
        hierarchy,
        card: PhenotypeTermCard {
            term: PhenotypeCard::Monarch {
                attribution,
                requested_id: q.id().clone(),
                returned_id: id.clone(),
                label: node.name,
                description: node.description,
                deprecated: node.deprecated,
                monarch_descendant_count: node.has_descendant_count,
                association_counts: node.association_counts.map(|v| v.0),
            },
            sections: sections.into(),
            components: super::components()?,
        },
        metadata,
    })
}
fn sections(node: &raw::Node) -> Vec<PhenotypeSection> {
    let mut out = Vec::new();
    if node.synonym.is_some()
        || node.exact_synonym.is_some()
        || node.broad_synonym.is_some()
        || node.narrow_synonym.is_some()
        || node.related_synonym.is_some()
    {
        out.push(PhenotypeSection::Synonyms);
    }
    if node.xref.is_some() {
        out.push(PhenotypeSection::Xrefs);
    }
    if node
        .node_hierarchy
        .as_ref()
        .is_some_and(|h| h.super_classes.is_some() || h.sub_classes.is_some())
    {
        out.push(PhenotypeSection::Hierarchy);
    }
    for (key, section) in [
        (
            "biolink:DiseaseToPhenotypicFeatureAssociation",
            PhenotypeSection::Diseases,
        ),
        (
            "biolink:GeneToPhenotypicFeatureAssociation",
            PhenotypeSection::Genes,
        ),
    ] {
        if node
            .association_counts
            .as_ref()
            .map(|v| v.0.as_ref())
            .unwrap_or_default()
            .iter()
            .any(|count| count.key.to_string() == key)
        {
            out.push(section);
        }
    }
    out
}
fn attribution(id: &biodata::HpId) -> Result<Attribution, RequestError> {
    let text = |s: &str| Text::new(s).map_err(|_| InputError::Serialization);
    Ok(Attribution {
        source: text("monarch")?,
        record: Some(text(id.as_str())?),
        url: Some(text(&format!(
            "https://monarchinitiative.org/HP:{}/",
            id.as_str().trim_start_matches("HP:")
        ))?),
        version: None,
        release: None,
        submitter: None,
        evaluated: None,
    })
}
