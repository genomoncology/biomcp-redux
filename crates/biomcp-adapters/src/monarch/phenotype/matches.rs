//! Monarch's one ranked list of genes or diseases for a set of HP terms.
use super::matches_raw::{BestMatch, Row, Rows, Settings, Term};
use crate::raw::Field;
use biodata::HpId;
use biomcp_fetch::{
    Arg, BadReply, Freshness, Input, Leg, PathSegment, Read, ReplyMetadata, RequestError,
};
use biomcp_types::{
    PHENOTYPE_MATCH_MAX, PhenotypeBestMatch, PhenotypeGroup, PhenotypeMatch, PhenotypeMatchMethod,
    PhenotypeMatchTerm, PhenotypeMetric, PhenotypeSearch, PhenotypeSearchMode,
};
use std::collections::BTreeMap;

/// Ranked rows, the queried terms as Monarch knew them, and retrieval metadata.
#[derive(Debug)]
pub struct Ranked {
    /// Rows in Monarch's order.
    pub items: Vec<PhenotypeMatch>,
    /// Queried terms in caller order with Monarch's labels.
    pub terms: Vec<PhenotypeMatchTerm>,
    /// Queried terms Monarch did not know; Monarch ranks without them.
    pub unknown: Vec<HpId>,
    /// Monarch returned no row at all, which it does when it knows none of the terms.
    pub empty: bool,
    /// Monarch's stated settings from the first row.
    pub method: Option<PhenotypeMatchMethod>,
    /// Rejected optional fields.
    pub problems: Vec<biomcp_types::Problem>,
    /// Rows refused for lacking an identity or score.
    pub refusals: crate::Refusal,
    /// Original retrieval metadata.
    pub metadata: ReplyMetadata,
}

/// Monarch's similarity search: a full search took 4.5 to 19.3 s when measured on
/// 2026-11-13, beyond the 15 s that suits other Monarch reads; the command budget still caps it.
pub static SPEC: biomcp_fetch::SourceSpec = biomcp_fetch::SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: biomcp_types::Source::Monarch,
    base: super::super::SPEC.base,
    terms: super::super::SPEC.terms,
    quota: &crate::quotas::MONARCH,
    key: biomcp_fetch::KeyUse::None,
    deadline: std::time::Duration::from_secs(30),
};

/// Monarch's search mode spelling.
fn mode(mode: PhenotypeSearchMode) -> &'static str {
    match mode {
        PhenotypeSearchMode::Hybrid => "hybrid",
        PhenotypeSearchMode::Full => "full",
    }
}

/// Monarch's group spelling.
fn group(group: PhenotypeGroup) -> &'static str {
    match group {
        PhenotypeGroup::Genes => "Human Genes",
        PhenotypeGroup::Diseases => "Human Diseases",
    }
}

/// Monarch's metric spelling.
fn metric(metric: PhenotypeMetric) -> &'static str {
    match metric {
        PhenotypeMetric::AncestorInformationContent => "ancestor_information_content",
        PhenotypeMetric::JaccardSimilarity => "jaccard_similarity",
        PhenotypeMetric::PhenodigmScore => "phenodigm_score",
    }
}

/// Send the caller's HP IDs, in order, to Monarch's semantic similarity search.
pub fn search(leg: &Leg<'_>, q: &PhenotypeSearch) -> Result<Ranked, RequestError> {
    crate::disease_sources::components::monarch(leg)?;
    let terms = q.terms().ok_or(biomcp_fetch::InputError::Path)?;
    let termset = terms.iter().map(HpId::as_str).collect::<Vec<_>>().join(",");
    let endpoint = SPEC
        .get("/v3/api/semsim/search")
        .with_segments(&[
            PathSegment::new(&termset)?,
            PathSegment::new(group(q.group()))?,
        ])
        .fresh(Freshness::Days(1));
    let wanted = metric(q.metric());
    let wanted_mode = mode(q.search_mode());
    // A hybrid shortlist grows with the limit, so a small limit changes the order itself.
    // Hybrid always asks for Monarch's full 50 rows and keeps the caller's first rows.
    let asked = match q.search_mode() {
        PhenotypeSearchMode::Hybrid => PHENOTYPE_MATCH_MAX,
        PhenotypeSearchMode::Full => q.limit(),
    };
    let args = [
        ("limit", Arg::Num(asked.into())),
        ("metric", Arg::Text(wanted)),
        ("search_mode", Arg::Text(wanted_mode)),
    ];
    let received = leg.call(
        &endpoint,
        Input::Params(&args),
        Read::<Rows>::json()
            .check(&|rows| {
                let stated = |field: fn(&Settings) -> &Option<Field<Box<str>>>, value: &str| {
                    rows.0.iter().all(|row| {
                        row.search_settings
                            .as_ref()
                            .and_then(Field::as_ref)
                            .and_then(|settings| field(settings).as_ref())
                            .and_then(Field::as_ref)
                            .is_none_or(|stated| stated.as_ref() == value)
                    })
                };
                let metric_ok =
                    stated(|s| &s.metric, wanted) && stated(|s| &s.search_mode, wanted_mode);
                if rows.0.len() > asked as usize || !metric_ok {
                    Err(BadReply)
                } else {
                    Ok(())
                }
            })
            // An empty reply is not cached: it can also be a transient Monarch answer.
            .cache_if(&|rows| !rows.0.is_empty() && rows.0.iter().all(Row::admitted)),
    )?;
    Ok(map(
        received.value,
        &terms,
        q.limit() as usize,
        received.meta,
    ))
}

fn map(rows: Rows, terms: &[HpId], keep: usize, metadata: ReplyMetadata) -> Ranked {
    let empty = rows.0.is_empty();
    let mut problems = Vec::new();
    let mut refusals = crate::Refusal::None;
    let mut labels = Labels::new();
    let mut method = None;
    let mut termset_seen = false;
    let mut items = Vec::new();
    for row in rows.0 {
        let similarity = row.similarity.and_then(Field::value);
        if let Some(set) = similarity
            .as_ref()
            .and_then(|s| s.object_termset.as_ref())
            .and_then(Field::as_ref)
        {
            termset_seen = true;
            collect_labels(set, &mut labels);
        }
        if method.is_none() {
            method = row
                .search_settings
                .and_then(Field::value)
                .map(Settings::mapped);
        }
        // Rows past the caller's limit still name the terms Monarch knew.
        if items.len() >= keep {
            continue;
        }
        let id = row
            .subject
            .as_ref()
            .and_then(|s| s.id.as_ref())
            .and_then(Field::as_ref);
        let score = row.score.as_ref().and_then(Field::as_ref).copied();
        let (Some(id), Some(score)) = (id.cloned(), score) else {
            refusals = crate::Refusal::Refused;
            continue;
        };
        let mut found = crate::mapped::Problems {
            source: biomcp_types::Source::Monarch,
            record: crate::mapped::record(&id),
            list: Vec::new(),
        };
        let subject = row.subject.unwrap_or_default();
        let best = similarity
            .and_then(|s| s.object_best_matches)
            .and_then(Field::value)
            .unwrap_or_default();
        let best_matches = best_matches(&best, terms, &mut found);
        items.push(PhenotypeMatch {
            rank: items.len() as u32 + 1,
            id,
            label: found.field("subject.name", subject.name),
            category: found.field("subject.category", subject.category),
            score,
            subject_to_object_score: found
                .field("subject_to_object_score", row.subject_to_object_score),
            object_to_subject_score: found
                .field("object_to_subject_score", row.object_to_subject_score),
            best_matches: best_matches.into(),
        });
        problems.append(&mut found.list);
    }
    let unknown = unknown(terms, &labels, termset_seen);
    let terms = terms
        .iter()
        .map(|term| PhenotypeMatchTerm {
            id: term.clone(),
            label: labels.get(term.as_str()).cloned().flatten(),
        })
        .collect();
    Ranked {
        items,
        terms,
        unknown,
        empty,
        method,
        problems,
        refusals,
        metadata,
    }
}

type Labels = BTreeMap<String, Option<Box<str>>>;
/// Keep the first label Monarch gave each queried term across rows.
fn collect_labels(set: &BTreeMap<String, Field<Term>>, labels: &mut Labels) {
    for (key, term) in set {
        let label = term
            .as_ref()
            .and_then(|term| term.label.as_ref())
            .and_then(Field::as_ref)
            .cloned();
        let entry = labels.entry(key.clone()).or_insert(None);
        if entry.is_none() {
            *entry = label;
        }
    }
}
/// Each distinct queried term's best match, in caller order.
fn best_matches(
    best: &BTreeMap<String, Field<BestMatch>>,
    terms: &[HpId],
    found: &mut crate::mapped::Problems,
) -> Vec<PhenotypeBestMatch> {
    let mut out = Vec::new();
    let mut seen = Vec::new();
    for term in terms {
        if seen.contains(&term.as_str()) {
            continue;
        }
        seen.push(term.as_str());
        if let Some(found_match) = best.get(term.as_str()).and_then(Field::as_ref)
            && let Some(mapped) = found_match.mapped(term, found)
        {
            out.push(mapped);
        }
    }
    out
}
/// Monarch lists a term it does not know without a label and ranks without it.
fn unknown(terms: &[HpId], labels: &Labels, termset_seen: bool) -> Vec<HpId> {
    let mut out: Vec<HpId> = Vec::new();
    for term in terms {
        let unlabeled = match labels.get(term.as_str()) {
            Some(label) => label.is_none(),
            None => termset_seen,
        };
        if unlabeled && !out.contains(term) {
            out.push(term.clone());
        }
    }
    out
}
