//! Choose which archived labels to read within the budget.
use super::history_read::{MOST_READS, Opened, Search};
use biomcp_types::LabelVersionState as State;
use std::collections::BTreeSet;

impl Search<'_, '_> {
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
    fn halve(&mut self, mut low: usize, mut high: usize) -> usize {
        // Versions already read between them narrow the range before any new read.
        if let Some((at, _)) = self
            .opened
            .range(low + 1..high)
            .find(|(_, o)| o.state == State::Match)
        {
            high = *at;
        }
        if let Some((at, _)) = self
            .opened
            .range(low + 1..high)
            .rev()
            .find(|(_, o)| o.usable())
        {
            low = *at;
        }
        let mut toward_match = false;
        while high - low > 1 {
            let middle = (low + high) / 2;
            // An unread midpoint gives no direction. Try halfway toward the known match
            // before using more reads near that midpoint.
            let nearer_match = middle + (high - middle) / 2;
            let next = toward_match
                .then_some(nearer_match)
                .into_iter()
                .chain(
                    (1..high - low)
                        .flat_map(|step| [middle + step / 2, middle.wrapping_sub(step / 2)]),
                )
                .filter(|at| low < *at && *at < high)
                .find(|at| self.get(*at).is_none());
            let Some(at) = next else { break };
            if !self.open(at) {
                break;
            }
            toward_match = false;
            match self.get(at) {
                Some(o) if o.state == State::Match => high = at,
                Some(o) if o.usable() => low = at,
                _ => toward_match = at == middle,
            }
        }
        low
    }

    /// Read in order from `from`, within the bound.
    pub(super) fn in_order(&mut self, from: usize) {
        for at in from..self.versions.len() {
            if !self.open(at) {
                break;
            }
        }
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
        // Try the first two versions in order before probing for a readable label.
        if let Some(early) = self.usable(0..count.min(2)) {
            return Some(early);
        }
        let (mut low, mut high) = (1, self.usable((2..count).rev())?);
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
pub(super) fn search(search: &mut Search<'_, '_>) -> Path {
    let count = search.versions.len();
    let Some(low) = search.earliest() else {
        return Path::Unreadable;
    };
    let high = search.usable((low + 1..count).rev());
    if search.matches(low) {
        if high.is_some_and(|high| !search.matches(high)) {
            search.in_order(0);
            return Path::Removed;
        }
        return Path::Present;
    }
    match high {
        Some(high) if search.matches(high) => {
            // Spend remaining reads checking for earlier appearances.
            let before = search.halve(low, high);
            let earlier = search.spread(low, before);
            if let Some(earlier) = earlier {
                search.halve(low, earlier);
            }
            Path::Halved(earlier.is_some())
        }
        _ => {
            if let Some(held) = search.spread(low, high.unwrap_or(low)) {
                search.halve(low, held);
                Path::Returned
            } else {
                Path::Absent
            }
        }
    }
}
