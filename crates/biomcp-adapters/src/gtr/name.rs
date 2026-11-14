//! GTR test-name matching, ignoring letter case and name hyphens, with optional query spaces and
//! hyphens, over the whole pinned version report, before counts and paging.
use super::{bundle, map, raw::Native};
use biodata::{GtrTestId, GtrTestVersion};
use biomcp_fetch::{BundleInfo, Find, KeyKind, Leg, RequestError};
use biomcp_types::{DiagnosticCandidate, DiagnosticSearch, GtrNameMatch, Problem};
use std::collections::HashMap;
type Version = Box<GtrTestVersion>;

/// Hyphens, which a name never needs to match: "Non-small" meets "Nonsmall".
fn hyphen(c: char) -> bool {
    matches!(c, '-' | '\u{2010}' | '\u{2011}' | '\u{ad}')
}

/// Folded `--name` text: lower-cased characters, each marked when a space or hyphen came
/// before it, where the name may hold spaces or nothing. Text of only those matches nothing.
fn needle(query: &str) -> Option<Vec<(char, bool)>> {
    let mut out = Vec::new();
    let mut gap = false;
    for c in query.chars() {
        if c.is_whitespace() || hyphen(c) {
            gap = !out.is_empty();
            continue;
        }
        for (i, lower) in c.to_lowercase().enumerate() {
            out.push((lower, gap && i == 0));
        }
        gap = false;
    }
    (!out.is_empty()).then_some(out)
}

/// Lower-cased name characters without hyphens, each with its source byte span.
fn fold(name: &str) -> Vec<(char, usize, usize)> {
    name.char_indices()
        .filter(|(_, c)| !hyphen(*c))
        .flat_map(|(at, c)| {
            c.to_lowercase()
                .map(move |lower| (lower, at, at + c.len_utf8()))
        })
        .collect()
}

/// Source byte spans of every match: name spaces are skipped only where the query had a gap,
/// so "CF" never meets "Amniotic Fluid" while "non-small cell" meets "Nonsmall cell".
fn spans(name: &str, needle: &[(char, bool)]) -> Vec<(usize, usize)> {
    let chars = fold(name);
    let mut out = Vec::new();
    for begin in 0..chars.len() {
        let mut rest = chars.get(begin..).unwrap_or_default().iter().peekable();
        let mut last = None;
        let matched = needle.iter().all(|(want, gap)| {
            if *gap && last.is_some() {
                while rest.next_if(|(c, ..)| c.is_whitespace()).is_some() {}
            }
            rest.next_if(|(c, ..)| c == want)
                .inspect(|v| last = Some(**v))
                .is_some()
        });
        if let (true, Some((_, start, _)), Some((_, _, end))) = (matched, chars.get(begin), last) {
            out.push((*start, end));
        }
    }
    out
}

/// Whether the query occurs in a native name cell under the folding rule above.
pub(super) fn contains(name: &str, query: &str) -> bool {
    needle(query).is_some_and(|needle| !spans(name, &needle).is_empty())
}

/// Whether a folded needle occurs in a name with no letter or digit touching either end.
fn whole_word(name: &str, needle: &[(char, bool)]) -> bool {
    spans(name, needle).into_iter().any(|(start, end)| {
        let before = name.get(..start).and_then(|v| v.chars().next_back());
        let after = name.get(end..).and_then(|v| v.chars().next());
        !before.is_some_and(char::is_alphanumeric) && !after.is_some_and(char::is_alphanumeric)
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
    let needle = needle(name);
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
        .filter_map(|(id, list)| Some((id.clone(), listed(list, needle.as_deref()?)?)))
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
fn listed(list: &[Version], needle: &[(char, bool)]) -> Option<GtrNameMatch> {
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
            .any(|name| !spans(name, needle).is_empty())
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
