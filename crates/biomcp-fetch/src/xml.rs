//! Bounded UTF-8 XML replies and ordered inline text.
use crate::Reply;
use quick_xml::{Reader, events::Event};
use serde::{
    Deserialize,
    de::{EnumAccess, SeqAccess, VariantAccess, Visitor},
};
use std::fmt;

// One bound protects ignored XML subtrees as well as serde's recursive visitors.
const NESTING: usize = 64;

#[derive(Clone, Copy)]
pub(crate) enum Codec {
    Json,
    Xml,
}
impl Codec {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Json => "application/json",
            Self::Xml => "application/xml",
        }
    }
    pub(crate) fn accepts(self, media: &str) -> bool {
        let mut parts = media.split(';');
        let name = parts.next().unwrap_or("").trim().to_ascii_lowercase();
        match self {
            Self::Json => {
                name == self.name() || name.starts_with("application/") && name.ends_with("+json")
            }
            Self::Xml => {
                (name == self.name()
                    || name == "text/xml"
                    || name.starts_with("application/") && name.ends_with("+xml"))
                    && parts.all(|part| {
                        let Some((key, value)) = part.split_once('=') else {
                            return true;
                        };
                        !key.trim().eq_ignore_ascii_case("charset")
                            || value.trim().trim_matches('"').eq_ignore_ascii_case("utf-8")
                    })
            }
        }
    }
    pub(crate) fn decode<T: Reply>(self, body: &[u8]) -> Option<T> {
        let reply: T = match self {
            Self::Json => serde_json::from_slice(body).ok()?,
            Self::Xml => {
                let text = std::str::from_utf8(body).ok()?;
                let text = text.strip_prefix('\u{feff}').unwrap_or(text);
                validate(text)?;
                let mut de = quick_xml::de::Deserializer::from_str(text);
                de.recursion_limit(NESTING);
                let value = T::deserialize(&mut de).ok()?;
                if !de.is_empty() {
                    return None;
                }
                value
            }
        };
        reply.check().ok()?;
        Some(reply)
    }
}

fn validate(text: &str) -> Option<()> {
    let mut reader = Reader::from_str(text);
    reader.config_mut().enable_all_checks(true);
    let mut depth = 0;
    let mut roots = 0;
    let mut declaration = false;
    let mut doctype = false;
    loop {
        let event = reader.read_event().ok()?;
        let empty = matches!(event, Event::Empty(_));
        match event {
            Event::Start(tag) | Event::Empty(tag) => {
                // Empty events consume one level too, but do not retain it.
                if depth == 0 {
                    roots += 1;
                }
                if roots > 1 || depth >= NESTING {
                    return None;
                }
                for attribute in tag.attributes() {
                    attribute
                        .ok()?
                        .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                        .ok()?;
                }
                if !empty {
                    depth += 1;
                }
            }
            Event::End(_) => {
                depth = depth.checked_sub(1)?;
            }
            Event::Eof => return (roots == 1 && depth == 0).then_some(()),
            Event::Decl(decl) => {
                if declaration
                    || roots != 0
                    || doctype
                    || reader.buffer_position() as usize != decl.len() + 4
                {
                    return None;
                }
                declaration = true;
                decl.version().ok()?;
                if let Some(encoding) = decl.encoding()
                    && !encoding.ok()?.eq_ignore_ascii_case("utf-8")
                {
                    return None;
                }
            }
            Event::DocType(_) => {
                if roots != 0 || doctype {
                    return None;
                }
                doctype = true;
            }
            Event::Text(value)
                if depth == 0
                    && !value
                        .as_ref()
                        .chars()
                        .all(|c| matches!(c, ' ' | '\n' | '\r' | '\t')) =>
            {
                return None;
            }
            Event::GeneralRef(value) => {
                if depth == 0 {
                    return None;
                }
                if value.resolve_char_ref().ok()?.is_none()
                    && !matches!(value.as_ref(), "lt" | "gt" | "amp" | "apos" | "quot")
                {
                    return None;
                }
            }
            Event::CData(_) if depth == 0 => return None,
            _ => {}
        }
    }
}

/// All text within an element, with inline markup removed in document order.
/// Use `#[serde(rename = "$value", default)]` on the containing field.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MixedText(String);
impl MixedText {
    /// Borrow text without changing its whitespace or inserting separators.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<'de> Deserialize<'de> for MixedText {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        de.deserialize_seq(TextVisitor)
    }
}
struct TextVisitor;
impl<'de> Visitor<'de> for TextVisitor {
    type Value = MixedText;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ordered XML content")
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<MixedText, A::Error> {
        let mut text = String::new();
        while let Some(Piece(piece)) = sequence.next_element()? {
            text.push_str(&piece);
        }
        Ok(MixedText(text))
    }
}
struct Piece(String);
impl<'de> Deserialize<'de> for Piece {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        de.deserialize_enum("Inline", &[], PieceVisitor)
    }
}
struct PieceVisitor;
impl<'de> Visitor<'de> for PieceVisitor {
    type Value = Piece;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("XML text or an inline element")
    }
    fn visit_enum<A: EnumAccess<'de>>(self, value: A) -> Result<Piece, A::Error> {
        #[derive(Deserialize)]
        struct Content {
            #[serde(rename = "$value", default)]
            text: MixedText,
        }
        let (name, value) = value.variant::<String>()?;
        if name == "$text" {
            value.newtype_variant().map(Piece)
        } else {
            value
                .newtype_variant::<Content>()
                .map(|content| Piece(content.text.0))
        }
    }
}
