//! Literal matches in original label text, before full-section pagination splits it.
use super::{
    Record,
    excerpt_text::{around, fold, match_end},
};
use crate::xml_element::{Element, body_nodes, descendants};
use biodata::Text;
use biomcp_fetch::{BadReply, RequestError};
use biomcp_types::{DrugLabelExcerpt, DrugLabelMatches, DrugLabelText, Source};
use std::ops::Range;

const EXCERPTS: usize = 2;
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
    let mut terms = terms
        .iter()
        .map(|term| {
            fold(term.as_str(), &[])
                .map(|(ch, _)| ch)
                .collect::<String>()
        })
        .filter(|term| !term.is_empty())
        .collect::<Vec<_>>();
    // Terms with the same folded spelling describe the same occurrence.
    terms.sort_unstable();
    terms.dedup();
    let mut search = Search {
        terms: terms
            .into_iter()
            .map(|term| {
                let characters = term.chars().count();
                (term, characters)
            })
            .collect(),
        excerpts: Vec::new(),
        total: 0,
        truncated: false,
    };
    let structured = |part: &str| table && part.trim_start().starts_with("<table");
    for group in parts.chunk_by(|a, b| !structured(a) && !structured(b)) {
        match group {
            [part] if structured(part) => {
                for table in descendants(part, "table")? {
                    search.table(table)?;
                }
            }
            [part] => search.add(part, &[], None, None),
            // A native fragment is a continuation of the same section text.
            parts => search.add(&parts.concat(), &[], None, None),
        }
    }
    Ok(DrugLabelMatches {
        matched: search.total != 0,
        total_matches: Some(search.total),
        excerpts: search.excerpts.into(),
        truncated: search.truncated,
    })
}

struct Search {
    terms: Vec<(String, usize)>,
    excerpts: Vec<DrugLabelExcerpt>,
    total: u64,
    truncated: bool,
}

impl Search {
    fn add(&mut self, text: &str, pipes: &[usize], header: Option<&str>, caption: Option<&str>) {
        let mut origins = fold(text, pipes);
        let folded = origins.clone().map(|(ch, _)| ch).collect::<String>();
        let mut windows: Vec<Range<usize>> = Vec::new();
        let mut last_start = None;
        for (at, _) in folded.char_indices() {
            let Some((_, span)) = origins.next() else {
                break;
            };
            if last_start.replace(span.start) == Some(span.start) {
                continue;
            }
            let Some(tail) = folded.get(at..) else {
                continue;
            };
            for (term, length) in &self.terms {
                if !tail.starts_with(term) {
                    continue;
                }
                let Some(end) = match_end(origins.clone(), span.end, *length) else {
                    continue;
                };
                self.total += 1;
                if windows
                    .iter()
                    .any(|window| window.start <= span.start && end <= window.end)
                {
                    continue;
                }
                if self.excerpts.len() == EXCERPTS {
                    self.truncated = true;
                    continue;
                }
                let (window, excerpt) = around(text, span.start..end);
                windows.push(window);
                self.excerpts.push(DrugLabelExcerpt {
                    text: excerpt,
                    table_header: header.map(context),
                    table_caption: caption.map(context),
                });
            }
        }
    }

    fn table(&mut self, table: Element<'_>) -> Result<(), BadReply> {
        let caption = table.value(&["caption"])?;
        let mut header = Vec::new();
        let mut rows = Vec::new();
        for (kind, element) in body_nodes(table.0)? {
            match kind.as_str() {
                "thead" => {
                    for row in element.children("tr")? {
                        header.push(cells(row)?);
                    }
                }
                "tbody" | "tfoot" => rows.extend(element.children("tr")?),
                "tr" => rows.push(element),
                _ => {}
            }
        }
        if let Some(caption) = caption.as_deref() {
            self.add(caption, &[], None, None);
        }
        let rows = rows.into_iter().map(cells).collect::<Result<Vec<_>, _>>()?;
        let lead = rows.iter().take_while(|row| row.only_headers).count();
        let (leading_headers, data) = rows.split_at(lead);
        header.extend_from_slice(leading_headers);
        let headings = header
            .iter()
            .map(|row| row.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        for row in data {
            self.add(
                &row.text,
                &row.pipes,
                nonblank(&headings),
                caption.as_deref(),
            );
        }
        for row in header {
            self.add(&row.text, &row.pipes, None, caption.as_deref());
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Row {
    text: String,
    pipes: Vec<usize>,
    only_headers: bool,
}

fn cells(row: Element<'_>) -> Result<Row, BadReply> {
    let mut text = String::new();
    let mut pipes = Vec::new();
    let mut only_headers = true;
    let mut first = true;
    for (kind, cell) in body_nodes(row.0)? {
        if matches!(kind.as_str(), "td" | "th") {
            only_headers &= kind == "th";
            if !first {
                text.push(' ');
                pipes.push(text.len());
                text.push_str("| ");
            }
            first = false;
            text.push_str(&cell.text()?);
        }
    }
    Ok(Row {
        text,
        pipes,
        only_headers: only_headers && !first,
    })
}

fn nonblank(value: &str) -> Option<&str> {
    (!value.trim().is_empty()).then_some(value)
}

fn context(text: &str) -> Box<str> {
    let clipped = text.chars().take(HEADER_CHARS).collect::<String>();
    if clipped.len() < text.len() {
        format!("{clipped}…").into()
    } else {
        clipped.into()
    }
}
