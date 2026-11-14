//! One pass over GTR's full XML release, keeping only specimens and lab certifications.
//! `GTRLabData` holds one `GTRLab` (with `LabCertification`) and its `GTRLabTest`
//! siblings (with `Specimen/SpecimenType`), per the publisher's GTRPublicData.xsd.
use biomcp_fetch::BadReply;
use quick_xml::{Reader, events::Event};
use serde::Serialize;
use std::io::BufRead;

/// The XSD nests well under this; deeper input is refused, not followed.
const NESTING: usize = 64;

#[derive(Serialize)]
struct Test<'a> {
    xml_test: &'a str,
    specimens: &'a [String],
}
#[derive(Serialize)]
struct Lab<'a> {
    xml_lab: &'a str,
    certifications: &'a [Certification],
}
#[derive(Default, Serialize)]
struct Certification {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_date: Option<String>,
}
/// Which kept leaf the reader is inside.
#[derive(Clone, Copy)]
enum Leaf {
    Specimen,
    Name,
    Number,
    Expires,
}
#[derive(Default)]
struct State {
    stack: Vec<String>,
    lab: Option<(Option<String>, Vec<Certification>)>,
    /// Only `Type="Certification"`; state licenses are already on the card.
    cert: Option<Certification>,
    test: Option<(Option<String>, Vec<String>)>,
    leaf: Option<(Leaf, String)>,
}
impl State {
    fn under(&self, path: &[&str]) -> bool {
        self.stack.len() >= path.len()
            && self.stack[self.stack.len() - path.len()..]
                .iter()
                .zip(path)
                .all(|(a, b)| a == b)
    }
}

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
            Event::Start(tag) => {
                let name = tag.name().as_ref().to_owned();
                s.stack.push(name);
                if s.stack.len() > NESTING {
                    return Err(BadReply);
                }
                if s.under(&["GTRLabData", "GTRLab"]) {
                    s.lab = Some((attr(&tag, "id")?, Vec::new()));
                } else if s.under(&["GTRLabData", "GTRLabTest"]) {
                    let id = attr(&tag, "GTRAccession")?
                        .zip(attr(&tag, "Version")?)
                        .map(|(a, v)| format!("{a}.{v}"))
                        .filter(|id| id.parse::<biodata::GtrTestId>().is_ok());
                    s.test = Some((id, Vec::new()));
                } else if s.under(&["GTRLabData", "GTRLab", "LabCertification"]) {
                    s.cert = (attr(&tag, "Type")?.as_deref() == Some("Certification"))
                        .then(Certification::default);
                } else if s.under(&["GTRLabTest", "Specimen", "SpecimenType"]) && s.test.is_some() {
                    s.leaf = Some((Leaf::Specimen, String::new()));
                } else if s.cert.is_some() {
                    let leaf = if s.under(&["LabCertification", "Name"]) {
                        Some(Leaf::Name)
                    } else if s.under(&["LabCertification", "Number"]) {
                        Some(Leaf::Number)
                    } else if s.under(&["LabCertification", "ExpirationDate"]) {
                        Some(Leaf::Expires)
                    } else {
                        None
                    };
                    s.leaf = leaf.map(|leaf| (leaf, String::new()));
                }
            }
            Event::End(_) => {
                end(&mut s, emit)?;
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
/// Close the element on top of the stack, emitting a lab or test that has kept values.
fn end(s: &mut State, emit: &mut dyn FnMut(Option<String>)) -> Result<(), BadReply> {
    if let Some((leaf, value)) = s.leaf.take() {
        let value = value.trim().to_owned();
        match leaf {
            Leaf::Specimen => {
                if let Some((_, list)) = &mut s.test
                    && !value.is_empty()
                {
                    list.push(value);
                }
            }
            Leaf::Name | Leaf::Number | Leaf::Expires => {
                if let Some(cert) = &mut s.cert
                    && !value.is_empty()
                {
                    match leaf {
                        Leaf::Name => cert.name = value,
                        Leaf::Number => cert.number = Some(value),
                        _ => cert.expiration_date = Some(value),
                    }
                }
            }
        }
        return Ok(());
    }
    if s.under(&["GTRLabData", "GTRLab", "LabCertification"]) {
        if let Some(cert) = s.cert.take()
            && let Some((_, list)) = &mut s.lab
        {
            list.push(cert);
        }
    } else if s.under(&["GTRLabData", "GTRLab"]) {
        if let Some((id, list)) = s.lab.take()
            && !list.is_empty()
        {
            let named = list.iter().all(|c| !c.name.is_empty());
            let row = id
                .filter(|_| named)
                .map(|id| {
                    serde_json::to_string(&Lab {
                        xml_lab: &id,
                        certifications: &list,
                    })
                })
                .transpose()
                .map_err(|_| BadReply)?;
            emit(row);
        }
    } else if s.under(&["GTRLabData", "GTRLabTest"])
        && let Some((id, list)) = s.test.take()
        && !list.is_empty()
    {
        let row = id
            .map(|id| {
                serde_json::to_string(&Test {
                    xml_test: &id,
                    specimens: &list,
                })
            })
            .transpose()
            .map_err(|_| BadReply)?;
        emit(row);
    }
    Ok(())
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
