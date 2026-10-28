use super::raw::Wrapped;
use crate::{
    Mapped,
    mapped::{Problems, record},
    raw::Field,
};
use biodata::{
    OrcidAssertionSource, OrcidClientId, OrcidExternalId, OrcidId, OrcidPublicVisibility,
    OrcidWorkAssertion, OrcidWorkGroup, PartialDate, Text,
};
use biomcp_fetch::{BadReply, Reply};
use biomcp_types::Source;
use serde::Deserialize;
use std::num::NonZeroU64;
#[derive(Deserialize)]
pub(super) struct Works {
    pub path: String,
    group: Vec<Group>,
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Group {
    external_ids: Identifiers,
    work_summary: Vec<Assertion>,
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Identifiers {
    external_id: Vec<ExternalId>,
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ExternalId {
    external_id_type: Text,
    external_id_value: Text,
    external_id_relationship: Text,
    external_id_url: Option<Field<Url>>,
}
#[derive(Deserialize)]
struct Url {
    value: Text,
}
impl Identifiers {
    fn convert(self, problems: &mut Problems) -> Box<[OrcidExternalId]> {
        self.external_id
            .into_iter()
            .map(|id| OrcidExternalId {
                kind: id.external_id_type,
                value: id.external_id_value,
                relationship: id.external_id_relationship,
                url: optional(problems, "external_ids.url", id.external_id_url).map(|v| v.value),
            })
            .collect()
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Assertion {
    path: String,
    put_code: NonZeroU64,
    visibility: Option<String>,
    title: Option<Field<Title>>,
    #[serde(rename = "type")]
    work_type: Text,
    publication_date: Option<Field<Date>>,
    journal_title: Option<Field<Wrapped>>,
    url: Option<Field<Url>>,
    external_ids: Identifiers,
    source: Origin,
    display_index: Option<Field<Text>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Title {
    title: Option<Field<Wrapped>>,
    subtitle: Option<Field<Wrapped>>,
    translated_title: Option<Field<TranslatedTitle>>,
}
#[derive(Deserialize)]
struct TranslatedTitle {
    content: Text,
}
#[derive(Deserialize)]
struct Date {
    year: Wrapped,
    month: Option<Wrapped>,
    day: Option<Wrapped>,
}
impl Date {
    fn convert(&self) -> Result<PartialDate, BadReply> {
        PartialDate::new(
            self.year.value.as_str().parse().map_err(|_| BadReply)?,
            self.month
                .as_ref()
                .map(|v| v.value.as_str().parse())
                .transpose()
                .map_err(|_| BadReply)?,
            self.day
                .as_ref()
                .map(|v| v.value.as_str().parse())
                .transpose()
                .map_err(|_| BadReply)?,
        )
        .map_err(|_| BadReply)
    }
}
#[derive(Deserialize)]
struct Identity<T> {
    path: T,
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Origin {
    source_orcid: Option<Field<Identity<OrcidId>>>,
    source_client_id: Option<Field<Identity<OrcidClientId>>>,
    source_name: Option<Field<Wrapped>>,
    assertion_origin_orcid: Option<Field<Identity<OrcidId>>>,
    assertion_origin_client_id: Option<Field<Identity<OrcidClientId>>>,
    assertion_origin_name: Option<Field<Wrapped>>,
}
impl Origin {
    fn convert(self, problems: &mut Problems) -> OrcidAssertionSource {
        OrcidAssertionSource {
            source_orcid: optional(problems, "source.source_orcid", self.source_orcid)
                .map(|v| v.path),
            source_client_id: optional(problems, "source.source_client_id", self.source_client_id)
                .map(|v| v.path),
            source_name: optional(problems, "source.source_name", self.source_name)
                .map(|v| v.value),
            assertion_origin_orcid: optional(
                problems,
                "source.assertion_origin_orcid",
                self.assertion_origin_orcid,
            )
            .map(|v| v.path),
            assertion_origin_client_id: optional(
                problems,
                "source.assertion_origin_client_id",
                self.assertion_origin_client_id,
            )
            .map(|v| v.path),
            assertion_origin_name: optional(
                problems,
                "source.assertion_origin_name",
                self.assertion_origin_name,
            )
            .map(|v| v.value),
        }
    }
}
impl Assertion {
    fn convert(self, problems: &mut Problems) -> OrcidWorkAssertion {
        let (title, subtitle, translated_title) = optional(problems, "title", self.title)
            .map(|v| {
                (
                    optional(problems, "title", v.title).map(|v| v.value),
                    optional(problems, "subtitle", v.subtitle).map(|v| v.value),
                    optional(problems, "translated_title", v.translated_title).map(|v| v.content),
                )
            })
            .unwrap_or_default();
        OrcidWorkAssertion {
            put_code: self.put_code,
            title,
            subtitle,
            translated_title,
            work_type: self.work_type,
            publication_date: optional(problems, "publication_date", self.publication_date)
                .and_then(|v| problems.optional("publication_date", Some(v.convert()))),
            journal_title: optional(problems, "journal_title", self.journal_title).map(|v| v.value),
            url: optional(problems, "url", self.url).map(|v| v.value),
            external_ids: self.external_ids.convert(problems),
            source: self.source.convert(problems),
            visibility: OrcidPublicVisibility::Public,
            display_index: optional(problems, "display_index", self.display_index),
        }
    }
}
impl Works {
    pub(super) fn identity(&self, id: &OrcidId) -> bool {
        self.path == format!("/{id}/works")
            && self
                .group
                .iter()
                .flat_map(|g| &g.work_summary)
                .all(|s| s.path == format!("/{id}/work/{}", s.put_code))
    }
    pub(super) fn groups(self) -> Mapped<Box<[OrcidWorkGroup]>> {
        let mut problems = Problems {
            source: Source::Orcid,
            record: record(&self.path),
            list: Vec::new(),
        };
        let value = self
            .group
            .into_iter()
            .filter_map(|g| {
                let assertions: Box<[_]> = g
                    .work_summary
                    .into_iter()
                    .filter(|s| s.visibility.as_deref() == Some("public"))
                    .map(|s| {
                        problems.record = record(&s.path);
                        s.convert(&mut problems)
                    })
                    .collect();
                if assertions.is_empty() {
                    None
                } else {
                    problems.record = record(&self.path);
                    let external_ids = g.external_ids.convert(&mut problems);
                    Some(OrcidWorkGroup {
                        external_ids,
                        assertions,
                    })
                }
            })
            .collect();
        Mapped {
            value,
            problems: problems.list,
        }
    }
}
impl Reply for Works {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
fn optional<T>(problems: &mut Problems, field: &str, value: Option<Field<T>>) -> Option<T> {
    problems.required(field, value?.value())
}
