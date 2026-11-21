//! Standard text for Adobe Symbol glyphs in built-in encodings and private-use character maps.
use super::fonts::{entry, resolve};
use lopdf::{Dictionary, Document, Object};

/// A PDF subset prepends six uppercase letters and a plus to the PostScript font name.
pub(super) fn is_symbol(document: &Document, font: &Dictionary) -> bool {
    let Some(Object::Name(name)) = entry(font, b"BaseFont").and_then(|v| resolve(document, v))
    else {
        return false;
    };
    let name = match name.split_at_checked(7) {
        Some(([tag @ .., b'+'], name)) if tag.iter().all(u8::is_ascii_uppercase) => name,
        _ => name.as_slice(),
    };
    matches!(name, b"Symbol" | b"SymbolMT")
}

/// Whether a character belongs to any Unicode private-use range.
pub fn is_private_use(c: char) -> bool {
    matches!(c, '\u{e000}'..='\u{f8ff}' | '\u{f0000}'..='\u{ffffd}' | '\u{100000}'..='\u{10fffd}')
}

/// Decode bytes in the built-in Symbol encoding. Retain unknown slots in the Windows Symbol private-use range.
pub(super) fn append_bytes(bytes: &[u8], output: &mut String) -> bool {
    let mut unknown = false;
    output.extend(bytes.iter().map(|&byte| {
        let slot = u32::from(byte);
        slot_character(slot).unwrap_or_else(|| {
            unknown = true;
            char::from_u32(0xf000 + slot).unwrap_or(char::REPLACEMENT_CHARACTER)
        })
    }));
    unknown
}

/// Append one decoded Symbol string, retaining and reporting any unknown private-use code.
pub(super) fn append(text: &str, output: &mut String) -> bool {
    let mut unknown = false;
    output.extend(text.chars().map(|c| {
        if is_private_use(c) {
            standard(c).unwrap_or_else(|| {
                unknown = true;
                c
            })
        } else {
            c
        }
    }));
    unknown
}

/// Windows Symbol maps U+F000 + byte to that slot in Adobe Symbol. Some maps instead
/// give Adobe's corporate private-use character for a glyph component or styled mark.
fn standard(c: char) -> Option<char> {
    let slot = match u32::from(c) {
        0xf000..=0xf0ff => u32::from(c) - 0xf000,
        0xf6d9 => 0xd3, // copyrightserif
        0xf6da => 0xd2, // registerserif
        0xf6db => 0xd4, // trademarkserif
        0xf8e6 => 0xbd, // arrowvertex
        0xf8e7 => 0xbe, // arrowhorizex
        0xf8e8..=0xf8ea => 0xe2 + u32::from(c) - 0xf8e8,
        0xf8eb..=0xf8f4 => 0xe6 + u32::from(c) - 0xf8eb,
        0xf8f5 => 0xf4, // integralex
        0xf8f6..=0xf8fe => 0xf6 + u32::from(c) - 0xf8f6,
        _ => return None,
    };
    slot_character(slot)
}

fn slot_character(slot: u32) -> Option<char> {
    match slot {
        0x20..=0x7e => LOW.get((slot - 0x20) as usize),
        0xa0..=0xfe => HIGH.get((slot - 0xa0) as usize),
        _ => None,
    }
    .copied()
    .filter(|&c| c != '\0')
}

// Derived from Adobe Symbol Encoding to Unicode (Copyright © 1991–2011 Unicode, Inc.).
// Where a slot has two readings, use the Greek letter, regular space or fraction slash.
// https://www.unicode.org/Public/MAPPINGS/VENDORS/ADOBE/symbol.txt
// The styled copyright, registered and trademark signs use their base characters. The
// extensible glyph pieces use Unicode's technical symbols where available. The radical
// extender (slot 60) has no standard code. Slot F0 is unassigned.
#[rustfmt::skip]
const LOW: [char; 95] = [
    ' ', '!', '∀', '#', '∃', '%', '&', '∋', '(', ')', '∗', '+', ',', '−', '.', '/', // 20
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', // 30
    '≅', 'Α', 'Β', 'Χ', 'Δ', 'Ε', 'Φ', 'Γ', 'Η', 'Ι', 'ϑ', 'Κ', 'Λ', 'Μ', 'Ν', 'Ο', // 40
    'Π', 'Θ', 'Ρ', 'Σ', 'Τ', 'Υ', 'ς', 'Ω', 'Ξ', 'Ψ', 'Ζ', '[', '∴', ']', '⊥', '_', // 50
    '\0', 'α', 'β', 'χ', 'δ', 'ε', 'φ', 'γ', 'η', 'ι', 'ϕ', 'κ', 'λ', 'μ', 'ν', 'ο', // 60
    'π', 'θ', 'ρ', 'σ', 'τ', 'υ', 'ϖ', 'ω', 'ξ', 'ψ', 'ζ', '{', '|', '}', '∼', // 70
];
#[rustfmt::skip]
const HIGH: [char; 95] = [
    '€', 'ϒ', '′', '≤', '⁄', '∞', 'ƒ', '♣', '♦', '♥', '♠', '↔', '←', '↑', '→', '↓', // A0
    '°', '±', '″', '≥', '×', '∝', '∂', '•', '÷', '≠', '≡', '≈', '…', '⏐', '⎯', '↵', // B0
    'ℵ', 'ℑ', 'ℜ', '℘', '⊗', '⊕', '∅', '∩', '∪', '⊃', '⊇', '⊄', '⊂', '⊆', '∈', '∉', // C0
    '∠', '∇', '®', '©', '™', '∏', '√', '⋅', '¬', '∧', '∨', '⇔', '⇐', '⇑', '⇒', '⇓', // D0
    '◊', '〈', '®', '©', '™', '∑', '⎛', '⎜', '⎝', '⎡', '⎢', '⎣', '⎧', '⎨', '⎩', '⎪', // E0
    '\0', '〉', '∫', '⌠', '⎮', '⌡', '⎞', '⎟', '⎠', '⎤', '⎥', '⎦', '⎫', '⎬', '⎭', // F0
];
