//! The label version in which text first appears in one section, read from the label PDFs
//! Drugs@FDA archives for each supplement (1066). It reports what the labels say and when, never
//! why a label changed, and it says it could not tell rather than state a date or an absence it
//! has not established.
//!
//! Versions are the application's distinct label PDFs, oldest first. The search reads the
//! oldest and newest usable versions. When the oldest lacks the text and the newest has it, it
//! halves the versions between them, which assumes text once added stays in every later version;
//! the reply says so. When both lack it, it reads versions spread evenly between them with the
//! reads left, and halves toward the first one that holds it. When the oldest has it and the
//! newest does not, it reads in order. Every read goes through the capped PDF reader, one call
//! reads at most [`MOST_READS`] PDFs, and a call that stops there says so and offers the step
//! that continues.
use super::history_reply::{answer, method, named};
use super::label_sections::{self, Found};
use super::label_text;
use crate::{Ctx, Outcome, status};
use biomcp_adapters::{drugsfda as source, pdf};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{
    DrugGet, DrugLabelHistory, LabelHistorySection, LabelVersion, LabelVersionMatch,
    LabelVersionState as State, PdfTextState, ReplyBody, Request, Source, StepKind,
};
use std::collections::{BTreeMap, BTreeSet};

/// Most label PDFs one call reads: halving reads about log2(n) + 2 of n versions, 9 for
/// Keytruda's 102, and reading in order stops here.
pub(crate) const MOST_READS: usize = 10;

/// One label version: a distinct label PDF and the supplement it came with.
pub(super) struct Version {
    pub(super) approved: Option<biodata::PartialDate>,
    pub(super) submission: biodata::Text,
    label: biodata::Text,
    letter: Option<biodata::Text>,
}

/// What reading one version found.
pub(super) struct Opened {
    pub(super) state: State,
    private_use: bool,
    heading: Option<String>,
    sentences: Vec<String>,
}

impl Opened {
    /// Read well enough to say whether the section holds the text.
    pub(super) fn usable(&self) -> bool {
        matches!(
            self.state,
            State::Match | State::NoMatch | State::SectionAbsent
        )
    }
    /// Unread, so it may hide the change: not usable and not a document that is not a label.
    pub(super) fn gap(&self) -> bool {
        !self.usable() && self.state != State::NotPrescribingInformation
    }
}

/// One document under the links Drugs@FDA gives it: without its scheme or a `#page=` part.
fn document(url: &str) -> &str {
    let url = url.split('#').next().unwrap_or(url);
    url.strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url)
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
    let mut seen = BTreeSet::new();
    found.retain(|v| seen.insert(document(v.label.as_str()).to_owned()));
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

pub(super) struct Search<'a, 'b> {
    leg: &'a Leg<'b>,
    pub(super) versions: &'a [Version],
    finds: &'a [biodata::Text],
    section: LabelHistorySection,
    pub(super) opened: BTreeMap<usize, Opened>,
    /// The read bound or the time limit stopped the search before it finished.
    pub(super) stopped: bool,
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
            self.stopped = true;
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
        let cleaned = label_text::clean(body);
        let flat = label_text::flatten(&cleaned);
        let section = match label_sections::find(&cleaned, &flat, self.section) {
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

    pub(super) fn get(&self, at: usize) -> Option<&Opened> {
        self.opened.get(&at)
    }

    /// The first usable version in `order`, opening as it goes.
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

    pub(super) fn matches(&self, at: usize) -> bool {
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

    /// Read in order from `from`, within the bound; the first match.
    fn in_order(&mut self, from: usize) -> Option<usize> {
        for at in from..self.versions.len() {
            if !self.open(at) {
                break;
            }
        }
        self.opened
            .range(from..)
            .find(|(_, o)| o.state == State::Match)
            .map(|(at, _)| *at)
    }

    /// Read versions spread evenly between `low` and `high` with the reads left; the earliest
    /// that holds the text.
    fn spread(&mut self, low: usize, high: usize) -> Option<usize> {
        if high <= low + 1 {
            return None;
        }
        let left = MOST_READS.saturating_sub(self.opened.len());
        let picks: BTreeSet<usize> = (1..=left)
            .map(|step| low + step * (high - low) / (left + 1))
            .filter(|at| low < *at && *at < high && self.get(*at).is_none())
            .collect();
        for at in picks {
            if !self.open(at) {
                break;
            }
        }
        self.opened
            .range(low + 1..high)
            .find(|(_, o)| o.state == State::Match)
            .map(|(at, _)| *at)
    }

    /// The earliest usable version: the oldest, or, when it cannot be read, the earliest found by
    /// halving toward the newest usable one, so older unreadable labels do not spend the reads.
    fn earliest(&mut self) -> Option<usize> {
        let count = self.versions.len();
        if !self.open(0) {
            return None;
        }
        if self.get(0).is_some_and(Opened::usable) {
            return Some(0);
        }
        let (mut low, mut high) = (0, self.usable((1..count).rev())?);
        while high - low > 1 {
            let middle = (low + high) / 2;
            if !self.open(middle) {
                break;
            }
            if self.get(middle).is_some_and(Opened::usable) {
                high = middle;
            } else {
                low = middle;
            }
        }
        Some(high)
    }

    /// The latest usable version, the newest the search could read.
    pub(super) fn newest(&self) -> Option<usize> {
        self.opened
            .iter()
            .rev()
            .find(|(_, o)| o.usable())
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Path {
    /// Halved; true when a version spread before the first match, read to test the assumption,
    /// also held the text, so the search halved toward it.
    Halved(bool),
    Present,
    /// Both ends lack the text and no version spread between them holds it.
    Absent,
    /// Both ends lack the text and a version spread between them holds it.
    Returned,
    Removed,
    Every(usize),
    Unreadable,
}

/// Read the oldest and newest versions, then halve, spread or read in order as they decide.
fn search(search: &mut Search<'_, '_>) -> (Path, Option<usize>) {
    let count = search.versions.len();
    let Some(low) = search.earliest() else {
        return (Path::Unreadable, None);
    };
    let high = search.usable((low + 1..count).rev());
    if search.matches(low) {
        if high.is_some_and(|high| !search.matches(high)) {
            return (Path::Removed, search.in_order(0));
        }
        return (Path::Present, Some(low));
    }
    match high {
        Some(high) if search.matches(high) => {
            // The reads left test the assumption on versions spread before the first match.
            let (before, first) = search.halve(low, high);
            match search.spread(low, before) {
                Some(earlier) => (Path::Halved(true), Some(search.halve(low, earlier).1)),
                None => (Path::Halved(false), Some(first)),
            }
        }
        _ => match search.spread(low, high.unwrap_or(low)) {
            Some(held) => (Path::Returned, Some(search.halve(low, held).1)),
            None => (Path::Absent, None),
        },
    }
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
    let mut found = Search {
        leg: &leg,
        versions: &versions,
        finds,
        section,
        opened: BTreeMap::new(),
        stopped: false,
    };
    let (path, first) = if every {
        let from = query.section_offset().unwrap_or(0) as usize;
        (Path::Every(from), found.in_order(from))
    } else {
        search(&mut found)
    };
    let body = reply(&found, row, section, path, first)?;
    // Versions left unread leave the answer open: the call stopped, or no version read has the
    // text while others were not read, or a version before the first match was not read.
    let open = found.stopped || (first.is_none() && body.not_opened > 0) || body.on_or_before;
    let unread = open || found.opened.values().any(Opened::gap);
    let resume = resume(&found, path, first);
    drop(found);
    merge(leg, out);
    if open && body.not_opened > 0 {
        status::missing(out, text("/result/not_opened")?, Source::Drugsfda, None);
    }
    if let Some(from) = resume.filter(|from| open && *from < versions.len()) {
        let request = query.every_version_from(from as u32);
        if let Some(request) = crate::next::checked(request, Source::Drugsfda, &mut out.problems) {
            out.next.push(crate::next::Step::run(
                StepKind::Page,
                &[Source::Drugsfda],
                Request::GetDrug(request),
                cx.fresh,
            ));
        }
    }
    status::item(out, true, false, unread);
    out.body = body.with_query(query.clone());
    Ok(())
}

/// The version an `--every-version` call continues from: after the last version this call read
/// in order, or after the last version read before an unread stretch.
fn resume(search: &Search<'_, '_>, path: Path, first: Option<usize>) -> Option<usize> {
    let last = search.opened.keys().next_back().map(|at| at + 1);
    match path {
        Path::Every(_) => last,
        _ => Some(first.map_or(0, |first| {
            search
                .opened
                .range(..first)
                .rev()
                .find(|(_, o)| o.usable())
                .map_or(0, |(at, _)| at + 1)
        })),
    }
}

pub(super) fn version(search: &Search<'_, '_>, at: usize, link: bool) -> Option<LabelVersion> {
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
    let not_opened = search.versions.len() - search.opened.len();
    let matched = match first {
        Some(at) => matched(search, at)?,
        None => None,
    };
    let before_version = before.and_then(|at| version(search, at, true));
    let answer = answer(
        section,
        (matched.as_ref(), before_version.as_ref()),
        on_or_before,
        search,
        first,
    );
    Ok(DrugLabelHistory {
        identity: row.header(),
        section,
        answer: text(&answer)?,
        method: text(&method(path, search))?,
        versions: search.versions.len() as u32,
        first: matched,
        before: before_version,
        on_or_before,
        not_opened: not_opened as u32,
        stopped: search.stopped,
        read: search
            .opened
            .keys()
            // `first` and `before` carry their own links, so the list gives only unread ones.
            .filter_map(|at| version(search, *at, false))
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

/// A version's submission and date, for the reply's sentences.
pub(super) fn label(search: &Search<'_, '_>, at: usize) -> Option<String> {
    version(search, at, false).map(|v| named(&v))
}
