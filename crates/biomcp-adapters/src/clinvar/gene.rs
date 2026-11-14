//! Gene-wide germline counts from ClinVar E-utilities searches, each Variation ID once.
//! `[prop]` classification filters follow the aggregate germline classification, so their
//! counts need no record download. `[rvst]` also indexes somatic review statuses, so review
//! buckets come from Variation ID lists, and only records listed under more than one status
//! are summarized to read their germline review status. Requests run one at a time: NCBI
//! refuses a fourth keyless request in any one second, and evenly paced parallel requests
//! still reach it.
//! <https://www.ncbi.nlm.nih.gov/clinvar/docs/help/>
use super::SPEC;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Input, Leg, Read, RequestError, bad_reply};
use biomcp_types::{ClinVarGeneCounts, ClinVarGermlineCounts, Source};
const INFO: Endpoint = SPEC.get("/einfo.fcgi").fresh(Freshness::Days(1));
const SEARCH: Endpoint = SPEC.get("/esearch.fcgi").fresh(Freshness::Days(1));
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

mod raw;
mod review;
use raw::{Info, Search};
use review::{REVIEW, review_counts};

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
    let (build, updated) = release.ok_or_else(|| bad_reply(Source::ClinVar, None))?;
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
            return Err(bad_reply(Source::ClinVar, None));
        };
        lists
            .get_mut(index)
            .ok_or_else(|| bad_reply(Source::ClinVar, None))?
            .1
            .extend(ids);
    }
    let numbers =
        <[u64; 10]>::try_from(numbers.as_slice()).map_err(|_| bad_reply(Source::ClinVar, None))?;
    let [records, p, lp, both, .., grch38] = numbers;
    let germline = germline(numbers);
    let total = (p + lp).saturating_sub(both);
    let review = review_counts(leg, &lists, total)?;
    Ok(ClinVarGeneCounts {
        build,
        updated,
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
