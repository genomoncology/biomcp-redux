//! A drug search that widens its typed name: the names are looked up once, kept with the chosen
//! discovery step for later pages, and named in one notice on the searched source.
use crate::{Ctx, Outcome, drug::search_names, source_snapshot, status};
use biodata::Text;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{DrugSearch, Paged, ReplyData, Request, SourceNotice, StepKind};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize)]
#[serde(bound(serialize = "S: Serialize", deserialize = "S: DeserializeOwned"))]
pub(super) struct Saved<S> {
    pub(super) names: Vec<Text>,
    sources: Vec<biomcp_types::SourceOutcome>,
    problems: Vec<biomcp_types::Problem>,
    /// What the name lookups said, for the one notice on the searched source.
    found: Vec<String>,
    omitted: usize,
    /// The discovery step chosen on the first page, for sources with more than one.
    pub(super) step: Option<S>,
    /// The provider refused the widened request, so the search kept the typed name alone.
    rejected: bool,
}

impl<S> Saved<S> {
    /// Keep the typed name alone for this and later pages, after the provider refused the group.
    pub(super) fn reject(&mut self, query: &DrugSearch) {
        self.names = query.query().cloned().into_iter().collect();
        self.omitted = 0;
        self.rejected = true;
        self.step = None;
    }
}

fn namespace(query: &DrugSearch) -> String {
    format!("{}-drug-names", query.source())
}

/// Read the first page's names from a continuation, or look them up for a first page.
/// `fits` says whether one native request can carry a name group.
pub(super) fn prepare<'a, S: Serialize + DeserializeOwned>(
    cx: &Ctx<'a>,
    query: &DrugSearch,
    deadline: &mut Deadline<'a>,
    fits: impl Fn(&[Text]) -> bool,
) -> Result<Option<Saved<S>>, RequestError> {
    if let Some(token) = query.name_snapshot() {
        let key = source_snapshot::key(&namespace(query), &query.restart())?;
        let saved: Option<Saved<S>> = source_snapshot::read(cx, &key, token.as_str(), *deadline)?;
        return Ok(saved.filter(|saved| fits(&saved.names)));
    }
    Ok(Some(resolve(cx, query, deadline, fits)))
}

fn resolve<'a, S>(
    cx: &Ctx<'a>,
    query: &DrugSearch,
    deadline: &mut Deadline<'a>,
    fits: impl Fn(&[Text]) -> bool,
) -> Saved<S> {
    let mut out = status::empty(ReplyData::PoolError { mismatch: false });
    let mut names: Vec<Text> = query.query().cloned().into_iter().collect();
    let mut omitted = 0;
    search_names::bounded(deadline, |lookup| {
        if let Some(input) = query.query() {
            let found = search_names::resolve(cx, input, &mut out, lookup, search_names::openfda);
            for name in found.names {
                if names
                    .iter()
                    .any(|prior| prior.as_str().eq_ignore_ascii_case(name.as_str()))
                {
                    continue;
                }
                let mut candidate = names.clone();
                candidate.push(name);
                if fits(&candidate) {
                    names = candidate;
                } else {
                    omitted += 1;
                }
            }
        }
    });
    let found = search_names::take_messages(&mut out.sources);
    Saved {
        names,
        sources: out.sources,
        problems: out.problems,
        found,
        omitted,
        step: None,
        rejected: false,
    }
}

/// Give later pages the first page's names and step through a snapshot token.
pub(super) fn retain<S: Serialize>(
    cx: &Ctx<'_>,
    query: &DrugSearch,
    saved: Option<&Saved<S>>,
    deadline: Deadline<'_>,
    success: bool,
    out: &mut Outcome,
) -> DrugSearch {
    let Some(saved) = saved.filter(|_| success && query.name_snapshot().is_none()) else {
        return query.clone();
    };
    let result = source_snapshot::key(&namespace(query), &query.restart()).and_then(|key| {
        let mut admitted: Vec<_> = saved
            .sources
            .iter()
            .filter(|source| source.state != biomcp_types::SourceState::Skipped)
            .cloned()
            .collect();
        admitted.extend(out.sources.iter().cloned());
        source_snapshot::write(cx, &key, saved, &admitted, deadline)
    });
    match crate::publish::keep(result, out) {
        Ok(Some(token)) => query.with_name_snapshot(token),
        Ok(None) => query.clone(),
        Err(error) => {
            out.local = Some(error);
            query.clone()
        }
    }
}

/// A continuation whose names are gone offers a restart from the first page.
pub(super) fn missing(query: &DrugSearch, out: &mut Outcome) {
    out.body = ReplyData::PoolError { mismatch: false };
    out.next.push(crate::next::Step::run(
        StepKind::Restart,
        &[],
        Request::SearchDrug(query.restart()),
        false,
    ));
}

/// One notice on the searched source names every name searched and why any other was not.
/// `searched` says how the source combined the names, for a search that widened.
pub(super) fn finish<S>(query: &DrugSearch, saved: Saved<S>, searched: &str, out: &mut Outcome) {
    let source = query.source().source();
    if let Some(input) = query.query()
        && let Some(outcome) = out.sources.iter_mut().find(|s| s.source == source)
    {
        outcome.notices.push(SourceNotice::DrugNameSearch {
            input: input.clone(),
            names: saved.names.clone().into(),
            message: message(&saved, query, searched).into(),
        });
    }
    // The searched source's own outcome stands; a lookup that read it stays out of the reply.
    out.sources.extend(
        saved
            .sources
            .into_iter()
            .filter(|lookup| lookup.source != source),
    );
    out.problems.extend(saved.problems);
}

fn message<S>(saved: &Saved<S>, query: &DrugSearch, searched: &str) -> String {
    let mut message = if saved.rejected {
        format!(
            "{} rejected the query holding the drug's other names; searched the typed name only.",
            query.source()
        )
    } else if saved.names.len() > 1 {
        searched.to_owned()
    } else if saved.found.is_empty() {
        "Found no other names to search; searched the typed name.".to_owned()
    } else {
        "Searched the typed name.".to_owned()
    };
    if !saved.rejected {
        for found in &saved.found {
            message.push(' ');
            message.push_str(found);
        }
        if saved.omitted != 0 {
            let noun = if saved.omitted == 1 { "name" } else { "names" };
            message.push_str(&format!(
                " Left out {} more {noun} to keep the query within its size limit.",
                saved.omitted
            ));
        }
    }
    message.push_str(" Use --exact-term to search only the typed name.");
    message
}
