use biomcp_fetch::{KeyKind, Keys, Row};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Native {
    #[serde(rename = "WHO Reference Number")]
    reference: biodata::Text,
    #[serde(rename = "INN, Dosage Form and Strength")]
    inn: Box<str>,
    #[serde(rename = "Product Type")]
    product_type: Box<str>,
    #[serde(rename = "Therapeutic Area")]
    therapeutic_area: Box<str>,
    #[serde(rename = "Applicant")]
    applicant: Box<str>,
    #[serde(rename = "Dosage Form")]
    dosage_form: Box<str>,
    #[serde(rename = "Basis of Listing")]
    basis_of_listing: Box<str>,
    #[serde(rename = "Date of Prequalification")]
    date: Box<str>,
}
impl Row for Native {
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![(KeyKind::Id, self.reference.to_string().into())],
            words: vec![self.inn.clone()],
        }
    }
}
impl Native {
    pub(super) fn matches(&self, query: &str) -> bool {
        let words = |s: &str| {
            s.split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .map(str::to_lowercase)
                .collect::<Vec<_>>()
        };
        let needle = words(query);
        let haystack = words(&self.inn);
        !needle.is_empty() && haystack.windows(needle.len()).any(|part| part == needle)
    }
    pub(super) fn reference(&self) -> &biodata::Text {
        &self.reference
    }
}
impl From<Native> for biomcp_types::DrugWhoRow {
    fn from(row: Native) -> Self {
        Self {
            reference: row.reference,
            inn_dosage_form_strength: row.inn,
            product_type: row.product_type,
            therapeutic_area: row.therapeutic_area,
            applicant: row.applicant,
            dosage_form: row.dosage_form,
            basis_of_listing: row.basis_of_listing,
            date_of_prequalification: row.date,
        }
    }
}
