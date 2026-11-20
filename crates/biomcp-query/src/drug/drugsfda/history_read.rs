//! Read and classify archived Drugs@FDA labels within the per-call PDF budget.
use super::history::text;
use super::label_sections::{self, Found};
use super::label_text;
use biomcp_adapters::{drugsfda as source, pdf};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{LabelHistorySection, LabelVersionState as State, PdfTextState};
use std::collections::{BTreeMap, BTreeSet};

/// Most label PDFs one call reads: halving reads about log2(n) + 2 of n versions, 9 for
/// Keytruda's 102, and reading in order stops here.
pub(crate) const MOST_READS: usize = 10;

/// One label version: a distinct label PDF and the supplement it came with.
pub(super) struct Version {
    pub(super) approved: Option<biodata::PartialDate>,
    pub(super) submission: biodata::Text,
    pub(super) label: biodata::Text,
    pub(super) letter: Option<biodata::Text>,
}

/// What reading one version found.
pub(super) struct Opened {
    pub(super) state: State,
    pub(super) private_use: bool,
    pub(super) heading: Option<String>,
    pub(super) sentences: Vec<String>,
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
        !self.state.read()
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
pub(super) fn versions(row: &source::Application) -> Result<Vec<Version>, RequestError> {
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
    pub(super) leg: &'a Leg<'b>,
    pub(super) versions: &'a [Version],
    pub(super) finds: &'a [biodata::Text],
    pub(super) section: LabelHistorySection,
    pub(super) opened: BTreeMap<usize, Opened>,
    /// The read bound or the time limit stopped the search before it finished.
    pub(super) stopped: bool,
}

impl Search<'_, '_> {
    /// Open version `at` once; false when the read bound or the time limit is spent.
    pub(super) fn open(&mut self, at: usize) -> bool {
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
        // A scan holds FDA's stamp alone, which the cleaning took out.
        if flat
            .chars()
            .filter(|c| c.is_alphabetic())
            .nth(200)
            .is_none()
        {
            return state(State::NoText);
        }
        let (section, whole) = match label_sections::find(&flat, self.section) {
            Found::Section(section) => (section, true),
            Found::Title(title) => (title, false),
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
            if !whole {
                return state(State::SectionNotBounded);
            }
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
}

fn state(state: State) -> Opened {
    Opened {
        state,
        private_use: false,
        heading: None,
        sentences: Vec::new(),
    }
}
