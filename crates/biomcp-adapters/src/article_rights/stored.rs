//! Offline rights collection and selective filtering for a copied existing store.
use super::{permitted, pmid};
use biodata::{License, Pmcid, Pmid};
use biomcp_fetch::BadReply;
use biomcp_types::{ArticleItem, Source};
use serde_json::Value;
use std::collections::BTreeMap;

/// Rights already held in source metadata; never contacts a provider.
#[derive(Default)]
pub struct StoredRights {
    licenses: BTreeMap<Pmid, License>,
    pmc: BTreeMap<Pmcid, License>,
    versions: BTreeMap<Pmcid, u32>,
    identities: BTreeMap<Pmid, Pmcid>,
    preprints: BTreeMap<String, License>,
}
impl StoredRights {
    /// Collect record-owned license metadata before any text is changed.
    pub fn observe(&mut self, source: Source, url: &str, body: &[u8]) -> Result<(), BadReply> {
        let Ok(value) = serde_json::from_slice::<Value>(body) else {
            return Ok(());
        };
        match source {
            Source::PmcOa => {
                let license = value["license_code"].as_str().map(crate::pmc_oa::license);
                if let (Some(license), Some(id)) = (
                    license,
                    value["pmcid"]
                        .as_str()
                        .and_then(|id| id.parse::<Pmcid>().ok()),
                ) {
                    let version = url
                        .split(&format!("{id}."))
                        .nth(1)
                        .and_then(|rest| rest.split('/').next())
                        .and_then(|version| version.parse().ok())
                        .unwrap_or(0);
                    if self
                        .versions
                        .get(&id)
                        .is_none_or(|current| version >= *current)
                    {
                        self.versions.insert(id, version);
                        self.pmc.insert(id, license);
                    }
                    if let Some(pmid) = pmid(&value["pmid"]) {
                        self.identities.entry(pmid).or_insert(id);
                    }
                }
            }
            Source::PubMed => {
                if let Some(rows) = value["result"].as_object() {
                    for row in rows.values() {
                        if let (Some(id), Some(ids)) =
                            (pmid(&row["uid"]), row["articleids"].as_array())
                        {
                            for native in ids {
                                if native["idtype"] == "pmc" {
                                    if let Some(pmc) =
                                        native["value"].as_str().and_then(|id| id.parse().ok())
                                    {
                                        self.identities.insert(id, pmc);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Source::EuropePmc => {
                let row = &value["result"];
                if let (Some(id), Some(license)) = (row["id"].as_str(), row["license"].as_str()) {
                    self.preprints
                        .insert(id.into(), crate::pmc_oa::license(license));
                }
            }
            _ => {}
        }
        Ok(())
    }
    /// Join metadata identities once after collection, before filtering rows.
    pub fn finish(mut self) -> Self {
        for (id, pmc) in &self.identities {
            if let Some(license) = self.pmc.get(pmc) {
                self.licenses.entry(*id).or_insert(*license);
            }
        }
        self
    }
    /// Preserve every paper and its rank while dropping text without sufficient rights.
    pub fn scrub_item(&self, item: &mut ArticleItem) -> bool {
        if let Some(sentence) = &mut item.sentence {
            sentence.license = sentence
                .license
                .or_else(|| item.pmid.and_then(|id| self.licenses.get(&id).copied()));
        }
        super::scrub_item(item)
    }
    /// Filter retained native article and citation payloads, leaving trial shapes intact.
    pub fn scrub_snapshot(&self, body: &[u8]) -> Result<Vec<u8>, BadReply> {
        let mut value: Value = serde_json::from_slice(body).map_err(|_| BadReply)?;
        strip_fulltext(&mut value);
        super::semantic::filter_with_licenses(value, None, &self.licenses)
    }
    /// Retain raw reply identity and permitted fields; `None` removes a wholly withheld body.
    pub fn scrub_reply(
        &self,
        source: Source,
        url: &str,
        body: &[u8],
    ) -> Result<Option<Vec<u8>>, BadReply> {
        let licenses = &self.licenses;
        let result = match source {
            Source::LitSense => super::litsense_body(
                serde_json::from_slice(body).map_err(|_| BadReply)?,
                licenses,
            )?,
            Source::PubTator => {
                let value: Value = serde_json::from_slice(body).map_err(|_| BadReply)?;
                if value.get("PubTator3").is_some() {
                    super::pubtator_body(value, licenses)?
                } else {
                    super::pubtator_search_body(body)?
                }
            }
            Source::PubMed if body.starts_with(b"<") => {
                super::pubmed::filter_with_licenses(body, licenses)?
            }
            Source::SemanticScholar => {
                let decoded = url.replace("PMID%3A", "PMID:");
                let parent = decoded
                    .split("PMID:")
                    .nth(1)
                    .and_then(|rest| rest.split('/').next())
                    .and_then(|id| id.parse().ok());
                super::semantic::filter_with_licenses(
                    serde_json::from_slice(body).map_err(|_| BadReply)?,
                    parent,
                    licenses,
                )?
            }
            Source::EuropePmc
                if serde_json::from_slice::<Value>(body)
                    .ok()
                    .is_some_and(|value| value.get("result").is_some()) =>
            {
                super::europepmc_body(body)?
            }
            Source::PmcOa | Source::EuropePmc if is_document(source, url, body) => {
                let license = if source == Source::PmcOa {
                    native_id(url, "PMC")
                        .and_then(|id| id.parse::<Pmcid>().ok())
                        .and_then(|id| self.pmc.get(&id).copied())
                } else {
                    native_id(url, "PPR").and_then(|id| self.preprints.get(&id).copied())
                };
                if !permitted(license, false) {
                    return Ok(None);
                }
                body.to_vec()
            }
            _ => body.to_vec(),
        };
        Ok(Some(result))
    }
}
fn native_id(url: &str, prefix: &str) -> Option<String> {
    let suffix = url.split(prefix).nth(1)?;
    let digits: String = suffix.chars().take_while(char::is_ascii_digit).collect();
    (!digits.is_empty()).then(|| format!("{prefix}{digits}"))
}
fn is_document(source: Source, url: &str, body: &[u8]) -> bool {
    if source == Source::EuropePmc {
        return url.contains("fullTextXML");
    }
    url.contains("/PMC")
        && !url.ends_with(".json")
        && !body
            .windows(17)
            .any(|window| window == b"<ListBucketResult")
}

fn strip_fulltext(value: &mut Value) {
    match value {
        Value::Object(object) => {
            let body = object
                .get("data")
                .and_then(Value::as_object)
                .filter(|data| data.contains_key("paragraphs"));
            let license = body
                .and_then(|data| data.get("license"))
                .and_then(|license| serde_json::from_value(license.clone()).ok());
            let article = object
                .get("source")
                .and_then(Value::as_str)
                .is_some_and(|source| matches!(source, "pmc-oa" | "europepmc"));
            if article && body.is_some() && !permitted(license, false) {
                object.remove("data");
                object.insert("state".into(), Value::String("skipped".into()));
                object.insert("reason".into(), Value::String("disabled".into()));
            }
            for child in object.values_mut() {
                strip_fulltext(child);
            }
        }
        Value::Array(rows) => {
            for child in rows {
                strip_fulltext(child);
            }
        }
        _ => {}
    }
}
