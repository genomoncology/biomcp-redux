//! The label-history reply's answer and method sentences (1066). An answer states a date or an
//! absence only when the versions read establish it; otherwise it says BioMCP could not tell
//! and how many versions were not read.
use super::history::{MOST_READS, Path, Search, label};
use biomcp_types::{LabelHistorySection, LabelVersion, LabelVersionMatch, LabelVersionState};

/// A version's submission and its date.
pub(super) fn named(v: &LabelVersion) -> String {
    match v.approved {
        Some(date) => format!("{}, approved {date}", v.submission.as_str()),
        None => v.submission.as_str().to_owned(),
    }
}

/// The answer in one or two sentences.
pub(super) fn answer(
    section: LabelHistorySection,
    (first, before): (Option<&LabelVersionMatch>, Option<&LabelVersion>),
    on_or_before: bool,
    search: &Search<'_, '_>,
    first_at: Option<usize>,
) -> String {
    let count = search.versions.len();
    let read = search.opened.values().filter(|o| !o.gap()).count();
    let unread = count - read;
    let Some(first) = first else {
        let usable = search.opened.values().filter(|o| o.usable()).count();
        return if usable == 0 {
            format!(
                "BioMCP could not tell: no label version could be read for its {section} section; \
                 {unread} of {count} versions were not read."
            )
        } else if unread == 0 {
            format!("None of the {count} label versions has the text in its {section} section.")
        } else {
            format!(
                "BioMCP could not tell: none of the {read} label versions read has the text in its \
                 {section} section, but {unread} of {count} were not read, so it may be in one of them."
            )
        };
    };
    let found = format!(
        "First found in the {section} section of {}",
        named(&first.version)
    );
    let mut said = match (before, on_or_before) {
        (_, true) => format!(
            "{found}; a version before it was not read, so the text appeared on or before {}.",
            first
                .version
                .approved
                .map_or("then".into(), |d| d.to_string())
        ),
        (Some(before), false) => format!(
            "{found}; {}, the version before it, lacks it.",
            named(before)
        ),
        (None, false) => format!("{found}, the first label version Drugs@FDA lists."),
    };
    if let (Some(newest), Some(at)) = (search.newest(), first_at)
        && newest > at
        && !search.matches(newest)
    {
        let held = search
            .opened
            .iter()
            .rev()
            .find(|(_, o)| o.state == LabelVersionState::Match)
            .and_then(|(at, _)| label(search, *at));
        said.push_str(&format!(
            " The newest version read, {}, lacks it, so the text was added and later removed; the \
             last version read that holds it is {}.",
            label(search, newest).unwrap_or_default(),
            held.unwrap_or_default()
        ));
    }
    said
}

/// How versions were chosen, and the assumption the answer rests on when it rests on one.
pub(super) fn method(path: Path, search: &Search<'_, '_>) -> String {
    let count = search.versions.len();
    let read = search.opened.len();
    let assumption = "This assumes text, once added to the section, stays in every later version";
    let spread =
        "both lack the text, so the reads left went to versions spread evenly between them";
    let mut said = match path {
        Path::Halved => format!(
            "Read the oldest and newest of {count} label versions, then halved between them: {read} \
             PDFs read, at most {MOST_READS} per call. {assumption}; the oldest version read lacked \
             it and the newest had it, as the assumption expects."
        ),
        Path::Present => format!(
            "Read the oldest and newest of {count} label versions: {read} PDFs read. The oldest \
             version read already has the text, and so does the newest."
        ),
        Path::Absent => format!(
            "Read the oldest and newest of {count} label versions; {spread}, and none holds it: \
             {read} PDFs read, at most {MOST_READS} per call. --every-version reads every version \
             in order."
        ),
        Path::Returned => format!(
            "Read the oldest and newest of {count} label versions; {spread}, and one holds it, so \
             the search halved between the oldest and that version: {read} PDFs read, at most \
             {MOST_READS} per call. {assumption} up to that version."
        ),
        Path::Removed => format!(
            "The oldest version read has the text and the newest does not, so it was removed at \
             some point and halving would not hold; read the versions in order instead: {read} of \
             {count} read, at most {MOST_READS} per call."
        ),
        Path::Every(from) => format!(
            "Read the versions in order from version {} of {count}, as --every-version asks: \
             {read} read, at most {MOST_READS} per call.",
            from + 1
        ),
        Path::Unreadable => format!("Tried {read} of {count} label versions; none could be read."),
    };
    if search.stopped {
        said.push_str(&format!(
            " The call stopped at that bound with {} of {count} versions not opened; the next step \
             continues with --every-version.",
            count - read
        ));
    }
    said
}
