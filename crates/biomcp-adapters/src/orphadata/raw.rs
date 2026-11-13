//! Orphadata product 9 XML files decoded into stored bundle rows.
use biomcp_fetch::{BadReply, KeyKind, Keys, Row, encode_bundle_row};
use serde::{Deserialize, Serialize};

/// Every admitted file states this licence in its own `Availability` element.
const LICENCE: &str = "CC-BY-4.0";

#[derive(Deserialize)]
struct Jdbor {
    #[serde(rename = "@date")]
    date: String,
    #[serde(rename = "@version", default)]
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
    #[serde(rename = "Disorder", default)]
    items: Vec<Disorder>,
}
#[derive(Deserialize, Default)]
struct Label {
    #[serde(rename = "$text", default)]
    text: String,
}
#[derive(Deserialize)]
struct Term {
    #[serde(rename = "Name", default)]
    name: Option<Label>,
}
#[derive(Deserialize)]
struct Disorder {
    #[serde(rename = "OrphaCode")]
    code: Label,
    #[serde(rename = "Name", default)]
    name: Option<Label>,
    #[serde(rename = "DisorderType", default)]
    disorder_type: Option<Term>,
    #[serde(rename = "PrevalenceList", default)]
    prevalence: Option<PrevalenceList>,
    #[serde(rename = "AverageAgeOfOnsetList", default)]
    onset: Option<OnsetList>,
    #[serde(rename = "TypeOfInheritanceList", default)]
    inheritance: Option<InheritanceList>,
}
#[derive(Deserialize)]
struct PrevalenceList {
    #[serde(rename = "Prevalence", default)]
    items: Vec<Prevalence>,
}
#[derive(Deserialize)]
struct OnsetList {
    #[serde(rename = "AverageAgeOfOnset", default)]
    items: Vec<Term>,
}
#[derive(Deserialize)]
struct InheritanceList {
    #[serde(rename = "TypeOfInheritance", default)]
    items: Vec<Term>,
}
#[derive(Deserialize)]
struct Prevalence {
    #[serde(rename = "@id", default)]
    id: Option<String>,
    #[serde(rename = "Source", default)]
    source: Option<Label>,
    #[serde(rename = "PrevalenceType", default)]
    kind: Option<Term>,
    #[serde(rename = "PrevalenceQualification", default)]
    qualification: Option<Term>,
    #[serde(rename = "PrevalenceClass", default)]
    class: Option<Term>,
    #[serde(rename = "ValMoy", default)]
    value: Option<Label>,
    #[serde(rename = "PrevalenceGeographic", default)]
    geography: Option<Term>,
    #[serde(rename = "PrevalenceValidationStatus", default)]
    validation: Option<Term>,
}

/// One disorder from one file, with the file's release date and version.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Native {
    pub(super) code: String,
    pub(super) name: Option<String>,
    pub(super) disorder_type: Option<String>,
    pub(super) date: String,
    pub(super) version: Option<String>,
    #[serde(default)]
    pub(super) prevalence: Vec<NativePrevalence>,
    #[serde(default)]
    pub(super) onset: Vec<String>,
    #[serde(default)]
    pub(super) inheritance: Vec<String>,
}
/// One published estimate, with every value kept as written.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
        .map(|v| v.text.trim().to_owned())
        .filter(|v| !v.is_empty())
}
fn term(value: Option<Term>) -> Option<String> {
    label(value.and_then(|v| v.name))
}
fn terms(values: Vec<Term>) -> Vec<String> {
    values.into_iter().filter_map(|v| term(Some(v))).collect()
}
impl Native {
    fn new(disorder: Disorder, date: &str, version: Option<&str>) -> Self {
        Self {
            code: disorder.code.text.trim().to_owned(),
            name: label(disorder.name),
            disorder_type: term(disorder.disorder_type),
            date: date.to_owned(),
            version: version.map(str::to_owned),
            prevalence: disorder
                .prevalence
                .map(|list| list.items)
                .unwrap_or_default()
                .into_iter()
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
            onset: terms(disorder.onset.map(|l| l.items).unwrap_or_default()),
            inheritance: terms(disorder.inheritance.map(|l| l.items).unwrap_or_default()),
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
            super::PREVALENCE if !ages => Ok(()),
            super::AGES if self.prevalence.is_empty() => Ok(()),
            _ => Err(BadReply),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Native, Row};
    // Cut from the live en_product9_prev.xml of 2026-06-23 (Kabuki syndrome, first estimate).
    const PREV: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<JDBOR date="2026-06-23 07:57:58" version="1.3.42 / 4.1.8 [2025-03-03]" copyright="Orphanet (c) 2026" dbserver="jdbc:sybase:Tds:canard.orpha.net:2020">
  <Availability> 
    <Licence>
      <FullName lang="en">Creative Commons Attribution 4.0 International</FullName>
      <ShortIdentifier>CC-BY-4.0</ShortIdentifier>
      <LegalCode>https://creativecommons.org/licenses/by/4.0/legalcode</LegalCode>
    </Licence>
  </Availability>
  <DisorderList count="1">
    <Disorder id="2177">
      <OrphaCode>2322</OrphaCode>
      <ExpertLink lang="en">http://www.orpha.net/consor/cgi-bin/OC_Exp.php?lng=en&amp;Expert=2322</ExpertLink>
      <Name lang="en">Kabuki syndrome</Name>
      <DisorderType id="21401">
        <Name lang="en">Malformation syndrome</Name>
      </DisorderType>
      <DisorderGroup id="36547">
        <Name lang="en">Disorder</Name>
      </DisorderGroup>
      <PrevalenceList count="1">
        <Prevalence id="3580">
          <Source>15108197[PMID]</Source>
          <PrevalenceType id="23683">
            <Name lang="en">Prevalence at birth</Name>
          </PrevalenceType>
          <PrevalenceQualification id="23711">
            <Name lang="en">Value and class</Name>
          </PrevalenceQualification>
          <PrevalenceClass id="23746">
            <Name lang="en">1-9 / 100 000</Name>
          </PrevalenceClass>
          <ValMoy>1.16</ValMoy>
          <PrevalenceGeographic id="24957">
            <Name lang="en">New Zealand</Name>
          </PrevalenceGeographic>
          <PrevalenceValidationStatus id="25958">
            <Name lang="en">Validated</Name>
          </PrevalenceValidationStatus>
        </Prevalence>
      </PrevalenceList>
    </Disorder>
  </DisorderList>
</JDBOR>
"#;
    #[test]
    fn decodes_one_published_disorder() {
        let rows = Native::decode_document(PREV.as_bytes()).expect("document decodes");
        assert_eq!(rows.len(), 1, "one disorder row");
        let row: Native = serde_json::from_str(&rows[0]).expect("row decodes");
        assert_eq!(row.code, "2322", "OrphaCode");
        assert_eq!(row.name.as_deref(), Some("Kabuki syndrome"), "name");
        assert_eq!(row.prevalence[0].value.as_deref(), Some("1.16"), "value");
        assert_eq!(
            row.prevalence[0].geography.as_deref(),
            Some("New Zealand"),
            "geography"
        );
        assert!(
            row.check_part(super::super::PREVALENCE).is_ok(),
            "prevalence part"
        );
    }
}
