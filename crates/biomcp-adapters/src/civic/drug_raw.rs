use super::{drug::Therapy, variant_raw as raw};
use biodata::Text;
use crate::{raw::Field, native_text::NativeText};
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
    pub id: Option<Field<u64>>,
    pub name: Option<Field<NativeText>>,
    pub raw_name: Option<Field<Text>>,
    pub link: Option<Field<Text>>,
    pub is_complex: Option<Field<bool>>,
    pub variants: Option<Field<Vec<Field<Member>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Evidence {
    #[serde(flatten)]
    pub claim: raw::Evidence,
    pub molecular_profile: Option<Field<Profile>>,
    pub therapy_interaction_type: Option<Field<Text>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Assertion {
    #[serde(flatten)]
    pub claim: raw::Assertion,
    pub molecular_profile: Option<Field<Profile>>,
    pub therapy_interaction_type: Option<Field<Text>>,
}
impl Reply for ReplyBody {
    #[derive(Deserialize)]
pub(super) struct Member {
    #[serde(flatten)]
    pub variant: raw::Member,
    pub coordinates: Option<Field<Coordinates>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Coordinates {
    pub reference_build: Option<Field<Text>>,
}
