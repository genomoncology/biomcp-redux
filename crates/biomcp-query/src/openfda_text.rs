//! Notices for openFDA text values sent as other words than typed (0841).
use crate::Outcome;
use biodata::Text;
use biomcp_types::{Source, SourceNotice, openfda_words};
use std::borrow::Cow;

/// Tell the caller, on the openFDA source's report, each typed value openFDA was sent as other
/// words: openFDA refuses some punctuation and every character outside ASCII, so those become
/// spaces and accented letters their base letters. Drug replies carry no source_query, so this
/// notice is the only place the change shows. A value searched with others as one OR group is
/// marked `true`: emptied, only that value matches nothing, while the group still returns rows
/// (0976).
pub(crate) fn disclose<'a>(
    out: &mut Outcome,
    source: Source,
    values: impl IntoIterator<Item = (&'static str, Option<&'a Text>, bool)>,
) {
    let notices: Vec<SourceNotice> = values
        .into_iter()
        .filter_map(|(option, value, grouped)| Some((option, value?, grouped)))
        .filter_map(|(option, value, grouped)| match openfda_words(value.as_str()) {
            Cow::Borrowed(_) => None,
            Cow::Owned(words) if words.is_empty() => Some(format!(
                "{option} held no letter or digit openFDA accepts, so {} matched nothing.",
                if grouped { "that value" } else { "it" }
            )),
            Cow::Owned(words) => Some(format!(
                "{option} was searched as \"{words}\": openFDA refuses some punctuation and every \
                 character outside ASCII."
            )),
        })
        .map(|message| SourceNotice::RequestGuidance {
            data_date: None,
            message: message.into(),
            retry_after_seconds: None,
        })
        .collect();
    if notices.is_empty() {
        return;
    }
    if let Some(report) = out
        .sources
        .iter_mut()
        .find(|report| report.source == source)
    {
        report.notices.extend(notices);
    }
}
