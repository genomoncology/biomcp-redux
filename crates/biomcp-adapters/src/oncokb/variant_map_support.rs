//! Source prose uses the shared field-disclosure boundary.
use biodata::Text;
use biomcp_types::{Problem, Source};

pub(super) fn prose_texts(values: &[String], problems: &mut Vec<Problem>) -> Box<[Text]> {
    values
        .iter()
        .filter_map(|s| {
            crate::mapped::field_text(
                problems,
                Source::Oncokb,
                None,
                "treatments.approvedIndications",
                Some(crate::raw::Field::Value(s.clone())),
            )
        })
        .collect()
}

pub(super) fn abstracts(
    rows: &[super::super::variant::Abstract],
) -> Box<[biomcp_types::OncokbCitation]> {
    rows.iter()
        .map(|row| biomcp_types::OncokbCitation {
            text: row.abstract_content.as_deref().map(Into::into),
            link: row.link.clone(),
        })
        .collect()
}

pub(super) fn text(value: &str, field: &str, problems: &mut Vec<Problem>) -> Option<Text> {
    crate::mapped::field_text(
        problems,
        Source::Oncokb,
        None,
        field,
        Some(crate::raw::Field::Value(value.to_owned())),
    )
}
