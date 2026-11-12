//! Retain the searched name group and the discovery step selected on its first page.
use crate::{Ctx, Outcome, drug::search_names, source_snapshot};
use biodata::Text;
use biomcp_adapters::openfda_label::{NameGroup, NameStep};
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{DrugSearch, Paged, ReplyData, Request, Source, SourceNotice, StepKind};
use serde::{Deserialize, Serialize};
mod resolve;
#[derive(Serialize, Deserialize)]
pub(super) struct Saved {
    names: Vec<Text>,
    sources: Vec<biomcp_types::SourceOutcome>,
    problems: Vec<biomcp_types::Problem>,
    /// What the name lookups said, for the one notice on the label source.
    #[serde(default)]
    found: Vec<String>,
    omitted: usize,
    step: Option<NameStep>,
    /// The provider refused the widened request, so the search kept the typed name alone.
    #[serde(default)]
    rejected: bool,
}
pub(super) fn prepare<'a>(
    cx: &Ctx<'a>,
    query: &DrugSearch,
    deadline: &mut Deadline<'a>,
) -> Result<Option<Saved>, RequestError> {
    if let Some(token) = query.name_snapshot() {
        let key = source_snapshot::key("label-drug-names", &query.restart())?;
        let saved: Option<Saved> = source_snapshot::read(cx, &key, token.as_str(), *deadline)?;
        return Ok(saved.filter(|saved| {
            saved.step.is_some() && NameGroup::new(&saved.names, query.indication()).is_some()
        }));
    }
    Ok(Some(resolve::resolve(cx, query, deadline)))
}
/// The typed name's own group, which the search falls back to and steps by.
pub(super) fn typed(query: &DrugSearch) -> Option<NameGroup> {
    let input = query.query()?;
    NameGroup::new(std::slice::from_ref(input), query.indication())
}
pub(super) fn search(
    leg: &biomcp_fetch::Leg<'_>,
    query: &DrugSearch,
    saved: &mut Saved,
) -> biomcp_adapters::openfda_label::Search {
    let page = (query.limit(), query.offset());
    let typed = typed(query);
    let group = NameGroup::new(&saved.names, query.indication());
    let (Some(typed), Some(group)) = (typed, group) else {
        // Both were checked before the search, so only a damaged snapshot reaches this.
        return biomcp_adapters::openfda_label::Search {
            page: Err(biomcp_fetch::CacheError::Snapshot.into()),
            free_text: None,
        };
    };
    let widened = (saved.names.len() > 1).then_some(&typed);
    let (result, step) = biomcp_adapters::openfda_label::search_expanded(
        leg, &group, widened, page, saved.step,
    );
    saved.step = Some(step);
    if saved.names.len() < 2 || !search_names::rejected(result.page.as_ref().err()) {
        return result;
    }
    // The typed name alone is searched and kept for later pages, in place of the refused group.
    saved.names = query.query().cloned().into_iter().collect();
    saved.omitted = 0;
    saved.rejected = true;
    let (result, step) =
        biomcp_adapters::openfda_label::search_expanded(leg, &typed, None, page, None);
    saved.step = Some(step);
    result
}
pub(super) fn retain(
    cx: &Ctx<'_>,
    query: &DrugSearch,
    saved: Option<&Saved>,
    deadline: Deadline<'_>,
    success: bool,
    out: &mut Outcome,
) -> DrugSearch {
    let Some(saved) = saved.filter(|_| success && query.name_snapshot().is_none()) else {
        return query.clone();
    };
    let result = source_snapshot::key("label-drug-names", &query.restart()).and_then(|key| {
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
pub(super) fn missing(query: &DrugSearch, out: &mut Outcome) {
    out.body = ReplyData::PoolError { mismatch: false };
    out.next.push(crate::next::Step::run(
        StepKind::Restart,
        &[],
        Request::SearchDrug(query.restart()),
        false,
    ));
}
/// One notice on the label source names every name searched and why any other was not.
pub(super) fn finish(query: &DrugSearch, saved: Option<Saved>, out: &mut Outcome) {
    let Some(saved) = saved else { return };
    if let Some(source) = out.sources.first_mut() {
        if saved.step == Some(NameStep::Names)
            && let Some(group) = NameGroup::new(&saved.names, query.indication())
        {
            source.term = group.term(NameStep::Names).parse().ok();
        }
        if let Some(input) = query.query() {
            source.notices.push(SourceNotice::DrugNameSearch {
                input: input.clone(),
                names: saved.names.clone().into(),
                message: message(&saved).into(),
            });
        }
    }
    // The label source's own outcome stands; a lookup that read it stays out of the reply.
    out.sources.extend(
        saved
            .sources
            .into_iter()
            .filter(|source| source.source != Source::OpenfdaLabel),
    );
    out.problems.extend(saved.problems);
}

fn message(saved: &Saved) -> String {
    let mut message = if saved.rejected {
        "openFDA rejected the label query holding the drug's other names; searched the typed name only."
            .to_owned()
    } else if saved.names.len() > 1 {
        "Searched these names in one label query; the total is openFDA's own count of matching \
         label sets, not a sum per name."
            .to_owned()
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
                " Left out {} more {noun} to keep the label query within its size limit.",
                saved.omitted
            ));
        }
    }
    message.push_str(" Use --exact-term to search only the typed name.");
    message
}
