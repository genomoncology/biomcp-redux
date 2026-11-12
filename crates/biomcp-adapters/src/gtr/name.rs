//! Literal GTR test-name matching over the whole pinned version report, before counts and paging.
use super::{bundle, map, raw::Native};
use biodata::{GtrTestId, GtrTestVersion};
use biomcp_fetch::{BundleInfo, Find, KeyKind, Leg, RequestError};
use biomcp_types::{DiagnosticCandidate, DiagnosticSearch, GtrNameMatch, Problem};
use std::collections::HashMap;
type Version = Box<GtrTestVersion>;

/// Case-insensitive literal containment of the query text in a native name cell.
pub(super) fn contains(name: &str, query: &str) -> bool {
    name.to_lowercase().contains(&query.to_lowercase())
}

/// Whether a lower-cased needle occurs in a name with no letter or digit touching either end.
fn whole_word(name: &str, needle: &str) -> bool {
    let name = name.to_lowercase();
    name.match_indices(needle).any(|(at, _)| {
        !name[..at]
            .chars()
            .next_back()
            .is_some_and(char::is_alphanumeric)
            && !name[at + needle.len()..]
                .chars()
                .next()
                .is_some_and(char::is_alphanumeric)
    })
}

/// Matching identities from the complete version report: whole-word matches, then
/// partial-word matches, each in report order.
pub(super) struct Named {
    ids: Vec<GtrTestId>,
    versions: HashMap<GtrTestId, Vec<Version>>,
    matches: HashMap<GtrTestId, GtrNameMatch>,
    pub(super) refusals: crate::Refusal,
    pub(super) problems: Vec<Problem>,
    pub(super) info: BundleInfo,
}
impl Named {
    /// How this exact identity's current or conflicting card matched, if it did.
    pub(super) fn matched(&self, id: &GtrTestId) -> Option<GtrNameMatch> {
        self.matches.get(id).copied()
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
    let matches: HashMap<_, _> = versions
        .iter()
        .filter_map(|(id, list)| Some((id.clone(), listed(list, &needle)?)))
        .collect();
    versions.retain(|id, _| matches.contains_key(id));
    ids.retain(|id| matches.contains_key(id));
    ids.sort_by_key(|id| matches.get(id) != Some(&GtrNameMatch::WholeWord));
    Ok(Named {
        ids,
        versions,
        matches,
        refusals,
        problems: cx.list,
        info,
    })
}

/// How a current card, or any row of an accession/version conflict, matches by name.
fn listed(list: &[Version], needle: &str) -> Option<GtrNameMatch> {
    if let [only] = list
        && only.now_current.as_ref() != "1"
    {
        return None;
    }
    let names = || list.iter().map(|v| &*v.lab_test_name);
    if names().any(|name| whole_word(name, needle)) {
        Some(GtrNameMatch::WholeWord)
    } else {
        names()
            .any(|name| name.to_lowercase().contains(needle))
            .then_some(GtrNameMatch::PartialWord)
    }
}

/// Page name-only matches; a conflict stays visible and no card is chosen for it.
pub(super) fn page(named: Named, query: &DiagnosticSearch) -> Result<super::Search, RequestError> {
    let Named {
        ids,
        versions,
        matches,
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
                    name_match: matches.get(id).copied(),
                }),
                [] => Err(biomcp_fetch::bad_reply(biomcp_types::Source::Gtr, None)),
                _ => Ok(DiagnosticCandidate::Conflict {
                    id: id.clone(),
                    problem: super::conflict(id),
                    name_match: matches.get(id).copied(),
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
