use biodata::{ComplexPortalId, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Search {
    pub total_number_of_results: u64,
    pub elements: Vec<Complex>,
}
impl Reply for Search {
    fn check(&self) -> Result<(), BadReply> {
        if self.elements.len() as u64 > self.total_number_of_results || self.elements.len() > 25 {
            return Err(BadReply);
        }
        if self.elements.iter().any(|c| {
            !taxon(&c.organism_name).is_ok_and(|value| value == Some(9606))
                || c.interactors
                    .as_ref()
                    .is_some_and(|rows| rows.iter().any(|r| !r.valid()))
        }) {
            return Err(BadReply);
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Complex {
    #[serde(rename = "complexAC")]
    pub id: ComplexPortalId,
    #[serde(rename = "complexName")]
    pub name: Option<crate::raw::Field<String>>,
    pub organism_name: String,
    pub predicted_complex: Option<crate::raw::Field<bool>>,
    pub interactors: Option<Vec<Member>>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Detail {
    #[serde(rename = "complexAc")]
    pub id: ComplexPortalId,
    pub name: Option<crate::raw::Field<String>>,
    pub species: String,
    pub predicted_complex: Option<crate::raw::Field<bool>>,
    pub participants: Vec<Member>,
}
impl Reply for Detail {
    fn check(&self) -> Result<(), BadReply> {
        if taxon(&self.species).is_ok_and(|value| value == Some(9606))
            && self.participants.iter().all(Member::valid)
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Member {
    pub identifier: String,
    pub interactor_type: String,
    pub organism_name: Option<crate::raw::Field<String>>,
    pub name: Option<crate::raw::Field<String>>,
    pub stochiometry: Option<crate::raw::Field<String>>,
    pub xrefs: Option<crate::raw::Field<Vec<crate::raw::Field<Xref>>>>,
    pub cross_references: Option<crate::raw::Field<Vec<crate::raw::Field<Xref>>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Xref {
    pub database: String,
    pub identifier: String,
}
pub(super) fn taxon(s: &str) -> Result<Option<u32>, BadReply> {
    let suffix = s.rsplit_once(';').ok_or(BadReply)?.1.trim();
    let value: i64 = suffix.parse().map_err(|_| BadReply)?;
    if value == -2 {
        return Ok(None);
    }
    let taxon = u32::try_from(value).map_err(|_| BadReply)?;
    if taxon == 0 {
        return Err(BadReply);
    }
    Ok(Some(taxon))
}

impl Member {
    fn valid(&self) -> bool {
        Text::new(&self.identifier).is_ok_and(|id| id.as_str() == self.identifier)
    }
}
