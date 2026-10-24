//! Source-specific JATS body mapping after the transport's bounded XML check.
mod blocks;
mod parts;
use crate::xml_element::Element;
use biodata::{FigureCaption, Pmcid, Text, TextPartKind};
use biomcp_fetch::{BadReply, Reply, xml::XmlReply};
pub use parts::{Part, Row, TEXT_BYTES, escaped_bytes};
use serde::{Deserialize, Deserializer};
/// Body text with source row boundaries and deterministic reading order.
#[derive(Debug)]
pub struct Body {
    /// Own PMC identity when present in JATS metadata.
    pub pmcid: Option<Pmcid>,
    /// Bounded body parts.
    pub parts: Vec<Part>,
    /// Whole original paragraphs and table rows, used to find text across fragment boundaries.
    pub rows: Vec<String>,
    /// Figure captions in source order.
    pub figures: Vec<FigureCaption>,
}
impl<'de> Deserialize<'de> for Body {
    fn deserialize<D: Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom("JATS requires XML"))
    }
}
impl Reply for Body {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl XmlReply for Body {
    fn decode_xml(xml: &str) -> Result<Self, BadReply> {
        let root = blocks::descendants(xml, "article")?
            .into_iter()
            .next()
            .ok_or(BadReply)?;
        let metadata = root.at(&["front", "article-meta"])?;
        let pmcid = own_id(metadata)?;
        let body = root.child("body")?.ok_or(BadReply)?;
        let tables = blocks::descendants(body.0, "table-wrap")?;
        let mut emitted = Vec::new();
        let mut parts = Vec::new();
        let mut rows = Vec::new();
        for (name, sec) in blocks::body_nodes(body.0)? {
            if matches!(name.as_str(), "table-wrap" | "table-wrap-group" | "fig" | "supplementary-material") { continue; }
            let title = sec
                .value(&["title"])?
                .filter(|v| !v.trim().is_empty())
                .unwrap_or_else(|| "Body".into());
            parts::append(
                &mut parts,
                &title,
                TextPartKind::Section,
                blocks::rows(sec.0, true)?,
                &mut rows,
            )?;
            for table in &tables {
                let id = table.attr("id")?;
                if id
                    .as_ref()
                    .is_some_and(|id| cited(sec, id).unwrap_or(false))
                    && !emitted.contains(&table.0)
                {
                    append_table(&mut parts, *table, &mut rows)?;
                    emitted.push(table.0);
                }
            }
        }
        if parts.is_empty() {
            parts::append(
                &mut parts,
                "Body",
                TextPartKind::Section,
                blocks::rows(body.0, true)?,
                &mut rows,
            )?;
        }
        for table in tables {
            if !emitted.contains(&table.0) {
                append_table(&mut parts, table, &mut rows)?;
            }
        }
        let figures = blocks::descendants(body.0, "fig")?
            .into_iter()
            .map(|fig| {
                Ok(FigureCaption {
                    label: Text::new(&fig.value(&["label"])?.unwrap_or_else(|| "Figure".into()))
                        .map_err(|_| BadReply)?,
                    caption: Text::new(
                        &fig.value(&["caption"])?.unwrap_or_else(|| "Figure".into()),
                    )
                    .map_err(|_| BadReply)?,
                })
            })
            .collect::<Result<Vec<_>, BadReply>>()?;
        Ok(Self {
            pmcid,
            parts,
            rows,
            figures,
        })
    }
}
fn own_id(metadata: Option<Element<'_>>) -> Result<Option<Pmcid>, BadReply> {
    let Some(metadata) = metadata else {
        return Ok(None);
    };
    let mut own = None;
    for id in metadata.children("article-id")? {
        if matches!(id.attr("pub-id-type")?.as_deref(), Some("pmc" | "pmcid")) {
            let text = id.text()?;
            let text = if text.starts_with("PMC") {
                text
            } else {
                format!("PMC{text}")
            };
            let checked = text.parse().map_err(|_| BadReply)?;
            if own.is_some_and(|v| v != checked) {
                return Err(BadReply);
            }
            own = Some(checked);
        }
    }
    Ok(own)
}
fn cited(sec: Element<'_>, id: &str) -> Result<bool, BadReply> {
    for node in blocks::descendants(sec.0, "xref")? {
        if matches!(
            node.attr("ref-type")?.as_deref(),
            Some("table" | "table-wrap")
        ) && node
            .attr("rid")?
            .is_some_and(|v| v.split_whitespace().any(|v| v == id))
        {
            return Ok(true);
        }
    }
    Ok(false)
}
fn append_table(
    parts: &mut Vec<Part>,
    table: Element<'_>,
    rows: &mut Vec<String>,
) -> Result<(), BadReply> {
    let title = table
        .value(&["label"])?
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "Table".into());
    parts::append(
        parts,
        &title,
        TextPartKind::Table,
        blocks::rows(table.0, false)?,
        rows,
    )
}
