//! Bounded UTF-8 XML replies and ordered inline text.
use crate::Reply;
use quick_xml::{Reader, events::Event};
use std::borrow::Cow;

// One bound protects ignored XML subtrees as well as serde's recursive visitors.
const NESTING: usize = 64;

#[derive(Clone, Copy)]
pub(crate) enum Codec {
    Json,
    Xml,
    Bytes(&'static str),
    Probe,
}
impl Codec {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Json => "application/json",
            Self::Xml => "application/xml",
            Self::Bytes(media) => media,
            Self::Probe => "*/*",
        }
    }
    pub(crate) fn accepts(self, media: &str) -> bool {
        let mut parts = media.split(';');
        let name = parts.next().unwrap_or("").trim().to_ascii_lowercase();
        match self {
            Self::Bytes(_) => true,
            Self::Probe => true,
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
}

/// Strict XML media and UTF-8 charset admission for source policies.
pub fn accepts_media(media: &str) -> bool {
    Codec::Xml.accepts(media)
}

/// An XML reply selects its raw decoding at the document boundary.
/// Ordinary fields use serde by default. Mixed elements use `MixedText::from_xml`.
pub trait XmlReply: Reply {
    /// Admit source media while ordinary XML keeps strict UTF-8 admission.
    fn accepts_media(media: &str) -> bool {
        accepts_media(media)
    }
    /// Convert source encoding before shared XML validation and cache admission.
    fn normalize_body<'a>(_media: &str, body: &'a [u8]) -> Cow<'a, [u8]> {
        Cow::Borrowed(body)
    }

    /// Decode raw XML without exposing parser messages to the caller.
    fn decode_xml(xml: &str) -> Result<Self, crate::BadReply> {
        let mut de = quick_xml::de::Deserializer::from_str(xml);
        de.recursion_limit(NESTING);
        Self::deserialize(&mut de).map_err(|_| crate::BadReply)
    }
}

type BodyCheck<'a> = dyn Fn(&[u8]) -> Result<(), biomcp_types::Reason> + 'a;
type ReplyCheck<'a, T> = dyn Fn(&T) -> Result<(), crate::BadReply> + 'a;
type RangeCheck<'a, T> =
    dyn Fn(&T, Option<crate::ContentRange>, u16) -> Result<(), crate::BadReply> + 'a;
type BodyNormalizer = for<'a> fn(&str, &'a [u8]) -> Cow<'a, [u8]>;
pub(crate) struct Decoder<'a, T> {
    validate: Option<&'a BodyCheck<'a>>,
    check: Option<&'a ReplyCheck<'a, T>>,
    cache_if: Option<&'a dyn Fn(&T) -> bool>,
    codec: Codec,
    xml_media: Option<fn(&str) -> bool>,
    normalize_body: Option<BodyNormalizer>,
    pub(crate) limit: Option<u64>,
    pub(crate) overflow_reason: biomcp_types::Reason,
    pub(crate) json_rejections: bool,
    pub(crate) json_absence: bool,
    octet_stream: Option<&'static str>,
    parse: fn(&[u8]) -> Option<T>,
    range_check: Option<&'a RangeCheck<'a, T>>,
}
impl<'a, T: Reply> Decoder<'a, T> {
    pub(crate) fn checked_json(check: &'a dyn Fn(&T) -> Result<(), crate::BadReply>) -> Self {
        let mut codec = Self::json();
        codec.check = Some(check);
        codec
    }
    pub(crate) fn counted_json(check: &'a RangeCheck<'a, T>) -> Self {
        let mut codec = Self::json();
        codec.range_check = Some(check);
        codec
    }
    pub(crate) fn decode_response(
        &self,
        body: &[u8],
        range: Option<&str>,
        status: u16,
    ) -> Option<T> {
        let reply = self.decode(body)?;
        reply.check_status(status).ok()?;
        if let Some(check) = self.range_check {
            check(&reply, range.and_then(crate::ContentRange::parse), status).ok()?;
        }
        Some(reply)
    }
    pub(crate) fn checked_json_string(
        check: &'a dyn Fn(&T) -> Result<(), crate::BadReply>,
    ) -> Self {
        let mut codec = Self::checked_json(check);
        codec.parse = |body| {
            let inner: String = serde_json::from_slice(body).ok()?;
            serde_json::from_str(&inner).ok()
        };
        codec
    }
    pub(crate) fn json() -> Self {
        Self {
            codec: Codec::Json,
            xml_media: None,
            normalize_body: None,
            validate: None,
            check: None,
            cache_if: None,
            range_check: None,
            octet_stream: None,
            limit: None,
            overflow_reason: biomcp_types::Reason::BadReply,
            json_rejections: false,
            json_absence: false,
            parse: |body| serde_json::from_slice(body).ok(),
        }
    }
    pub(crate) fn checked_xml(check: &'a dyn Fn(&T) -> Result<(), crate::BadReply>) -> Self
    where
        T: XmlReply,
    {
        let mut codec = Self::xml();
        codec.check = Some(check);
        codec
    }
    pub(crate) fn xml() -> Self
    where
        T: XmlReply,
    {
        Self {
            codec: Codec::Xml,
            xml_media: Some(T::accepts_media),
            normalize_body: Some(T::normalize_body),
            validate: None,
            check: None,
            cache_if: None,
            range_check: None,
            octet_stream: None,
            limit: None,
            overflow_reason: biomcp_types::Reason::BadReply,
            json_rejections: false,
            json_absence: false,
            parse: |body| {
                let text = std::str::from_utf8(body).ok()?;
                let text = text.strip_prefix('\u{feff}').unwrap_or(text);
                validate(text)?;
                T::decode_xml(text).ok()
            },
        }
    }
    pub(crate) fn cache_if(mut self, predicate: &'a dyn Fn(&T) -> bool) -> Self {
        self.cache_if = Some(predicate);
        self
    }
    pub(crate) fn cacheable(&self, reply: &T) -> bool {
        self.cache_if.is_none_or(|predicate| predicate(reply))
    }
    pub(crate) fn validate(&self, bytes: &[u8]) -> Result<(), biomcp_types::Reason> {
        self.validate.map_or(Ok(()), |check| check(bytes))
    }
    pub(crate) fn binary(&self) -> bool {
        matches!(self.codec, Codec::Bytes(_))
    }
    pub(crate) fn octet_stream(mut self, accepted: Option<&'static str>) -> Self {
        self.octet_stream = accepted;
        self
    }
    pub(crate) fn name(&self) -> &'static str {
        self.codec.name()
    }
    pub(crate) fn accepts(&self, media: &str) -> bool {
        self.xml_media
            .map_or_else(|| self.codec.accepts(media), |accepts| accepts(media))
            || self.octet_stream.is_some_and(|accepted| {
                media
                    .split(';')
                    .next()
                    .is_some_and(|value| value.trim().eq_ignore_ascii_case(accepted))
            })
    }
    pub(crate) fn normalized_body<'b>(&self, media: &str, body: &'b [u8]) -> Cow<'b, [u8]> {
        self.normalize_body
            .map_or(Cow::Borrowed(body), |normalize| normalize(media, body))
    }
    pub(crate) fn decode(&self, body: &[u8]) -> Option<T> {
        let reply = (self.parse)(body)?;
        reply.check().ok()?;
        if let Some(check) = self.check {
            check(&reply).ok()?;
        }
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
                validate_declaration(&decl)?;
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

fn validate_declaration(decl: &quick_xml::events::BytesDecl<'_>) -> Option<()> {
    decl.xml_version().ok()?;
    let attributes = quick_xml::events::BytesStart::from_content(decl.as_ref(), 3);
    let mut previous = 0;
    for attribute in attributes.attributes() {
        let attribute = attribute.ok()?;
        let order = match attribute.key.as_ref() {
            "version" => 1,
            "encoding" if attribute.value.eq_ignore_ascii_case("utf-8") => 2,
            "standalone" if matches!(attribute.value.as_ref(), "yes" | "no") => 3,
            _ => return None,
        };
        if order <= previous {
            return None;
        }
        previous = order;
    }
    Some(())
}

/// All text within an XML element, with inline markup removed in document order.
#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(transparent)]
pub struct MixedText(String);
impl MixedText {
    /// Borrow text without changing whitespace or inserting separators.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Read one complete element, including arbitrary nested inline text.
    /// Attributes and comments contribute no text. DTDs are never resolved.
    pub fn from_xml(element: &str) -> Result<Self, crate::BadReply> {
        let element = element.strip_prefix('\u{feff}').unwrap_or(element);
        validate(element).ok_or(crate::BadReply)?;
        let mut reader = Reader::from_str(element);
        let mut text = String::new();
        let mut depth = 0;
        loop {
            match reader.read_event().map_err(|_| crate::BadReply)? {
                Event::Start(_) => depth += 1,
                Event::End(_) => depth -= 1,
                Event::Text(value) if depth != 0 => text.push_str(value.as_ref()),
                Event::CData(value) => text.push_str(value.as_ref()),
                Event::GeneralRef(value) => {
                    if let Some(ch) = value.resolve_char_ref().map_err(|_| crate::BadReply)? {
                        text.push(ch);
                    } else {
                        text.push_str(
                            quick_xml::escape::resolve_predefined_entity(value.as_ref())
                                .ok_or(crate::BadReply)?,
                        );
                    }
                }
                Event::Eof => return Ok(Self(text)),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
#[path = "xml_tests.rs"]
mod tests;

#[derive(serde::Deserialize)]
pub(crate) struct Probe;
impl Reply for Probe {
    fn check(&self) -> Result<(), crate::BadReply> {
        Ok(())
    }
}
impl Decoder<'_, Probe> {
    pub(crate) fn probe() -> Self {
        Self {
            codec: Codec::Probe,
            xml_media: None,
            normalize_body: None,
            validate: None,
            check: None,
            cache_if: None,
            range_check: None,
            limit: None,
            overflow_reason: biomcp_types::Reason::BadReply,
            json_rejections: false,
            json_absence: false,
            octet_stream: None,
            parse: |_| Some(Probe),
        }
    }
}
#[derive(serde::Deserialize)]
pub(crate) struct Bytes(pub(crate) Vec<u8>);
impl Reply for Bytes {
    fn check(&self) -> Result<(), crate::BadReply> {
        Ok(())
    }
    fn check_status(&self, status: u16) -> Result<(), crate::BadReply> {
        // Binary callers retrieve complete resources and never send Range requests.
        if status == 200 {
            Ok(())
        } else {
            Err(crate::BadReply)
        }
    }
}
impl<'a> Decoder<'a, Bytes> {
    pub(crate) fn document_media(mut self, media: crate::DocumentMedia) -> Self {
        self.codec = Codec::Bytes(media.media());
        self
    }

    pub(crate) fn checked_bytes(
        limit: u64,
        check: &'a dyn Fn(&[u8]) -> Result<(), biomcp_types::Reason>,
    ) -> Self {
        let mut codec = Self::bytes(limit);
        codec.validate = Some(check);
        codec
    }
    pub(crate) fn bytes(limit: u64) -> Self {
        Self {
            codec: Codec::Bytes("application/octet-stream"),
            xml_media: None,
            normalize_body: None,
            validate: None,
            check: None,
            cache_if: None,
            range_check: None,
            octet_stream: None,
            limit: Some(limit),
            overflow_reason: biomcp_types::Reason::BadReply,
            json_rejections: false,
            json_absence: false,
            parse: |body| Some(Bytes(body.to_vec())),
        }
    }
}

impl crate::Leg<'_> {
    /// Apply a request-bound XML check on cache/replay reads and before live cache admission.
    pub fn xml_received_checked<T: XmlReply>(
        &self,
        ep: &crate::Endpoint,
        params: &[(&str, crate::Arg<'_>)],
        check: impl Fn(&T) -> Result<(), crate::BadReply>,
    ) -> Result<(T, crate::ReplyMetadata), crate::RequestError> {
        self.parameters(ep, params, Decoder::checked_xml(&check))
    }
}

impl crate::Leg<'_> {
    /// Preserve XML metadata while withholding refused record identities from raw cache.
    pub fn xml_received_checked_cache<T: XmlReply>(
        &self,
        ep: &crate::Endpoint,
        params: &[(&str, crate::Arg<'_>)],
        cache_if: impl Fn(&T) -> bool,
    ) -> Result<(T, crate::ReplyMetadata), crate::RequestError> {
        self.parameters(ep, params, Decoder::xml().cache_if(&cache_if))
    }
}
