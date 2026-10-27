use biomcp_types::Problem;

/// A source value with rejected fields retained as problems.
#[derive(Debug)]
pub struct Mapped<T> {
    /// Accepted mapped value.
    pub value: T,
    /// Fields that could not be accepted.
    pub problems: Vec<Problem>,
}

/// Construct a rejected-field disclosure in one place for every adapter.
pub(crate) fn problem(
    source: biomcp_types::Source,
    record: Option<biodata::Text>,
    field: &str,
) -> Problem {
    Problem {
        source,
        record,
        field: field.into(),
        code: biomcp_types::ProblemCode::ValueUnknown,
    }
}

/// Report each rejected field once per source record.
pub(crate) fn push_problem(
    problems: &mut Vec<Problem>,
    source: biomcp_types::Source,
    record: Option<biodata::Text>,
    field: &str,
) {
    let value = problem(source, record, field);
    if !problems.contains(&value) {
        problems.push(value);
    }
}

/// Strict construction for required text such as source identity.
pub(crate) fn text(value: &str) -> Result<biodata::Text, biomcp_fetch::BadReply> {
    biodata::Text::new(value).map_err(|_| biomcp_fetch::BadReply)
}

/// Mapping context for optional fields; absent values produce no problem.
pub(crate) struct Problems {
    pub(crate) source: biomcp_types::Source,
    pub(crate) record: Option<biodata::Text>,
    pub(crate) list: Vec<Problem>,
}
impl Problems {
    pub(crate) fn required<T>(&mut self, field: &str, value: Option<T>) -> Option<T> {
        if value.is_none() {
            push_problem(&mut self.list, self.source, self.record.clone(), field);
        }
        value
    }
    pub(crate) fn optional<T, E>(&mut self, field: &str, value: Option<Result<T, E>>) -> Option<T> {
        self.required(field, value?.ok())
    }
    pub(crate) fn text(&mut self, field: &str, value: Option<&str>) -> Option<biodata::Text> {
        self.optional(field, value.map(text))
    }
}

/// Convert one present optional provider field, preserving invalid shapes as problems.
pub(crate) fn field_text(
    problems: &mut Vec<Problem>,
    source: biomcp_types::Source,
    record: Option<biodata::Text>,
    field: &str,
    value: Option<crate::raw::Field<String>>,
) -> Option<biodata::Text> {
    let value = value?.value().and_then(|s| text(&s).ok());
    if value.is_none() {
        push_problem(problems, source, record, field);
    }
    value
}
