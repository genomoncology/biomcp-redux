//! The values kept from GTR's XML release (see `xml`), the reader's position, and the rows it emits.
use biomcp_fetch::BadReply;
use serde::Serialize;

#[derive(Default, Serialize)]
pub(super) struct Test {
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
pub(super) struct Lab {
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
pub(super) struct Address {
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
#[derive(Default, Serialize)]
pub(super) struct Certification {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_date: Option<String>,
}
/// Which kept leaf the reader is inside.
#[derive(Clone, Copy)]
pub(super) enum Leaf {
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
pub(super) struct State {
    pub stack: Vec<String>,
    /// The lab, with `None` as its ID when the attribute is missing.
    pub lab: Option<(Option<String>, Lab)>,
    /// Only `Type="Certification"`; state licenses are already on the card.
    pub cert: Option<Certification>,
    /// Inside a public `MailingAddress`.
    pub address: Option<Address>,
    /// Inside a `Personnel` with `IsDirector="true"`.
    pub director: bool,
    pub test: Option<(Option<String>, Test)>,
    pub leaf: Option<(Leaf, String)>,
}
impl State {
    pub(super) fn under(&self, path: &[&str]) -> bool {
        self.stack
            .len()
            .checked_sub(path.len())
            .and_then(|start| self.stack.get(start..))
            .is_some_and(|tail| tail.iter().zip(path).all(|(a, b)| a == b))
    }
    /// The kept leaf just opened, if any, given the open test, address, director or certification.
    pub(super) fn leaf(&self) -> Option<Leaf> {
        let first = |pairs: &[(&[&str], Leaf)]| {
            pairs
                .iter()
                .find(|(path, _)| self.under(path))
                .map(|(_, leaf)| *leaf)
        };
        if self.test.is_some() {
            first(&[
                (&["GTRLabTest", "Specimen", "SpecimenType"], Leaf::Specimen),
                (
                    &["GTRLabData", "GTRLabTest", "Order", "URL"],
                    Leaf::OrderUrl,
                ),
                (&["GTRLabData", "GTRLabTest", "TestURL"], Leaf::TestUrl),
            ])
        } else if self.address.is_some() {
            first(&[
                (&["MailingAddress", "Line1"], Leaf::Line),
                (&["MailingAddress", "Line2"], Leaf::Line),
                (&["MailingAddress", "Line3"], Leaf::Line),
                (&["MailingAddress", "City"], Leaf::City),
                (&["MailingAddress", "State"], Leaf::State),
                (&["MailingAddress", "PostCode"], Leaf::PostCode),
                (&["MailingAddress", "Country"], Leaf::Country),
            ])
        } else if self.director {
            first(&[(&["Personnel", "Person", "DisplayName"], Leaf::Director)])
        } else if self.cert.is_some() {
            first(&[
                (&["LabCertification", "Name"], Leaf::Name),
                (&["LabCertification", "Number"], Leaf::Number),
                (&["LabCertification", "ExpirationDate"], Leaf::Expires),
            ])
        } else {
            None
        }
    }
    /// Store one closed leaf's trimmed, nonempty value where it belongs.
    fn keep(&mut self, leaf: Leaf, value: String) {
        match leaf {
            Leaf::Specimen | Leaf::OrderUrl | Leaf::TestUrl => {
                if let Some((_, t)) = &mut self.test {
                    match leaf {
                        Leaf::Specimen => t.specimens.push(value),
                        Leaf::OrderUrl => t.order_url = Some(value),
                        _ => t.test_url = Some(value),
                    }
                }
            }
            Leaf::Director => {
                if let Some((_, lab)) = &mut self.lab {
                    lab.directors.push(value);
                }
            }
            Leaf::Name | Leaf::Number | Leaf::Expires => {
                if let Some(c) = &mut self.cert {
                    match leaf {
                        Leaf::Name => c.name = value,
                        Leaf::Number => c.number = Some(value),
                        _ => c.expiration_date = Some(value),
                    }
                }
            }
            Leaf::Line | Leaf::City | Leaf::State | Leaf::PostCode | Leaf::Country => {
                if let Some(a) = &mut self.address {
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
    }
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
/// Close the element on top of the stack, emitting a lab or test that has kept values.
pub(super) fn end(s: &mut State, emit: &mut dyn FnMut(Option<String>)) -> Result<(), BadReply> {
    if let Some((leaf, value)) = s.leaf.take() {
        let value = value.trim().to_owned();
        if !value.is_empty() {
            s.keep(leaf, value);
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
