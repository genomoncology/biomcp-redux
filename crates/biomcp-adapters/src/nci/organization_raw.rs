//! Complete native organization observations reject missing totals or cut collections.
use crate::raw::Field;
use biodata::Text;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Page {
    pub total: u64,
    pub data: Vec<Organization>,
}
impl Reply for Page {
    fn check(&self) -> Result<(), BadReply> {
        if self.total != self.data.len() as u64 { return Err(BadReply); }
        Ok(())
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Organization {
    pub name: Text,
    pub count: u64,
    pub org_city: Option<Field<Text>>,
    pub org_state_or_province: Option<Field<Text>>,
    pub org_country: Option<Field<Text>>,
    pub org_address_line_1: Option<Field<Text>>,
    pub org_address_line_2: Option<Field<Text>>,
    pub org_postal_code: Option<Field<Text>>,
    pub org_family: Option<Field<Text>>,
    pub org_email: Option<Field<Text>>,
    pub org_fax: Option<Field<Text>>,
    pub contact_name: Option<Field<Text>>,
    pub contact_phone: Option<Field<Text>>,
    pub contact_email: Option<Field<Text>>,
    pub org_coordinates: Option<Field<biomcp_types::OrganizationCoordinates>>,
    pub org_va: Option<Field<bool>>,
}
