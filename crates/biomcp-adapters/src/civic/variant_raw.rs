use biodata::Text;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Profiles {
    pub data: Data,
    pub errors: Option<serde::de::IgnoredAny>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Data {
    pub molecular_profiles: Connection<Profile>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Connection<T> {
    pub total_count: u64,
    pub nodes: Vec<T>,
    pub page_info: Page,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Page {
    pub has_next_page: bool,
    pub end_cursor: Option<Text>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Profile {
    pub id: u64,
    pub name: Text,
    pub link: Text,
    pub is_complex: bool,
    pub variants: Vec<Member>,
    pub evidence_items: Connection<Evidence>,
    pub assertions: Connection<Assertion>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Member {
    pub id: u64,
    pub name: Text,
    pub link: Text,
    pub allele_registry_id: Option<Text>,
    pub hgvs_descriptions: Option<Vec<Text>>,
    pub feature: Feature,
}
#[derive(Deserialize)]
pub(super) struct Feature {
    pub name: Text,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Evidence {
    pub id: u64,
    pub status: Text,
    pub evidence_type: Text,
    pub evidence_level: Text,
    pub evidence_direction: Text,
    pub significance: Text,
    pub disease: Option<Disease>,
    pub therapies: Vec<Therapy>,
    pub source: Citation,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Assertion {
    pub id: u64,
    pub status: Text,
    pub assertion_type: Text,
    pub assertion_direction: Text,
    pub significance: Text,
    pub amp_level: Option<Text>,
    pub disease: Option<Disease>,
    pub therapies: Vec<Therapy>,
    pub variant_origin: Text,
}
#[derive(Deserialize)]
pub(super) struct Disease {
    pub name: Text,
    pub doid: Option<Text>,
}
#[derive(Deserialize)]
pub(super) struct Therapy {
    pub name: Text,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Citation {
    pub citation_id: Option<Text>,
    pub source_type: Text,
}
impl<T> Connection<T> {
    fn check(&self) -> Result<(), BadReply> {
        if self.nodes.len() as u64 > self.total_count
            || self.page_info.has_next_page && self.page_info.end_cursor.is_none()
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
impl Reply for Profiles {
    fn check(&self) -> Result<(), BadReply> {
        if self.errors.is_some() {
            return Err(BadReply);
        }
        self.data.molecular_profiles.check()?;
        for profile in &self.data.molecular_profiles.nodes {
            if profile.id == 0 || profile.variants.is_empty()
                || !profile.link.as_str().starts_with("/molecular-profiles/")
                || profile.variants.iter().any(|v| v.id == 0)
            {
                return Err(BadReply);
            }
            profile.evidence_items.check()?;
            profile.assertions.check()?;
        }
        Ok(())
    }
}
