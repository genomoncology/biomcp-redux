//! Native transcript/residue counts; public reachability grants no redistribution rights.
use crate::{Mapped, quotas};
use biodata::{GeneSymbol, Text};

use biomcp_fetch::{
    BadReply, Endpoint, Freshness, KeyUse, Leg, PathSegment, Reply, RequestError, SourceSpec,
};
use biomcp_types::{HotspotAlternate, HotspotMembership, HotspotRecord, Source};
use serde::Deserialize;
use std::time::Duration;
/// Publisher terms remain unverified; raw captures remain external.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "Publisher-specific terms; data reuse unverified",
        url: "https://www.cancerhotspots.org/",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::CancerHotspots,
    base: "https://www.cancerhotspots.org",
    key: KeyUse::None,
    quota: &quotas::HOTSPOTS,
    deadline: Duration::from_secs(8),
};
const GENE: Endpoint = SPEC
    .get("/api/hotspots/single/byGene")
    .fresh(Freshness::Days(7));
#[derive(Deserialize)]
#[serde(transparent)]
struct Rows(Vec<Row>);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    hugo_symbol: GeneSymbol,
    transcript_id: Option<Text>,
    residue: Text,
    tumor_count: u64,
    variant_amino_acid: std::collections::BTreeMap<String, u64>,
}
impl Reply for Rows {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.iter().any(|r| {
            r.variant_amino_acid.is_empty()
                || r.variant_amino_acid
                    .keys()
                    .any(|key| Text::new(key).is_err())
        }) {
            return Err(BadReply);
        }
        Ok(())
    }
}
/// Read native gene rows as candidates until a transcript/consequence join is established.
pub fn gene(leg: &Leg<'_>, gene: &GeneSymbol) -> Result<Mapped<Vec<HotspotRecord>>, RequestError> {
    let segment = PathSegment::new(gene.as_str()).map_err(RequestError::Input)?;
    let endpoint = GENE.clone().with_segments(&[segment]);
    let raw: Rows = leg.json_checked_cache(&endpoint, &[], |rows: &Rows| {
        rows.0.iter().all(|r| &r.hugo_symbol == gene)
    })?;
    let mut problems = Vec::new();
    let mut rows = Vec::new();
    for row in raw.0 {
        if &row.hugo_symbol != gene {
            problems.push(biomcp_types::Problem {
                source: Source::CancerHotspots,
                record: Some(row.residue),
                field: "transcript_gene".into(),
                code: biomcp_types::ProblemCode::ValueUnknown,
            });
            continue;
        }
        let transcript = row.transcript_id;
        rows.push(HotspotRecord {
            membership: HotspotMembership::Candidate,
            gene: row.hugo_symbol,
            transcript,
            residue: row.residue,
            tumor_count: row.tumor_count,
            alternates: row
                .variant_amino_acid
                .into_iter()
                .map(|(amino_acid, count)| {
                    Text::new(&amino_acid)
                        .map(|amino_acid| HotspotAlternate { amino_acid, count })
                        .map_err(|_| {
                            biomcp_fetch::SourceError {
                                source: Source::CancerHotspots,
                                reason: biomcp_types::Reason::BadReply,
                                status: None,
                                retry_after: None,
                            }
                            .into()
                        })
                })
                .collect::<Result<Box<[_]>, RequestError>>()?,
        });
    }
    Ok(Mapped {
        value: rows,
        problems,
    })
}
