//! Monarch's one ranked list of genes or diseases for a set of HP terms.
use crate::raw::Field;
use biodata::{Curie, FiniteNumber, HpId};
use biomcp_fetch::{
    Arg, BadReply, Freshness, Input, Leg, PathSegment, Read, Reply, ReplyMetadata, RequestError,
};
use biomcp_types::{
    PhenotypeBestMatch, PhenotypeGroup, PhenotypeMatch, PhenotypeMatchMethod, PhenotypeMatchTerm,
    PhenotypeMetric, PhenotypeSearch,
};
use serde::Deserialize;
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
    /// Monarch's stated settings from the first row.
    pub method: Option<PhenotypeMatchMethod>,
    /// Rejected optional fields.
    pub problems: Vec<biomcp_types::Problem>,
    /// Rows refused for lacking an identity or score.
    pub refusals: crate::Refusal,
    /// Original retrieval metadata.
    pub metadata: ReplyMetadata,
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
    let termset = terms
        .iter()
        .map(HpId::as_str)
        .collect::<Vec<_>>()
        .join(",");
    let endpoint = super::super::SPEC
        .get("/v3/api/semsim/search")
        .with_segments(&[
            PathSegment::new(&termset)?,
            PathSegment::new(group(q.group()))?,
        ])
        .fresh(Freshness::Days(1));
    let wanted = metric(q.metric());
    let args = [
        ("limit", Arg::Num(q.limit().into())),
        ("metric", Arg::Text(wanted)),
    ];
    let received = leg.call(
        &endpoint,
        Input::Params(&args),
        Read::<Rows>::json()
            .check(&|rows| {
                let metric_ok = rows.0.iter().all(|row| {
                    row.search_settings
                        .as_ref()
                        .and_then(Field::as_ref)
                        .and_then(|settings| settings.metric.as_ref())
                        .and_then(Field::as_ref)
                        .is_none_or(|stated| stated.as_ref() == wanted)
                });
                if rows.0.len() > q.limit() as usize || !metric_ok {
                    Err(BadReply)
                } else {
                    Ok(())
                }
            })
            .cache_if(&|rows| rows.0.iter().all(Row::admitted)),
    )?;
    Ok(map(received.value, &terms, received.meta))
}

fn map(rows: Rows, terms: &[HpId], metadata: ReplyMetadata) -> Ranked {
    let mut problems = Vec::new();
    let mut refusals = crate::Refusal::None;
    let mut labels: BTreeMap<String, Option<Box<str>>> = BTreeMap::new();
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
        if method.is_none() {
            method = row.search_settings.and_then(Field::value).map(Settings::mapped);
        }
        let id = row.subject.as_ref().and_then(|s| s.id.as_ref()).and_then(Field::as_ref);
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
        let mut best_matches = Vec::new();
        let mut seen = Vec::new();
        for term in terms {
            if seen.contains(&term.as_str()) {
                continue;
            }
            seen.push(term.as_str());
            if let Some(found_match) = best.get(term.as_str()).and_then(Field::as_ref)
                && let Some(mapped) = found_match.mapped(term, &mut found)
            {
                best_matches.push(mapped);
            }
        }
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
    // Monarch lists a term it does not know without a label and ranks without it.
    // An empty reply names no term: one known term always ranks entities, so none was known.
    let none_ranked = items.is_empty() && refusals == crate::Refusal::None;
    let mut unknown: Vec<HpId> = Vec::new();
    for term in terms {
        let unlabeled = match labels.get(term.as_str()) {
            Some(label) => label.is_none(),
            None => termset_seen,
        };
        if (none_ranked || unlabeled) && !unknown.contains(term) {
            unknown.push(term.clone());
        }
    }
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
        method,
        problems,
        refusals,
        metadata,
    }
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct Rows(Vec<Row>);
impl Reply for Rows {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
struct Row {
    subject: Option<Subject>,
    score: Option<Field<FiniteNumber>>,
    subject_to_object_score: Option<Field<FiniteNumber>>,
    object_to_subject_score: Option<Field<FiniteNumber>>,
    similarity: Option<Field<Similarity>>,
    search_settings: Option<Field<Settings>>,
}
impl Row {
    fn admitted(&self) -> bool {
        self.subject
            .as_ref()
            .and_then(|s| s.id.as_ref())
            .and_then(Field::as_ref)
            .is_some()
            && self.score.as_ref().and_then(Field::as_ref).is_some()
    }
}
#[derive(Debug, Default, Deserialize)]
struct Subject {
    id: Option<Field<Curie>>,
    category: Option<Field<Curie>>,
    name: Option<Field<Box<str>>>,
}
#[derive(Debug, Deserialize)]
struct Similarity {
    object_termset: Option<Field<BTreeMap<String, Field<Term>>>>,
    object_best_matches: Option<Field<BTreeMap<String, Field<BestMatch>>>>,
}
#[derive(Debug, Deserialize)]
struct Term {
    label: Option<Field<Box<str>>>,
}
#[derive(Debug, Deserialize)]
struct BestMatch {
    match_source: Option<Field<Curie>>,
    match_target: Option<Field<Curie>>,
    match_target_label: Option<Field<Box<str>>>,
    match_subsumer: Option<Field<Curie>>,
    match_subsumer_label: Option<Field<Box<str>>>,
    score: Option<Field<FiniteNumber>>,
}
impl BestMatch {
    fn mapped(
        &self,
        term: &HpId,
        problems: &mut crate::mapped::Problems,
    ) -> Option<PhenotypeBestMatch> {
        let source = self.match_source.as_ref().and_then(Field::as_ref);
        let matched = self.match_target.as_ref().and_then(Field::as_ref);
        let (Some(source), Some(matched)) = (source, matched) else {
            crate::mapped::push_problem(
                &mut problems.list,
                problems.source,
                problems.record.clone(),
                "similarity.object_best_matches",
            );
            return None;
        };
        if source.as_str() != term.as_str() {
            crate::mapped::push_problem(
                &mut problems.list,
                problems.source,
                problems.record.clone(),
                "similarity.object_best_matches.match_source",
            );
            return None;
        }
        Some(PhenotypeBestMatch {
            term: source.clone(),
            matched: matched.clone(),
            matched_label: self
                .match_target_label
                .as_ref()
                .and_then(Field::as_ref)
                .cloned(),
            ancestor: self.match_subsumer.as_ref().and_then(Field::as_ref).cloned(),
            ancestor_label: self
                .match_subsumer_label
                .as_ref()
                .and_then(Field::as_ref)
                .cloned(),
            score: self.score.as_ref().and_then(Field::as_ref).copied(),
        })
    }
}
#[derive(Debug, Deserialize)]
struct Settings {
    engine: Option<Field<Box<str>>>,
    metric: Option<Field<Box<str>>>,
    directionality: Option<Field<Box<str>>>,
    search_mode: Option<Field<Box<str>>>,
    information_content: Option<Field<Box<str>>>,
}
impl Settings {
    fn mapped(self) -> PhenotypeMatchMethod {
        PhenotypeMatchMethod {
            metric: self.metric.and_then(Field::value),
            directionality: self.directionality.and_then(Field::value),
            search_mode: self.search_mode.and_then(Field::value),
            information_content: self.information_content.and_then(Field::value),
            engine: self.engine.and_then(Field::value),
        }
    }
}
