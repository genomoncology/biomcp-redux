//! One pass over GTR's full XML release, keeping only specimens, order and test links,
//! and each lab's certifications, mailing address and directors.
//! `GTRLabData` holds one `GTRLab` (with `LabCertification`, `Organization/MailingAddress`
//! and `Personnel`) and its `GTRLabTest` siblings (with `Specimen/SpecimenType`,
//! `Order/URL` and `TestURL`), per the publisher's GTRPublicData.xsd.
use biomcp_fetch::BadReply;
use quick_xml::{Reader, events::Event};
use serde::Serialize;
use std::io::BufRead;

/// The XSD nests well under this; deeper input is refused, not followed.
const NESTING: usize = 64;

#[derive(Default, Serialize)]
struct Test {
    #[serde(rename = "xml_test")]
    id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    specimens: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_url: Option<String>,
}
#[derive(Default, Serialize)]
struct Lab {
    #[serde(rename = "xml_lab")]
    id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    certifications: Vec<Certification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    directors: Vec<String>,
}
/// The lab's `MailingAddress`, the address GTR's own esummary gives as `offererlocation`.
#[derive(Default, Serialize)]
struct Address {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
}
impl Address {
    fn is_empty(&self) -> bool {
        self.lines.is_empty()
            && self.city.is_none()
            && self.state.is_none()
            && self.postcode.is_none()
            && self.country.is_none()
    }
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
    OrderUrl,
    TestUrl,
    Name,
    Number,
    Expires,
    Line,
    City,
    State,
    PostCode,
    Country,
    Director,
}
#[derive(Default)]
struct State {
    stack: Vec<String>,
    /// The lab, with `None` as its ID when the attribute is missing.
    lab: Option<(Option<String>, Lab)>,
    /// Only `Type="Certification"`; state licenses are already on the card.
    cert: Option<Certification>,
    /// Inside a public `MailingAddress`.
    address: Option<Address>,
    /// Inside a `Personnel` with `IsDirector="true"`.
    director: bool,
    test: Option<(Option<String>, Test)>,
    leaf: Option<(Leaf, String)>,
}
impl State {
    fn under(&self, path: &[&str]) -> bool {
        self.stack
            .len()
            .checked_sub(path.len())
            .and_then(|start| self.stack.get(start..))
            .is_some_and(|tail| tail.iter().zip(path).all(|(a, b)| a == b))
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
                    s.lab = Some((attr(&tag, "id")?, Lab::default()));
                } else if s.under(&["GTRLabData", "GTRLabTest"]) {
                    let id = attr(&tag, "GTRAccession")?
                        .zip(attr(&tag, "Version")?)
                        .map(|(a, v)| format!("{a}.{v}"))
                        .filter(|id| id.parse::<biodata::GtrTestId>().is_ok());
                    s.test = Some((id, Test::default()));
                } else if s.under(&["GTRLabData", "GTRLab", "LabCertification"]) {
                    s.cert = (attr(&tag, "Type")?.as_deref() == Some("Certification"))
                        .then(Certification::default);
                } else if s.under(&["GTRLab", "Organization", "MailingAddress"]) {
                    // Only addresses the publisher marks public (Public is required by the XSD).
                    s.address = (attr(&tag, "Public")?.as_deref() != Some("false"))
                        .then(Address::default);
                } else if s.under(&["GTRLabData", "GTRLab", "Personnel"]) {
                    s.director = attr(&tag, "IsDirector")?.as_deref() == Some("true");
                } else if s.test.is_some() {
                    let leaf = if s.under(&["GTRLabTest", "Specimen", "SpecimenType"]) {
                        Some(Leaf::Specimen)
                    } else if s.under(&["GTRLabData", "GTRLabTest", "Order", "URL"]) {
                        Some(Leaf::OrderUrl)
                    } else if s.under(&["GTRLabData", "GTRLabTest", "TestURL"]) {
                        Some(Leaf::TestUrl)
                    } else {
                        None
                    };
                    s.leaf = leaf.map(|leaf| (leaf, String::new()));
                } else if s.address.is_some() {
                    let leaf = if s.under(&["MailingAddress", "Line1"])
                        || s.under(&["MailingAddress", "Line2"])
                        || s.under(&["MailingAddress", "Line3"])
                    {
                        Some(Leaf::Line)
                    } else if s.under(&["MailingAddress", "City"]) {
                        Some(Leaf::City)
                    } else if s.under(&["MailingAddress", "State"]) {
                        Some(Leaf::State)
                    } else if s.under(&["MailingAddress", "PostCode"]) {
                        Some(Leaf::PostCode)
                    } else if s.under(&["MailingAddress", "Country"]) {
                        Some(Leaf::Country)
                    } else {
                        None
                    };
                    s.leaf = leaf.map(|leaf| (leaf, String::new()));
                } else if s.director && s.under(&["Personnel", "Person", "DisplayName"]) {
                    s.leaf = Some((Leaf::Director, String::new()));
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
        if value.is_empty() {
            return Ok(());
        }
        match leaf {
            Leaf::Specimen | Leaf::OrderUrl | Leaf::TestUrl => {
                if let Some((_, t)) = &mut s.test {
                    match leaf {
                        Leaf::Specimen => t.specimens.push(value),
                        Leaf::OrderUrl => t.order_url = Some(value),
                        _ => t.test_url = Some(value),
                    }
                }
            }
            Leaf::Director => {
                if let Some((_, lab)) = &mut s.lab {
                    lab.directors.push(value);
                }
            }
            Leaf::Name | Leaf::Number | Leaf::Expires => {
                if let Some(cert) = &mut s.cert {
                    match leaf {
                        Leaf::Name => cert.name = value,
                        Leaf::Number => cert.number = Some(value),
                        _ => cert.expiration_date = Some(value),
                    }
                }
            }
            Leaf::Line | Leaf::City | Leaf::State | Leaf::PostCode | Leaf::Country => {
                if let Some(a) = &mut s.address {
                    match leaf {
                        Leaf::Line => a.lines.push(value),
                        Leaf::City => a.city = Some(value),
                        Leaf::State => a.state = Some(value),
                        Leaf::PostCode => a.postcode = Some(value),
                        _ => a.country = Some(value),
                    }
                }
            }
        }
        return Ok(());
    }
    if s.under(&["GTRLabData", "GTRLab", "LabCertification"]) {
        if let Some(cert) = s.cert.take()
            && let Some((_, lab)) = &mut s.lab
        {
            lab.certifications.push(cert);
        }
    } else if s.under(&["GTRLab", "Organization", "MailingAddress"]) {
        if let Some(a) = s.address.take().filter(|a| !a.is_empty())
            && let Some((_, lab)) = &mut s.lab
        {
            lab.address = Some(a);
        }
    } else if s.under(&["GTRLabData", "GTRLab", "Personnel"]) {
        s.director = false;
    } else if s.under(&["GTRLabData", "GTRLab"]) {
        if let Some((id, lab)) = s.lab.take()
            && (!lab.certifications.is_empty()
                || lab.address.is_some()
                || !lab.directors.is_empty())
        {
            let named = lab.certifications.iter().all(|c| !c.name.is_empty());
            let row = id
                .filter(|_| named)
                .map(|id| serde_json::to_string(&Lab { id, ..lab }))
                .transpose()
                .map_err(|_| BadReply)?;
            emit(row);
        }
    } else if s.under(&["GTRLabData", "GTRLabTest"])
        && let Some((id, test)) = s.test.take()
        && (!test.specimens.is_empty() || test.order_url.is_some() || test.test_url.is_some())
    {
        let row = id
            .map(|id| serde_json::to_string(&Test { id, ..test }))
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
