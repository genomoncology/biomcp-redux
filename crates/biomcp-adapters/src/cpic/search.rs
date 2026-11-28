//! Gene pair search, with a page past the end read as empty with its total (1061).
use super::read::{Page, ordered, read};
use biodata::PgxPair;
use biomcp_fetch::{Arg, Leg, RequestError};
use biomcp_types::PgxSearch;

/// Search exact gene pairs, preserving native level namespaces and row order.
///
/// CPIC answers an offset past the last row with status 416, so that page is read as empty
/// and the total comes from a one-row read of the same search.
pub fn search(leg: &Leg<'_>, query: &PgxSearch) -> Result<Page<PgxPair>, RequestError> {
    match pairs(leg, query, query.offset(), query.limit()) {
        Err(RequestError::Source(error)) if error.status == Some(416) && query.offset() > 0 => {
            let first = pairs(leg, query, 0, 1)?;
            if u64::from(query.offset()) < first.total {
                if let Some(gene) = query.gene()
                    && let Some(pair) = first.items.first()
                    && (pair.genesymbol.as_str() != gene.as_str()
                        || query
                            .cpic_level()
                            .is_some_and(|level| pair.cpiclevel.as_deref() != Some(level.as_str())))
                {
                    return pair_page(leg, query, query.offset(), query.limit(), true);
                }
                return Err(error.into());
            }
            Ok(Page {
                refusals: crate::Refusal::None,
                items: Vec::new(),
                total: first.total,
                consumed: 0,
                problems: Vec::new(),
            })
        }
        reply => reply,
    }
}

/// Read one pair page with native level namespaces and row order.
fn pairs(
    leg: &Leg<'_>,
    query: &PgxSearch,
    offset: u32,
    limit: u32,
) -> Result<Page<PgxPair>, RequestError> {
    let ignore_case = query.drug().is_some();
    let page = pair_page(leg, query, offset, limit, ignore_case);
    if !ignore_case && page.as_ref().is_ok_and(|p| p.total == 0) {
        return pair_page(leg, query, offset, limit, true);
    }
    page
}
fn pair_page(
    leg: &Leg<'_>,
    query: &PgxSearch,
    offset: u32,
    limit: u32,
    ignore_case: bool,
) -> Result<Page<PgxPair>, RequestError> {
    let filter = |value: &str| {
        if ignore_case {
            super::case_insensitive(value)
        } else {
            format!("eq.{value}")
        }
    };
    let selector = if let Some(gene) = query.gene() {
        ("genesymbol", filter(gene.as_str()))
    } else if let Some(drug) = query.drug() {
        ("drugname", filter(drug.as_str()))
    } else {
        return Err(biomcp_fetch::InputError::Serialization.into());
    };
    let level = query.cpic_level().map(|v| filter(v.as_str()));
    let mut args = vec![
        ("select", Arg::Text("*")),
        (selector.0, Arg::Text(&selector.1)),
        ("order", Arg::Text("pairid.asc")),
        ("limit", Arg::Num(limit.into())),
        ("offset", Arg::Num(offset.into())),
    ];
    if let Some(level) = &level {
        args.push(("cpiclevel", Arg::Text(level)));
    }
    read::<super::raw::PgxPair>(
        leg,
        "/pair_view",
        &args,
        offset,
        limit,
        false,
        |rows: &[PgxPair], _range| {
            rows.iter().all(|r| {
                query.gene().is_none_or(|gene| r.genesymbol == *gene)
                    && query.drug().is_none_or(|drug| {
                        r.drugname
                            .as_deref()
                            .is_some_and(|name| name.eq_ignore_ascii_case(drug.as_str()))
                    })
                    && query.cpic_level().is_none_or(|v| {
                        r.cpiclevel
                            .as_deref()
                            .is_some_and(|level| level.eq_ignore_ascii_case(v.as_str()))
                    })
            }) && ordered(rows.iter().map(|r| r.pairid.id))
        },
    )
}
