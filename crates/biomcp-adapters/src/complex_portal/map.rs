use super::{Complex, bad, raw};
use biodata::{ProteinComplexHeader, ProteinComplexMember, UniProtIsoform};
use biomcp_fetch::RequestError;
pub(super) fn members(rows: Vec<raw::Member>) -> Result<Vec<ProteinComplexMember>, RequestError> {
    rows.into_iter()
        .map(|r| {
            let protein = if r.interactor_type == "protein" {
                UniProtIsoform::new(r.identifier.as_str())
                    .ok()
                    .filter(|p| p.as_str() == r.identifier.as_str())
            } else {
                None
            };
            let xrefs = r
                .xrefs
                .or(r.cross_references)
                .into_iter()
                .flatten()
                .map(|x| {
                    format!("{}:{}", x.database, x.identifier)
                        .parse()
                        .map_err(|_| bad())
                })
                .collect::<Result<Box<[_]>, RequestError>>()?;
            Ok(ProteinComplexMember {
                id: crate::mapped::text(&r.identifier).map_err(|_| bad())?,
                participant_type: crate::mapped::text(&r.interactor_type).map_err(|_| bad())?,
                taxon: r.organism_name.as_deref().and_then(raw::taxon),
                organism: r
                    .organism_name
                    .as_deref()
                    .map(|v| crate::mapped::text(v).map_err(|_| bad()))
                    .transpose()?,
                name: r.name,
                stoichiometry: r.stochiometry,
                xrefs,
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
    let members = members(rows)?;
    if !members.iter().any(|m| m.protein.as_ref() == Some(anchor)) {
        return Ok(None);
    }
    let header = ProteinComplexHeader {
        id: raw.id.clone(),
        name: raw.name,
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
    Ok(Some(Complex { header, members }))
}
