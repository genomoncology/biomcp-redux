//! Source observations and recovery commands for an unfinished drug name lookup.
use crate::{Outcome, status};
use biomcp_types::{
    DrugSource, Reason, Request, Source, SourceOutcome, SourceState, SourceStatus, StepKind,
};

#[derive(Default)]
pub(super) struct Evidence {
    pub(super) sources: Vec<SourceOutcome>,
    pub(super) problems: Vec<biomcp_types::Problem>,
    pub(super) local: Option<biomcp_fetch::RequestError>,
}
impl Evidence {
    /// Not finding the record is an answer only when every name lookup read a current copy
    /// (1059, 1087).
    pub(super) fn lookup_failed(&self) -> bool {
        self.local.is_some() || self.sources.iter().any(unread)
    }
    pub(super) fn deadline(&mut self, source: Source) {
        self.sources.push(biomcp_adapters::registry::outcome(
            source,
            SourceStatus::Failed(Some(Reason::DeadlineExceeded)),
        ));
    }

    pub(super) fn observe(&mut self, out: &Outcome, source: DrugSource) {
        self.sources
            .extend(out.sources.iter().cloned().map(|mut read| {
                if out.dropped && read.source == source.source() && read.state == SourceState::Ok {
                    read.set_status(SourceStatus::Failed(None));
                }
                read
            }));
        status::append(&mut self.problems, out.problems.iter().cloned());
        self.local = self.local.or(out.local);
    }

    pub(super) fn append(&self, out: &mut Outcome, selected: DrugSource) {
        for source in &self.sources {
            // Successful component metadata cannot replace the selected native outcome.
            if source.source == selected.source() && !unread(source) {
                continue;
            }
            if let Some(existing) = out.sources.iter_mut().find(|s| s.source == source.source) {
                status::retain_source_state(existing, source);
            } else {
                out.sources.push(source.clone());
            }
        }
        for problem in &self.problems {
            if !out.problems.contains(problem) {
                out.problems.push(problem.clone())
            }
        }
        out.local = out.local.or(self.local);
    }
}

/// A source read a lookup could not use: a stale copy, or a failed read other than "not found".
fn unread(source: &SourceOutcome) -> bool {
    matches!(source.state, SourceState::Stale | SourceState::Syncing)
        || source.state == SourceState::Failed && source.reason != Some(Reason::NotFound)
}

/// Offer the step that reads each source the lookup could not use again: the sync of a stored
/// copy, otherwise the same call without cached replies (1087).
pub(in crate::drug) fn recovery(out: &mut Outcome, request: Request) {
    let mut network = Vec::new();
    for source in out.sources.iter().filter(|s| unread(s)).map(|s| s.source) {
        let step = match crate::sources::bundles().find(|b| b.source() == source) {
            Some(bundle) => {
                crate::next::Step::new(StepKind::Sync, &[source], crate::Action::Sync { bundle })
            }
            None if !network.contains(&source) => {
                network.push(source);
                continue;
            }
            None => continue,
        };
        if !out.next.contains(&step) {
            out.next.push(step);
        }
    }
    if !network.is_empty() {
        let step = crate::next::Step::run(StepKind::Fresh, &network, request, true);
        if !out.next.contains(&step) {
            out.next.push(step);
        }
    }
}
