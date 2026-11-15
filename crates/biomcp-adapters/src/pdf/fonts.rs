//! Glyph widths of a PDF page's fonts, read from the font dictionaries for word placement.
//!
//! A font whose widths are missing or not numbers has no widths, and the reader then keeps
//! lopdf's word breaks for it. A reference to an object the file does not hold reads as
//! absent: PDF 32000-1 §7.3.10 treats it as the null object, not an error. A composite
//! (Type0) font is read as Identity-H: each two-byte code is the glyph ID its `W` widths
//! name. Every FDA document recorded for BioMCP's cases uses that encoding; a font with
//! another CMap gets widths for the wrong codes, which can misplace a word break.
use lopdf::{Dictionary, Document, Object};
use std::collections::BTreeMap;

/// Glyph widths in thousandths of an em, keyed by the first code of each run of codes.
pub(super) struct Widths {
    /// Composite fonts show two-byte codes; simple fonts show one byte per glyph.
    pub(super) two_byte: bool,
    runs: BTreeMap<u32, (u32, f32)>,
    default: f32,
}

impl Widths {
    /// The width of one character code.
    pub(super) fn of(&self, code: u32) -> f32 {
        self.runs
            .range(..=code)
            .next_back()
            .filter(|(_, (last, _))| code <= *last)
            .map_or(self.default, |(_, (_, width))| *width)
    }
}

/// The widths a font dictionary gives, or none for a Type3 font or one without widths.
pub(super) fn widths(document: &Document, font: &Dictionary) -> Option<Widths> {
    match resolve(document, entry(font, b"Subtype")?)? {
        Object::Name(name) if name == b"Type0" => composite(document, font),
        Object::Name(name) if name == b"Type3" => None,
        _ => simple(document, font),
    }
}

/// A dictionary value, absent when the key is.
fn entry<'a>(dictionary: &'a Dictionary, key: &[u8]) -> Option<&'a Object> {
    dictionary
        .iter()
        .find_map(|(name, value)| (name.as_slice() == key).then_some(value))
}

/// The object a value names, following references the file holds.
fn resolve<'a>(document: &'a Document, object: &'a Object) -> Option<&'a Object> {
    let mut object = object;
    for _ in 0..8 {
        match object {
            Object::Reference(id) => object = document.objects.get(id)?,
            _ => return Some(object),
        }
    }
    None
}

/// A PDF number as a float.
pub(super) fn number(object: &Object) -> Option<f32> {
    match object {
        Object::Integer(value) => Some(*value as f32),
        Object::Real(value) => Some(*value),
        _ => None,
    }
}

fn resolved_number(document: &Document, object: &Object) -> Option<f32> {
    number(resolve(document, object)?)
}

fn array<'a>(document: &'a Document, object: &'a Object) -> Option<&'a [Object]> {
    match resolve(document, object)? {
        Object::Array(items) => Some(items),
        _ => None,
    }
}

fn dictionary<'a>(document: &'a Document, object: &'a Object) -> Option<&'a Dictionary> {
    match resolve(document, object)? {
        Object::Dictionary(dictionary) => Some(dictionary),
        _ => None,
    }
}

/// PDF 32000-1 §9.6.1: `Widths` from `FirstChar`; a code outside them has `MissingWidth`.
fn simple(document: &Document, font: &Dictionary) -> Option<Widths> {
    let first = resolved_number(document, entry(font, b"FirstChar")?)? as u32;
    let mut runs = BTreeMap::new();
    for (at, width) in array(document, entry(font, b"Widths")?)?.iter().enumerate() {
        let code = first.saturating_add(at as u32);
        runs.insert(code, (code, resolved_number(document, width)?));
    }
    let default = entry(font, b"FontDescriptor")
        .and_then(|descriptor| dictionary(document, descriptor))
        .and_then(|descriptor| entry(descriptor, b"MissingWidth"))
        .and_then(|width| resolved_number(document, width))
        .unwrap_or(0.0);
    Some(Widths {
        two_byte: false,
        runs,
        default,
    })
}

/// PDF 32000-1 §9.7.4.3: the descendant font's `W` runs over two-byte codes, else `DW`.
fn composite(document: &Document, font: &Dictionary) -> Option<Widths> {
    let descendant = array(document, entry(font, b"DescendantFonts")?)?.first()?;
    let descendant = dictionary(document, descendant)?;
    let default = entry(descendant, b"DW")
        .and_then(|width| resolved_number(document, width))
        .unwrap_or(1000.0);
    let mut runs = BTreeMap::new();
    let mut items = match entry(descendant, b"W") {
        Some(list) => array(document, list)?.iter(),
        None => [].iter(),
    };
    while let Some(first) = items.next() {
        let first = resolved_number(document, first)? as u32;
        match resolve(document, items.next()?)? {
            Object::Array(each) => {
                for (at, width) in each.iter().enumerate() {
                    let code = first.saturating_add(at as u32);
                    runs.insert(code, (code, resolved_number(document, width)?));
                }
            }
            last => {
                let last = number(last)? as u32;
                runs.insert(first, (last, resolved_number(document, items.next()?)?));
            }
        }
    }
    Some(Widths {
        two_byte: true,
        runs,
        default,
    })
}
