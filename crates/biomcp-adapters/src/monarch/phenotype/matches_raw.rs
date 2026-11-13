//! Monarch semantic similarity search rows as received, before mapping.
use crate::raw::Field;
use biodata::{Curie, FiniteNumber, HpId};
use biomcp_fetch::{BadReply, Reply};
use biomcp_types::{PhenotypeBestMatch, PhenotypeMatchMethod};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(super) struct Rows(pub(super) Vec<Row>);
impl Reply for Rows {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Row {
    pub(super) subject: Option<Subject>,
    pub(super) score: Option<Field<FiniteNumber>>,
    pub(super) subject_to_object_score: Option<Field<FiniteNumber>>,
    pub(super) object_to_subject_score: Option<Field<FiniteNumber>>,
    pub(super) similarity: Option<Field<Similarity>>,
    pub(super) search_settings: Option<Field<Settings>>,
}
impl Row {
    pub(super) fn admitted(&self) -> bool {
        self.subject
            .as_ref()
            .and_then(|s| s.id.as_ref())
            .and_then(Field::as_ref)
            .is_some()
            && self.score.as_ref().and_then(Field::as_ref).is_some()
    }
}
#[derive(Debug, Default, Deserialize)]
pub(super) struct Subject {
    pub(super) id: Option<Field<Curie>>,
    pub(super) category: Option<Field<Curie>>,
    pub(super) name: Option<Field<Box<str>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Similarity {
    pub(super) object_termset: Option<Field<BTreeMap<String, Field<Term>>>>,
    pub(super) object_best_matches: Option<Field<BTreeMap<String, Field<BestMatch>>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Term {
    pub(super) label: Option<Field<Box<str>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct BestMatch {
    pub(super) match_source: Option<Field<Curie>>,
    pub(super) match_target: Option<Field<Curie>>,
    pub(super) match_target_label: Option<Field<Box<str>>>,
    pub(super) match_subsumer: Option<Field<Curie>>,
    pub(super) match_subsumer_label: Option<Field<Box<str>>>,
    pub(super) score: Option<Field<FiniteNumber>>,
}
impl BestMatch {
    pub(super) fn mapped(
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
            ancestor: self
                .match_subsumer
                .as_ref()
                .and_then(Field::as_ref)
                .cloned(),
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
pub(super) struct Settings {
    pub(super) engine: Option<Field<Box<str>>>,
    pub(super) metric: Option<Field<Box<str>>>,
    pub(super) directionality: Option<Field<Box<str>>>,
    pub(super) search_mode: Option<Field<Box<str>>>,
    pub(super) information_content: Option<Field<Box<str>>>,
}
impl Settings {
    pub(super) fn mapped(self) -> PhenotypeMatchMethod {
        PhenotypeMatchMethod {
            metric: self.metric.and_then(Field::value),
            directionality: self.directionality.and_then(Field::value),
            search_mode: self.search_mode.and_then(Field::value),
            information_content: self.information_content.and_then(Field::value),
            engine: self.engine.and_then(Field::value),
        }
    }
}
