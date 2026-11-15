//! One pass over GTR's full XML release, keeping only specimens, order and test links,
//! and each lab's certifications, mailing address and directors.
//! `GTRLabData` holds one `GTRLab` (with `LabCertification`, `Organization/MailingAddress`
//! and `Personnel`) and its `GTRLabTest` siblings (with `Specimen/SpecimenType`,
//! `Order/URL` and `TestURL`), per the publisher's GTRPublicData.xsd.
use biomcp_fetch::BadReply;
use quick_xml::{Reader, events::Event};
use std::io::BufRead;

use super::kept::{self, Lab, State, Test};

/// The XSD nests well under this; deeper input is refused, not followed.
const NESTING: usize = 64;

pub(super) fn rows(
    xml: &mut dyn BufRead,
    emit: &mut dyn FnMut(Option<String>),
) -> Result<(), BadReply> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().check_end_names = true;
    let mut buf = Vec::new();
    let mut s = State::default();
    loop {
        match reader.read_event_into(&mut buf).map_err(|_| BadReply)? {
            Event::Start(tag) => start(&mut s, &tag)?,
            Event::End(_) => {
                kept::end(&mut s, emit)?;
                s.stack.pop();
            }
            Event::Text(text) => {
                if let Some((_, value)) = &mut s.leaf {
                    value.push_str(&text);
                }
            }
            Event::CData(text) => {
                if let Some((_, value)) = &mut s.leaf {
                    value.push_str(&text);
                }
            }
            Event::GeneralRef(entity) => {
                if let Some((_, value)) = &mut s.leaf {
                    if let Some(ch) = entity.resolve_char_ref().map_err(|_| BadReply)? {
                        value.push(ch);
                    } else {
                        value.push_str(
                            quick_xml::escape::resolve_predefined_entity(&entity)
                                .ok_or(BadReply)?,
                        );
                    }
                }
            }
            Event::Eof => {
                return if s.stack.is_empty() {
                    Ok(())
                } else {
                    Err(BadReply)
                };
            }
            _ => {}
        }
        buf.clear();
    }
}
/// Open one element: a lab, test, certification, address or person, or a kept leaf.
fn start(s: &mut State, tag: &quick_xml::events::BytesStart<'_>) -> Result<(), BadReply> {
    let name = tag.name().as_ref().to_owned();
    s.stack.push(name);
    if s.stack.len() > NESTING {
        return Err(BadReply);
    }
    if s.under(&["GTRLabData", "GTRLab"]) {
        s.lab = Some((attr(tag, "id")?, Lab::default()));
    } else if s.under(&["GTRLabData", "GTRLabTest"]) {
        let id = attr(tag, "GTRAccession")?
            .zip(attr(tag, "Version")?)
            .map(|(a, v)| format!("{a}.{v}"))
            .filter(|id| id.parse::<biodata::GtrTestId>().is_ok());
        s.test = Some((id, Test::default()));
    } else if s.under(&["GTRLabData", "GTRLab", "LabCertification"]) {
        s.cert = (attr(tag, "Type")?.as_deref() == Some("Certification")).then(Default::default);
    } else if s.under(&["GTRLab", "Organization", "MailingAddress"]) {
        // Only addresses the publisher marks public (Public is required by the XSD).
        s.address = (xsd_bool(attr(tag, "Public")?) != Some(false)).then(Default::default);
    } else if s.under(&["GTRLabData", "GTRLab", "Personnel"]) {
        s.director = xsd_bool(attr(tag, "IsDirector")?) == Some(true);
    } else {
        s.leaf = s.leaf().map(|leaf| (leaf, String::new()));
    }
    Ok(())
}
/// An XSD boolean: `true` or `1`, `false` or `0`; anything else is unknown.
fn xsd_bool(value: Option<String>) -> Option<bool> {
    match value.as_deref() {
        Some("true" | "1") => Some(true),
        Some("false" | "0") => Some(false),
        _ => None,
    }
}
fn attr(tag: &quick_xml::events::BytesStart<'_>, name: &str) -> Result<Option<String>, BadReply> {
    for attr in tag.attributes() {
        let attr = attr.map_err(|_| BadReply)?;
        if attr.key.as_ref() == name {
            let value = attr
                .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                .map_err(|_| BadReply)?
                .trim()
                .to_owned();
            return Ok((!value.is_empty()).then_some(value));
        }
    }
    Ok(None)
}
