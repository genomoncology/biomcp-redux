//! cBioPortal's own discrete copy-number sample counts for one gene; BioMCP counts nothing itself.
use super::SPEC;
use super::rows::{acquired, copy_number_profiles, gene, selected};
use biodata::StudyProfileId;
use biomcp_fetch::{BadReply, Freshness, Input, Leg, PathSegment, Read, Reply, RequestError};
use biomcp_types::{StudyCopyNumber, StudyGet};
use serde::{Deserialize, Serialize};

const MAX_BYTES: u64 = 64 * 1024;
/// GISTIC calls: 2 is high-level amplification, -2 is deep (homozygous) deletion.
const AMPLIFIED: i32 = 2;
const DEEP_DELETED: i32 = -2;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Count {
    molecular_profile_id: StudyProfileId,
    entrez_gene_id: u32,
    alteration: i32,
    number_of_samples: u32,
    // cBioPortal omits this when no sample has the call (SRY in brca_tcga_gistic, 2026-11-17).
    number_of_samples_with_alteration_in_gene: Option<u32>,
}
#[derive(Deserialize)]
#[serde(transparent)]
struct Counts(Vec<Count>);
impl Reply for Counts {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Identifier {
    entrez_gene_id: u32,
    alteration: i32,
}

/// Resolve the gene and discrete profile, then read cBioPortal's counts for both calls.
pub fn copy_number(leg: &Leg<'_>, q: &StudyGet) -> Result<StudyCopyNumber, RequestError> {
    let gene = gene(leg, q)?;
    let profile = selected(copy_number_profiles(leg, q)?, q)?;
    let body = [AMPLIFIED, DEEP_DELETED].map(|alteration| Identifier {
        entrez_gene_id: gene.get(),
        alteration,
    });
    let endpoint = SPEC
        .post_json("/molecular-profiles")
        .with_segments(&[
            PathSegment::new(profile.as_str())?,
            PathSegment::new("discrete-copy-number-counts")?,
            PathSegment::new("fetch")?,
        ])
        .fresh(Freshness::Days(1))
        .bytes(MAX_BYTES);
    let received = leg.call(
        &endpoint,
        Input::json(&[], &body),
        Read::<Counts>::json().check(&|v| {
            let first = v.0.first().map(|c| c.number_of_samples);
            let scoped = v.0.iter().all(|c| {
                c.molecular_profile_id == profile
                    && c.entrez_gene_id == gene.get()
                    && Some(c.number_of_samples) == first
            });
            let each = [AMPLIFIED, DEEP_DELETED]
                .iter()
                .all(|a| v.0.iter().filter(|c| c.alteration == *a).count() == 1);
            if v.0.len() == 2 && scoped && each {
                Ok(())
            } else {
                Err(BadReply)
            }
        }),
    )?;
    let counts = received.value.0;
    let number_of_samples = counts
        .first()
        .map(|c| c.number_of_samples)
        .ok_or_else(|| biomcp_fetch::bad_reply(biomcp_types::Source::Cbioportal, None))?;
    let samples = |alteration| {
        counts
            .iter()
            .find(|c| c.alteration == alteration)
            .and_then(|c| c.number_of_samples_with_alteration_in_gene)
            .unwrap_or(0)
    };
    Ok(StudyCopyNumber {
        study_id: q.id().clone(),
        molecular_profile_id: profile.clone(),
        entrez_gene_id: gene,
        number_of_samples,
        amplified: samples(AMPLIFIED),
        deep_deleted: samples(DEEP_DELETED),
        acquired: acquired(received.meta.fetched)?,
    })
}
