use super::{bundle, map, raw::Native};
use biodata::{GtrMethod, GtrTestRelation};
use biomcp_fetch::{BundleInfo, Find, KeyKind, Leg, RequestError};
use biomcp_types::{DiagnosticGet, GtrCard, Problem};
/// Exact source identity and independent relation collections under one local snapshot.
#[derive(Debug)]
pub struct Record {
    /// Actual refused native records, independent of optional fields.
    pub refusals: crate::Refusal,
    /// Rejected optional source prose, without discarding the native identity.
    pub problems: Vec<Problem>,
    /// Selected card, absent for missing or conflicting identities.
    pub card: Option<GtrCard>,
    /// Conflict supplied before any current flag selection.
    pub conflict: Option<Problem>,
    /// Native relation rows, including rows without a current card.
    pub relations: Vec<GtrTestRelation>,
    /// Independent method lists with original indices.
    pub methods: Vec<GtrMethod>,
    /// Stored snapshot identity.
    pub info: BundleInfo,
}
/// Read one exact accession/version; no latest-version fallback, and no refresh of the stored pair.
pub fn get(
    leg: &Leg<'_>,
    query: &DiagnosticGet,
    id: &biodata::GtrTestId,
) -> Result<Record, RequestError> {
    let (rows, info) = bundle::read(leg, Find::Exact(KeyKind::Id, query.id().as_str()))?;
    super::revision(query.bundle_revision(), &info)?;
    let mut versions = Vec::new();
    let mut xml = None;
    let mut relations = Vec::new();
    let mut cx = crate::mapped::Problems {
        source: biomcp_types::Source::Gtr,
        record: None,
        list: Vec::new(),
    };
    let mut refusals = crate::Refusal::None;
    for row in rows {
        match row.value {
            Native::Version(v) => versions.push(v),
            Native::Test(x) => xml = xml.or(Some(x)),
            Native::Lab(_) => {}
            Native::Refused(r) => {
                refusals = crate::Refusal::Refused;
                r.disclose(&mut cx);
            }
            Native::Relation(r) => {
                if let Some(relation) = map::relation(&r, &mut cx, &mut refusals) {
                    relations.push(relation);
                }
            }
        }
    }
    let conflict = if versions.len() > 1 {
        Some(super::conflict(id))
    } else {
        None
    };
    let mut problems = cx.list;
    let card = if conflict.is_none()
        && let Some(v) = versions.first()
    {
        let lab = lab(leg, &v.lab_id, &info)?;
        Some(map::card(v, xml.as_ref(), lab.as_ref(), &mut problems)?)
    } else {
        None
    };
    let methods = if conflict.is_none() {
        versions
            .first()
            .map(|v| map::methods(v))
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    Ok(Record {
        refusals,
        problems,
        card,
        conflict,
        relations,
        methods,
        info,
    })
}
/// The laboratory's XML row, read under the card's snapshot.
fn lab(
    leg: &Leg<'_>,
    lab_id: &str,
    info: &BundleInfo,
) -> Result<Option<super::raw::XmlLab>, RequestError> {
    if lab_id.is_empty() {
        return Ok(None);
    }
    let key = super::raw::lab_key(lab_id);
    let (rows, other) = bundle::read(leg, Find::Exact(KeyKind::Xref, &key))?;
    if other.content_revision != info.content_revision {
        return Err(biomcp_fetch::bad_reply(biomcp_types::Source::Gtr, None));
    }
    Ok(rows.into_iter().find_map(|row| match row.value {
        Native::Lab(lab) if *lab.id == *lab_id => Some(lab),
        _ => None,
    }))
}
