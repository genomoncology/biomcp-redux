//! Resolve whole drug names once, using the same source identity checks as get drug.
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_types::{DrugGet, DrugSource, ReplyData, SourceNotice, SourceState};

/// Source names in deterministic order, shared by trial and article consumers.
pub(crate) fn resolve(cx: &Ctx<'_>, input: &Text, out: &mut Outcome) -> Vec<Text> {
    let deadline = cx.fetch.deadline(cx.budget);
    for source in [
        DrugSource::Mychem,
        DrugSource::OpenfdaLabel,
        DrugSource::Chembl,
    ] {
        if cx.fetch.disabled(source.source()) || deadline.remaining().is_zero() {
            continue;
        }
        let query = DrugGet::named(input.clone(), source);
        let found = super::resolve::get(&super::resolve::within(cx, deadline.remaining()), &query);
        if found.local.is_some()
            || found.dropped
            || found
                .sources
                .iter()
                .any(|s| s.source == source.source() && s.state != SourceState::Ok)
        {
            continue;
        }
        let ReplyData::DrugCard {
            body: Some(card), ..
        } = &found.body
        else {
            continue;
        };
        if !super::aliases::literal_name(card, input) {
            continue;
        }
        let ambiguous = super::aliases::ambiguous(card);
        let mut names: Vec<_> = if ambiguous {
            vec![input.clone()]
        } else {
            super::aliases::names(card).into_iter().cloned().collect()
        };
        names.sort_by_key(|name| name.as_str().to_lowercase());
        for mut evidence in found.sources {
            if evidence.source == source.source() {
                evidence.notices.push(SourceNotice::DrugNameSearch {
                    input: input.clone(),
                    names: names.clone().into(),
                    message: if ambiguous {
                        "The name matches more than one drug record; searched the typed term without expansion.".into()
                    } else {
                        format!("Searched the names stated by {} record {}.", card.header.source, card.header.id).into()
                    },
                });
            }
            if let Some(existing) = out.sources.iter_mut().find(|s| s.source == evidence.source) {
                status::retain_source_state(existing, &evidence);
                existing.notices.extend(evidence.notices);
            } else {
                out.sources.push(evidence);
            }
        }
        out.problems.extend(found.problems);
        return names;
    }
    Vec::new()
}

/// Source names are literal terms inside a native Boolean group.
pub(crate) fn alternatives(names: &[Text]) -> Option<Text> {
    Text::new(&format!(
        "({})",
        names
            .iter()
            .map(|name| {
                format!(
                    "\"{}\"",
                    name.as_str().replace('\\', "\\\\").replace('"', "\\\"")
                )
            })
            .collect::<Vec<_>>()
            .join(" OR ")
    ))
    .ok()
}
