//! Gene-wide germline counts from ClinVar E-utilities searches, each Variation ID once.
//! `[prop]` classification filters follow the aggregate germline classification, so their
//! counts need no record download. `[rvst]` also indexes somatic review statuses, so review
//! buckets come from Variation ID lists, and only records listed under more than one status
//! are summarized to read their germline review status. Requests run one at a time: NCBI
//! refuses a fourth keyless request in any one second, and evenly paced parallel requests
//! still reach it.
//! <https://www.ncbi.nlm.nih.gov/clinvar/docs/help/>
use super::SPEC;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Input, Leg, Read, Reply, RequestError};
use biomcp_types::{ClinVarGeneCounts, ClinVarGermlineCounts, ClinVarReviewCounts};
use serde::Deserialize;
use std::collections::BTreeMap;
const INFO: Endpoint = SPEC.get("/einfo.fcgi").fresh(Freshness::Days(1));
const SEARCH: Endpoint = SPEC.get("/esearch.fcgi").fresh(Freshness::Days(1));
const SUMMARY: Endpoint = SPEC.get("/esummary.fcgi").fresh(Freshness::Days(1));
/// E-utilities returns at most this many IDs per search page.
const ID_PAGE: u64 = 10_000;
/// Summaries per request, keeping the ID list well inside a URL.
const SUMMARY_PAGE: usize = 200;
const P: &str = "\"clinsig pathogenic\"[prop]";
const LP: &str = "\"clinsig likely pathogenic\"[prop]";
const VUS: &str = "\"clinsig vus\"[prop]";
const CONFLICTS: &str = "\"clinsig has conflicts\"[prop]";
const LB: &str = "\"clinsig likely benign\"[prop]";
const B: &str = "\"clinsig benign\"[prop]";
/// Any GRCh38 base position.
const GRCH38: &str = "1:999999999[cpos]";
/// Germline review statuses in the order of `ClinVarReviewCounts`.
const REVIEW: [&str; 5] = [
    "practice guideline",
    "reviewed by expert panel",
    "criteria provided, multiple submitters, no conflicts",
    "criteria provided, single submitter",
    "no assertion criteria provided",
];

#[derive(Deserialize)]
struct Info {
    einforesult: InfoResult,
}
#[derive(Deserialize)]
struct InfoResult {
    dbinfo: Vec<DbInfo>,
}
#[derive(Deserialize)]
struct DbInfo {
    dbname: String,
    dbbuild: String,
    lastupdate: String,
}
impl Reply for Info {
    fn check(&self) -> Result<(), BadReply> {
        match self.einforesult.dbinfo.as_slice() {
            [info]
                if info.dbname == "clinvar"
                    && !info.dbbuild.trim().is_empty()
                    && !info.lastupdate.trim().is_empty() =>
            {
                Ok(())
            }
            _ => Err(BadReply),
        }
    }
    fn data_date(&self) -> Option<String> {
        self.einforesult
            .dbinfo
            .first()
            .map(|info| info.lastupdate.clone())
    }
}

#[derive(Deserialize)]
struct Search {
    esearchresult: Found,
}
#[derive(Deserialize)]
struct Found {
    count: String,
    retstart: String,
    #[serde(default)]
    idlist: Vec<String>,
    #[serde(rename = "ERROR")]
    error: Option<serde::de::IgnoredAny>,
    errorlist: Option<serde::de::IgnoredAny>,
}
impl Found {
    fn total(&self) -> Result<u64, BadReply> {
        self.count.parse().map_err(|_| BadReply)
    }
    fn ids(&self) -> Result<Vec<u64>, BadReply> {
        self.idlist
            .iter()
            .map(|id| id.parse().map_err(|_| BadReply))
            .collect()
    }
}
impl Reply for Search {
    /// An unknown property or status phrase arrives as an error list with a zero count.
    fn check(&self) -> Result<(), BadReply> {
        let found = &self.esearchresult;
        if found.error.is_some() || found.errorlist.is_some() {
            return Err(BadReply);
        }
        found.total()?;
        found.ids()?;
        found.retstart.parse::<u64>().map_err(|_| BadReply)?;
        Ok(())
    }
}

#[derive(Deserialize)]
struct Summary {
    result: SummaryResult,
}
#[derive(Deserialize)]
struct SummaryResult {
    uids: Vec<String>,
    #[serde(flatten)]
    records: BTreeMap<String, SummaryRecord>,
}
#[derive(Deserialize)]
struct SummaryRecord {
    germline_classification: Germline,
}
#[derive(Deserialize)]
struct Germline {
    review_status: String,
}
impl Reply for Summary {
    fn check(&self) -> Result<(), BadReply> {
        let result = &self.result;
        if result.uids.len() != result.records.len()
            || result
                .uids
                .iter()
                .any(|uid| !result.records.contains_key(uid))
        {
            return Err(BadReply);
        }
        Ok(())
    }
}

fn bad(_: BadReply) -> RequestError {
    biomcp_fetch::bad_reply(biomcp_types::Source::ClinVar, None)
}

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
    reply.esearchresult.total().map_err(bad)
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
    Ok((found.total().map_err(bad)?, found.ids().map_err(bad)?))
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
        .ok_or_else(|| bad(BadReply))?;
    Ok((
        crate::mapped::source_text(&info.dbbuild).map_err(bad)?,
        crate::mapped::source_text(&info.lastupdate).map_err(bad)?,
    ))
}

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

enum Job {
    Info,
    Count(String),
    Ids(String, u64),
}
enum Done {
    Info(biodata::Text, biodata::Text),
    Count(u64),
    Ids(u64, Vec<u64>),
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

fn run(child: &Leg<'_>, job: &Job) -> Result<Done, RequestError> {
    Ok(match job {
        Job::Info => {
            let (build, updated) = info(child)?;
            Done::Info(build, updated)
        }
        Job::Count(term) => Done::Count(count(child, term)?),
        Job::Ids(term, start) => {
            let (total, ids) = ids(child, term, *start)?;
            Done::Ids(total, ids)
        }
    })
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
    let jobs: Vec<Job> = std::iter::once(Job::Info)
        .chain(count_terms(&base, &plp).into_iter().map(Job::Count))
        .chain(reviews.iter().map(|term| Job::Ids(term.clone(), 0)))
        .collect();
    let mut release = None;
    let mut numbers = Vec::new();
    let mut lists = Vec::new();
    for job in &jobs {
        match run(leg, job)? {
            Done::Info(build, updated) => release = Some((build, updated)),
            Done::Count(n) => numbers.push(n),
            Done::Ids(total, ids) => lists.push((total, ids)),
        }
    }
    let (build, updated) = release.ok_or_else(|| bad(BadReply))?;
    // Later pages for a status listing more than one page of records.
    let (owners, more): (Vec<usize>, Vec<Job>) = reviews
        .iter()
        .zip(&lists)
        .enumerate()
        .flat_map(|(index, (term, (total, _)))| {
            (1..total.div_ceil(ID_PAGE))
                .map(move |page| (index, Job::Ids(term.clone(), page * ID_PAGE)))
        })
        .unzip();
    for (index, job) in owners.into_iter().zip(&more) {
        let Done::Ids(_, ids) = run(leg, job)? else {
            return Err(bad(BadReply));
        };
        lists
            .get_mut(index)
            .ok_or_else(|| bad(BadReply))?
            .1
            .extend(ids);
    }
    let numbers = <[u64; 10]>::try_from(numbers.as_slice()).map_err(|_| bad(BadReply))?;
    let [records, p, lp, both, .., grch38] = numbers;
    let germline = germline(numbers);
    let total = (p + lp).saturating_sub(both);
    let review = review_counts(leg, &lists, total)?;
    Ok(ClinVarGeneCounts {
        build,
        updated,
        scope: crate::mapped::text(&format!(
            "Each ClinVar variation record (VCV) that lists NCBI Gene {gene_id}, counted once whatever its assemblies, including records that also list other genes; aggregate germline classification only."
        ))
        .map_err(bad)?,
        term: crate::mapped::text(&base).map_err(bad)?,
        records,
        germline,
        pathogenic_or_likely: total,
        pathogenic_or_likely_grch38: grch38,
        pathogenic_or_likely_review: review,
    })
}

/// Place each listed record in its one germline review status.
fn review_counts(
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
            *buckets.get_mut(*index).ok_or_else(|| bad(BadReply))? += 1;
        } else {
            shared.push(*id);
        }
    }
    for status in review_statuses(leg, &shared)? {
        match REVIEW.iter().position(|known| *known == status) {
            Some(index) => *buckets.get_mut(index).ok_or_else(|| bad(BadReply))? += 1,
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
