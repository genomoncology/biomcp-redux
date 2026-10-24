//! One bounded text partition shared by outlines, direct reads and finds.
use biodata::{PdfPart, PdfText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Document {
    pub(super) outline: PdfText,
    pub(super) parts: Vec<Part>,
}
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Part {
    pub(super) meta: PdfPart,
    pub(super) rows: Vec<Row>,
}
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Row {
    pub(super) paragraph: u32,
    pub(super) page: u32,
    pub(super) text: String,
}
const PART_BYTES: usize = 16_000;
const ROW_BYTES: usize = 3_200;
impl Document {
    pub(super) fn new(mut outline: PdfText, pages: Vec<String>) -> Option<Self> {
        let mut parts = Vec::<Part>::new();
        let mut bytes = 2;
        let mut paragraph_number = 0_u32;
        for (index, page) in pages.into_iter().enumerate() {
            let number = u32::try_from(index + 1).ok()?;
            for paragraph in page.split("\n\n").filter(|text| !text.trim().is_empty()) {
                paragraph_number = paragraph_number.checked_add(1)?;
                for text in fragments(paragraph) {
                    let size = biomcp_adapters::jats::escaped_bytes(&text);
                    if parts.is_empty() || bytes + size + 4 > PART_BYTES {
                        parts.push(Part {
                            meta: PdfPart {
                                number: u32::try_from(parts.len() + 1).ok()?,
                                first_page: number,
                                last_page: number,
                                tokens: 0,
                            },
                            rows: Vec::new(),
                        });
                        bytes = 2;
                    }
                    let part = parts.last_mut()?;
                    part.meta.last_page = number;
                    bytes += size + if part.rows.is_empty() { 0 } else { 4 };
                    part.meta.tokens = u32::try_from(bytes.div_ceil(4)).ok()?;
                    part.rows.push(Row {
                        paragraph: paragraph_number,
                        page: number,
                        text,
                    });
                }
            }
        }
        outline.parts = parts.iter().map(|part| part.meta.clone()).collect();
        Some(Self { outline, parts })
    }
    pub(super) fn url(&self) -> &str {
        self.outline.url.as_str()
    }
}
fn fragments(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut bytes = 2;
    for (index, character) in text.char_indices() {
        let size = match character {
            '"' | '\\' | '\n' | '\r' | '\t' => 2,
            c if c.is_control() => 6,
            c => c.len_utf8(),
        };
        if bytes + size > ROW_BYTES - 4 {
            if let Some(value) = text.get(start..index) {
                out.push(value.to_owned());
            }
            start = index;
            bytes = 2;
        }
        bytes += size;
    }
    if let Some(value) = text.get(start..)
        && !value.is_empty()
    {
        out.push(value.to_owned());
    }
    out
}
