//! One native association conjunction; no local prefix intersection.
use super::{ASSOCIATIONS, Page, raw};
use biomcp_fetch::Input;
use biomcp_fetch::{Arg, BadReply, Leg, RequestError};
use biomcp_types::GwasSearch;
use std::collections::BTreeMap;
pub(super) mod identity;

const NATIVE_PAGE_SIZE: u32 = 10;

/// Read the checked native page under ascending source p value.
pub fn search(leg: &Leg<'_>, q: &GwasSearch) -> Result<Page, RequestError> {
    if q.limit() <= NATIVE_PAGE_SIZE {
        return native(leg, q, selectors(q));
    }
    // Large source replies can consume the command budget. Bound rows per request,
    // and use the fetch layer's bounded workers, source pacing and shared deadline.
    let start = q.offset() - q.offset() % q.limit();
    let end = start.saturating_add(q.limit());
    let pages: Vec<_> = (start / NATIVE_PAGE_SIZE..end.div_ceil(NATIVE_PAGE_SIZE)).collect();
    let mut combined = Page {
        refusals: crate::Refusal::None,
        received: 0,
        problems: Vec::new(),
        rows: Vec::new(),
        total: 0,
    };
    let mut seen = std::collections::BTreeSet::new();
    for (index, result) in leg
        .batch(&pages, |child, page| {
            let mut values = selectors(q);
            values.insert("page".into(), page.to_string());
            values.insert("size".into(), NATIVE_PAGE_SIZE.to_string());
            native(child, q, values)
        })
        .into_iter()
        .enumerate()
    {
        let page = result?;
        if index > 0 && combined.total != page.total {
            return Err(biomcp_fetch::bad_reply(
                biomcp_types::Source::Gwas,
                Some(200),
            ));
        }
        combined.total = page.total;
        combined.refusals.include(page.refusals);
        combined.problems.extend(page.problems);
        for row in page.rows {
            if !seen.insert(row.value.id) {
                return Err(biomcp_fetch::bad_reply(
                    biomcp_types::Source::Gwas,
                    Some(200),
                ));
            }
            combined.rows.push(row);
        }
    }
    if combined.rows.windows(2).any(|pair| match pair {
        [a, b] => {
            matches!((&a.value.exact_p_value, &b.value.exact_p_value), (Some(a), Some(b)) if a > b)
        }
        _ => false,
    }) {
        return Err(biomcp_fetch::bad_reply(
            biomcp_types::Source::Gwas,
            Some(200),
        ));
    }
    combined.received = combined
        .total
        .saturating_sub(start.into())
        .min(q.limit().into()) as u32;
    Ok(combined)
}

fn native(
    leg: &Leg<'_>,
    q: &GwasSearch,
    values: BTreeMap<String, String>,
) -> Result<Page, RequestError> {
    let params: Vec<_> = values
        .iter()
        .map(|(k, v)| (k.as_str(), Arg::Text(v)))
        .collect();
    let mut response = leg
        .call(
            &ASSOCIATIONS,
            Input::Params(&params),
            biomcp_fetch::Read::<raw::Response>::json()
                .check(&|reply| admission(reply, q, &values))
                .cache_if(&raw::Response::identities_admitted),
        )?
        .value;
    if q.limit() > NATIVE_PAGE_SIZE
        && let Some(rows) = &mut response.embedded
    {
        let start = response.page.number * response.page.size;
        let end = (q.offset() - q.offset() % q.limit()).saturating_add(q.limit());
        rows.associations
            .truncate(end.saturating_sub(start) as usize);
        let skip = q
            .offset()
            .saturating_sub(start)
            .min(rows.associations.len() as u32);
        rows.associations.drain(..skip as usize);
    }
    let skip = if q.limit() <= NATIVE_PAGE_SIZE {
        (q.offset() % q.limit()) as usize
    } else {
        0
    };
    Ok(super::page(response, skip))
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
    if Some(r.page.number.to_string()) != values.get("page").cloned()
        || Some(r.page.size.to_string()) != values.get("size").cloned()
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
