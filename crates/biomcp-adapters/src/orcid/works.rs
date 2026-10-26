use super::raw::Wrapped;
use biodata::{OrcidAssertionSource, OrcidClientId, OrcidExternalId, OrcidId, OrcidPublicVisibility, OrcidWorkAssertion, OrcidWorkGroup, PartialDate, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
use std::num::NonZeroU64;
#[derive(Deserialize)]
pub(super) struct Works { pub path: String, group: Vec<Group> }
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Group { external_ids: Identifiers, work_summary: Vec<Assertion> }
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Identifiers { external_id: Vec<ExternalId> }
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ExternalId {
    external_id_type: Text,
    external_id_value: Text,
    external_id_relationship: Text,
    external_id_url: Option<Wrapped>,
}
impl Identifiers {
    fn convert(self) -> Box<[OrcidExternalId]> {
        self.external_id.into_iter().map(|id| OrcidExternalId { kind: id.external_id_type, value: id.external_id_value, relationship: id.external_id_relationship, url: id.external_id_url.map(|v| v.value) }).collect()
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Assertion {
    path: String,
    put_code: NonZeroU64,
    visibility: Option<String>,
    title: Option<Title>,
    #[serde(rename = "type")]
    work_type: Text,
    publication_date: Option<Date>,
    journal_title: Option<Wrapped>,
    url: Option<Wrapped>,
    external_ids: Identifiers,
    source: Origin,
    display_index: Option<Text>,
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Title { title: Option<Wrapped>, subtitle: Option<Wrapped>, translated_title: Option<Wrapped> }
#[derive(Deserialize)]
struct Date { year: Wrapped, month: Option<Wrapped>, day: Option<Wrapped> }
impl Date {
    fn convert(&self) -> Result<PartialDate, BadReply> {
        PartialDate::new(
            self.year.value.as_str().parse().map_err(|_| BadReply)?,
            self.month.as_ref().map(|v| v.value.as_str().parse()).transpose().map_err(|_| BadReply)?,
            self.day.as_ref().map(|v| v.value.as_str().parse()).transpose().map_err(|_| BadReply)?,
        ).map_err(|_| BadReply)
    }
}
#[derive(Deserialize)]
struct Identity<T> { path: T }
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Origin {
    source_orcid: Option<Identity<OrcidId>>,
    source_client_id: Option<Identity<OrcidClientId>>,
    source_name: Option<Wrapped>,
    assertion_origin_orcid: Option<Identity<OrcidId>>,
    assertion_origin_client_id: Option<Identity<OrcidClientId>>,
    assertion_origin_name: Option<Wrapped>,
}
impl Origin {
    fn convert(self) -> OrcidAssertionSource {
        OrcidAssertionSource {
            source_orcid: self.source_orcid.map(|v| v.path),
            source_client_id: self.source_client_id.map(|v| v.path),
            source_name: self.source_name.map(|v| v.value),
            assertion_origin_orcid: self.assertion_origin_orcid.map(|v| v.path),
            assertion_origin_client_id: self.assertion_origin_client_id.map(|v| v.path),
            assertion_origin_name: self.assertion_origin_name.map(|v| v.value),
        }
    }
}
impl Assertion {
    fn convert(self) -> Result<OrcidWorkAssertion, BadReply> {
        let (title, subtitle, translated_title) = self.title.map(|v| (v.title.map(|v| v.value), v.subtitle.map(|v| v.value), v.translated_title.map(|v| v.value))).unwrap_or_default();
        Ok(OrcidWorkAssertion {
            put_code: self.put_code, title, subtitle, translated_title,
            work_type: self.work_type,
            publication_date: self.publication_date.as_ref().map(Date::convert).transpose()?,
            journal_title: self.journal_title.map(|v| v.value), url: self.url.map(|v| v.value),
            external_ids: self.external_ids.convert(), source: self.source.convert(),
            visibility: OrcidPublicVisibility::Public, display_index: self.display_index,
        })
    }
}
impl Works {
    pub fn identity(&self, id: &OrcidId) -> bool {
        self.path == format!("/{id}/works") && self.group.iter().flat_map(|g| &g.work_summary).all(|s| s.path == format!("/{id}/work/{}", s.put_code))
    }
    pub fn groups(self) -> Result<Box<[OrcidWorkGroup]>, BadReply> {
        self.group.into_iter().filter_map(|g| {
            let assertions = g.work_summary.into_iter().filter(|s| s.visibility.as_deref() == Some("public")).map(Assertion::convert).collect::<Result<Box<[_]>, _>>();
            match assertions {
                Ok(assertions) if assertions.is_empty() => None,
                Ok(assertions) => Some(Ok(OrcidWorkGroup { external_ids: g.external_ids.convert(), assertions })),
                Err(e) => Some(Err(e)),
            }
        }).collect()
    }
}
impl Reply for Works {
    fn check(&self) -> Result<(), BadReply> {
        for s in self.group.iter().flat_map(|g| &g.work_summary).filter(|s| s.visibility.as_deref() == Some("public")) {
            if let Some(date) = &s.publication_date { date.convert()?; }
        }
        Ok(())
    }
}
