//! The versioned error response and its conversion from usage failures.
use super::{ErrorCode, ErrorMessage, ErrorStatus, literals::ErrorSchema};
use crate::{CacheProblem, Step, StepKind, UsageError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    url: Option<crate::Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source: Option<crate::Source>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    option: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    valid_values: Option<Box<[crate::Text]>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    count: Option<u64>,
    /// Chosen `--fields` names with no field to place them in, which the next call leaves out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    unplaced: Option<Box<[crate::Text]>>,
    code: ErrorCode,
    message: ErrorMessage,
}

/// A versioned error response. Of rejected caller values it repeats only `error.unplaced` names: letters, digits, `_` and `.`, up to 64 characters.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ErrorEnvelope {
    schema: ErrorSchema,
    status: ErrorStatus,
    error: ErrorDetail,
    /// Bounded cache or recording failure when local startup cannot open its store or capture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_error: Option<CacheProblem>,
    /// Typed restart when a snapshot cannot answer this query.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next: Vec<Step>,
}

impl From<crate::UsageError> for ErrorEnvelope {
    fn from(usage: crate::UsageError) -> Self {
        let mut out = Self::empty(
            ErrorStatus::Usage,
            ErrorCode::Arguments,
            ErrorMessage::Arguments,
        );
        match usage {
            UsageError::Arguments(arguments) => out.next = out.error.arguments(arguments),
            UsageError::Schema => {
                out.error.code = ErrorCode::SchemaUsage;
                out.error.message = ErrorMessage::SchemaUsage;
            }
            UsageError::UnknownCommand { next } => {
                out.error.code = ErrorCode::UnknownCommand;
                out.error.message = ErrorMessage::UnknownCommand;
                out.next = vec![next];
            }
            UsageError::Retired(message, next) => out.retired(message, next),
            UsageError::McpUnavailable(message) => {
                out.error.code = ErrorCode::McpUnavailable;
                out.error.message = message;
            }
            UsageError::HostedKey(source) => {
                out.error.code = ErrorCode::McpUnavailable;
                out.error.message = ErrorMessage::HostedKey;
                out.error.source = Some(source);
            }
            UsageError::McpResultTooBroad { cap } => {
                out.error.code = ErrorCode::TooBroad;
                out.error.message = ErrorMessage::McpResultTooBroad;
                out.error.count = Some(cap);
            }
            UsageError::BiomarkerTooBroad => {
                out.error.code = ErrorCode::TooBroad;
                out.error.message = ErrorMessage::BiomarkerTooBroad;
                out.error.source = Some(crate::Source::Nci);
            }
            UsageError::OrganizationTooBroad => {
                out.error.code = ErrorCode::TooBroad;
                out.error.message = ErrorMessage::OrganizationTooBroad;
                out.error.source = Some(crate::Source::Nci);
            }
            UsageError::UnsupportedAdverseEvent => {}
            UsageError::TextPositionError(option) => out.error.option = Some(option.into()),
            UsageError::TrialContinuationRejected => {
                out.error
                    .trial_continuation(ErrorMessage::TrialContinuationRejected);
            }
            UsageError::TrialContinuationUnavailable => {
                out.error
                    .trial_continuation(ErrorMessage::TrialContinuationUnavailable);
            }
            UsageError::ArticlePartRange { input, last } => out.article_part_range(input, last),
            UsageError::UnsupportedTrialFilter { source, option } => {
                out.error.filter(source, option.as_str())
            }
            UsageError::UnsupportedFilter { source, option } => {
                out.error.filter(source, option.as_str())
            }
            UsageError::NciDiseaseError => out.error.message = ErrorMessage::NciDisease,
            UsageError::VariantQueryPolicyError => {
                out.error.message = ErrorMessage::VariantQueryPolicy
            }
            UsageError::PdfSaveRefused(link) => out.error.url = Some(link.url),
            UsageError::ArticleSaveRefused(url) => out.error.url = Some(url),
            UsageError::TooManyPmids { count } => {
                out.error.code = ErrorCode::TooManyPmids;
                out.error.message = ErrorMessage::TooManyPmids;
                out.error.count = Some(count);
            }
            selector @ UsageError::SourceSelector { .. } => out.source_selector(selector),
            fields @ UsageError::FieldsUnknown { .. } => out.fields_unknown(fields),
            UsageError::PoolMismatch { restart } => {
                out.error.code = ErrorCode::PoolMismatch;
                out.error.message = ErrorMessage::PoolMismatch;
                out.next = vec![restart];
            }
        }
        out
    }
}

impl ErrorEnvelope {
    /// Chosen `--fields` names a call's rows lack, the names it cannot place and the call with the rest.
    fn fields_unknown(&mut self, usage: UsageError) {
        if let UsageError::FieldsUnknown {
            source,
            valid_values,
            unplaced,
            message,
            next,
        } = usage
        {
            self.error
                .source_selector(source, "--fields".into(), valid_values);
            self.error.message = message;
            self.error.unplaced = (!unplaced.is_empty()).then_some(unplaced);
            self.next = next;
        }
    }

    fn retired(&mut self, message: ErrorMessage, next: Vec<Step>) {
        self.error.code = ErrorCode::Retired;
        self.error.message = message;
        self.error.valid_values = retired_values(message);
        self.next = next;
    }

    fn source_selector(&mut self, usage: crate::UsageError) {
        if let crate::UsageError::SourceSelector {
            source,
            message,
            option,
            valid_values,
            next,
        } = usage
        {
            self.error.source_selector(source, option, valid_values);
            self.error.message = message;
            self.next = next;
        }
    }
    fn article_part_range(&mut self, input: crate::ArticleInput, last: u32) {
        self.error.option = Some("--part".into());
        self.error.message = ErrorMessage::ArgumentValue;
        self.error.count = Some(u64::from(last));
        self.error.valid_values = Some(if last == 0 {
            Box::default()
        } else {
            crate::Text::new(&format!("1..{last}"))
                .ok()
                .into_iter()
                .collect()
        });
        self.next.push(Step {
            kind: StepKind::Section,
            args: vec![
                "get".into(),
                "article".into(),
                input.to_string().into(),
                "fulltext".into(),
            ]
            .into(),
            options: Default::default(),
        });
    }

    /// Report a local failure without paths or provider diagnostics.
    pub fn local() -> Self {
        Self::empty(ErrorStatus::Local, ErrorCode::Local, ErrorMessage::Local)
    }
    /// Report a local failure with fixed guidance.
    pub fn local_message(message: ErrorMessage) -> Self {
        Self::empty(ErrorStatus::Local, ErrorCode::Local, message)
    }
    /// Name a rejected configuration field without its value.
    pub fn configuration(option: String) -> Self {
        let mut out = Self::local();
        out.error.option = Some(option.into_boxed_str());
        out
    }
    /// Report an expired pool independently of command usage.
    pub fn expired_pool(restart: Step) -> Self {
        let mut out = Self::empty(
            ErrorStatus::Expired,
            ErrorCode::PoolExpired,
            ErrorMessage::PoolExpired,
        );
        out.next = vec![restart];
        out
    }
    fn empty(status: ErrorStatus, code: ErrorCode, message: ErrorMessage) -> Self {
        Self {
            schema: ErrorSchema::V2,
            status,
            error: ErrorDetail {
                url: None,
                source: None,
                option: None,
                valid_values: None,
                count: None,
                unplaced: None,
                code,
                message,
            },
            local_error: None,
            next: Vec::new(),
        }
    }
}

impl ErrorDetail {
    fn trial_continuation(&mut self, message: ErrorMessage) {
        self.source = Some(crate::Source::ClinicalTrialsGov);
        self.option = Some("--next-page".into());
        self.message = message;
    }
    fn arguments(&mut self, arguments: crate::UsageArguments) -> Vec<Step> {
        self.option = arguments.argument;
        self.message = arguments.message.unwrap_or_else(|| {
            if self.option.is_some() && arguments.valid_values.is_some() {
                ErrorMessage::ArgumentValue
            } else {
                ErrorMessage::Arguments
            }
        });
        self.valid_values = arguments.valid_values;
        arguments.next
    }
    fn source_selector(
        &mut self,
        source: impl Into<Option<crate::Source>>,
        option: Box<str>,
        values: Box<[crate::Text]>,
    ) {
        self.source = source.into();
        self.option = Some(option);
        self.valid_values = Some(values);
    }
    fn filter(&mut self, source: crate::Source, option: &str) {
        self.source = Some(source);
        self.option = Some(option.into());
        self.code = ErrorCode::UnsupportedFilter;
        self.message = ErrorMessage::UnsupportedFilter;
    }
}
fn retired_values(message: ErrorMessage) -> Option<Box<[crate::Text]>> {
    (message == ErrorMessage::VaersInfluenzaCodes).then(|| {
        crate::VAERS_INFLUENZA
            .iter()
            .filter_map(|(code, label)| crate::Text::new(&format!("{code}: {label}")).ok())
            .collect()
    })
}
