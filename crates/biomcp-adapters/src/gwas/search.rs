//! One native association conjunction; no local prefix intersection.
use super::{Page, native, scope::Selectors};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::GwasSearch;
use std::collections::BTreeMap;
pub(super) mod identity;
pub(super) mod traits;

const NATIVE_PAGE_SIZE: u32 = 10;

/// Read a bounded p-value-ascending window, sorting locally only for a complete match.
pub fn search(leg: &Leg<'_>, q: &GwasSearch) -> Result<Page, RequestError> {
    window(
        leg,
        selectors(q),
        q.limit(),
        q.offset() == 0,
        (q.offset() % q.limit()) as usize,
        |row| identity::row(row, q),
    )
}

pub(super) fn window(
    leg: &Leg<'_>,
    sorted: BTreeMap<String, String>,
    size: u32,
    at_start: bool,
    skip: usize,
    matches_row: impl Fn(&Selectors) -> bool + Sync,
) -> Result<Page, RequestError> {
    if size <= NATIVE_PAGE_SIZE || !at_start {
        return native(leg, &sorted, skip, &matches_row);
    }
    // Sorted native pages have unstable ties. Only sort locally after retrieving
    // a complete bounded match; a partial result retains the native global order.
    let mut values = sorted.clone();
    values.remove("sort");
    values.remove("direction");
    values.insert("size".into(), NATIVE_PAGE_SIZE.to_string());
    let mut combined = native(leg, &values, 0, &matches_row)?;
    if combined.total > u64::from(size) {
        return native(leg, &sorted, 0, &matches_row);
    }
    let pages: Vec<_> = (1..combined.total.div_ceil(NATIVE_PAGE_SIZE.into())).collect();
    let mut seen: std::collections::BTreeSet<_> =
        combined.rows.iter().map(|row| row.value.id).collect();
    for result in leg.batch(&pages, |child, page| {
        let mut values = values.clone();
        values.insert("page".into(), page.to_string());
        native(child, &values, 0, &matches_row)
    }) {
        let page = result?;
        if combined.total != page.total {
            return Err(biomcp_fetch::bad_reply(
                biomcp_types::Source::Gwas,
                Some(200),
            ));
        }
        combined.refusals.include(page.refusals);
        combined.problems.extend(page.problems);
        combined.received += page.received;
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
    if u64::from(combined.received) != combined.total {
        return Err(biomcp_fetch::bad_reply(
            biomcp_types::Source::Gwas,
            Some(200),
        ));
    }
    // Optional metadata and refused identities must retain the original native
    // page's usable rows and diagnostics when exact local ordering is unprovable.
    if combined.rows.len() as u64 != combined.total
        || combined
            .rows
            .iter()
            .any(|row| row.value.exact_p_value.is_none())
    {
        return native(leg, &sorted, 0, &matches_row);
    }
    combined.rows.sort_by(|a, b| {
        a.value
            .exact_p_value
            .cmp(&b.value.exact_p_value)
            .then_with(|| a.value.id.get().cmp(&b.value.id.get()))
    });
    Ok(combined)
}

fn selectors(q: &GwasSearch) -> BTreeMap<String, String> {
    let mut values = super::association_params(q.offset() / q.limit(), q.limit());
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
    values
}
