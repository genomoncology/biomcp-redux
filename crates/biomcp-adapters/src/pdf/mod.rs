//! PDF extraction, called only after the worker installs operating-system limits.
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// Maximum publisher PDF bytes, independent of the ordinary source body limit.
pub const MAX_BYTES: u64 = biomcp_fetch::PdfWorker::MAX_BYTES;
/// Maximum original pages decoded into text.
pub const MAX_PAGES: usize = biomcp_fetch::PdfWorker::MAX_PAGES;
/// Maximum materialized text bytes across all selected pages.
pub const MAX_TEXT: usize = biomcp_fetch::PdfWorker::MAX_TEXT;
/// Maximum serialized worker response, allowing JSON escaping of every text byte.
pub const MAX_RESULT: u64 = biomcp_fetch::PdfWorker::MAX_RESULT;

/// Typed output shared by the child and its parent; no PDF bytes survive extraction.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum Extraction {
    /// Selected page text and original page count.
    Data {
        /// Original document page count.
        pages: u32,
        /// Text in original page order, including blank pages.
        text: Vec<String>,
        /// Whether later original pages were omitted.
        truncated: bool,
    },
    /// Password-protected document.
    Encrypted,
    /// No selectable text layer.
    Empty,
    /// Invalid document, page extraction, panic or resource refusal.
    BadReply,
}

/// Parse selected pages without the convenience helper that swallows page errors.
/// Call only inside the resource-limited worker, never on a source leg's thread.
pub fn extract(bytes: &[u8]) -> Extraction {
    std::panic::catch_unwind(|| decode(bytes)).unwrap_or(Extraction::BadReply)
}
fn decode(bytes: &[u8]) -> Extraction {
    let doc = match pdf_extract::Document::load_mem(bytes) {
        Ok(doc) => doc,
        Err(pdf_extract::Error::Decryption(_)) => return Extraction::Encrypted,
        Err(_) => return Extraction::BadReply,
    };
    if doc.is_encrypted() {
        return Extraction::Encrypted;
    }
    let pages = doc.get_pages();
    let Ok(count) = u32::try_from(pages.len()) else {
        return Extraction::BadReply;
    };
    let mut text = Vec::new();
    let mut total = 0;
    for number in pages.keys().take(MAX_PAGES) {
        let mut writer = BoundedText {
            bytes: Vec::new(),
            remaining: MAX_TEXT - total,
        };
        let output = &mut writer as &mut dyn Write;
        if pdf_extract::output_doc_page(
            &doc,
            &mut pdf_extract::PlainTextOutput::new(output),
            *number,
        )
        .is_err()
        {
            return Extraction::BadReply;
        }
        total += writer.bytes.len();
        let Ok(page) = String::from_utf8(writer.bytes) else {
            return Extraction::BadReply;
        };
        text.push(page);
    }
    if text.iter().all(|page| page.trim().is_empty()) {
        return Extraction::Empty;
    }
    Extraction::Data {
        pages: count,
        truncated: pages.len() > MAX_PAGES,
        text,
    }
}
struct BoundedText {
    bytes: Vec<u8>,
    remaining: usize,
}
impl Write for BoundedText {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.remaining {
            return Err(io::Error::other("PDF text limit"));
        }
        self.bytes.extend_from_slice(bytes);
        self.remaining -= bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
