//! Literal matches in original label text, before full-section pagination splits it.
use super::Record;
use crate::xml_element::{Element, body_nodes, descendants};
use biodata::Text;
use biomcp_fetch::{BadReply, RequestError};
use biomcp_types::{DrugLabelExcerpt, DrugLabelMatches, DrugLabelText, Source};

const EXCERPTS: usize = 2;
const EXCERPT_CHARS: usize = 300;
const HEADER_CHARS: usize = 200;

impl Record {
    /// Search one native field. A missing field has no body, and a rejected field is unread.
    pub fn find_label(
        &self,
        heading: &str,
        terms: &[Text],
    ) -> Result<Option<DrugLabelText>, RequestError> {
        let bad = || biomcp_fetch::bad_reply(Source::OpenfdaLabel, Some(200));
        if self.0.problems.iter().any(|p| p.field.as_ref() == heading) {
            return Err(bad());
        }
        let Some(parts) = self.0.parts(heading) else {
            return Ok(None);
        };
        Ok(Some(DrugLabelText {
            label: self.0.reference()?,
            heading: heading.parse().map_err(|_| bad())?,
            remaining: None,
            text: "".into(),
            search: Some(excerpts(parts, terms, heading.ends_with("_table")).map_err(|_| bad())?),
            retrieved: None,
            fragments: Box::default(),
            offset: 0,
            total: None,
        }))
    }
}

fn excerpts(parts: &[String], terms: &[Text], table: bool) -> Result<DrugLabelMatches, BadReply> {
    let mut search = Search {
        terms: terms.iter().map(|term| fold(term.as_str())).collect(),
        excerpts: Vec::new(),
        truncated: false,
    };
    for part in parts {
        if table && part.trim_start().starts_with("<table") {
            for table in descendants(part, "table")? {
                search.table(table)?;
            }
        } else {
            for paragraph in part.replace("\r\n", "\n").split("\n\n") {
                search.add(paragraph, None, None);
            }
        }
    }
    Ok(DrugLabelMatches {
        excerpts: search.excerpts.into(),
        truncated: search.truncated,
    })
}

struct Search {
    terms: Vec<String>,
    excerpts: Vec<DrugLabelExcerpt>,
    truncated: bool,
}

impl Search {
    fn add(&mut self, text: &str, header: Option<&str>, caption: Option<&str>) {
        if self.truncated {
            return;
        }
        let text = words(text);
        if text.is_empty() {
            return;
        }
        let folded = text.to_ascii_lowercase();
        let Some((start, size)) = self
            .terms
            .iter()
            .filter_map(|term| folded.find(term).map(|at| (at, term.len())))
            .min()
        else {
            return;
        };
        if self.excerpts.len() == EXCERPTS {
            self.truncated = true;
            return;
        }
        self.excerpts.push(DrugLabelExcerpt {
            text: around(&text, start, size),
            table_header: header.map(context),
            table_caption: caption.map(context),
        });
    }

    fn table(&mut self, table: Element<'_>) -> Result<(), BadReply> {
        let caption = table.value(&["caption"])?;
        let mut header = Vec::new();
        let mut rows = Vec::new();
        for (kind, element) in body_nodes(table.0)? {
            match kind.as_str() {
                "thead" => {
                    for row in element.children("tr")? {
                        header.push(cells(row)?.0);
                    }
                }
                "tbody" | "tfoot" => rows.extend(element.children("tr")?),
                "tr" => rows.push(element),
                _ => {}
            }
        }
        if let Some(caption) = caption.as_deref() {
            self.add(caption, None, None);
        }
        let mut data_started = false;
        for row in rows {
            let (text, only_headers) = cells(row)?;
            if !data_started && only_headers {
                header.push(text);
                continue;
            }
            data_started = true;
            let headings = header.join("\n");
            self.add(&text, nonblank(&headings), caption.as_deref());
        }
        let headings = header.join("\n");
        self.add(&headings, None, caption.as_deref());
        Ok(())
    }
}

fn cells(row: Element<'_>) -> Result<(String, bool), BadReply> {
    let mut cells = Vec::new();
    let mut only_headers = true;
    for (kind, cell) in body_nodes(row.0)? {
        if matches!(kind.as_str(), "td" | "th") {
            only_headers &= kind == "th";
            cells.push(cell.text()?);
        }
    }
    Ok((cells.join(" | "), only_headers && !cells.is_empty()))
}

fn nonblank(value: &str) -> Option<&str> {
    (!value.trim().is_empty()).then_some(value)
}

fn words(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn fold(text: &str) -> String {
    words(text).to_ascii_lowercase()
}

fn context(text: &str) -> Box<str> {
    let clipped = text.chars().take(HEADER_CHARS).collect::<String>();
    if clipped.len() < text.len() {
        format!("{clipped}…").into()
    } else {
        clipped.into()
    }
}

fn around(text: &str, start: usize, size: usize) -> Box<str> {
    let prefix = text[..start].chars().count();
    let matched = text[start..start + size].chars().count();
    let skip = prefix.saturating_sub((EXCERPT_CHARS - matched) / 2);
    let mut chars = text.chars().skip(skip);
    let excerpt = chars.by_ref().take(EXCERPT_CHARS).collect::<String>();
    let before = if skip > 0 { "…" } else { "" };
    let after = if chars.next().is_some() { "…" } else { "" };
    format!("{before}{excerpt}{after}").into()
}
