//! Build the structured label-history reply and word only the claims it supports.
use super::history::text;
use super::history_read::{MOST_READS, Search};
use super::history_search::Path;
use super::history_settle::{holds, settle};
use biomcp_adapters::drugsfda as source;
use biomcp_fetch::RequestError;
use biomcp_types::{
    DrugLabelHistory, LabelHistorySection, LabelVersion, LabelVersionMatch,
    LabelVersionState as State,
};

pub(super) fn reply(
    search: &Search<'_, '_>,
    row: &source::Application,
    section: LabelHistorySection,
    path: Path,
) -> Result<(DrugLabelHistory, bool), RequestError> {
    let settled = settle(search);
    let mut body = DrugLabelHistory {
        identity: row.header(),
        section,
        answer: text("BioMCP could not tell: the answer disagrees with the versions read.")?,
        method: text(&method(path, search))?,
        versions: search.versions.len() as u32,
        first: settled
            .first
            .map(|at| matched(search, at))
            .transpose()?
            .flatten(),
        before: settled.before.and_then(|at| version(search, at, true)),
        on_or_before: settled.on_or_before,
        not_opened: (search.versions.len() - search.opened.len()) as u32,
        stopped: search.stopped,
        read: search
            .opened
            .keys()
            .filter_map(|&at| version(search, at, false))
            .collect(),
    };
    let consistent = holds(search, &body);
    if consistent {
        body.answer = text(&answer(&body))?;
    } else {
        body.first = None;
        body.before = None;
        body.on_or_before = false;
    }
    Ok((body, consistent))
}

fn version(search: &Search<'_, '_>, at: usize, link: bool) -> Option<LabelVersion> {
    let v = search.versions.get(at)?;
    let state = search.get(at)?.state;
    Some(LabelVersion {
        approved: v.approved,
        submission: v.submission.clone(),
        state,
        label: (link || !state.read()).then(|| v.label.clone()),
    })
}

fn matched(search: &Search<'_, '_>, at: usize) -> Result<Option<LabelVersionMatch>, RequestError> {
    let (Some(opened), Some(found), Some(version)) = (
        search.get(at),
        search.versions.get(at),
        version(search, at, true),
    ) else {
        return Ok(None);
    };
    Ok(Some(LabelVersionMatch {
        version,
        letter: found.letter.clone(),
        heading: opened.heading.as_deref().map(text).transpose()?,
        sentences: opened
            .sentences
            .iter()
            .map(|s| text(s))
            .collect::<Result<_, _>>()?,
        private_use: opened.private_use,
    }))
}

/// A version's submission and its date.
fn named(v: &LabelVersion) -> String {
    match v.approved {
        Some(date) => format!("{}, approved {date}", v.submission.as_str()),
        None => v.submission.as_str().to_owned(),
    }
}

fn answer(body: &DrugLabelHistory) -> String {
    let section = body.section;
    let Some(first) = &body.first else {
        let count = body.versions as usize;
        let read = body.read.iter().filter(|v| v.state.read()).count();
        let unread = count - read;
        let usable = body
            .read
            .iter()
            .any(|v| matches!(v.state, State::NoMatch | State::SectionAbsent));
        return if !usable {
            format!(
                "BioMCP could not tell: no label version could be read for its {section} section; \
                 {unread} of {count} versions could not be read or were not opened."
            )
        } else if unread == 0 {
            format!("None of the {count} label versions has the text in its {section} section.")
        } else {
            format!(
                "BioMCP could not tell: none of the {read} label versions read has the text in its \
                 {section} section, but {unread} of {count} could not be read or were not opened, so it \
                 may be in one of them."
            )
        };
    };
    let name = named(&first.version);
    let mut said = if body.on_or_before {
        format!("Text in {section} on or before {name}. Older labels remain unchecked.")
    } else if let Some(before) = &body.before {
        format!(
            "First found in the {section} section of {name}. {} lacks it.",
            named(before)
        )
    } else {
        format!("First found in the {section} section of {name}.")
    };
    // Read directly from the checked list. A missing section is not a no-match.
    if let Some(newest) = body.read.iter().rev().find(|v| {
        matches!(
            v.state,
            State::Match | State::NoMatch | State::SectionAbsent
        )
    }) && newest.state == State::NoMatch
        && let Some(last_match) = body.read.iter().rev().find(|v| v.state == State::Match)
    {
        said.push_str(&format!(
            " The newest readable label, {}, lacks it. The last match read is {}.",
            named(newest),
            named(last_match)
        ));
    }
    said
}

fn method(path: Path, search: &Search<'_, '_>) -> String {
    let count = search.versions.len();
    let read = search.opened.len();
    let strategy = match path {
        Path::Halved(false) => "Halved and sampled earlier labels.".to_owned(),
        Path::Halved(true) => {
            "Halved, sampled earlier labels and halved toward an earlier match.".to_owned()
        }
        Path::Present => "Read from both ends.".to_owned(),
        Path::Absent => "Read from both ends and sampled between them.".to_owned(),
        Path::Returned => {
            "Read from both ends, sampled between them and halved toward a match.".to_owned()
        }
        Path::Removed => "Read from both ends, then in order.".to_owned(),
        Path::Every(from) => format!(
            "Read in order from version {}. This page uses only its own reads.",
            from + 1
        ),
        Path::Unreadable => "Looked for readable labels.".to_owned(),
    };
    let assumption = if matches!(path, Path::Halved(_) | Path::Returned) {
        " Halving assumes text stays once added."
    } else {
        ""
    };
    format!("{strategy} {read}/{count} opened (limit {MOST_READS}).{assumption}")
}
