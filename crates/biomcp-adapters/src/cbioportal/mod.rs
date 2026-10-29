//! Bounded explicit-cohort mutation observations and stored public study metadata.
mod raw;
use biodata::NcbiGeneId;
use biomcp_fetch::{Arg, BadReply, Freshness, KeyUse, Leg, PathSegment, RequestError, SourceSpec};
use biomcp_types::{RecurrenceStudy, Source, VariantRecurrenceQuery};
pub use raw::Mutation;
use std::time::Duration;
/// Reviewed DataHub directory API origin.
pub(crate) const GITHUB_ORIGIN: &str = "https://api.github.com";
/// Study-specific terms must be verified independently of public read access.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Additional(&[GITHUB_ORIGIN]),
    user_agent: None,
    source: Source::Cbioportal,
    base: "https://www.cbioportal.org/api",
    key: KeyUse::None,
    quota: &crate::quotas::CBIOPORTAL,
    deadline: Duration::from_secs(15),
    terms: biomcp_fetch::Terms {
        name: "cBioPortal study-specific terms unverified; commercial restrictions may apply",
        url: "https://docs.cbioportal.org/user-guide/faq/",
        usage: biomcp_types::Use::Restricted,
    },
};
/// Checked metadata for the exact caller-selected cohort.
#[derive(Debug)]
pub struct Cohort {
    /// Source title, description and citation without access-group license inference.
    pub study: RecurrenceStudy,
    /// Selected list size, without assuming gene assay eligibility.
    pub selected_sample_count: Option<u64>,
    /// Failure to establish selected-list metadata; mutation observations remain usable.
    pub count_error: Option<RequestError>,
}
/// Fetch and bind all cohort selectors before reading mutation rows.
pub fn cohort(leg: &Leg<'_>, query: &VariantRecurrenceQuery) -> Result<Cohort, RequestError> {
    let endpoint = SPEC
        .get("/studies")
        .with_segments(&[PathSegment::new(query.study().as_str())?])
        .fresh(Freshness::Days(1));
    let (study, _) = leg.json_received_checked::<raw::Study>(&endpoint, &[], |v| {
        if v.study_id == *query.study() {
            Ok(())
        } else {
            Err(BadReply)
        }
    })?;
    let endpoint = SPEC
        .get("/molecular-profiles")
        .with_segments(&[PathSegment::new(query.profile().as_str())?])
        .fresh(Freshness::Days(1));
    leg.json_received_checked::<raw::Profile>(&endpoint, &[], |v| {
        if v.study_id == *query.study()
            && v.molecular_profile_id == *query.profile()
            && v.molecular_alteration_type.as_str() == "MUTATION_EXTENDED"
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    })?;
    let count = sample_count(leg, query);
    Ok(Cohort {
        study: RecurrenceStudy {
            name: study.name,
            description: study.description.map(Into::into),
            citation: study.citation.map(Into::into),
        },
        selected_sample_count: count.as_ref().ok().copied().flatten(),
        count_error: count.err(),
    })
}
/// One native gene page. Wrong study, profile or gene cannot enter the cache.
pub fn mutations(
    leg: &Leg<'_>,
    query: &VariantRecurrenceQuery,
    gene: NcbiGeneId,
) -> Result<Vec<Mutation>, RequestError> {
    let endpoint = SPEC
        .get("/molecular-profiles")
        .with_segments(&[
            PathSegment::new(query.profile().as_str())?,
            PathSegment::new("mutations")?,
        ])
        .fresh(Freshness::Days(1));
    let params = [
        ("sampleListId", Arg::Text(query.sample_set().as_str())),
        ("entrezGeneId", Arg::Num(u64::from(gene.get()))),
        ("projection", Arg::Text("SUMMARY")),
        ("pageSize", Arg::Num(u64::from(query.limit()))),
        ("pageNumber", Arg::Num(u64::from(query.page()))),
    ];
    let (rows, _) = leg.json_received_checked::<raw::Mutations>(&endpoint, &params, |rows| {
        if rows.0.len() <= query.limit() as usize
            && rows.0.iter().all(|v| {
                v.study_id == *query.study()
                    && v.molecular_profile_id == *query.profile()
                    && v.entrez_gene_id == gene.get()
            })
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    })?;
    Ok(rows.0)
}

// This bounds metadata discovery, never mutation-cohort traversal.
const LIST_PAGE_SIZE: u64 = 50;
const LIST_PAGES: u64 = 3;
fn sample_count(
    leg: &Leg<'_>,
    query: &VariantRecurrenceQuery,
) -> Result<Option<u64>, RequestError> {
    let endpoint = SPEC
        .get("/studies")
        .with_segments(&[
            PathSegment::new(query.study().as_str())?,
            PathSegment::new("sample-lists")?,
        ])
        .fresh(Freshness::Days(1));
    for page in 0..LIST_PAGES {
        let params = [
            ("projection", Arg::Text("SUMMARY")),
            ("pageSize", Arg::Num(LIST_PAGE_SIZE)),
            ("pageNumber", Arg::Num(page)),
            ("sortBy", Arg::Text("sampleListId")),
            ("direction", Arg::Text("ASC")),
        ];
        let (lists, _) =
            leg.json_received_checked::<raw::SampleLists>(&endpoint, &params, |v| {
                if v.0.len() <= LIST_PAGE_SIZE as usize
                    && v.0.iter().all(|l| l.study_id == *query.study())
                {
                    Ok(())
                } else {
                    Err(BadReply)
                }
            })?;
        if let Some(list) = lists
            .0
            .iter()
            .find(|l| l.sample_list_id == *query.sample_set())
        {
            return Ok(list.sample_count);
        }
        if lists.0.len() < LIST_PAGE_SIZE as usize {
            break;
        }
    }
    Err(biomcp_fetch::SourceError {
        source: Source::Cbioportal,
        reason: biomcp_types::Reason::Unavailable,
        status: None,
        retry_after: None,
    }
    .into())
}

/// Stored public catalog metadata.
pub mod studies;
pub use studies::sync as sync_studies;

pub(crate) use studies::bundle_state;
