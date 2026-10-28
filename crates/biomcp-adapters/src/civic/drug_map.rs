use super::{drug::Therapy, drug_raw as raw, variant::failure, variant_claims, variant_map};
use biodata::{Text, VariantRef};
use biomcp_fetch::RequestError;
use biomcp_types::{
    DrugCivicClaim, DrugCivicMember, DrugCivicProfile, DrugHeader, Problem, Reason,
};

fn context(raw: raw::Profile) -> Result<(DrugCivicProfile, Option<VariantRef>), RequestError> {
    let record = variant_map::record(raw.id, &raw.link)?;
    let members = raw
        .variants
        .iter()
        .map(|row| {
            Ok::<_, RequestError>(DrugCivicMember {
                member: variant_map::member(&row.variant)?,
                reference_build: row
                    .coordinates
                    .as_ref()
                    .and_then(|c| c.reference_build.clone()),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let subject = if members.iter().any(|member| member.member.subject.is_none()) {
        None
    } else if raw.is_complex || members.len() != 1 {
        VariantRef::profile(
            record.id.clone(),
            raw.name.clone(),
            members
                .iter()
                .filter_map(|m| m.member.subject.clone())
                .collect(),
        )
        .ok()
    } else {
        members.first().and_then(|m| m.member.subject.clone())
    };
    Ok((
        DrugCivicProfile {
            record,
            label: raw.name,
            compound: raw.is_complex,
            members: members.into(),
        },
        subject,
    ))
}
fn therapies(raw: &[super::variant_raw::Therapy]) -> Result<Box<[DrugHeader]>, RequestError> {
    raw.iter()
        .map(|row| {
            let id = row.id.ok_or_else(|| failure(Reason::BadReply))?;
            Therapy {
                id,
                name: row.name.clone(),
                link: Text::new(&format!("/therapies/{id}"))
                    .map_err(|_| failure(Reason::BadReply))?,
            }
            .header()
        })
        .collect()
}
pub(super) fn evidence(
    raw: raw::Evidence,
    problems: &mut Vec<Problem>,
) -> Result<DrugCivicClaim, RequestError> {
    let (profile, subject) = context(raw.molecular_profile)?;
    let claim = variant_claims::evidence(
        &raw.claim,
        subject.as_ref().filter(|_| !raw.claim.status.is_unknown()),
        problems,
    )?;
    Ok(DrugCivicClaim {
        profile,
        claim,
        therapies: therapies(&raw.claim.therapies)?,
        therapy_interaction_type: raw.therapy_interaction_type,
        oncokb_missing: Box::default(),
        oncokb_lookup: None,
    })
}
pub(super) fn assertion(
    raw: raw::Assertion,
    problems: &mut Vec<Problem>,
) -> Result<DrugCivicClaim, RequestError> {
    let (profile, subject) = context(raw.molecular_profile)?;
    let claim = variant_claims::assertion(
        &raw.claim,
        subject.as_ref().filter(|_| !raw.claim.status.is_unknown()),
        problems,
    )?;
    Ok(DrugCivicClaim {
        profile,
        claim,
        therapies: therapies(&raw.claim.therapies)?,
        therapy_interaction_type: raw.therapy_interaction_type,
        oncokb_missing: Box::default(),
        oncokb_lookup: None,
    })
}
