//! One record-owned license decision for article snippets and retained pools.
pub(crate) mod pubmed;
pub(crate) mod semantic;
use biodata::Pmid;
use biomcp_fetch::{BadReply, Leg};
use biomcp_types::{ArticleItem, Use};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

/// At most twenty record lookups and three seconds per text-bearing response.
const MAX_RECORDS: usize = 20;
const LOOKUP_BUDGET: Duration = Duration::from_secs(3);
const FINISH_BUDGET: Duration = Duration::from_millis(200);

/// Resolve explicit PMC commercial licenses; unavailable metadata grants no rights.
pub fn licenses(leg: &Leg<'_>, ids: &[Pmid]) -> BTreeMap<Pmid, biodata::License> {
    let mut seen = BTreeSet::new();
    let ids: Vec<_> = ids
        .iter()
        .copied()
        .filter(|id| seen.insert(*id))
        .take(MAX_RECORDS)
        .collect();
    if ids.is_empty() {
        return BTreeMap::new();
    }
    let deadline = leg.local_deadline();
    let mut remaining = LOOKUP_BUDGET.min(leg.remaining().saturating_sub(FINISH_BUDGET));
    if remaining.is_zero() {
        return BTreeMap::new();
    }
    let before = leg.remaining();
    let headers = leg.dependency(&crate::pubmed::SPEC, remaining, |dependency| {
        crate::pubmed::summaries(dependency, &ids, None)
    });
    remaining = remaining.saturating_sub(before.saturating_sub(leg.remaining()));
    let mut allowed = BTreeMap::new();
    if let Ok(headers) = headers {
        for header in headers.value {
            let (Some(id), Some(pmcid)) = (header.pmid, header.pmcid) else {
                continue;
            };
            if remaining.is_zero() || deadline.expired() {
                break;
            }
            let before = leg.remaining();
            let license = leg.dependency(&crate::pmc_oa::SPEC, remaining, |dependency| {
                crate::pmc_oa::record_license(dependency, pmcid, Some(id))
            });
            remaining = remaining.saturating_sub(before.saturating_sub(leg.remaining()));
            if let Some(license) = license {
                allowed.insert(id, license);
            }
        }
    }
    allowed
}

/// Filter LitSense's original bytes before either raw or mapped persistence.
pub(crate) fn litsense(leg: &Leg<'_>, body: &[u8]) -> Result<Vec<u8>, BadReply> {
    if !leg.disabled_use(Use::PerRecord) {
        return Ok(body.to_vec());
    }
    let mut rows: Value = serde_json::from_slice(body).map_err(|_| BadReply)?;
    let rows_array = rows.as_array_mut().ok_or(BadReply)?;
    let ids: Vec<_> = rows_array
        .iter()
        .filter_map(|row| pmid(&row["pmid"]))
        .collect();
    let allowed = licenses(leg, &ids);
    litsense_body(rows, &allowed)
}
fn litsense_body(
    mut rows: Value,
    allowed: &BTreeMap<Pmid, biodata::License>,
) -> Result<Vec<u8>, BadReply> {
    for row in rows.as_array_mut().ok_or(BadReply)? {
        let license = pmid(&row["pmid"]).and_then(|id| allowed.get(&id).copied());
        if let Some(row) = row.as_object_mut() {
            row.remove("license");
            if let Some(license) = license {
                row.insert(
                    "license".into(),
                    serde_json::to_value(license).map_err(|_| BadReply)?,
                );
            }
        }
        let abstract_text = row["section"].as_str().is_some_and(is_abstract);
        if !permitted(license, abstract_text)
            && let Some(row) = row.as_object_mut()
        {
            row.remove("text");
            row.remove("section");
            row.insert("text_withheld".into(), Value::Bool(true));
        }
    }
    serde_json::to_vec(&rows).map_err(|_| BadReply)
}

/// Stored article pools repeat the current rule before returning or rewriting rows.
pub fn items(leg: &Leg<'_>, rows: &mut [ArticleItem]) {
    if !leg.disabled_use(Use::PerRecord) {
        return;
    }
    let ids: Vec<_> = rows
        .iter()
        .filter(|row| {
            row.sentence
                .as_ref()
                .is_some_and(|sentence| sentence.license.is_none())
        })
        .filter_map(|row| row.pmid)
        .collect();
    let allowed = if ids.is_empty() {
        BTreeMap::new()
    } else {
        licenses(leg, &ids)
    };
    for row in rows {
        if let Some(sentence) = &mut row.sentence {
            sentence.license = sentence
                .license
                .or_else(|| row.pmid.and_then(|id| allowed.get(&id).copied()));
        }
        scrub_item(row);
    }
}
/// Apply the same per-record rule to a checked stored item without provider access.
pub fn scrub_item(row: &mut ArticleItem) -> bool {
    let license = row.sentence.as_ref().and_then(|sentence| sentence.license);
    let abstract_text = row
        .sentence
        .as_ref()
        .and_then(|sentence| sentence.section.as_ref())
        .is_some_and(|section| is_abstract(section.as_str()));
    let sentence = row.sentence.is_some() && !permitted(license, abstract_text);
    let contexts = row.contexts.is_some() && !permitted(row.context_license, false);
    if sentence {
        row.sentence = None;
    }
    if contexts {
        row.contexts = None;
    }
    if sentence || contexts {
        row.text_withheld = row.pmid.map(withheld);
    }
    sentence || contexts
}

pub(crate) fn withheld(id: Pmid) -> biomcp_types::WithheldArticleText {
    biomcp_types::WithheldArticleText {
        reason: biomcp_types::Reason::Disabled,
        url: biodata::Text::new(&format!("https://pubmed.ncbi.nlm.nih.gov/{id}/")).ok(),
    }
}
fn pmid(value: &Value) -> Option<Pmid> {
    value
        .as_str()
        .map(str::to_owned)
        .or_else(|| value.as_u64().map(|id| id.to_string()))?
        .parse()
        .ok()
}

/// Abstracts retain their source policy when no explicit record license exists.
pub(crate) fn permitted(license: Option<biodata::License>, abstract_text: bool) -> bool {
    if abstract_text {
        !matches!(
            license,
            Some(
                biodata::License::CcByNc
                    | biodata::License::CcByNcSa
                    | biodata::License::CcByNcNd
                    | biodata::License::Tdm
            )
        )
    } else {
        license.is_some_and(crate::pmc_oa::commercial)
    }
}
fn is_abstract(section: &str) -> bool {
    matches!(section.to_ascii_lowercase().as_str(), "title" | "abstract")
}

/// Strip publisher passages, including fields ignored by the annotation mapper.
pub(crate) fn pubtator(leg: &Leg<'_>, body: &[u8]) -> Result<Vec<u8>, BadReply> {
    if !leg.disabled_use(Use::PerRecord) {
        return Ok(body.to_vec());
    }
    let mut root: Value = serde_json::from_slice(body).map_err(|_| BadReply)?;
    let documents = root["PubTator3"].as_array_mut().ok_or(BadReply)?;
    let ids: Vec<_> = documents
        .iter()
        .filter_map(|row| pmid(&row["id"]))
        .collect();
    let licenses = licenses(leg, &ids);
    pubtator_body(root, &licenses)
}
fn pubtator_body(
    mut root: Value,
    licenses: &BTreeMap<Pmid, biodata::License>,
) -> Result<Vec<u8>, BadReply> {
    for document in root["PubTator3"].as_array_mut().ok_or(BadReply)? {
        let license = pmid(&document["id"]).and_then(|id| licenses.get(&id).copied());
        if let Some(passages) = document["passages"].as_array_mut() {
            for passage in passages {
                let abstract_text = passage
                    .pointer("/infons/type")
                    .and_then(Value::as_str)
                    .is_some_and(is_abstract);
                if !permitted(license, abstract_text)
                    && let Some(passage) = passage.as_object_mut()
                {
                    passage.remove("text");
                    passage.remove("sentences");
                    // Annotation mentions are copied fragments of the same passage.
                    if let Some(annotations) =
                        passage.get_mut("annotations").and_then(Value::as_array_mut)
                    {
                        for annotation in annotations {
                            if let Some(annotation) = annotation.as_object_mut() {
                                annotation.remove("text");
                            }
                        }
                    }
                }
            }
        }
    }
    serde_json::to_vec(&root).map_err(|_| BadReply)
}
/// The core record itself supplies Europe PMC's explicit abstract license.
pub(crate) fn europepmc(leg: &Leg<'_>, body: &[u8]) -> Result<Vec<u8>, BadReply> {
    if !leg.disabled_use(Use::PerRecord) {
        return Ok(body.to_vec());
    }
    europepmc_body(body)
}
fn europepmc_body(body: &[u8]) -> Result<Vec<u8>, BadReply> {
    let mut root: Value = serde_json::from_slice(body).map_err(|_| BadReply)?;
    let record = &mut root["result"];
    let license = record["license"].as_str().map(crate::pmc_oa::license);
    if !permitted(license, true)
        && let Some(record) = record.as_object_mut()
    {
        record.remove("abstractText");
    }
    serde_json::to_vec(&root).map_err(|_| BadReply)
}

/// Search rows can contain provider text omitted by the public mapping.
pub(crate) fn pubtator_search(leg: &Leg<'_>, body: &[u8]) -> Result<Vec<u8>, BadReply> {
    if !leg.disabled_use(Use::PerRecord) {
        return Ok(body.to_vec());
    }
    pubtator_search_body(body)
}
fn pubtator_search_body(body: &[u8]) -> Result<Vec<u8>, BadReply> {
    let mut root: Value = serde_json::from_slice(body).map_err(|_| BadReply)?;
    if let Some(rows) = root["results"].as_array_mut() {
        for row in rows {
            if let Some(row) = row.as_object_mut() {
                row.retain(|key, _| key == "pmid");
            }
        }
    }
    serde_json::to_vec(&root).map_err(|_| BadReply)
}

/// Repeat abstract rights when reading a source-native author continuation.
pub fn snapshot(leg: &Leg<'_>, body: &[u8]) -> Result<Vec<u8>, BadReply> {
    semantic::filter(leg, body, None)
}
