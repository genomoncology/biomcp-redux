use super::{variant::{bounds, failure}, variant_claims, variant_raw as raw};
use crate::{Mapped, mapped::Problems, raw::Field};
use biodata::{GeneSymbol, HgvsText, Text, VariantRef};
use biomcp_fetch::RequestError;
use biomcp_types::{CivicClaimKind, CivicClaimPage, CivicMember, CivicMembership, CivicProfile, CivicStatement, Reason, Source, SourceRecord, VariantCivic, VariantGet};

pub(super) fn identity(value: Option<Field<u64>>, field: &str, cx: &mut Problems) -> Option<u64> {
    let value = cx.required_field(field, value)?;
    cx.optional(field, Some(if raw::id(value) { Ok(value) } else { Err(()) }))
}
pub(super) fn page(raw: raw::Connection<raw::Profile>, query: &VariantGet, allele: &str) -> Result<Mapped<VariantCivic>, RequestError> {
    let consumed = raw.nodes.len();
    let mut cx = Problems { source: Source::Civic, record: None, list: Vec::new() };
    let mut profiles = Vec::new();
    for profile in raw.nodes {
        cx.record = None;
        if let Some(profile) = map(profile, query, allele, &mut cx)? { profiles.push(profile); }
    }
    let page = bounds(if query.civic_profile().is_some() { 0 } else { query.section_offset() }, profiles.len(), raw.total_count);
    let next = if query.civic_profile().is_none() && raw.page_info.has_next_page {
        Some(query.at_section_offset(query.section_offset().checked_add(consumed as u32).ok_or_else(|| failure(Reason::TooLarge))?))
    } else { None };
    Ok(Mapped { value: VariantCivic { profiles: profiles.into(), page, next }, problems: cx.list })
}
fn map(raw: raw::Profile, query: &VariantGet, allele: &str, cx: &mut Problems) -> Result<Option<CivicProfile>, RequestError> {
    let Some(id) = identity(raw.id, "id", cx) else { return Ok(None); };
    cx.record = crate::mapped::record(id);
    if query.civic_profile().is_some_and(|requested| requested != id) {
        cx.required::<()>("id", None); return Ok(None);
    }
    let start = cx.list.len();
    let label = cx.source_field("name", raw.name);
    let compound = cx.field("isComplex", raw.is_complex);
    let raw_name = cx.field("rawName", raw.raw_name);
    let native_count = raw.variants.as_ref().and_then(Field::as_ref).map(Vec::len);
    let native_members = cx.fields("variants", raw.variants);
    let (members, exact) = checked_members(native_members, allele, cx);
    let context_bad = cx.list.len() != start;
    let complete = native_count == Some(members.len()) && !context_bad;
    let link = cx.field("link", raw.link);
    let Some(record) = record(id, link.as_ref(), cx) else { return Ok(None); };
    let subject = subject(&record, label.as_ref(), raw_name.as_ref(), compound, members.iter().collect(), complete, cx);
    let mut evidence_page = claim_page(&raw.evidence_items, query, id, CivicClaimKind::Evidence, exact)?;
    let mut assertion_page = claim_page(&raw.assertions, query, id, CivicClaimKind::Assertions, exact)?;
    let mut statements = Vec::<CivicStatement>::new();
    if exact {
        for row in raw.evidence_items.nodes {
            if let Some(row) = variant_claims::evidence(row, subject.as_ref(), context_bad, cx) { statements.push(row.claim); }
        }
        evidence_page.returned = statements.len() as u32;
        let before = statements.len();
        for row in raw.assertions.nodes {
            if let Some(row) = variant_claims::assertion(row, subject.as_ref(), context_bad, cx) { statements.push(row.claim); }
        }
        assertion_page.returned = (statements.len() - before) as u32;
    } else {
        cx.record = Some(record.id.clone());
        if !context_bad { cx.required::<()>("molecular_profile.membership", None); }
    }
    Ok(Some(CivicProfile { record, label, raw_name, compound, membership: if exact { CivicMembership::Exact } else { CivicMembership::Candidate }, members: members.into(), evidence_count: raw.evidence_items.total_count, assertion_count: raw.assertions.total_count, statements: statements.into(), evidence_page, assertion_page }))
}
fn checked_members(raw: Box<[raw::Member]>, allele: &str, cx: &mut Problems) -> (Vec<CivicMember>, bool) {
    let parent = cx.record.clone();
    let mut members = Vec::new();
    let mut exact = false;
    for value in raw.into_vec() {
        cx.record = parent.clone();
        if let Some((member, caid)) = member(value, cx) {
            exact |= caid.as_ref().is_some_and(|id| id.as_str() == allele);
            members.push(member);
        }
    }
    cx.record = parent;
    (members, exact)
}
fn claim_page<T>(raw: &raw::Connection<T>, query: &VariantGet, profile: u64, kind: CivicClaimKind, exact: bool) -> Result<CivicClaimPage, RequestError> {
    let next = if exact && (raw.page_info.has_next_page || raw.nodes.is_empty() && raw.total_count > 0) {
        Some(query.with_civic_page(profile, kind, raw.page_info.end_cursor.clone()).map_err(|_| failure(Reason::BadReply))?)
    } else { None };
    Ok(CivicClaimPage { returned: if exact { raw.nodes.len() as u32 } else { 0 }, total: raw.total_count, next })
}
pub(super) fn member(raw: raw::Member, cx: &mut Problems) -> Option<(CivicMember, Option<Text>)> {
    let id = identity(raw.id, "variants.id", cx)?;
    cx.record = crate::mapped::record(id);
    let before = cx.list.len();
    let label = cx.source_field("variants.name", raw.name);
    let feature = cx.field("feature", raw.feature);
    let (feature, feature_record) = if let Some(feature) = feature {
        let name = cx.source_field("feature.name", feature.name);
        let id = cx.field("feature.id", feature.id).and_then(|id| cx.optional("feature.id", Some(if raw::id(id) { format!("civic-feature:{id}").parse() } else { Err(biodata::Error::CurieInvalid) })));
        (name, id)
    } else { (None, None) };
    let hgvs_descriptions = cx.fields("hgvsDescriptions", raw.hgvs_descriptions);
    let caid = cx.field("alleleRegistryId", raw.allele_registry_id);
    let complete = cx.list.len() == before;
    let subject = if complete { member_subject(label.as_ref(), feature.as_ref(), &hgvs_descriptions) } else { None };
    let link = cx.field("variants.link", raw.link);
    Some((CivicMember { record: record(id, link.as_ref(), cx)?, label, feature, feature_record, hgvs_descriptions, subject }, caid))
}
fn member_subject(label: Option<&Text>, feature: Option<&Text>, forms: &[Text]) -> Option<VariantRef> {
    let label = label?;
    // Fusion and other native features remain literal when the common gene model cannot express them.
    let Some(Ok(symbol)) = feature.map(|feature| feature.as_str().parse::<GeneSymbol>()) else { return None; };
    let Some(Ok(gene)) = Some(biodata::GeneRef::try_from(symbol)) else { return None; };
    let proteins: Vec<_> = forms.iter().filter(|form| form.as_str().split_once(':').is_some_and(|(_, body)| body.starts_with("p.")))
        .flat_map(|form| form.as_str().parse::<HgvsText>()).collect();
    match proteins.as_slice() {
        [] => None,
        [protein] => Some(VariantRef::protein_hgvs(gene, protein.clone(), Some(label.clone()))),
        _ => Some(VariantRef::protein(gene, label.clone())),
    }
}
pub(super) fn subject(record: &SourceRecord, label: Option<&Text>, raw_name: Option<&Text>, compound: Option<bool>, members: Vec<&CivicMember>, complete: bool, cx: &mut Problems) -> Option<VariantRef> {
    if !complete { return None; }
    match (compound, members.as_slice()) {
        (Some(false), [member]) => member.subject.clone(),
        (Some(true), [_, ..]) => {
            let label = label?;
            let native = members.iter().map(|member| {
                let id = format!("civic:{}", member.record.id).parse().into_iter().next()?;
                VariantRef::source_with_feature(id, member.feature.clone()?, member.feature_record.clone(), member.subject.clone().into_iter().collect()).into_iter().next()
            }).collect::<Option<Vec<_>>>()?;
            let expression = cx.required_field("rawName", raw_name.cloned().map(Field::Value))?;
            let expression = cx.optional("rawName", Some(biodata::ProfileExpression::civic(expression)))?;
            cx.optional("molecular_profile.expression", Some(VariantRef::profile_expression(record.id.clone(), label.clone(), native.into(), expression)))
        }
        _ => None,
    }
}
pub(super) fn record(id: u64, link: Option<&Text>, cx: &mut Problems) -> Option<SourceRecord> {
    let id = cx.optional("id", Some(Text::new(&id.to_string())))?;
    let url = cx.optional("link", link.map(|link| {
        if !link.as_str().starts_with('/') { return Err(()); }
        Text::new(&format!("https://civicdb.org{link}")).map_err(|_| ())
    }));
    Some(SourceRecord { id, url })
}
