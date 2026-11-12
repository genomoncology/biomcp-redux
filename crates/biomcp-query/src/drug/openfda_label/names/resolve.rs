//! Reuse complete source-stated names, then apply the whole native selector bound.
use super::Saved;
use crate::{Ctx, drug::search_names, status};
use biodata::Text;
use biomcp_adapters::openfda_label::NameGroup;
use biomcp_fetch::Deadline;
use biomcp_types::{DrugSearch, ReplyData};
pub(super) fn resolve<'a>(cx: &Ctx<'a>, query: &DrugSearch, deadline: &mut Deadline<'a>) -> Saved {
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
                if NameGroup::new(&candidate, query.indication()).is_some() {
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
