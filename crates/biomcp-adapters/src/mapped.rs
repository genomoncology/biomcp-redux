use crate::native_text::NativeText;
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
    pub(crate) fn field<T>(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<T>>,
    ) -> Option<T> {
        self.optional(field, value.map(|v| v.value().ok_or(())))
    }
    pub(crate) fn source_field(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<NativeText>>,
    ) -> Option<biodata::Text> {
        self.field(field, value).map(Into::into)
    }
    pub(crate) fn required_source_field(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<NativeText>>,
    ) -> Option<biodata::Text> {
        let value = self.source_field(field, value);
        self.required(field, value)
    }
    pub(crate) fn required_field<T>(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<T>>,
    ) -> Option<T> {
        let value = self.field(field, value);
        self.required(field, value)
    }
    pub(crate) fn fields<T>(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<Vec<crate::raw::Field<T>>>>,
    ) -> Box<[T]> {
        self.field(field, value)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| self.field(field, Some(v)))
            .collect()
    }
    /// Admit native text through the source boundary with visible control escapes.
    pub(crate) fn native_text(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<Box<str>>>,
    ) -> Option<Box<str>> {
        let value = self.field(field, value)?;
        self.text(field, Some(&value))
            .map(|text| text.as_str().into())
    }
    pub(crate) fn checked_native_text(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<Box<str>>>,
    ) -> Option<Box<str>> {
        let value = self.field(field, value)?;
        self.checked_text(field, Some(&value)).map(|_| value)
    }
    pub(crate) fn required_checked_native_text(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<Box<str>>>,
    ) -> Option<Box<str>> {
        let value = self.checked_native_text(field, value);
        self.required(field, value)
    }
    pub(crate) fn required_native_text(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<Box<str>>>,
    ) -> Option<Box<str>> {
        let value = self.native_text(field, value);
        self.required(field, value)
    }
    pub(crate) fn native_texts(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<Vec<crate::raw::Field<Box<str>>>>>,
    ) -> Box<[Box<str>]> {
        self.field(field, value)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|value| self.native_text(field, Some(value)))
            .collect()
    }
    /// Keep the native spelling of a checked annotation URL without download policy.
    pub(crate) fn url(
        &mut self,
        field: &str,
        value: Option<crate::raw::Field<biodata::Text>>,
    ) -> Option<biodata::Text> {
        let value = self.field(field, value)?;
        self.optional(
            field,
            Some(biomcp_fetch::check_annotation_url(value.as_str())),
        )?;
        Some(value)
    }
    pub(crate) fn checked_text(
        &mut self,
        field: &str,
        value: Option<&str>,
    ) -> Option<biodata::Text> {
        self.optional(field, value.map(biodata::Text::new))
    }
    pub(crate) fn text(&mut self, field: &str, value: Option<&str>) -> Option<biodata::Text> {
        self.optional(field, value.map(biodata::Text::from_source))
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
    checked_field_text(
        problems,
        source,
        record,
        field,
        value,
        biodata::Text::from_source,
    )
}

/// Preserve strict admission for native source codes and identities.
pub(crate) fn field_code_text(
    problems: &mut Vec<Problem>,
    source: biomcp_types::Source,
    record: Option<biodata::Text>,
    field: &str,
    value: Option<crate::raw::Field<String>>,
) -> Option<biodata::Text> {
    checked_field_text(problems, source, record, field, value, biodata::Text::new)
}

fn checked_field_text(
    problems: &mut Vec<Problem>,
    source: biomcp_types::Source,
    record: Option<biodata::Text>,
    field: &str,
    value: Option<crate::raw::Field<String>>,
    admit: impl Fn(&str) -> Result<biodata::Text, biodata::Error>,
) -> Option<biodata::Text> {
    match value?.value().map(|s| admit(&s)) {
        Some(Ok(value)) => Some(value),
        _ => {
            push_problem(problems, source, record, field);
            None
        }
    }
}

/// Preserve a typed source identity as neutral problem provenance.
pub(crate) fn record(value: impl std::fmt::Display) -> Option<biodata::Text> {
    text(&value.to_string()).ok()
}

/// Admit source prose while keeping the strict identity helper separate.
pub(crate) fn source_text(value: &str) -> Result<biodata::Text, biomcp_fetch::BadReply> {
    biodata::Text::from_source(value).map_err(|_| biomcp_fetch::BadReply)
}

/// Convert redacted bundle row rejections to the existing problem channel.
pub(crate) fn bundle_problems(
    source: biomcp_types::Source,
    info: &biomcp_fetch::BundleInfo,
) -> Vec<Problem> {
    info.rejections
        .iter()
        .map(|rejection| {
            let (part, reason) = rejection
                .reason
                .rsplit_once(": ")
                .map_or((None, rejection.reason.as_str()), |(part, reason)| {
                    (Some(part), reason)
                });
            let reason = match reason {
                "no usable query key" => "no_usable_query_key",
                "wrong number of columns" => "column_count",
                _ => "decode",
            };
            let location = match part {
                Some(part) => format!("{}.{part}", info.name),
                None => info.name.to_owned(),
            };
            problem(
                source,
                None,
                &format!("{location}.rows.{}.{reason}", rejection.row),
            )
        })
        .collect()
}

/// Convert ordinary bundle synchronization metadata with its rejected rows.
pub(crate) fn bundle_sync(
    source: biomcp_types::Source,
    info: biomcp_fetch::BundleInfo,
) -> Mapped<biomcp_types::SyncResult> {
    Mapped {
        problems: bundle_problems(source, &info),
        value: biomcp_types::SyncResult {
            snapshot: None,
            etag: None,
            last_modified: None,
            duplicate_identities: None,
            rows: info.rows,
            bytes: info.bytes,
            downloaded_bytes: info.downloaded_bytes,
        },
    }
}
