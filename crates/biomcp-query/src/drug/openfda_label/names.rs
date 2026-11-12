//! Search a widened label name group at the step its typed name selects.
use crate::{Ctx, Outcome, drug::search_names, drug::widen};
use biomcp_adapters::openfda_label::{NameGroup, NameStep, Search};
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::DrugSearch;

pub(super) type Saved = widen::Saved<NameStep>;

/// How the names were searched, for the one notice.
const SEARCHED: &str = "one label query, exact names first";

pub(super) fn prepare<'a>(
    cx: &Ctx<'a>,
    query: &DrugSearch,
    deadline: &mut Deadline<'a>,
) -> Result<Option<Saved>, RequestError> {
    widen::prepare(cx, query, deadline, |names| {
        NameGroup::new(names, query.indication()).is_some()
    })
}

/// The typed name's own group, which the search falls back to and steps by.
pub(super) fn typed(query: &DrugSearch) -> Option<NameGroup> {
    let input = query.query()?;
    NameGroup::new(std::slice::from_ref(input), query.indication())
}

pub(super) fn search(leg: &biomcp_fetch::Leg<'_>, query: &DrugSearch, saved: &mut Saved) -> Search {
    let page = (query.limit(), query.offset());
    let (Some(typed), Some(group)) = (
        typed(query),
        NameGroup::new(&saved.names, query.indication()),
    ) else {
        // Both were checked before the search, so only a damaged snapshot reaches this.
        return Search {
            page: Err(biomcp_fetch::CacheError::Snapshot.into()),
            free_text: None,
        };
    };
    let widened = (saved.names.len() > 1).then_some(&typed);
    let (result, step) =
        biomcp_adapters::openfda_label::search_expanded(leg, &group, widened, page, saved.step);
    saved.step = Some(step);
    if saved.names.len() < 2 || !search_names::rejected(result.page.as_ref().err()) {
        return result;
    }
    // The typed name alone is searched and kept for later pages, in place of the refused group.
    saved.reject(query);
    let (result, step) =
        biomcp_adapters::openfda_label::search_expanded(leg, &typed, None, page, None);
    saved.step = Some(step);
    result
}

/// Show the name selector searched and the one notice naming the names.
pub(super) fn finish(query: &DrugSearch, saved: Option<Saved>, out: &mut Outcome) {
    let Some(saved) = saved else { return };
    if saved.step == Some(NameStep::Names)
        && let Some(group) = NameGroup::new(&saved.names, query.indication())
        && let Some(source) = out.sources.first_mut()
    {
        source.term = match group.term(NameStep::Names).parse() {
            Ok(term) => Some(term),
            // The selector was built from checked names, so only a local fault reaches this.
            Err(_) => {
                out.local = Some(biomcp_fetch::CacheError::Snapshot.into());
                None
            }
        };
    }
    widen::finish(query, saved, SEARCHED, out);
}
