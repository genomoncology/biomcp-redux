//! One native association conjunction; no local prefix intersection.
use super::{ASSOCIATIONS, Page, raw};
use biomcp_fetch::Input;
use biomcp_fetch::{Arg, BadReply, Leg, RequestError};
use biomcp_types::GwasSearch;
use std::collections::BTreeMap;
mod identity;

/// Read the checked native page under ascending source p value.
pub fn search(leg: &Leg<'_>, q: &GwasSearch) -> Result<Page, RequestError> {
    let values = selectors(q);
    let params: Vec<_> = values
        .iter()
        .map(|(k, v)| (k.as_str(), Arg::Text(v)))
        .collect();
    let response = leg
        .call::<raw::Response>(
            &ASSOCIATIONS,
            Input::Params(&params),
            biomcp_fetch::Read::json()
                .check(&(|reply| admission(reply, q, &values)))
                .cache_if(&(raw::Response::identities_admitted)),
        )
        .map(|received| received.value)?;
    Ok(super::page(response, (q.offset() % q.limit()) as usize))
}

fn selectors(q: &GwasSearch) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    for (key, value) in [
        ("rs_id", q.variant().map(ToString::to_string)),
        ("efo_trait", q.trait_label().map(ToString::to_string)),
        ("efo_id", q.trait_id().map(ToString::to_string)),
        ("mapped_gene", q.gene().map(ToString::to_string)),
        ("accession_id", q.study().map(ToString::to_string)),
        ("pubmed_id", q.pubmed().map(ToString::to_string)),
    ] {
        if let Some(value) = value {
            values.insert(key.into(), value);
        }
    }
    if q.trait_id().is_some() {
        values.insert(
            "show_child_trait".into(),
            q.include_child_traits().to_string(),
        );
    }
    if q.gene().is_some() {
        values.insert("extended_geneset".into(), q.extended_geneset().to_string());
    }
    values.insert("sort".into(), "p_value".into());
    values.insert("direction".into(), "asc".into());
    values.insert("page".into(), (q.offset() / q.limit()).to_string());
    values.insert("size".into(), q.limit().to_string());
    values
}
fn admission(
    r: &raw::Response,
    q: &GwasSearch,
    values: &BTreeMap<String, String>,
) -> Result<(), BadReply> {
    if r.page.number != q.offset() / q.limit()
        || r.page.size != q.limit()
        || !identity::self_link(r.links.current.href.as_str(), values)
    {
        return Err(BadReply);
    }
    let rows = r
        .embedded
        .as_ref()
        .map(|e| e.associations.as_slice())
        .unwrap_or_default();
    if !rows.iter().all(|row| identity::row(&row.selectors, q)) {
        return Err(BadReply);
    }
    // Exact parts govern order even when native doubles underflow.
    let mut previous = None;
    for row in rows.iter().filter_map(|row| row.value.as_ref()) {
        if let Some(exact) = row.exact_p_value() {
            if previous.as_ref().is_some_and(|last| last > &exact) {
                return Err(BadReply);
            }
            previous = Some(exact);
        }
    }
    Ok(())
}
