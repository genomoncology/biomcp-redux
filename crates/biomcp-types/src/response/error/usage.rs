//! Each usage refusal as its fixed error response.
use super::{ErrorCode, ErrorEnvelope, ErrorMessage, ErrorStatus};
use crate::UsageError;

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
            UsageError::FieldsUnknown {
                source,
                valid_values,
                unplaced,
                next,
            } => {
                out.error
                    .source_selector(source, "--fields".into(), valid_values);
                (out.error.message, out.error.unplaced) = (
                    ErrorMessage::FieldsUnknown,
                    (!unplaced.is_empty()).then_some(unplaced),
                );
                out.next = next;
            }
            UsageError::PoolMismatch { restart } => {
                out.error.code = ErrorCode::PoolMismatch;
                out.error.message = ErrorMessage::PoolMismatch;
                out.next = vec![restart];
            }
        }
        out
    }
}
