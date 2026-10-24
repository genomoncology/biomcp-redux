use super::{
    variant::{bounds, failure},
    variant_claims, variant_raw as raw,
};
use crate::Mapped;
use biodata::{
    CivicClaimKind, CivicClaimPage, CivicMember, CivicMembership, CivicProfile, CivicStatement,
    GeneSymbol, HgvsText, Problem, ProblemCode, Reason, Source, SourceRecord, Text, VariantCivic,
    VariantGet, VariantRef,
};
use biomcp_fetch::RequestError;

pub(super) fn page(
    raw: raw::Connection<raw::Profile>,
    query: &VariantGet,
    allele: &str,
) -> Result<Mapped<VariantCivic>, RequestError> {
    let mut problems = Vec::new();
    let mut profiles = Vec::new();
    for profile in raw.nodes {
        profiles.push(map(profile, query, allele, &mut problems)?);
    }
    let page = bounds(
        if query.civic_profile().is_some() {
            0
        } else {
            query.section_offset()
        },
        profiles.len(),
        raw.total_count,
    );
    let next = if query.civic_profile().is_none() && raw.page_info.has_next_page {
        Some(
            query.at_section_offset(
                query
                    .section_offset()
                    .checked_add(profiles.len() as u32)
                    .ok_or_else(|| failure(Reason::TooLarge))?,
            ),
        )
    } else {
        None
    };
    Ok(Mapped {
        value: VariantCivic {
            profiles: profiles.into(),
            page,
            next,
        },
        problems,
    })
}
fn map(
    raw: raw::Profile,
    query: &VariantGet,
    allele: &str,
    problems: &mut Vec<Problem>,
) -> Result<CivicProfile, RequestError> {
    let exact = raw.variants.iter().any(|member| {
        member
            .allele_registry_id
            .as_ref()
            .is_some_and(|id| id.as_str() == allele)
    }) && query.civic_profile().is_none_or(|id| id == raw.id);
    let members = raw
        .variants
        .iter()
        .map(member)
        .collect::<Result<Vec<_>, _>>()?;
    let record = record(raw.id, &raw.link)?;
    let subject = if raw.is_complex || members.len() != 1 {
        VariantRef::profile(
            record.id.clone(),
            raw.name.clone(),
            members.iter().filter_map(|m| m.subject.clone()).collect(),
        )
        .ok()
    } else {
        members.first().and_then(|m| m.subject.clone())
    };
    let mut statements = Vec::<CivicStatement>::new();
    if exact {
        for row in &raw.evidence_items.nodes {
            statements.push(variant_claims::evidence(row, subject.as_ref(), problems)?);
        }
        for row in &raw.assertions.nodes {
            statements.push(variant_claims::assertion(row, subject.as_ref(), problems)?);
        }
    } else {
        problems.push(Problem {
            source: Source::Civic,
            record: Some(record.id.clone()),
            field: "molecular_profile.membership".into(),
            code: ProblemCode::ValueUnknown,
        });
    }
    Ok(CivicProfile {
        record,
        label: raw.name,
        membership: if exact {
            CivicMembership::Exact
        } else {
            CivicMembership::Candidate
        },
        members: members.into(),
        evidence_count: raw.evidence_items.total_count,
        assertion_count: raw.assertions.total_count,
        statements: statements.into(),
        evidence_page: claim_page(
            &raw.evidence_items,
            query,
            raw.id,
            CivicClaimKind::Evidence,
            exact,
        )?,
        assertion_page: claim_page(
            &raw.assertions,
            query,
            raw.id,
            CivicClaimKind::Assertions,
            exact,
        )?,
    })
}
fn claim_page<T>(
    raw: &raw::Connection<T>,
    query: &VariantGet,
    profile: u64,
    kind: CivicClaimKind,
    exact: bool,
) -> Result<CivicClaimPage, RequestError> {
    let next =
        if exact && (raw.page_info.has_next_page || raw.nodes.is_empty() && raw.total_count > 0) {
            Some(
                query
                    .with_civic_page(profile, kind, raw.page_info.end_cursor.clone())
                    .map_err(|_| failure(Reason::BadReply))?,
            )
        } else {
            None
        };
    Ok(CivicClaimPage {
        returned: if exact { raw.nodes.len() as u32 } else { 0 },
        total: raw.total_count,
        next,
    })
}
fn member(raw: &raw::Member) -> Result<CivicMember, RequestError> {
    let gene = raw.feature.name.as_str().parse::<GeneSymbol>().ok();
    let protein = raw.hgvs_descriptions.as_ref().is_some_and(|forms| {
        forms.iter().any(|form| {
            form.as_str().parse::<HgvsText>().is_ok()
                && form
                    .as_str()
                    .split_once(':')
                    .is_some_and(|(_, body)| body.starts_with("p."))
        })
    });
    let subject = if protein {
        gene.map(|gene| VariantRef::protein(gene, raw.name.clone()))
    } else {
        None
    };
    Ok(CivicMember {
        record: record(raw.id, &raw.link)?,
        label: raw.name.clone(),
        feature: raw.feature.name.clone(),
        subject,
    })
}
pub(super) fn record(id: u64, link: &Text) -> Result<SourceRecord, RequestError> {
    Ok(SourceRecord {
        id: Text::new(&id.to_string()).map_err(|_| failure(Reason::BadReply))?,
        url: Some(
            Text::new(&format!("https://civicdb.org{}", link.as_str()))
                .map_err(|_| failure(Reason::BadReply))?,
        ),
    })
}
