//! ChEMBL single-protein targets of one UniProt accession, with ChEMBL's own assay and activity totals.
use super::SPEC as CHEMBL;
use super::cell::PageMeta;
use biodata::{Text, UniProtIsoform};
use biomcp_fetch::{Arg, BadReply, Freshness, Input, Leg, Reply, RequestError};
use biomcp_types::{ProteinChembl, ProteinChemblTarget, Source};
use serde::Deserialize;
/// One protein's section takes three paced ChEMBL reads: on 2026-11-17 a fresh target lookup took
/// 2.7 s from the M5 and each count about 1 s, and fresh cards for EGFR alone and for POLQ with
/// PARP1 ran out of the 20 s default after UniProt (pm job j42227). The section gets 30 s per protein and its command 60 s, the paced ceiling.
pub static TARGET_COUNTS: biomcp_fetch::SourceSpec = biomcp_fetch::SourceSpec {
    destinations: CHEMBL.destinations,
    user_agent: CHEMBL.user_agent,
    terms: CHEMBL.terms,
    source: Source::Chembl,
    base: CHEMBL.base,
    key: CHEMBL.key,
    quota: CHEMBL.quota,
    deadline: std::time::Duration::from_secs(30),
    command_budget: Some(std::time::Duration::from_secs(60)),
};
const SPEC: &biomcp_fetch::SourceSpec = &TARGET_COUNTS;
const TARGETS: biomcp_fetch::Endpoint = SPEC.get("/target.json").fresh(Freshness::Days(1));
const ASSAYS: biomcp_fetch::Endpoint = SPEC.get("/assay.json").fresh(Freshness::Days(1));
const ACTIVITIES: biomcp_fetch::Endpoint = SPEC.get("/activity.json").fresh(Freshness::Days(1));
/// ChEMBL's target type for one protein, as opposed to complexes, families and selectivity groups.
const SINGLE: &str = "SINGLE PROTEIN";
#[derive(Debug, Deserialize)]
struct Targets {
    targets: Vec<Target>,
    page_meta: PageMeta,
}
#[derive(Debug, Deserialize)]
struct Target {
    target_chembl_id: String,
    target_type: String,
    pref_name: Option<String>,
    organism: Option<String>,
    target_components: Vec<Component>,
}
#[derive(Debug, Deserialize)]
struct Component {
    accession: Option<String>,
}
impl Reply for Targets {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
/// A count reply: one row at most, kept only to check its target.
#[derive(Debug, Deserialize)]
struct Counted {
    #[serde(alias = "assays", alias = "activities")]
    rows: Vec<Row>,
    page_meta: PageMeta,
}
#[derive(Debug, Deserialize)]
struct Row {
    target_chembl_id: Option<String>,
}
impl Reply for Counted {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
fn checked(value: &str) -> Result<Text, RequestError> {
    crate::mapped::text(value).map_err(|_| biomcp_fetch::bad_reply(Source::Chembl, Some(200)))
}
/// The accession ChEMBL is asked about: ChEMBL names canonical accessions, so an isoform's
/// canonical accession.
pub fn canonical(accession: &UniProtIsoform) -> &str {
    accession
        .as_str()
        .split_once('-')
        .map_or(accession.as_str(), |(base, _)| base)
}
/// Every single-protein target ChEMBL lists for the accession, up to `limit`, with each target's
/// assay and activity totals as ChEMBL's `page_meta.total_count` reports them.
///
/// ChEMBL names canonical accessions, so an isoform asks for its canonical accession.
pub fn protein_targets(
    leg: &Leg<'_>,
    accession: &UniProtIsoform,
    limit: u32,
) -> Result<ProteinChembl, RequestError> {
    let canonical = canonical(accession);
    let reply = leg
        .call(
            &TARGETS,
            Input::Params(&[
                ("target_components__accession", Arg::Text(canonical)),
                ("target_type", Arg::Text(SINGLE)),
                ("limit", Arg::Num(limit.into())),
                ("offset", Arg::Num(0)),
            ]),
            biomcp_fetch::Read::json().check(&|r: &Targets| {
                r.page_meta.check(r.targets.len(), limit, 0)?;
                let exact = r.targets.iter().all(|t| {
                    t.target_type == SINGLE
                        && t.target_components
                            .iter()
                            .any(|c| c.accession.as_deref() == Some(canonical))
                });
                if exact { Ok(()) } else { Err(BadReply) }
            }),
        )?
        .value;
    let mut items = Vec::new();
    for t in reply.targets {
        let id = checked(&t.target_chembl_id)?;
        items.push(ProteinChemblTarget {
            assay_count: total(leg, &ASSAYS, &t.target_chembl_id)?,
            activity_count: total(leg, &ACTIVITIES, &t.target_chembl_id)?,
            id,
            name: t.pref_name.as_deref().map(checked).transpose()?,
            organism: t.organism.as_deref().map(checked).transpose()?,
        });
    }
    Ok(ProteinChembl {
        accession: Some(checked(canonical)?),
        total: reply.page_meta.total_count,
        items: items.into(),
    })
}
/// ChEMBL's own total for one target's rows; the one row read must name that target.
fn total(leg: &Leg<'_>, endpoint: &biomcp_fetch::Endpoint, id: &str) -> Result<u64, RequestError> {
    let reply = leg
        .call(
            endpoint,
            Input::Params(&[
                ("target_chembl_id", Arg::Text(id)),
                ("limit", Arg::Num(1)),
                ("offset", Arg::Num(0)),
            ]),
            biomcp_fetch::Read::json().check(&|r: &Counted| {
                r.page_meta.check(r.rows.len(), 1, 0)?;
                if r.rows
                    .iter()
                    .all(|row| row.target_chembl_id.as_deref() == Some(id))
                {
                    Ok(())
                } else {
                    Err(BadReply)
                }
            }),
        )?
        .value;
    Ok(reply.page_meta.total_count)
}
