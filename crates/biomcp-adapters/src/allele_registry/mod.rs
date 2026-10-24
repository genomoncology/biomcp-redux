//! Read-only ClinGen Allele Registry resolution, without local allele repair.
mod map;
mod raw;
use crate::quotas;
use biodata::{Source, Text, VariantHeader};
use biomcp_fetch::{Arg, Endpoint, KeyUse, Leg, PathSegment, RequestError, SourceSpec};
use std::time::Duration;
/// Public GET access; raw output redistribution rights remain unverified.
pub static SPEC: SourceSpec = SourceSpec {
    terms: biomcp_fetch::Terms {
        name: "ClinGen Allele Registry service terms; output rights require review",
        url: "https://clinicalgenome.org/about/terms-of-use/",
        usage: biodata::Use::Restricted,
    },
    source: Source::AlleleRegistry,
    base: "https://reg.genome.network",
    key: KeyUse::None,
    quota: &quotas::ALLELE_REGISTRY,
    deadline: Duration::from_secs(15),
};
const ALLELE: Endpoint = SPEC.get("/allele").json_rejections();
const FIELDS: &str = "none+@id+communityStandardTitle+genomicAlleles+transcriptAlleles.MANE+externalRecords.dbSNP+externalRecords.ClinVarVariations";
/// Checked service result including a source validation code.
#[derive(Debug)]
pub enum Resolution {
    /// Source-resolved canonical identity.
    Found(crate::Mapped<VariantHeader>),
    /// Source rejected the unchanged notation.
    Rejected(Text),
}
/// Resolve unchanged HGVS through the declared GET endpoint.
pub fn hgvs(leg: &Leg<'_>, input: &str) -> Result<Resolution, RequestError> {
    let reply = leg.json::<raw::Allele>(
        &ALLELE,
        &[("hgvs", Arg::Text(input)), ("fields", Arg::Text(FIELDS))],
    )?;
    map::allele(reply).map_err(|_| bad())
}
/// Read a canonical source record directly, without another resolver.
pub fn by_id(leg: &Leg<'_>, input: &str) -> Result<Resolution, RequestError> {
    let endpoint = ALLELE.clone().with_segments(&[PathSegment::new(input)?]);
    let reply = leg.json::<raw::Allele>(&endpoint, &[("fields", Arg::Text(FIELDS))])?;
    let mapped = map::allele(reply).map_err(|_| bad())?;
    match &mapped {
        Resolution::Found(v) if v.value.record.id.as_str() != input => {
            Err(biomcp_fetch::SourceError {
                source: Source::AlleleRegistry,
                reason: biodata::Reason::BadReply,
                status: None,
                retry_after: None,
            }
            .into())
        }
        _ => Ok(mapped),
    }
}

fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::AlleleRegistry,
        reason: biodata::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
