//! PubMed abstracts follow source terms unless the record has explicit restrictions.
use crate::xml_element::{Element, descendants};
use biomcp_fetch::{BadReply, Leg};
use biomcp_types::Use;

pub(crate) fn filter(leg: &Leg<'_>, body: &[u8]) -> Result<Vec<u8>, BadReply> {
    if !leg.disabled_use(Use::PerRecord) {
        return Ok(body.to_vec());
    }
    let xml = std::str::from_utf8(body).map_err(|_| BadReply)?;
    let articles = descendants(xml, "PubmedArticle")?;
    let mut ids = Vec::new();
    for article in &articles {
        if let Some(id) = article
            .value(&["MedlineCitation", "PMID"])?
            .as_deref()
            .and_then(super::policy::parse_pmid)
        {
            ids.push(id);
        }
    }
    let licenses = super::licenses(leg, &ids);
    filter_with_licenses(body, &licenses)
}
pub(super) fn filter_with_licenses(
    body: &[u8],
    licenses: &std::collections::BTreeMap<biodata::Pmid, biodata::License>,
) -> Result<Vec<u8>, BadReply> {
    let xml = std::str::from_utf8(body).map_err(|_| BadReply)?;
    let articles = descendants(xml, "PubmedArticle")?;
    let mut filtered = xml.to_owned();
    for article in articles {
        let id = article
            .value(&["MedlineCitation", "PMID"])?
            .as_deref()
            .and_then(super::policy::parse_pmid);
        let license = id.and_then(|id| licenses.get(&id).copied());
        if super::permitted(license, true) {
            continue;
        }
        let mut replacement = article.0.to_owned();
        for name in ["Abstract", "OtherAbstract"] {
            for text in descendants(article.0, name)? {
                replacement = replacement.replace(text.0, "");
            }
        }
        let closing = replacement.rfind("</PubmedArticle").ok_or(BadReply)?;
        replacement.insert_str(closing, "<BioMcpTextWithheld/>");
        filtered = filtered.replacen(article.0, &replacement, 1);
    }
    // Keep all citation records, including those with no readable abstract.
    Element::root(&filtered, "PubmedArticleSet")?;
    Ok(filtered.into_bytes())
}
