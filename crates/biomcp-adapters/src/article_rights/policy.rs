//! One record-owned license decision for article snippets and retained pools.
use super::semantic;
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
            // Cache only the native fields this route actually consumes.
            row.retain(|key, _| matches!(key.as_str(), "pmid" | "section" | "text"));
            if let Some(license) = license {
                row.insert(
                    "biomcp_license".into(),
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
            row.insert("biomcp_text_withheld".into(), Value::Bool(true));
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
        filter_item(row);
    }
}
/// Apply the same per-record rule to a checked stored item without provider access.
fn filter_item(row: &mut ArticleItem) -> bool {
    let license = row.sentence.as_ref().and_then(|sentence| sentence.license);
    let abstract_text = row
        .sentence
        .as_ref()
        .and_then(|sentence| sentence.section.as_ref())
        .is_some_and(|section| is_abstract(section.as_str()));
    let sentence = row.sentence.is_some() && !permitted(license, abstract_text);
    let contexts = row
        .contexts
        .as_ref()
        .is_some_and(|contexts| !contexts.items.is_empty())
        && !permitted(row.context_license, false);
    if sentence {
        row.sentence = None;
    }
    if contexts {
        row.contexts = None;
    }
    if sentence || contexts {
        row.text_withheld = Some(withheld_item(row));
    }
    sentence || contexts
}

pub(crate) fn withheld_item(row: &ArticleItem) -> biomcp_types::WithheldArticleText {
    use biomcp_types::ArticleKey;
    if let Some(id) = row.pmid {
        return withheld(id);
    }
    let url = row.key.as_ref().map(|key| match key {
        ArticleKey::Pubmed(id) => format!("https://pubmed.ncbi.nlm.nih.gov/{id}/"),
        ArticleKey::Pmc(id) => format!("https://pmc.ncbi.nlm.nih.gov/articles/{id}/"),
        ArticleKey::Doi(id) => format!("https://doi.org/{id}"),
        ArticleKey::Europepmc(id) => format!("https://europepmc.org/article/PPR/{id}"),
        ArticleKey::SemanticScholar(id) => format!("https://www.semanticscholar.org/paper/{id}"),
    });
    biomcp_types::WithheldArticleText {
        reason: biomcp_types::Reason::Disabled,
        url: url.and_then(|url| biodata::Text::new(&url).ok()),
    }
}
pub(crate) fn withheld(id: Pmid) -> biomcp_types::WithheldArticleText {
    text_withheld(&biomcp_types::ArticleInput::Pmid(id))
}
/// Explain a text policy refusal and point to the requested paper's public record.
pub fn text_withheld(input: &biomcp_types::ArticleInput) -> biomcp_types::WithheldArticleText {
    use biomcp_types::ArticleInput;
    let url = match input {
        ArticleInput::Pmid(id) => format!("https://pubmed.ncbi.nlm.nih.gov/{id}/"),
        ArticleInput::Pmcid(id) => format!("https://pmc.ncbi.nlm.nih.gov/articles/{id}/"),
        ArticleInput::Doi(id) => format!("https://doi.org/{id}"),
        ArticleInput::EuropePmc(id) => format!("https://europepmc.org/article/PPR/{id}"),
    };
    biomcp_types::WithheldArticleText {
        reason: biomcp_types::Reason::Disabled,
        url: biodata::Text::new(&url).ok(),
    }
}
pub(super) fn pmid(value: &Value) -> Option<Pmid> {
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
pub(super) fn is_abstract(section: &str) -> bool {
    matches!(
        section.trim().to_ascii_lowercase().as_str(),
        "title" | "abstract"
    )
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
    let record = root.get_mut("result").ok_or(BadReply)?;
    let license = record["license"].as_str().map(crate::pmc_oa::license);
    if !permitted(license, true)
        && let Some(record) = record.as_object_mut()
    {
        record.remove("abstractText");
    }
    serde_json::to_vec(&root).map_err(|_| BadReply)
}

/// Repeat abstract rights when reading a source-native author continuation.
pub fn snapshot(leg: &Leg<'_>, body: &[u8]) -> Result<Vec<u8>, BadReply> {
    semantic::snapshot(leg, body)
}
