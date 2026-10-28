use super::{Complex, bad, raw};
use biodata::{ProteinComplexHeader, ProteinComplexMember, UniProtIsoform};
use biomcp_fetch::RequestError;
pub(super) fn members(
    rows: Vec<raw::Member>,
    problems: &mut crate::mapped::Problems,
) -> Result<Vec<ProteinComplexMember>, RequestError> {
    rows.into_iter()
        .map(|r| {
            let protein = if r.interactor_type == "protein" {
                problems.required(
                    "participants.protein",
                    UniProtIsoform::new(&r.identifier)
                        .ok()
                        .filter(|p| p.as_str() == r.identifier),
                )
            } else {
                None
            };
            let mut xrefs = Vec::new();
            if let Some(field) = r.xrefs.or(r.cross_references) {
                if let Some(values) = problems.required("participants.xrefs", field.value()) {
                    for value in values {
                        let parsed = value
                            .value()
                            .and_then(|x| format!("{}:{}", x.database, x.identifier).parse().ok());
                        if let Some(value) = problems.required("participants.xrefs", parsed) {
                            xrefs.push(value);
                        }
                    }
                }
            }
            let organism = crate::mapped::field_text(
                &mut problems.list,
                problems.source,
                problems.record.clone(),
                "participants.organismName",
                r.organism_name,
            );
            Ok(ProteinComplexMember {
                id: crate::mapped::text(&r.identifier).map_err(|_| bad())?,
                participant_type: crate::mapped::text(&r.interactor_type).map_err(|_| bad())?,
                taxon: organism.as_ref().and_then(|s| raw::taxon(s.as_str())),
                organism,
                name: crate::mapped::field_text(
                    &mut problems.list,
                    problems.source,
                    problems.record.clone(),
                    "participants.name",
                    r.name,
                ),
                stoichiometry: crate::mapped::field_text(
                    &mut problems.list,
                    problems.source,
                    problems.record.clone(),
                    "participants.stochiometry",
                    r.stochiometry,
                ),
                xrefs: xrefs.into(),
                protein,
            })
        })
        .collect()
}
pub(super) fn complex(
    raw: raw::Complex,
    anchor: &UniProtIsoform,
) -> Result<Option<Complex>, RequestError> {
    let rows = match raw.interactors {
        Some(rows) => rows,
        None => return Ok(None),
    };
    let mut problems = crate::mapped::Problems {
        source: biomcp_types::Source::ComplexPortal,
        record: crate::mapped::record(&raw.id),
        list: Vec::new(),
    };
    let members = members(rows, &mut problems)?;
    if !members.iter().any(|m| m.protein.as_ref() == Some(anchor)) {
        return Ok(None);
    }
    let header = ProteinComplexHeader {
        id: raw.id.clone(),
        name: crate::mapped::field_text(
            &mut problems.list,
            problems.source,
            problems.record.clone(),
            "name",
            raw.name,
        ),
        taxon: 9606,
        predicted: raw.predicted_complex,
        participant_total: members.len() as u64,
        matched_participant: crate::mapped::text(anchor.as_str()).map_err(|_| bad())?,
        url: crate::mapped::text(&format!(
            "https://www.ebi.ac.uk/complexportal/complex/{}",
            raw.id
        ))
        .map_err(|_| bad())?,
    };
    Ok(Some(Complex {
        header,
        members,
        problems: problems.list,
    }))
}
