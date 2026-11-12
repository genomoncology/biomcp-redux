use super::{
    bundle, map,
    raw::{Native, Object, Relation},
};
use biodata::GtrTestId;
use biomcp_fetch::{BundleInfo, Find, KeyKind, Leg, RequestError};
use biomcp_types::{DiagnosticCandidate, DiagnosticSearch};
use std::collections::{HashMap, HashSet};
/// One bounded source page and its complete distinct-test count.
#[derive(Debug)]
pub struct Search {
    /// Actual refused native records, independent of optional fields.
    pub refusals: crate::Refusal,
    /// Rejected optional source prose on retained candidates.
    pub problems: Vec<biomcp_types::Problem>,
    /// Native test candidates in relation report order.
    pub items: Box<[DiagnosticCandidate]>,
    /// Distinct exact accession/version identities, including orphan relations and conflicts.
    pub total: u64,
    /// Exact local source snapshot.
    pub info: BundleInfo,
}
/// Match only complete native relation fields, retaining the source relation that matched.
pub fn search(leg: &Leg<'_>, query: &DiagnosticSearch) -> Result<Search, RequestError> {
    let named = query
        .name()
        .map(|name| super::name::read(leg, query, name.as_str()))
        .transpose()?;
    let selected = query.gene().is_some()
        || query.gene_id().is_some()
        || query.condition_id().is_some()
        || query.test_type().is_some();
    match named {
        Some(named) if !selected => super::name::page(named, query),
        named => relations(leg, query, named.as_ref()),
    }
}
/// Relation-selected tests, narrowed to name matches when a name is given.
fn relations(
    leg: &Leg<'_>,
    query: &DiagnosticSearch,
    named: Option<&super::name::Named>,
) -> Result<Search, RequestError> {
    let key;
    let find = if let Some(symbol) = query.gene() {
        Find::Exact(KeyKind::Symbol, symbol.as_str())
    } else if let Some(id) = query.gene_id() {
        key = format!("gene:{id}");
        Find::Exact(KeyKind::Xref, &key)
    } else if let Some(id) = query.condition_id() {
        Find::Exact(KeyKind::Xref, id.as_str())
    } else if let Some(kind) = query.test_type() {
        Find::Exact(KeyKind::Alias, kind.as_str())
    } else {
        Find::Exact(KeyKind::Band, "relations")
    };
    let (rows, info) = bundle::read(leg, find)?;
    super::revision(query.bundle_revision(), &info)?;
    if named.is_some_and(|n| n.info.content_revision != info.content_revision) {
        return Err(biomcp_fetch::bad_reply(biomcp_types::Source::Gtr, None));
    }
    let mut seen = HashSet::new();
    let mut matched = Vec::new();
    let mut cx = crate::mapped::Problems {
        source: biomcp_types::Source::Gtr,
        record: None,
        list: named.map(|n| n.problems.clone()).unwrap_or_default(),
    };
    let mut refusals = named.map_or(crate::Refusal::None, |n| n.refusals);
    for row in rows {
        if let Native::Refused(r) = &row.value
            && r.matches(query)
        {
            refusals = crate::Refusal::Refused;
            r.disclose(&mut cx);
        }
        if let Native::Relation(r) = row.value
            && let Some(relation) = map::relation(&r, &mut cx, &mut refusals)
            && matches(&r, query)
            && named.is_none_or(|n| n.contains(&r.id))
            && seen.insert(r.id.clone())
        {
            matched.push((r, relation));
        }
    }
    let total = matched.len() as u64;
    let page: Vec<_> = matched
        .into_iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
        .collect();
    let mut problems = cx.list;
    let items = cards(leg, &info, &page, &mut problems)?;
    Ok(Search {
        refusals,
        items,
        total,
        info,
        problems,
    })
}
/// Look up only the page's version rows under the same snapshot.
fn cards(
    leg: &Leg<'_>,
    info: &BundleInfo,
    page: &[(Relation, biodata::GtrTestRelation)],
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<Box<[DiagnosticCandidate]>, RequestError> {
    let ids: Vec<_> = page.iter().map(|(r, _)| r.id.to_string()).collect();
    let keys: Vec<_> = ids.iter().map(String::as_str).collect();
    let (versions, other) = bundle::read(leg, Find::AnyExact(KeyKind::Id, &keys))?;
    if other.content_revision != info.content_revision {
        return Err(biomcp_fetch::bad_reply(biomcp_types::Source::Gtr, None));
    }
    let mut by_id: HashMap<GtrTestId, Vec<_>> = HashMap::new();
    for row in versions {
        if let Native::Version(v) = row.value {
            by_id
                .entry(v.test_accession_ver.clone())
                .or_default()
                .push(v);
        }
    }
    page.iter()
        .map(|(r, relation)| {
            candidate(
                r,
                relation.clone(),
                by_id.get(&r.id).map(Vec::as_slice).unwrap_or_default(),
                problems,
            )
        })
        .collect()
}
fn matches(r: &Relation, q: &DiagnosticSearch) -> bool {
    q.test_type()
        .is_none_or(|v| r.test_type.as_ref().and_then(crate::raw::Field::as_ref) == Some(&v))
        && q.gene()
            .is_none_or(|v| r.object == Object::Gene && &*r.gene_symbol == v.as_str())
        && q.gene_id()
            .is_none_or(|v| r.object == Object::Gene && *r.object_id == v.to_string())
        && q.condition_id().is_none_or(|v| {
            r.object == Object::Condition && format!("MedGen:{}", r.medgen) == v.as_str()
        })
}
fn candidate(
    r: &Relation,
    relation: biodata::GtrTestRelation,
    versions: &[Box<biodata::GtrTestVersion>],
    problems: &mut Vec<biomcp_types::Problem>,
) -> Result<DiagnosticCandidate, RequestError> {
    Ok(match versions {
        [v] if v.now_current.as_ref() == "1" => DiagnosticCandidate::Resolved {
            header: map::header(v, problems)?,
            matched: Some(relation),
        },
        [] | [_] => DiagnosticCandidate::MissingCard {
            id: r.id.clone(),
            matched: relation,
        },
        _ => DiagnosticCandidate::Conflict {
            id: r.id.clone(),
            problem: super::conflict(&r.id),
        },
    })
}
