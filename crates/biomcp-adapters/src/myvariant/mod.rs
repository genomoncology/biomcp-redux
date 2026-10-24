//! MyVariant exact source identities and compact component-attributed facts.
mod map;
mod raw;
use crate::{Mapped, quotas};
use biodata::{Source, Text, VariantHeader};
use biomcp_fetch::{Arg, Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use std::time::Duration;
/// Public reads retain each component's independent terms.
pub static SPEC: SourceSpec = SourceSpec {
    terms: biomcp_fetch::Terms {
        name: "MyVariant.info terms and component data licenses",
        url: "https://docs.myvariant.info/en/latest/terms.html",
        usage: biodata::Use::Restricted,
    },
    source: Source::MyVariant,
    base: "https://myvariant.info/v1",
    key: KeyUse::None,
    quota: &quotas::BIOTHINGS,
    deadline: Duration::from_secs(15),
};
const QUERY: Endpoint = SPEC.get("/query").fresh(Freshness::Days(1));
const FIELDS: &str = "_id,dbsnp.rsid,dbsnp.gene.symbol,clinvar.variant_id,cosmic.cosmic_id";
/// Resolve the exact published rsID or ClinVar ID, keeping every native candidate.
/// Pages are bounded by the provider cap and source totals, never by taking its first hit.
pub fn lookup(leg: &Leg<'_>, query: &str, build: biodata::Build) -> Result<Mapped<Vec<VariantHeader>>, RequestError> {
    let assembly = if build == biodata::Build::Grch38 { "hg38" } else { "hg19" };
    let mut offset = 0;
    let mut hits = Vec::new();
    loop {
        let page = leg.json::<raw::Page>(
            &QUERY,
            &[
                ("q", Arg::Text(query)),
                ("size", Arg::Num(1000)),
                ("from", Arg::Num(offset)),
                ("fields", Arg::Text(FIELDS)),
                ("assembly", Arg::Text(assembly)),
            ],
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
    Ok(map::candidates(hits, query, build))
}
fn leg_bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::MyVariant,
        reason: biodata::Reason::BadReply,
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
