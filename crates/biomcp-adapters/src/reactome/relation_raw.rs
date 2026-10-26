use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
use std::num::NonZeroU64;
#[derive(Deserialize)]
pub(super) struct Participants(pub Vec<Participant>);
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Participant {
    pub pe_db_id: NonZeroU64,
    pub display_name: String,
    pub schema_class: String,
    pub ref_entities: Vec<Reference>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Reference {
    pub st_id: String,
    pub identifier: String,
    pub display_name: Option<String>,
    pub schema_class: String,
    pub url: Option<String>,
}
impl Reply for Participants {
    fn check(&self) -> Result<(), BadReply> {
        self.0
            .iter()
            .all(|p| {
                !p.display_name.trim().is_empty()
                    && !p.schema_class.trim().is_empty()
                    && p.ref_entities.iter().all(|r| {
                        r.st_id.split_once(':').is_some_and(|(namespace, id)| {
                            !namespace.is_empty() && id == r.identifier && !id.trim().is_empty()
                        }) && !r.schema_class.trim().is_empty()
                    })
            })
            .then_some(())
            .ok_or(BadReply)
    }
}
#[derive(Deserialize)]
pub(super) struct Events(pub Vec<Event>);
#[derive(Deserialize)]
#[serde(untagged)]
pub(super) enum Event {
    Object(Object),
    Database(NonZeroU64),
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Object {
    pub db_id: NonZeroU64,
    pub st_id: Option<String>,
    pub st_id_version: Option<String>,
    pub display_name: Option<String>,
    pub schema_class: Option<String>,
    pub species_name: Option<String>,
}
impl Reply for Events {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
