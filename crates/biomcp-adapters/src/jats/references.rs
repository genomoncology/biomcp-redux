//! Preserve explicit source reference identities for later joins; perform no citation lookup.
use super::blocks;
use biodata::{Doi, Pmid, Text};
use biomcp_fetch::BadReply;
/// One JATS reference's own checked identifiers.
#[derive(Debug)]
pub struct Reference {
    /// Source reference identity, matched only to explicit xref/rid values.
    pub id: Text,
    /// Explicit PubMed identity, when canonical.
    pub pmid: Option<Pmid>,
    /// Explicit DOI, when canonical.
    pub doi: Option<Doi>,
}
pub(super) fn rows(xml: &str) -> Result<Vec<Reference>, BadReply> {
    let mut out = Vec::new();
    for reference in blocks::descendants(xml, "ref")? {
        let Some(id) = reference.attr("id")? else {
            continue;
        };
        let mut row = Reference {
            id: Text::new(&id).map_err(|_| BadReply)?,
            pmid: None,
            doi: None,
        };
        for node in blocks::descendants(reference.0, "pub-id")? {
            let value = node.text()?;
            match node.attr("pub-id-type")?.as_deref() {
                Some("pmid") => row.pmid = value.parse().ok(),
                Some("doi") => row.doi = value.parse().ok(),
                _ => {}
            }
        }
        out.push(row);
    }
    Ok(out)
}
pub(super) fn ids(xml: &str) -> Result<Box<[Text]>, BadReply> {
    let mut out = Vec::new();
    for node in blocks::descendants(xml, "xref")? {
        if !matches!(node.attr("ref-type")?.as_deref(), Some("bibr" | "ref")) {
            continue;
        }
        if let Some(ids) = node.attr("rid")? {
            for id in ids.split_whitespace() {
                let id = Text::new(id).map_err(|_| BadReply)?;
                if !out.contains(&id) {
                    out.push(id);
                }
            }
        }
    }
    Ok(out.into())
}
