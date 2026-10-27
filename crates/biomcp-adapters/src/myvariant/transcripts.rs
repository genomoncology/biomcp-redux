//! Preserve only source-paired transcript rows; independent annotation arrays are not joined.
use crate::raw::{Field, OneOrMany};
use biodata::Text;
use biomcp_types::{Problem, ProblemCode, Source, VariantTranscript};
use serde::Deserialize;

pub(super) const FIELD: &str = "snpeff.ann";

#[derive(Deserialize)]
pub(super) struct Snpeff {
    ann: Option<Field<OneOrMany<Field<Annotation>>>>,
}
#[derive(Deserialize)]
struct Annotation {
    feature_id: Text,
    feature_type: Text,
    genename: Option<Text>,
    hgvs_c: Option<Text>,
    hgvs_p: Option<Text>,
}
pub(super) fn map(
    raw: Option<Field<Snpeff>>,
    record: &Text,
    problems: &mut Vec<Problem>,
) -> Box<[VariantTranscript]> {
    let problem = || Problem {
        source: Source::MyVariant,
        record: Some(record.clone()),
        field: FIELD.into(),
        code: ProblemCode::ValueUnknown,
    };
    let Some(raw) = raw else {
        return Box::default();
    };
    let Some(raw) = raw.value() else {
        problems.push(problem());
        return Box::default();
    };
    let Some(ann) = raw.ann else {
        return Box::default();
    };
    let Some(ann) = ann.value() else {
        problems.push(problem());
        return Box::default();
    };
    let mut rows = Vec::new();
    for annotation in ann.values() {
        let Some(a) = annotation.value() else {
            problems.push(problem());
            continue;
        };
        if a.feature_type.as_str() != "transcript" {
            continue;
        }
        if a.hgvs_c.as_ref().is_some_and(|v| {
            !["c.", "n.", "r."]
                .iter()
                .any(|prefix| v.as_str().starts_with(prefix))
        }) || a
            .hgvs_p
            .as_ref()
            .is_some_and(|v| !v.as_str().starts_with("p."))
        {
            problems.push(problem());
            continue;
        }
        rows.push(VariantTranscript {
            source: Source::MyVariant,
            component: Text::new(FIELD).expect("static component"),
            transcript: a.feature_id,
            gene: a.genename,
            hgvs_c: a.hgvs_c,
            hgvs_p: a.hgvs_p,
            matched: false,
        });
    }
    rows.into()
}
