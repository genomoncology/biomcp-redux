//! The label version in which text first appears in one section, read from the label PDFs
//! Drugs@FDA archives for each supplement (1066). It reports what the labels say and when, never
//! why a label changed.
//!
//! Versions are the application's distinct label PDFs, oldest first. The search reads the
//! oldest and newest usable versions, then halves the versions between them. Halving assumes
//! text once added stays in every later version; the reply says so whenever the answer rests on
//! it. When the oldest version has the text and the newest does not, the assumption fails, so
//! the versions are read in order instead. Every read goes through the capped PDF reader, and
//! one call reads at most [`MOST_READS`] PDFs.
use super::label_text::{self, Found};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{drugsfda as source, pdf};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    DrugGet, DrugLabelHistory, LabelHistorySection, LabelVersion, LabelVersionMatch,
    LabelVersionState as State, PdfTextState, ReplyBody, Source,
};
use std::collections::BTreeMap;

/// Most label PDFs one call reads: halving reads about log2(n) + 2 of n versions, 9 for
/// Keytruda's 102, and reading in order stops here.
pub(crate) const MOST_READS: usize = 10;

/// One label version: a distinct label PDF and the supplement it came with.
struct Version {
    approved: Option<biodata::PartialDate>,
    submission: biodata::Text,
    label: biodata::Text,
    letter: Option<biodata::Text>,
}

/// What reading one version found.
struct Opened {
    state: State,
    private_use: bool,
    heading: Option<String>,
    sentences: Vec<String>,
}

impl Opened {
    /// Read well enough to say whether the section holds the text.
    fn usable(&self) -> bool {
        matches!(
            self.state,
            State::Match | State::NoMatch | State::SectionAbsent
        )
    }
    /// Unread, so it may hide the change: not usable and not a document that is not a label.
    fn gap(&self) -> bool {
        !self.usable() && self.state != State::NotPrescribingInformation
    }
}

/// The application's label versions, oldest first, one per distinct PDF.
fn versions(row: &source::Application) -> Result<Vec<Version>, RequestError> {
    let mut found: Vec<Version> = Vec::new();
    let (submissions, _) = row.submissions((None, None), 0, u32::MAX)?;
    for submission in submissions.iter() {
        let docs = submission.application_docs.as_deref().unwrap_or_default();
        let kind = submission.event.kind.as_ref().map_or("", |k| k.as_str());
        let name = format!("{kind} {}", submission.submission_number.as_str());
        for label in of(docs, "Label") {
            found.push(Version {
                approved: submission.event.dates.first().map(|d| d.date),
                submission: text(name.trim())?,
                label,
                letter: of(docs, "Letter").next(),
            });
        }
    }
    let number = |v: &Version| {
        let n = v
            .submission
            .as_str()
            .rsplit(' ')
            .next()
            .unwrap_or_default()
            .trim_start_matches('0');
        (n.len(), n.to_owned())
    };
    found.sort_by_cached_key(|v| (v.approved.map(|d| d.to_string()), number(v)));
    let mut seen = std::collections::BTreeSet::new();
    found.retain(|v| seen.insert(v.label.as_str().to_owned()));
    Ok(found)
}

/// The links of `docs` of one kind.
fn of<'a>(
    docs: &'a [biomcp_types::DrugFdaDocument],
    kind: &'a str,
) -> impl Iterator<Item = biodata::Text> + 'a {
    docs.iter()
        .filter(move |d| d.r#type.as_ref().is_some_and(|t| t.as_str() == kind))
        .filter_map(|d| d.url.clone())
}

struct Search<'a, 'b> {
    leg: &'a Leg<'b>,
    versions: &'a [Version],
    finds: &'a [biodata::Text],
    section: LabelHistorySection,
    opened: BTreeMap<usize, Opened>,
}

impl Search<'_, '_> {
    /// Open version `at` once; false when the read bound or the time limit is spent.
    fn open(&mut self, at: usize) -> bool {
        if self.opened.contains_key(&at) {
            return true;
        }
        let Some(version) = self.versions.get(at) else {
            return false;
        };
        // A read needs time to fetch and parse; with under a second left, stop and say so.
        if self.opened.len() >= MOST_READS
            || self.leg.remaining() < std::time::Duration::from_secs(1)
        {
            return false;
        }
        let opened = match pdf::label(self.leg, &source::DOCUMENTS, &version.label) {
            Err(_) => state(State::NotFetched),
            Ok((text, body)) => self.searched(text.state, &body),
        };
        self.opened.insert(at, opened);
        true
    }

    fn searched(&self, read: PdfTextState, body: &str) -> Opened {
        let partial = read == PdfTextState::Partial;
        match read {
            PdfTextState::Text | PdfTextState::Partial => {}
            PdfTextState::NoText => return state(State::NoText),
            PdfTextState::TooLarge => return state(State::TooLarge),
            PdfTextState::Garbled => return state(State::Garbled),
            PdfTextState::Damaged => return state(State::Damaged),
            PdfTextState::Refused => return state(State::Refused),
        }
        let flat = label_text::clean(body);
        let section = match label_text::find(&flat, self.section) {
            Found::Section(section) => section,
            Found::Absent if partial => return state(State::Partial),
            Found::Absent => return state(State::SectionAbsent),
            Found::Unbounded => return state(State::SectionNotBounded),
            Found::NotPrescribing => return state(State::NotPrescribingInformation),
            Found::Older => return state(State::OlderFormat),
        };
        let hits = label_text::hits(&section, self.finds);
        let private_use = section
            .chars()
            .any(|c| ('\u{e000}'..='\u{f8ff}').contains(&c));
        let Some(&at) = hits.first() else {
            if private_use {
                return state(State::PrivateUse);
            }
            return state(if partial {
                State::Partial
            } else {
                State::NoMatch
            });
        };
        Opened {
            state: State::Match,
            private_use,
            heading: label_text::heading(&flat, &section, at),
            sentences: label_text::sentences(&section, &hits),
        }
    }

    fn get(&self, at: usize) -> Option<&Opened> {
        self.opened.get(&at)
    }

    /// The first usable version from `from` toward `to`, opening as it goes.
    fn usable(&mut self, order: impl Iterator<Item = usize>) -> Option<usize> {
        for at in order {
            if !self.open(at) {
                return None;
            }
            if self.get(at).is_some_and(Opened::usable) {
                return Some(at);
            }
        }
        None
    }

    fn matches(&self, at: usize) -> bool {
        self.get(at).is_some_and(|o| o.state == State::Match)
    }

    /// Halve between a version without the text and a later one with it.
    fn halve(&mut self, mut low: usize, mut high: usize) -> (usize, usize) {
        while high - low > 1 {
            let middle = (low + high) / 2;
            let next = (1..high - low)
                .flat_map(|step| [middle + step / 2, middle.wrapping_sub(step / 2)])
                .filter(|at| low < *at && *at < high)
                .find(|at| self.get(*at).is_none());
            let Some(at) = next else { break };
            if !self.open(at) {
                break;
            }
            match self.get(at) {
                Some(o) if o.state == State::Match => high = at,
                Some(o) if o.usable() => low = at,
                _ => {}
            }
        }
        (low, high)
    }

    /// Read in order, within the bound; the first match.
    fn in_order(&mut self) -> Option<usize> {
        for at in 0..self.versions.len() {
            if !self.open(at) {
                break;
            }
        }
        self.opened
            .iter()
            .find(|(_, o)| o.state == State::Match)
            .map(|(at, _)| *at)
    }
}

fn state(state: State) -> Opened {
    Opened {
        state,
        private_use: false,
        heading: None,
        sentences: Vec::new(),
    }
}

/// How the search went, for the reply's method.
enum Path {
    Halved,
    Present,
    Absent,
    Removed,
    Every,
    Unreadable,
}

/// Answer one label-history get for the application in `row`.
pub(super) fn select(
    cx: &Ctx<'_>,
    query: &DrugGet,
    row: &source::Application,
    deadline: biomcp_fetch::Deadline<'_>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let (finds, every, section) = query.label_history();
    let section = section.unwrap_or(LabelHistorySection::BoxedWarning);
    let versions = versions(row)?;
    let leg = cx.fetch.leg(&source::DOCUMENTS, deadline).fresh(cx.fresh);
    let mut search = Search {
        leg: &leg,
        versions: &versions,
        finds,
        section,
        opened: BTreeMap::new(),
    };
    let last = versions.len().checked_sub(1);
    let (path, first) = if every {
        let first = search.in_order();
        (Path::Every, first)
    } else {
        match search.usable(0..versions.len()) {
            None => (Path::Unreadable, None),
            Some(low) if search.matches(low) => {
                let high = last.and_then(|last| search.usable((low + 1..=last).rev()));
                if high.is_some_and(|high| !search.matches(high)) {
                    (Path::Removed, search.in_order())
                } else {
                    (Path::Present, Some(low))
                }
            }
            Some(low) => match last.and_then(|last| search.usable((low + 1..=last).rev())) {
                Some(high) if search.matches(high) => {
                    (Path::Halved, Some(search.halve(low, high).1))
                }
                _ => (Path::Absent, None),
            },
        }
    };
    let body = reply(&search, row, section, path, first)?;
    let unread = search.opened.values().any(Opened::gap) || body.not_opened > 0;
    drop(search);
    merge(leg, out);
    if body.not_opened > 0 {
        status::missing(out, text("/result/not_opened")?, Source::Drugsfda, None);
    }
    status::item(out, true, false, unread);
    out.body = body.with_query(query.clone());
    Ok(())
}

fn version(search: &Search<'_, '_>, at: usize, link: bool) -> Option<LabelVersion> {
    let v = search.versions.get(at)?;
    let state = search.get(at)?.state;
    Some(LabelVersion {
        approved: v.approved,
        submission: v.submission.clone(),
        state,
        label: (link || !search.get(at)?.usable() && state != State::NotPrescribingInformation)
            .then(|| v.label.clone()),
    })
}

fn named(v: &LabelVersion) -> String {
    match v.approved {
        Some(date) => format!("{}, approved {date}", v.submission.as_str()),
        None => v.submission.as_str().to_owned(),
    }
}

fn reply(
    search: &Search<'_, '_>,
    row: &source::Application,
    section: LabelHistorySection,
    path: Path,
    first: Option<usize>,
) -> Result<DrugLabelHistory, RequestError> {
    let before = first.and_then(|first| {
        search
            .opened
            .range(..first)
            .rev()
            .find(|(_, o)| o.usable())
            .map(|(at, _)| *at)
    });
    let span = first.map_or(0..0, |first| before.map_or(0, |b| b + 1)..first);
    let on_or_before = span
        .clone()
        .any(|at| search.get(at).is_none_or(Opened::gap));
    let not_opened = span.filter(|at| search.get(*at).is_none()).count();
    let matched = match first {
        Some(at) => matched(search, at)?,
        None => None,
    };
    let before_version = before.and_then(|at| version(search, at, true));
    let answer = answer(
        section,
        matched.as_ref(),
        before_version.as_ref(),
        on_or_before,
        search,
    );
    Ok(DrugLabelHistory {
        identity: row.header(),
        section,
        answer: text(&answer)?,
        method: text(&method(&path, search))?,
        versions: search.versions.len() as u32,
        first: matched,
        before: before_version,
        on_or_before,
        not_opened: not_opened as u32,
        read: search
            .opened
            .keys()
            .filter_map(|at| version(search, *at, Some(*at) == first || Some(*at) == before))
            .collect(),
    })
}

/// The matching version with its heading and sentences.
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

/// Source text for the reply; text the reader gave is never empty where it is used.
fn text(value: &str) -> Result<biodata::Text, RequestError> {
    biodata::Text::from_source(value)
        .map_err(|_| biomcp_fetch::bad_reply(Source::Drugsfda, Some(200)))
}

fn answer(
    section: LabelHistorySection,
    first: Option<&LabelVersionMatch>,
    before: Option<&LabelVersion>,
    on_or_before: bool,
    search: &Search<'_, '_>,
) -> String {
    let Some(first) = first else {
        let read = search.opened.values().filter(|o| o.usable()).count();
        return if read == 0 {
            format!("No label version could be read for its {section} section.")
        } else {
            format!("None of the {read} label versions read has the text in its {section} section.")
        };
    };
    let found = format!(
        "First found in the {section} section of {}",
        named(&first.version)
    );
    match (before, on_or_before) {
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
    }
}

fn method(path: &Path, search: &Search<'_, '_>) -> String {
    let n = search.versions.len();
    let read = search.opened.len();
    let assumption = "This assumes text, once added to the section, stays in every later version";
    match path {
        Path::Halved => format!(
            "Read the oldest and newest of {n} label versions, then halved between them: {read} \
             PDFs read, at most {MOST_READS} per call. {assumption}; the oldest version read lacked \
             it and the newest had it, as the assumption expects."
        ),
        Path::Present => format!(
            "Read the oldest and newest of {n} label versions: {read} PDFs read. The oldest version \
             read already has the text, and so does the newest."
        ),
        Path::Absent => format!(
            "Read the oldest and newest of {n} label versions: {read} PDFs read; the newest lacks the \
             text. {assumption}, so the versions between were not read."
        ),
        Path::Removed => format!(
            "The oldest version read has the text and the newest does not, so it was removed at some \
             point and halving would not hold; read the versions in order instead: {read} of {n} read, \
             at most {MOST_READS} per call."
        ),
        Path::Every => format!(
            "Read the versions in order, as --every-version asks: {read} of {n} read, at most \
             {MOST_READS} per call."
        ),
        Path::Unreadable => format!("Tried {read} of {n} label versions; none could be read."),
    }
}

/// Fold the document reads' report into the Drugs@FDA source row.
fn merge(leg: Leg<'_>, out: &mut Outcome) {
    let report = status::source(Source::Drugsfda, leg.finish(), None);
    if let Some(row) = out
        .sources
        .iter_mut()
        .find(|s| s.source == Source::Drugsfda)
    {
        row.notices.extend(report.notices);
    }
}

/// An empty label-history reply for a get that found no application.
pub(super) fn empty(query: &DrugGet) -> Outcome {
    super::empty(DrugLabelHistory::into_reply(query.clone(), None))
}
