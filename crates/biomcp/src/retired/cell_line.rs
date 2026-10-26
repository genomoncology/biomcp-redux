//! Deferred cell line routes fail before parsing or provider admission.
use biomcp_types::ErrorMessage;
pub(super) fn message(words: &super::Words<'_>) -> Option<ErrorMessage> {
    match words.positional.as_slice() {
        ["cell-line", "drug-response", ..] => Some(ErrorMessage::CellLineDrugResponse),
        ["gene", "cell-lines", ..] => Some(ErrorMessage::CellLineExpression),
        ["get", "cell-line", input @ ..] if !words.literal => {
            if words
                .sections
                .iter()
                .any(|s| matches!(*s, "drug_response" | "drug-response"))
                || input
                    .iter()
                    .skip(1)
                    .any(|s| matches!(*s, "drug_response" | "drug-response"))
            {
                Some(ErrorMessage::CellLineDrugResponse)
            } else {
                None
            }
        }
        _ => None,
    }
}
