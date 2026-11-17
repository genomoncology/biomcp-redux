//! Map OLS cross-references without replacing source attribution with their targets.
use super::raw;
use crate::mapped::{Problems, record as record_text};
use crate::{Mapped, raw::Field};
use biodata::{Attribution, DiseaseEndpoint, DiseaseRelation};
use biomcp_types::{DiseaseXrefItem, Source};

pub(super) fn map(term: &mut raw::Term, attribution: &Attribution) -> Mapped<Vec<DiseaseXrefItem>> {
    let mut mapped = Mapped {
        refusals: crate::Refusal::None,
        value: Vec::new(),
        problems: Vec::new(),
    };
    let mut metadata = Problems {
        source: Source::Ols,
        record: record_text(&term.obo_id),
        list: Vec::new(),
    };
    let Some(rows) = metadata.field("obo_xref", term.obo_xref.take()) else {
        mapped.problems = metadata.list;
        return mapped;
    };
    for native in rows {
        let Field::Value(native) = native else {
            metadata.required::<()>("obo_xref", None);
            continue;
        };
        let mut problems = Problems {
            source: Source::Ols,
            record: record_text(&term.obo_id),
            list: Vec::new(),
        };
        let id = match (native.database, native.id) {
            (Field::Value(database), Field::Value(id)) => {
                problems.optional("xref.id", Some(format!("{database}:{id}").parse()))
            }
            _ => problems.required("xref.id", None),
        };
        let description = problems.optional(
            "xref.description",
            native.description.as_ref().map(|v| match v {
                Field::Value(v) => Ok::<_, ()>(v.as_str().into()),
                Field::Invalid(_) => Err(()),
            }),
        );
        let target_url = problems.url("xref.target_url", native.url);
        mapped.problems.extend(problems.list);
        let Some(id) = id else {
            continue;
        };
        let Ok(predicate) = "oboInOwl:hasDbXref".parse() else {
            continue;
        };
        mapped.value.push(DiseaseXrefItem {
            relation: DiseaseRelation {
                subject: DiseaseEndpoint {
                    id: term.obo_id.clone(),
                    label: match &term.label {
                        Some(Field::Value(v)) => Some(v.clone().into()),
                        _ => None,
                    },
                },
                predicate,
                object: DiseaseEndpoint { id, label: None },
                attribution: attribution.clone(),
                description,
            },
            target_url,
        });
    }
    mapped.problems.extend(metadata.list);
    mapped
}
