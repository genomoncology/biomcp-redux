//! Gene-wide germline counts from ClinVar E-utilities searches, each Variation ID once.
//! `[prop]` classification filters follow the aggregate germline classification, so their
//! counts need no record download. `[rvst]` also indexes somatic review statuses, so review
//! buckets come from Variation ID lists, and only records listed under more than one status
//! are summarized to read their germline review status. Requests run one at a time: NCBI
//! refuses a fourth keyless request in any one second, and evenly paced parallel requests
//! still reach it. Other clients on the same address share that limit, so a throttled
//! request waits the retry time NCBI names and asks again while the budget lasts.
//! <https://www.ncbi.nlm.nih.gov/clinvar/docs/help/>
use super::GENE_COUNTS as SPEC;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Input, Leg, Read, RequestError, bad_reply};
use biomcp_types::{ClinVarGeneCounts, ClinVarGermlineCounts, Source};
const INFO: Endpoint = SPEC
    .get("/einfo.fcgi")
    .fresh(Freshness::Days(1))
    .wait_throttle();
const SEARCH: Endpoint = SPEC
    .get("/esearch.fcgi")
    .fresh(Freshness::Days(1))
    .wait_throttle();
/// E-utilities returns at most this many IDs per search page.
const ID_PAGE: u64 = 10_000;
const P: &str = "\"clinsig pathogenic\"[prop]";
const LP: &str = "\"clinsig likely pathogenic\"[prop]";
const VUS: &str = "\"clinsig vus\"[prop]";
const CONFLICTS: &str = "\"clinsig has conflicts\"[prop]";
const LB: &str = "\"clinsig likely benign\"[prop]";
const B: &str = "\"clinsig benign\"[prop]";
/// Any GRCh38 base position.
const GRCH38: &str = "1:999999999[cpos]";

use super::gene_reply::{Info, Search};
use super::gene_review::{REVIEW, review_counts};

fn count(leg: &Leg<'_>, term: &str) -> Result<u64, RequestError> {
    let reply = leg
        .call(
            &SEARCH,
            Input::Params(&[
                ("db", Arg::Text("clinvar")),
                ("term", Arg::Text(term)),
                ("retmax", Arg::Num(0)),
                ("retmode", Arg::Text("json")),
                ("tool", Arg::Text("biomcp")),
            ]),
            Read::<Search>::json(),
        )?
        .value;
    reply
        .esearchresult
        .total()
        .map_err(|_| bad_reply(Source::ClinVar, None))
}

/// One page of Variation IDs and the search's total.
fn ids(leg: &Leg<'_>, term: &str, start: u64) -> Result<(u64, Vec<u64>), RequestError> {
    let check = |reply: &Search| {
        let found = &reply.esearchresult;
        let total = found.total()?;
        let expected = total.saturating_sub(start).min(ID_PAGE);
        if found.retstart != start.to_string() || found.idlist.len() as u64 != expected {
            return Err(BadReply);
        }
        Ok(())
    };
    let mut params = vec![
        ("db", Arg::Text("clinvar")),
        ("term", Arg::Text(term)),
        ("retmax", Arg::Num(ID_PAGE)),
        ("retmode", Arg::Text("json")),
        ("tool", Arg::Text("biomcp")),
    ];
    if start != 0 {
        params.push(("retstart", Arg::Num(start)));
    }
    let reply = leg
        .call(
            &SEARCH,
            Input::Params(&params),
            Read::<Search>::json().check(&check),
        )?
        .value;
    let found = reply.esearchresult;
    Ok((
        found
            .total()
            .map_err(|_| bad_reply(Source::ClinVar, None))?,
        found.ids().map_err(|_| bad_reply(Source::ClinVar, None))?,
    ))
}

fn info(leg: &Leg<'_>) -> Result<(biodata::Text, biodata::Text), RequestError> {
    let reply = leg
        .call(
            &INFO,
            Input::Params(&[
                ("db", Arg::Text("clinvar")),
                ("retmode", Arg::Text("json")),
                ("tool", Arg::Text("biomcp")),
            ]),
            Read::<Info>::json(),
        )?
        .value;
    let info = reply
        .einforesult
        .dbinfo
        .into_iter()
        .next()
        .ok_or_else(|| bad_reply(Source::ClinVar, None))?;
    Ok((
        crate::mapped::source_text(&info.dbbuild).map_err(|_| bad_reply(Source::ClinVar, None))?,
        crate::mapped::source_text(&info.lastupdate)
            .map_err(|_| bad_reply(Source::ClinVar, None))?,
    ))
}

fn count_terms(base: &str, plp: &str) -> [String; 10] {
    [
        base.to_owned(),
        format!("{base} AND {P}"),
        format!("{base} AND {LP}"),
        format!("{base} AND {P} AND {LP}"),
        format!("{base} AND {VUS}"),
        format!("{base} AND {CONFLICTS}"),
        format!("{base} AND {LB}"),
        format!("{base} AND {B}"),
        format!("{base} AND {B} AND {LB}"),
        format!("{plp} AND {GRCH38}"),
    ]
}

/// Split the overlapping property counts, in `count_terms` order, into disjoint buckets.
fn germline(counts: [u64; 10]) -> ClinVarGermlineCounts {
    let [records, p, lp, both, vus, conflicts, lb, b, b_lb, _] = counts;
    ClinVarGermlineCounts {
        pathogenic: p.saturating_sub(both),
        likely_pathogenic: lp.saturating_sub(both),
        pathogenic_likely_pathogenic: both,
        uncertain_significance: vus,
        conflicting: conflicts,
        likely_benign: lb.saturating_sub(b_lb),
        benign: b.saturating_sub(b_lb),
        benign_likely_benign: b_lb,
        other: records.saturating_sub(
            (p + lp).saturating_sub(both) + vus + conflicts + (lb + b).saturating_sub(b_lb),
        ),
    }
}

/// Count ClinVar records listing this NCBI Gene ID.
pub fn gene_counts(leg: &Leg<'_>, gene_id: u32) -> Result<ClinVarGeneCounts, RequestError> {
    let base = format!("{gene_id}[gid]");
    let plp = format!("{base} AND ({P} OR {LP})");
    let reviews: Vec<String> = REVIEW
        .iter()
        .map(|status| format!("{plp} AND \"{status}\"[rvst]"))
        .collect();
    let (build, updated) = info(leg)?;
    let mut numbers = [0u64; 10];
    for (number, term) in numbers.iter_mut().zip(count_terms(&base, &plp)) {
        *number = count(leg, &term)?;
    }
    let mut lists = Vec::new();
    for term in &reviews {
        let (total, mut found) = ids(leg, term, 0)?;
        // Later pages for a status listing more than one page of records.
        for page in 1..total.div_ceil(ID_PAGE) {
            found.extend(ids(leg, term, page * ID_PAGE)?.1);
        }
        lists.push((total, found));
    }
    let [records, p, lp, both, .., grch38] = numbers;
    let germline = germline(numbers);
    let total = (p + lp).saturating_sub(both);
    let review = review_counts(leg, &lists, total)?;
    Ok(ClinVarGeneCounts {
        build,
        build_updated: updated,
        scope: crate::mapped::text(&format!(
            "Each ClinVar variation record (VCV) that lists NCBI Gene {gene_id}, counted once \
             whatever its assemblies, including records that also list other genes; aggregate \
             germline classification only."
        ))
        .map_err(|_| bad_reply(Source::ClinVar, None))?,
        term: crate::mapped::text(&base).map_err(|_| bad_reply(Source::ClinVar, None))?,
        records,
        germline,
        pathogenic_or_likely: total,
        pathogenic_or_likely_grch38: grch38,
        pathogenic_or_likely_review: review,
    })
}
