//! Orphadata product 9 XML files decoded into stored bundle rows.
use biomcp_fetch::{BadReply, KeyKind, Keys, Row, encode_bundle_row};
use serde::{Deserialize, Serialize};

/// Every admitted file states this licence in its own `Availability` element.
const LICENCE: &str = "CC-BY-4.0";

#[derive(Deserialize)]
struct Jdbor {
    #[serde(rename = "@date")]
    date: String,
    #[serde(rename = "@version")]
    version: Option<String>,
    #[serde(rename = "Availability")]
    availability: Availability,
    #[serde(rename = "DisorderList")]
    disorders: DisorderList,
}
#[derive(Deserialize)]
struct Availability {
    #[serde(rename = "Licence")]
    licence: Licence,
}
#[derive(Deserialize)]
struct Licence {
    #[serde(rename = "ShortIdentifier")]
    short: String,
}
#[derive(Deserialize)]
struct DisorderList {
    #[serde(rename = "Disorder")]
    items: Option<Vec<Disorder>>,
}
#[derive(Deserialize)]
struct Label {
    #[serde(rename = "$text")]
    text: Option<String>,
}
#[derive(Deserialize)]
struct Term {
    #[serde(rename = "Name")]
    name: Option<Label>,
}
#[derive(Deserialize)]
struct Disorder {
    #[serde(rename = "OrphaCode")]
    code: Label,
    #[serde(rename = "Name")]
    name: Option<Label>,
    #[serde(rename = "DisorderType")]
    disorder_type: Option<Term>,
    #[serde(rename = "PrevalenceList")]
    prevalence: Option<PrevalenceList>,
    #[serde(rename = "AverageAgeOfOnsetList")]
    onset: Option<OnsetList>,
    #[serde(rename = "TypeOfInheritanceList")]
    inheritance: Option<InheritanceList>,
}
#[derive(Deserialize)]
struct PrevalenceList {
    #[serde(rename = "Prevalence")]
    items: Option<Vec<Prevalence>>,
}
#[derive(Deserialize)]
struct OnsetList {
    #[serde(rename = "AverageAgeOfOnset")]
    items: Option<Vec<Term>>,
}
#[derive(Deserialize)]
struct InheritanceList {
    #[serde(rename = "TypeOfInheritance")]
    items: Option<Vec<Term>>,
}
#[derive(Deserialize)]
struct Prevalence {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "Source")]
    source: Option<Label>,
    #[serde(rename = "PrevalenceType")]
    kind: Option<Term>,
    #[serde(rename = "PrevalenceQualification")]
    qualification: Option<Term>,
    #[serde(rename = "PrevalenceClass")]
    class: Option<Term>,
    #[serde(rename = "ValMoy")]
    value: Option<Label>,
    #[serde(rename = "PrevalenceGeographic")]
    geography: Option<Term>,
    #[serde(rename = "PrevalenceValidationStatus")]
    validation: Option<Term>,
}

/// One disorder from one file, with the file's release date and version.
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Native {
    pub(super) code: String,
    pub(super) name: Option<String>,
    pub(super) disorder_type: Option<String>,
    pub(super) date: String,
    pub(super) version: Option<String>,
    pub(super) prevalence: Vec<NativePrevalence>,
    pub(super) onset: Vec<String>,
    pub(super) inheritance: Vec<String>,
}
/// One published estimate, with every value kept as written.
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct NativePrevalence {
    pub(super) id: Option<String>,
    pub(super) source: Option<String>,
    pub(super) kind: Option<String>,
    pub(super) qualification: Option<String>,
    pub(super) class: Option<String>,
    pub(super) value: Option<String>,
    pub(super) geography: Option<String>,
    pub(super) validation: Option<String>,
}
fn label(value: Option<Label>) -> Option<String> {
    value
        .and_then(|v| v.text)
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty())
}
fn term(value: Option<Term>) -> Option<String> {
    label(value.and_then(|v| v.name))
}
// An absent list element and an empty one both state no entries.
fn terms(values: Option<Vec<Term>>) -> Vec<String> {
    values
        .into_iter()
        .flatten()
        .filter_map(|v| term(Some(v)))
        .collect()
}
impl Native {
    fn new(disorder: Disorder, date: &str, version: Option<&str>) -> Self {
        Self {
            // A missing or non-numeric code gives no key, so admission refuses the row visibly.
            code: label(Some(disorder.code)).unwrap_or_else(String::new),
            name: label(disorder.name),
            disorder_type: term(disorder.disorder_type),
            date: date.to_owned(),
            version: version.map(str::to_owned),
            prevalence: disorder
                .prevalence
                .and_then(|list| list.items)
                .into_iter()
                .flatten()
                .map(|p| NativePrevalence {
                    id: p.id,
                    source: label(p.source),
                    kind: term(p.kind),
                    qualification: term(p.qualification),
                    class: term(p.class),
                    value: label(p.value),
                    geography: term(p.geography),
                    validation: term(p.validation),
                })
                .collect(),
            onset: terms(disorder.onset.and_then(|l| l.items)),
            inheritance: terms(disorder.inheritance.and_then(|l| l.items)),
        }
    }
    /// Stored exact key, matching MONDO's Orphanet cross-reference spelling.
    pub(super) fn key(code: &str) -> String {
        format!("Orphanet:{code}")
    }
}
impl Row for Native {
    fn decode_document(body: &[u8]) -> Result<Vec<String>, BadReply> {
        let text = std::str::from_utf8(body).map_err(|_| BadReply)?;
        let document: Jdbor = quick_xml::de::from_str(text).map_err(|_| BadReply)?;
        if document.availability.licence.short.trim() != LICENCE {
            return Err(BadReply);
        }
        let version = document.version.as_deref();
        document
            .disorders
            .items
            .into_iter()
            .flatten()
            .map(|disorder| encode_bundle_row(&Self::new(disorder, &document.date, version)))
            .collect()
    }
    fn keys(&self) -> Keys {
        let exact = if !self.code.is_empty() && self.code.bytes().all(|b| b.is_ascii_digit()) {
            vec![(KeyKind::Id, Self::key(&self.code).into())]
        } else {
            Vec::new()
        };
        Keys {
            exact,
            words: Vec::new(),
        }
    }
    // Each file carries only its own product: estimates, or onset and inheritance.
    fn check_part(&self, part: &str) -> Result<(), BadReply> {
        let ages = !self.onset.is_empty() || !self.inheritance.is_empty();
        match part {
            super::bundle::PREVALENCE if !ages => Ok(()),
            super::bundle::AGES if self.prevalence.is_empty() => Ok(()),
            _ => Err(BadReply),
        }
    }
}
