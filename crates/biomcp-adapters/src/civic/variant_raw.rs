use biodata::Text;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Profiles {
    pub data: Data,
    pub errors: Option<Vec<serde::de::IgnoredAny>>,
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
    pub status: biodata::SourceText,
    pub evidence_type: biodata::SourceText,
    pub evidence_level: Text,
    pub evidence_direction: Text,
    pub significance: biodata::SourceText,
    pub disease: Option<Disease>,
    pub therapies: Vec<Therapy>,
    pub source: Citation,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Assertion {
    pub id: u64,
    pub status: biodata::SourceText,
    pub assertion_type: biodata::SourceText,
    pub assertion_direction: Text,
    pub significance: biodata::SourceText,
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
    pub id: Option<u64>,
    pub name: Text,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Citation {
    pub citation_id: Option<Text>,
    pub source_type: Text,
}
impl<T> Connection<T> {
    pub(super) fn progresses(&self, previous: Option<&Text>) -> bool {
        !self.page_info.has_next_page
            || !self.nodes.is_empty()
                && self
                    .page_info
                    .end_cursor
                    .as_ref()
                    .is_some_and(|cursor| Some(cursor) != previous)
    }
    pub(super) fn check(&self) -> Result<(), BadReply> {
        if self.nodes.len() as u64 > self.total_count
            || self.page_info.has_next_page
                && !self.nodes.is_empty()
                && self.page_info.end_cursor.is_none()
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
impl Profiles {
    pub(super) fn has_errors(&self) -> bool {
        self.errors
            .as_ref()
            .is_some_and(|errors| !errors.is_empty())
    }
    pub(super) fn progresses(
        &self,
        query: &biomcp_types::VariantGet,
        after: Option<&Text>,
    ) -> bool {
        use biomcp_types::CivicClaimKind;
        self.data.molecular_profiles.progresses(after)
            && self.data.molecular_profiles.nodes.iter().all(|profile| {
                (query.civic_kind() == Some(CivicClaimKind::Assertions)
                    || profile.evidence_items.progresses(query.civic_cursor()))
                    && (query.civic_kind() == Some(CivicClaimKind::Evidence)
                        || profile.assertions.progresses(query.civic_cursor()))
            })
    }
    pub(super) fn matches(&self, allele: &str, profile: Option<u64>) -> bool {
        self.data.molecular_profiles.nodes.iter().all(|p| {
            profile.is_none_or(|id| p.id == id)
                && p.variants.iter().any(|m| {
                    m.allele_registry_id
                        .as_ref()
                        .is_some_and(|id| id.as_str() == allele)
                })
        })
    }
}
impl Reply for Profiles {
    fn check(&self) -> Result<(), BadReply> {
        self.data.molecular_profiles.check()?;
        for profile in &self.data.molecular_profiles.nodes {
            if !id(profile.id)
                || profile.variants.is_empty()
                || !profile.link.as_str().starts_with("/molecular-profiles/")
                || profile.variants.iter().any(|v| !id(v.id))
                || profile.evidence_items.nodes.iter().any(|row| !id(row.id))
                || profile.assertions.nodes.iter().any(|row| !id(row.id))
            {
                return Err(BadReply);
            }
            profile.evidence_items.check()?;
            profile.assertions.check()?;
        }
        Ok(())
    }
}
fn id(value: u64) -> bool {
    value > 0 && value <= i32::MAX as u64
}
