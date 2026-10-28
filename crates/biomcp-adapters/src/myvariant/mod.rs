//! MyVariant exact source identities and compact component-attributed facts.
/// Independent terms for carried components.
pub mod components;
/// MyVariant-carried historical COSMIC fragments.
pub mod cosmic;
mod map;
/// Capture-proven component measurements.
pub mod measurements;
mod metadata;
mod raw;
mod search;
mod search_build;
mod transcripts;
use crate::{Mapped, quotas};
use biodata::Text;
use biomcp_fetch::{Arg, Endpoint, Freshness, KeyUse, Leg, PathSegment, RequestError, SourceSpec};
use biomcp_types::{Source, VariantHeader};
pub use search::{SearchPage, search};
use std::time::Duration;
/// Public reads retain each component's independent terms.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "MyVariant.info terms and component data licenses",
        url: "https://myvariant.info/terms",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::MyVariant,
    base: "https://myvariant.info/v1",
    key: KeyUse::None,
    quota: &quotas::BIOTHINGS,
    deadline: Duration::from_secs(15),
};
const QUERY: Endpoint = SPEC.get("/query").fresh(Freshness::Days(1));
const VARIANT: Endpoint = SPEC.get("/variant").fresh(Freshness::Days(1));
const FIELDS: &str = "_id,dbsnp.rsid,dbsnp.gene.symbol,clinvar.variant_id";
/// Resolve the exact published rsID or ClinVar ID, keeping every native candidate.
/// Pages are bounded by the provider cap and source totals, never by taking its first hit.
pub fn lookup(
    leg: &Leg<'_>,
    query: &str,
    build: biodata::Build,
) -> Result<Mapped<Vec<VariantHeader>>, RequestError> {
    if query.starts_with("chr") {
        return annotation(leg, query, build);
    }
    let assembly = if build == biodata::Build::GRCh38 {
        "hg38"
    } else {
        "hg19"
    };
    if query.starts_with("clinvar.variant_id:") && components::disabled_on_leg(leg, "clinvar") {
        return Err(biomcp_fetch::SourceError {
            source: Source::MyVariant,
            reason: biomcp_types::Reason::Disabled,
            status: None,
            retry_after: None,
        }
        .into());
    }
    let fields = fields(leg);
    let mut offset = 0;
    let mut hits = Vec::new();
    loop {
        let page = leg.json_checked_cache::<raw::Page>(
            &QUERY,
            &[
                ("q", Arg::Text(query)),
                ("size", Arg::Num(1000)),
                ("from", Arg::Num(offset)),
                ("fields", Arg::Text(&fields)),
                ("assembly", Arg::Text(assembly)),
            ],
            |page| page.hits.iter().all(|hit| hit.matches(query)),
        )?;
        let count = page.hits.len() as u64;
        hits.extend(page.hits);
        offset += count;
        if offset >= page.total {
            break;
        }
        if count == 0 || offset > 10_000 {
            return Err(leg_bad());
        }
    }
    Ok(map::candidates(
        hits,
        query,
        Some(build),
        !components::disabled_on_leg(leg, "clinvar"),
        !components::disabled_on_leg(leg, "snpeff"),
    ))
}
fn leg_bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::MyVariant,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
/// Match CAR's exact source identifier to MyVariant's published genomic record key.
/// This builds only the documented chromosome spelling, never converts an allele.
pub fn genomic_id(chromosome: &str, notation: &Text) -> Option<String> {
    let (_, description) = notation.as_str().split_once(':')?;
    Some(format!("chr{chromosome}:{description}"))
}

fn annotation(
    leg: &Leg<'_>,
    id: &str,
    build: biodata::Build,
) -> Result<Mapped<Vec<VariantHeader>>, RequestError> {
    let assembly = if build == biodata::Build::GRCh38 {
        "hg38"
    } else {
        "hg19"
    };
    let endpoint = VARIANT.clone().with_segments(&[PathSegment::new(id)?]);
    let fields = fields(leg);
    let (hit, _) = leg.json_received_checked::<raw::Hit>(
        &endpoint,
        &[
            ("fields", Arg::Text(&fields)),
            ("assembly", Arg::Text(assembly)),
        ],
        |hit| {
            if hit.matches(id) {
                Ok(())
            } else {
                Err(biomcp_fetch::BadReply)
            }
        },
    )?;
    Ok(map::candidates(
        vec![hit],
        id,
        Some(build),
        !components::disabled_on_leg(leg, "clinvar"),
        !components::disabled_on_leg(leg, "snpeff"),
    ))
}

/// Filter independently disabled carried components before creating a request identity.
pub(super) fn fields(leg: &Leg<'_>) -> String {
    annotation_fields(leg, FIELDS)
}
/// Include the shared source-paired transcript projection in identity reads.
pub(super) fn annotation_fields(leg: &Leg<'_>, fields: &str) -> String {
    allowed_fields(leg, &format!("{fields},{}", transcripts::FIELD))
}
/// Apply the same component policy to each declared source projection.
pub(super) fn allowed_fields(leg: &Leg<'_>, fields: &str) -> String {
    fields
        .split(',')
        .filter(|field| !components::disabled_on_leg(leg, field.split('.').next().unwrap_or(field)))
        .collect::<Vec<_>>()
        .join(",")
}
