//! Source bytes for explicit saves; the binary owns paths and filesystem writes.
use crate::Outcome;
use biodata::{ArticleInput, Text};
use biomcp_adapters::pmc_oa::Document;
/// Result before the binary installs a file.
#[derive(Debug)]
pub struct Download {
    /// Ordinary typed source outcome.
    pub out: Outcome,
    /// Source bytes, absent when policy or a source refused the save.
    pub file: Option<File>,
}
/// Checked original material and its source metadata, without a destination.
#[derive(Debug)]
pub struct File {
    /// Whole Markdown document or original supplement bytes.
    pub bytes: Vec<u8>,
    /// Original material's source URL.
    pub url: Text,
    /// Ordered text part count, absent for supplement bytes.
    pub parts: Option<u32>,
}
pub(super) struct State {
    pub(super) allow: bool,
    pub(super) file: Option<File>,
}
impl State {
    pub(super) fn new(allow: bool) -> Self { Self { allow, file: None } }
    pub(super) fn text(&mut self, input: &ArticleInput, doc: &Document) -> Option<()> {
        if !self.allow { return Some(()); }
        let mut text = format!("# {}\n\nID: {input}\n", doc.body.title.as_ref().map_or("Article", Text::as_str));
        for (kind, id) in &doc.body.identifiers {
            text.push_str(&format!("{kind}: {id}\n"));
        }
        text.push_str(&format!("License: {}\nSource: {}\n\n", doc.license, doc.url));
        // Parts can split a source row or repeat a table heading for reader pages.
        // Reassemble each original row once, in its first part's document order.
        let mut previous = None;
        for part in &doc.body.parts {
            for row in &part.rows {
                if previous != Some(row.source_row) {
                    if previous.is_some() { text.push_str(part.separator); }
                    previous = Some(row.source_row);
                }
                text.push_str(&row.text);
            }
        }
        text.push('\n');
        self.file = Some(File { bytes: text.into_bytes(), url: doc.url.clone(), parts: Some(u32::try_from(doc.body.parts.len()).ok()?) });
        Some(())
    }
}
