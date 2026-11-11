//! Literal GTR test-name matching over the whole pinned version report, before counts and paging.
use super::{bundle, map, raw::Native};
use biodata::{GtrTestId, GtrTestVersion};
use biomcp_fetch::{BundleInfo, Find, KeyKind, Leg, RequestError};
use biomcp_types::{DiagnosticCandidate, DiagnosticSearch, Problem};
use std::collections::HashMap;
type Version = Box<GtrTestVersion>;

/// Case-insensitive literal containment of the query text in a native name cell.
pub(super) fn contains(name: &str, query: &str) -> bool {
    name.to_lowercase().contains(&query.to_lowercase())
}

/// Matching identities from the complete version report, in report order.
pub(super) struct Named {
    ids: Vec<GtrTestId>,
    versions: HashMap<GtrTestId, Vec<Version>>,
    pub(super) refusals: crate::Refusal,
    pub(super) problems: Vec<Problem>,
    pub(super) info: BundleInfo,
}
impl Named {
    /// Whether this exact identity has a matching current or conflicting card.
    pub(super) fn contains(&self, id: &GtrTestId) -> bool {
        self.versions.contains_key(id)
    }
}

/// Read every version row, including tests with no gene or condition relation.
pub(super) fn read(
    leg: &Leg<'_>,
    query: &DiagnosticSearch,
    name: &str,
) -> Result<Named, RequestError> {
    let (rows, info) = bundle::read(leg, Find::Exact(KeyKind::Band, "versions"))?;
    super::revision(query.bundle_revision(), &info)?;
    let needle = name.to_lowercase();
    let mut cx = crate::mapped::Problems {
        source: biomcp_types::Source::Gtr,
        record: None,
        list: Vec::new(),
    };
    let mut refusals = crate::Refusal::None;
    let mut ids = Vec::new();
    let mut versions: HashMap<GtrTestId, Vec<Version>> = HashMap::new();
    for row in rows {
        match row.value {
            Native::Refused(r) if r.matches(query) => {
                refusals = crate::Refusal::Refused;
                r.disclose(&mut cx);
            }
            Native::Version(v) => {
                let list = versions.entry(v.test_accession_ver.clone()).or_default();
                if list.is_empty() {
                    ids.push(v.test_accession_ver.clone());
                }
                list.push(v);
            }
            Native::Refused(_) | Native::Relation(_) => {}
        }
    }
    versions.retain(|_, list| listed(list, &needle));
    ids.retain(|id| versions.contains_key(id));
    Ok(Named {
        ids,
        versions,
        refusals,
        problems: cx.list,
        info,
    })
}

/// A current card, or every row of an accession/version conflict, whose name matches.
fn listed(list: &[Version], needle: &str) -> bool {
    (list.len() > 1 || list.iter().all(|v| v.now_current.as_ref() == "1"))
        && list
            .iter()
            .any(|v| v.lab_test_name.to_lowercase().contains(needle))
}

/// Page name-only matches; a conflict stays visible and no card is chosen for it.
pub(super) fn page(named: Named, query: &DiagnosticSearch) -> Result<super::Search, RequestError> {
    let Named {
        ids,
        versions,
        refusals,
        mut problems,
        info,
    } = named;
    let total = ids.len() as u64;
    let items = ids
        .iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
        .map(
            |id| match versions.get(id).map(Vec::as_slice).unwrap_or_default() {
                [v] => Ok(DiagnosticCandidate::Resolved {
                    header: map::header(v, &mut problems)?,
                    matched: None,
                }),
                [] => Err(biomcp_fetch::bad_reply(biomcp_types::Source::Gtr, None)),
                _ => Ok(DiagnosticCandidate::Conflict {
                    id: id.clone(),
                    problem: super::conflict(id),
                }),
            },
        )
        .collect::<Result<Box<[_]>, RequestError>>()?;
    Ok(super::Search {
        refusals,
        problems,
        items,
        total,
        info,
    })
}
