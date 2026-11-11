//! Card facts the shared card type does not hold: enrollment and the reason a study stopped.
use super::Problems;
use crate::raw::Field;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Enrollment {
    count: Option<Field<u64>>,
    #[serde(rename = "type")]
    kind: Option<Field<biodata::TrialDateKind>>,
}
/// Protocol facts shown beside the card that its shared type does not hold.
#[derive(Default)]
pub(super) struct Posted {
    pub(super) results_first_posted: Option<biodata::PartialDate>,
    pub(super) enrollment: Option<biomcp_types::TrialEnrollment>,
    pub(super) why_stopped: Option<Box<str>>,
}
impl Enrollment {
    /// A qualifier without a count says nothing, so the count is required.
    pub(super) fn mapped(self, problems: &mut Problems) -> Option<biomcp_types::TrialEnrollment> {
        let count = problems.required_field("enrollmentInfo.count", self.count)?;
        Some(biomcp_types::TrialEnrollment {
            count,
            kind: problems.field("enrollmentInfo.type", self.kind),
        })
    }
}
/// The registry escapes this plain sentence as Markdown; readers get the sentence.
pub(super) fn why_stopped(
    value: Option<Field<Box<str>>>,
    problems: &mut Problems,
) -> Option<Box<str>> {
    let escaped = problems.field("whyStopped", value)?;
    let plain = crate::source_text::unescape_markdown(&escaped);
    problems
        .text("whyStopped", Some(plain.as_ref()))
        .map(|text| text.as_str().into())
}
