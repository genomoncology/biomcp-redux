//! A page's text in drawing order, including text drawn by the form XObjects it paints.
//!
//! Some FDA labels draw each page's body from a form XObject (`Do`), so a reader of the page's
//! own content stream finds only the "Reference ID" footer: four Rinvoq labels did (1068).
//! A form brings its own `/Resources` or uses those of the stream that paints it (PDF
//! 32000-1 §8.10.1). Forms nest at most [`DEPTH`] deep, which also ends a form that paints
//! itself; a page draws at most [`DRAWS`] forms; every content stream counts
//! against one expanded-size limit; and the deadline is checked at each form draw and every
//! [`CHECK_EVERY`] operations. A form that cannot be read is skipped and the page keeps the rest
//! of its text, marked as possibly missing some.
use super::cmap::{Cache, Font};
use super::fonts::{array, dictionary, entry, resolve};
use super::state::Matrix;
use super::words::{Fonts, Reader};
use lopdf::{Dictionary, Document, Object, ObjectId, Stream, content::Content};
use std::{collections::BTreeMap, rc::Rc};

/// Deepest form nesting read; deeper forms are skipped.
const DEPTH: usize = 8;
/// Most form draws read on one page; later ones are skipped.
pub(super) const DRAWS: usize = 2000;
/// Operations between deadline checks.
const CHECK_EVERY: usize = 4096;

type XObjects<'a> = BTreeMap<&'a [u8], &'a Object>;

/// Why a page's reading stopped.
pub(super) enum Halt {
    /// The deadline passed.
    Expired,
    /// The PDF failed to read, or passed the expanded-size limit.
    Pdf(lopdf::Error),
}

impl From<lopdf::Error> for Halt {
    fn from(issue: lopdf::Error) -> Self {
        Self::Pdf(issue)
    }
}

/// One page's text, the byte spans of it shown in fonts without a character map, and whether
/// a form was skipped, so text may be missing.
pub(super) struct PageText {
    pub(super) text: String,
    pub(super) unsound: Vec<(usize, usize)>,
    pub(super) skipped: bool,
}

/// One page's walk: the document, the expanded bytes left, and the text so far.
struct Walk<'a, 'c> {
    document: &'a Document,
    cache: &'c mut Cache<'a>,
    expired: &'c dyn Fn() -> bool,
    left: usize,
    draws: usize,
    operations: usize,
    skipped: bool,
    reader: Reader,
}

/// Embedded text of one page, its expanded content bounded by `limit` bytes.
pub(super) fn page<'a>(
    document: &'a Document,
    page: ObjectId,
    limit: usize,
    cache: &mut Cache<'a>,
    expired: &dyn Fn() -> bool,
) -> Result<PageText, Halt> {
    let bytes = document.get_page_content_with_limit(page, limit)?;
    let mut walk = Walk {
        document,
        cache,
        expired,
        left: limit.saturating_sub(bytes.len()),
        draws: 0,
        operations: 0,
        skipped: false,
        reader: Reader::default(),
    };
    let mut fonts = Fonts::new();
    for (name, font) in document.get_page_fonts(page)? {
        fonts.insert(name, walk.font(font)?);
    }
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
    walk.stream(&bytes, &fonts, &xobjects, 0)?;
    let skipped = walk.skipped;
    let (text, unsound) = walk.reader.finish();
    Ok(PageText {
        text,
        unsound,
        skipped,
    })
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

/// Whether an error is the expanded-size limit, which ends the document, not one form.
pub(super) const fn too_large(issue: &lopdf::Error) -> bool {
    matches!(
        issue,
        lopdf::Error::Decompress(lopdf::DecompressError::MemoryLimitExceeded { .. })
    )
}

impl<'a> Walk<'a, '_> {
    /// Walk a content stream one piece of whole operations at a time.
    fn stream(
        &mut self,
        bytes: &[u8],
        fonts: &Fonts<'a>,
        xobjects: &XObjects<'a>,
        depth: usize,
    ) -> Result<(), Halt> {
        let (pieces, dropped) = super::chunks::pieces(bytes);
        self.skipped |= dropped;
        for piece in pieces {
            let content = Content::decode(piece)?;
            self.apply_all(&content, fonts, xobjects, depth)?;
        }
        Ok(())
    }

    fn apply_all(
        &mut self,
        content: &Content,
        fonts: &Fonts<'a>,
        xobjects: &XObjects<'a>,
        depth: usize,
    ) -> Result<(), Halt> {
        for operation in &content.operations {
            self.operations += 1;
            if self.operations.is_multiple_of(CHECK_EVERY) && (self.expired)() {
                return Err(Halt::Expired);
            }
            if operation.operator != "Do" {
                self.reader
                    .apply(&operation.operator, &operation.operands, fonts)?;
                continue;
            }
            let form = match operation.operands.first() {
                Some(Object::Name(name)) => xobjects.get(name.as_slice()).copied(),
                _ => None,
            };
            let Some(form) = form.and_then(|object| self.form(object)) else {
                continue;
            };
            self.draws += 1;
            if depth >= DEPTH || self.draws > DRAWS {
                self.skipped = true;
                continue;
            }
            if (self.expired)() {
                return Err(Halt::Expired);
            }
            match self.form_text(form, fonts, xobjects, depth) {
                Err(Halt::Pdf(issue)) if !too_large(&issue) => self.skipped = true,
                other => other?,
            }
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
    ) -> Result<(), Halt> {
        let bytes = if entry(&form.dict, b"Filter").is_some() {
            form.decompressed_content_with_limit(self.left)?
        } else {
            form.content.clone()
        };
        self.charge(bytes.len())?;
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
        let result = self.stream(&bytes, fonts, xobjects, depth + 1);
        self.reader.leave_form();
        result
    }

    /// Spend `bytes` of the expanded-size limit.
    fn charge(&mut self, bytes: usize) -> lopdf::Result<()> {
        if bytes > self.left {
            return Err(lopdf::DecompressError::MemoryLimitExceeded { limit: self.left }.into());
        }
        self.left -= bytes;
        Ok(())
    }

    /// A font from the document's cache.
    fn font(&mut self, font: &'a Dictionary) -> lopdf::Result<Rc<Font<'a>>> {
        self.cache.font(self.document, font, self.left)
    }

    fn fonts(&mut self, entries: &[(&'a [u8], &'a Object)]) -> lopdf::Result<Fonts<'a>> {
        let mut fonts = BTreeMap::new();
        for (name, value) in entries {
            let Some(font) = dictionary(self.document, value) else {
                continue;
            };
            fonts.insert(name.to_vec(), self.font(font)?);
        }
        Ok(fonts)
    }
}
