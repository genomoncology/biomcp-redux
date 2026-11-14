//! Germline review buckets from per-status Variation ID lists, summarizing only shared records.
use super::super::SPEC;
use super::raw::Summary;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Input, Leg, Read, RequestError, bad_reply};
use biomcp_types::{ClinVarReviewCounts, Source};
use std::collections::BTreeMap;
const SUMMARY: Endpoint = SPEC.get("/esummary.fcgi").fresh(Freshness::Days(1));
/// Summaries per request, keeping the ID list well inside a URL.
const SUMMARY_PAGE: usize = 200;
/// Germline review statuses in the order of `ClinVarReviewCounts`.
pub(super) const REVIEW: [&str; 5] = [
    "practice guideline",
    "reviewed by expert panel",
    "criteria provided, multiple submitters, no conflicts",
    "criteria provided, single submitter",
    "no assertion criteria provided",
];

/// Germline review statuses for records that more than one status search listed.
fn review_statuses(leg: &Leg<'_>, ids: &[u64]) -> Result<Vec<String>, RequestError> {
    let pages: Vec<Vec<Box<str>>> = ids
        .chunks(SUMMARY_PAGE)
        .map(|page| page.iter().map(|id| id.to_string().into()).collect())
        .collect();
    let mut statuses = Vec::new();
    for page in &pages {
        let reply = leg
            .call(
                &SUMMARY,
                Input::Params(&[
                    ("db", Arg::Text("clinvar")),
                    ("id", Arg::Ids(page)),
                    ("retmode", Arg::Text("json")),
                    ("tool", Arg::Text("biomcp")),
                ]),
                Read::<Summary>::json().check(&|reply: &Summary| {
                    let mut uids: Vec<&str> =
                        reply.result.uids.iter().map(String::as_str).collect();
                    uids.sort_unstable();
                    let mut asked: Vec<&str> = page.iter().map(AsRef::as_ref).collect();
                    asked.sort_unstable();
                    if uids == asked { Ok(()) } else { Err(BadReply) }
                }),
            )?
            .value;
        statuses.extend(
            reply
                .result
                .records
                .into_values()
                .map(|record| record.germline_classification.review_status),
        );
    }
    Ok(statuses)
}

/// Place each listed record in its one germline review status.
pub(super) fn review_counts(
    leg: &Leg<'_>,
    lists: &[(u64, Vec<u64>)],
    total: u64,
) -> Result<ClinVarReviewCounts, RequestError> {
    let mut seen: BTreeMap<u64, (usize, u32)> = BTreeMap::new();
    for (index, (_, ids)) in lists.iter().enumerate() {
        for id in ids {
            let entry = seen.entry(*id).or_insert((index, 0));
            entry.1 += 1;
        }
    }
    let mut buckets = [0u64; 5];
    let mut other = 0u64;
    let mut shared = Vec::new();
    for (id, (index, times)) in &seen {
        if *times == 1 {
            *buckets
                .get_mut(*index)
                .ok_or_else(|| bad_reply(Source::ClinVar, None))? += 1;
        } else {
            shared.push(*id);
        }
    }
    for status in review_statuses(leg, &shared)? {
        match REVIEW.iter().position(|known| *known == status) {
            Some(index) => {
                *buckets
                    .get_mut(index)
                    .ok_or_else(|| bad_reply(Source::ClinVar, None))? += 1
            }
            None => other += 1,
        }
    }
    other += total.saturating_sub(seen.len() as u64);
    let [
        practice_guideline,
        expert_panel,
        multiple_submitters,
        single_submitter,
        no_assertion_criteria,
    ] = buckets;
    Ok(ClinVarReviewCounts {
        practice_guideline,
        expert_panel,
        multiple_submitters,
        single_submitter,
        no_assertion_criteria,
        other,
    })
}
