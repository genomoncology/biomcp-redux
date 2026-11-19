//! A page's text in drawing order, including text drawn by the form XObjects it paints.
//!
//! Some FDA labels draw each page's body from a form XObject (`Do`), so a reader of the page's
//! own content stream finds only the "Reference ID" footer: four Rinvoq labels did (1068).
//! A form brings its own `/Resources` or uses those of the stream that paints it (PDF
//! 32000-1 §8.10.1). Forms nest at most [`DEPTH`] deep, which also ends a form that paints
//! itself, and every content stream counts against one expanded-size limit.
use super::fonts::{array, dictionary, entry, resolve, widths};
use super::state::Matrix;
use super::words::{Fonts, Reader};
use lopdf::{Dictionary, Document, Object, ObjectId, Stream, content::Content};
use std::collections::BTreeMap;

/// Deepest form nesting read; deeper forms are skipped.
const DEPTH: usize = 8;

type XObjects<'a> = BTreeMap<&'a [u8], &'a Object>;

/// One page's walk: the document, the expanded bytes left, and the text so far.
struct Walk<'a> {
    document: &'a Document,
    left: usize,
    reader: Reader,
}

/// Embedded text of one page, its expanded content bounded by `limit` bytes.
pub(super) fn page(document: &Document, page: ObjectId, limit: usize) -> lopdf::Result<String> {
    let mut fonts = BTreeMap::new();
    for (name, font) in document.get_page_fonts(page)? {
        let encoding = font.get_font_encoding_with_limit(document, limit)?;
        fonts.insert(name, (encoding, widths(document, font)));
    }
    let bytes = document.get_page_content_with_limit(page, limit)?;
    let content = Content::decode(&bytes)?;
    let (own, inherited) = document.get_page_resources(page)?;
    let mut xobjects = XObjects::new();
    let inherited = inherited
        .iter()
        .filter_map(|id| document.objects.get(id))
        .filter_map(|object| dictionary(document, object));
    // The nearest resources come first, so their names win.
    for resources in own.into_iter().chain(inherited) {
        for (name, object) in named(document, resources, b"XObject") {
            xobjects.entry(name).or_insert(object);
        }
    }
    let mut walk = Walk {
        document,
        left: limit.saturating_sub(bytes.len()),
        reader: Reader::default(),
    };
    walk.stream(&content, &fonts, &xobjects, 0)?;
    Ok(walk.reader.finish())
}

/// The entries of one resource category, such as `/Font`, in a resources dictionary.
fn named<'a>(
    document: &'a Document,
    resources: &'a Dictionary,
    category: &[u8],
) -> Vec<(&'a [u8], &'a Object)> {
    entry(resources, category)
        .and_then(|value| dictionary(document, value))
        .map(|entries| {
            entries
                .iter()
                .map(|(name, value)| (name.as_slice(), value))
                .collect()
        })
        .unwrap_or_default()
}

impl<'a> Walk<'a> {
    fn stream(
        &mut self,
        content: &Content,
        fonts: &Fonts<'a>,
        xobjects: &XObjects<'a>,
        depth: usize,
    ) -> lopdf::Result<()> {
        for operation in &content.operations {
            if operation.operator == "Do" {
                let form = match operation.operands.first() {
                    Some(Object::Name(name)) => xobjects.get(name.as_slice()).copied(),
                    _ => None,
                };
                if let Some(form) = form.and_then(|object| self.form(object))
                    && depth < DEPTH
                {
                    self.form_text(form, fonts, xobjects, depth)?;
                }
                continue;
            }
            self.reader
                .apply(&operation.operator, &operation.operands, fonts)?;
        }
        Ok(())
    }

    /// The stream a `Do` names, when it is a form; images and broken references draw no text.
    fn form(&self, object: &'a Object) -> Option<&'a Stream> {
        let Object::Stream(stream) = resolve(self.document, object)? else {
            return None;
        };
        let subtype = resolve(self.document, entry(&stream.dict, b"Subtype")?)?;
        matches!(subtype, Object::Name(name) if name == b"Form").then_some(stream)
    }

    fn form_text(
        &mut self,
        form: &'a Stream,
        fonts: &Fonts<'a>,
        xobjects: &XObjects<'a>,
        depth: usize,
    ) -> lopdf::Result<()> {
        let bytes = if entry(&form.dict, b"Filter").is_some() {
            form.decompressed_content_with_limit(self.left)?
        } else {
            form.content.clone()
        };
        if bytes.len() > self.left {
            return Err(lopdf::DecompressError::MemoryLimitExceeded { limit: self.left }.into());
        }
        self.left -= bytes.len();
        let content = Content::decode(&bytes)?;
        let resources =
            entry(&form.dict, b"Resources").and_then(|value| dictionary(self.document, value));
        let own_fonts = match resources.map(|r| named(self.document, r, b"Font")) {
            Some(entries) if !entries.is_empty() => Some(self.fonts(&entries)?),
            _ => None,
        };
        let own_xobjects: Option<XObjects<'a>> = resources
            .map(|r| named(self.document, r, b"XObject"))
            .filter(|entries| !entries.is_empty())
            .map(|entries| entries.into_iter().collect());
        let matrix = entry(&form.dict, b"Matrix")
            .and_then(|value| array(self.document, value))
            .and_then(Matrix::from);
        self.reader.enter_form(matrix);
        let fonts = own_fonts.as_ref().unwrap_or(fonts);
        let xobjects = own_xobjects.as_ref().unwrap_or(xobjects);
        let result = self.stream(&content, fonts, xobjects, depth + 1);
        self.reader.leave_form();
        result
    }

    fn fonts(&self, entries: &[(&'a [u8], &'a Object)]) -> lopdf::Result<Fonts<'a>> {
        let mut fonts = BTreeMap::new();
        for (name, value) in entries {
            let Some(font) = dictionary(self.document, value) else {
                continue;
            };
            let encoding = font.get_font_encoding_with_limit(self.document, self.left)?;
            fonts.insert(name.to_vec(), (encoding, widths(self.document, font)));
        }
        Ok(fonts)
    }
}
