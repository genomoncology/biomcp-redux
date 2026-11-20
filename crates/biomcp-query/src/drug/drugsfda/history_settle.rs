//! Settle chronology from the complete read state and verify it against the reply.
use super::history_read::Search;
use biomcp_types::{DrugLabelHistory, LabelVersionState as State};

pub(super) struct Settled {
    pub(super) first: Option<usize>,
    pub(super) before: Option<usize>,
    pub(super) on_or_before: bool,
}

pub(super) fn settle(search: &Search<'_, '_>) -> Settled {
    let mut before = None;
    let mut gap = false;
    for (at, version) in search.versions.iter().enumerate() {
        match search.get(at).map(|opened| opened.state) {
            Some(State::Match) => {
                return Settled {
                    first: Some(at),
                    before,
                    on_or_before: gap || version.approved.is_none(),
                };
            }
            Some(State::NoMatch) if version.approved.is_some() => before = Some(at),
            Some(state) if state.read() => {}
            _ => gap = true,
        }
    }
    Settled {
        first: None,
        before: None,
        on_or_before: false,
    }
}

/// Check the fields that will supply the answer, using the finished list of versions read.
/// The list omits unopened versions, so compare its positions with the full timeline.
pub(super) fn holds(search: &Search<'_, '_>, reply: &DrugLabelHistory) -> bool {
    if reply.read.len() != search.opened.len()
        || !reply
            .read
            .iter()
            .zip(search.opened.values())
            .all(|(listed, opened)| listed.state == opened.state)
    {
        return false;
    }
    let listed = || search.opened.keys().copied().zip(reply.read.iter());
    let earliest = listed()
        .enumerate()
        .find(|(_, (_, v))| v.state == State::Match);
    let Some((first, (pos, (index, earliest)))) = reply.first.as_ref().zip(earliest) else {
        return reply.first.is_none()
            && earliest.is_none()
            && reply.before.is_none()
            && !reply.on_or_before;
    };
    let same =
        |stated: &biomcp_types::LabelVersion, listed: &biomcp_types::LabelVersion, at: usize| {
            stated.state == listed.state
                && stated.submission == listed.submission
                && stated.approved == listed.approved
                && search
                    .versions
                    .get(at)
                    .is_some_and(|v| stated.label == v.label)
        };
    first.version.state == State::Match
        && same(&first.version, earliest, index)
        && reply.before.as_ref().is_none_or(|before| {
            before.state == State::NoMatch && listed().take(pos).any(|(at, v)| same(before, v, at))
        })
        && (reply.on_or_before || (index == pos && listed().take(pos).all(|(_, v)| v.state.read())))
}
