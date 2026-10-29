use super::{drug::Therapy, variant_raw as raw};
use biodata::Text;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct ReplyBody {
    pub data: Data,
    pub errors: Option<Vec<serde::de::IgnoredAny>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Data {
    pub therapy: Option<Therapy>,
    pub evidence_items: Option<raw::Connection<Evidence>>,
    pub assertions: Option<raw::Connection<Assertion>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Profile {
    pub id: u64,
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    pub name: biodata::Text,
    pub link: Text,
    pub is_complex: bool,
    pub variants: Vec<Member>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Evidence {
    #[serde(flatten)]
    pub claim: raw::Evidence,
    pub molecular_profile: Profile,
    pub therapy_interaction_type: Option<Text>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Assertion {
    #[serde(flatten)]
    pub claim: raw::Assertion,
    pub molecular_profile: Profile,
    pub therapy_interaction_type: Option<Text>,
}
impl Reply for ReplyBody {
    fn check(&self) -> Result<(), BadReply> {
        if self.errors.as_ref().is_some_and(|rows| !rows.is_empty()) {
            return Err(BadReply);
        }
        if let Some(therapy) = &self.data.therapy {
            therapy.check()?;
        }
        if let Some(rows) = &self.data.evidence_items {
            rows.check()?;
            for row in &rows.nodes {
                check(row.claim.id, &row.molecular_profile)?;
            }
        }
        if let Some(rows) = &self.data.assertions {
            rows.check()?;
            for row in &rows.nodes {
                check(row.claim.id, &row.molecular_profile)?;
            }
        }
        Ok(())
    }
}
fn check(id: u64, profile: &Profile) -> Result<(), BadReply> {
    if id == 0
        || id > i32::MAX as u64
        || profile.id == 0
        || profile.id > i32::MAX as u64
        || profile.variants.is_empty()
        || profile.link.as_str() != format!("/molecular-profiles/{}", profile.id)
        || profile
            .variants
            .iter()
            .any(|v| v.variant.id == 0 || v.variant.id > i32::MAX as u64)
    {
        return Err(BadReply);
    }
    Ok(())
}

#[derive(Deserialize)]
pub(super) struct Member {
    #[serde(flatten)]
    pub variant: raw::Member,
    pub coordinates: Option<Coordinates>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Coordinates {
    pub reference_build: Option<Text>,
}
