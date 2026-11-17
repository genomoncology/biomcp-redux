//! The eEML's schema.org JSON-LD medicine and recommendation shapes, and its search page links.
use crate::{mapped::Problems, native_text::NativeText, raw::Field};
use biodata::Text;
use biomcp_fetch::{BadReply, Reply};
use biomcp_types::{DrugEssentialMedicineDecision, DrugEssentialMedicineListing};
use serde::Deserialize;
use std::collections::BTreeMap;

/// One eEML medicine page.
#[derive(Deserialize)]
pub(crate) struct Medicine {
    name: Option<Field<NativeText>>,
    /// The recommendations that list this medicine.
    guideline: Option<Field<Links>>,
    /// The recommendations that name this medicine as a therapeutic alternative.
    #[serde(rename = "equivalentFor")]
    equivalent_for: Option<Field<Links>>,
}
impl Reply for Medicine {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
/// The eEML serves a link list as an array, or as an object keyed by position.
#[derive(Deserialize)]
#[serde(untagged)]
enum Links {
    List(Vec<Field<Link>>),
    Keyed(BTreeMap<String, Field<Link>>),
}
#[derive(Deserialize)]
struct Link {
    #[serde(rename = "@id")]
    id: Option<Field<String>>,
}
impl Medicine {
    /// The medicine's name and its recommendation numbers: those listing it first, then those
    /// naming it as an alternative, each in number order.
    pub(crate) fn map(self, cx: &mut Problems) -> (Option<Text>, Vec<u32>) {
        let name = cx.source_field("name", self.name);
        let mut found = numbers("guideline", self.guideline, cx);
        for number in numbers("equivalentFor", self.equivalent_for, cx) {
            if !found.contains(&number) {
                found.push(number);
            }
        }
        (name, found)
    }
}
fn numbers(field: &str, links: Option<Field<Links>>, cx: &mut Problems) -> Vec<u32> {
    let links = match cx.field(field, links) {
        Some(Links::List(links)) => links,
        Some(Links::Keyed(links)) => links.into_values().collect(),
        None => Vec::new(),
    };
    let mut numbers: Vec<u32> = links
        .into_iter()
        .filter_map(|link| {
            let id = cx.field(field, Some(link))?.id;
            let id = cx.required_field(field, id)?;
            let number = id.strip_prefix("/recommendations/").map(str::parse::<u32>);
            let number = cx.required(field, number)?;
            cx.optional(field, Some(number))
        })
        .collect();
    numbers.sort_unstable();
    numbers.dedup();
    numbers
}

/// One eEML recommendation page: a medicine's listing for one indication.
#[derive(Deserialize)]
pub(crate) struct Recommendation {
    #[serde(rename = "guidelineSubject")]
    subject: Option<Field<Subject>>,
    #[serde(rename = "evidenceOrigin")]
    evidence: Option<Field<Vec<Option<Field<NativeText>>>>>,
    section: Option<Field<SectionName>>,
    #[serde(rename = "drugUnit")]
    units: Option<Field<Vec<Field<NativeText>>>>,
    list: Option<Field<String>>,
    #[serde(rename = "includedInEmlc")]
    children: Option<Field<bool>>,
    #[serde(rename = "statusForIndication")]
    status: Option<Field<NativeText>>,
    history: Option<Field<Vec<Field<Decision>>>>,
    #[serde(rename = "therapeuticEquivalents")]
    equivalents: Option<Field<Vec<Field<Equivalent>>>>,
}
impl Reply for Recommendation {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Deserialize)]
struct Subject {
    code: Option<Field<Vec<Field<Code>>>>,
    drug: Option<Field<Vec<Field<Named>>>>,
    indication: Option<Field<Indication>>,
}
#[derive(Deserialize)]
struct Code {
    #[serde(rename = "codeValue")]
    value: Option<Field<NativeText>>,
    #[serde(rename = "codingSystem")]
    system: Option<Field<String>>,
}
#[derive(Deserialize)]
struct Named {
    name: Option<Field<NativeText>>,
}
#[derive(Deserialize)]
struct Indication {
    name: Option<Field<NativeText>>,
    code: Option<Field<Code>>,
}
#[derive(Deserialize)]
struct SectionName {
    name: Option<Field<NativeText>>,
}
#[derive(Deserialize)]
struct Decision {
    #[serde(rename = "@type")]
    kind: Option<Field<NativeText>>,
    #[serde(rename = "guidelineDate")]
    date: Option<Field<String>>,
    list: Option<Field<String>>,
}
#[derive(Deserialize)]
struct Equivalent {
    drug: Option<Field<Named>>,
}
/// The eEML's list names, read plainly.
fn list_word(value: Option<Field<String>>, cx: &mut Problems) -> Option<Text> {
    let value = cx.field("list", value)?;
    let word = match value.as_str() {
        "CoreList" => "core",
        "ComplementaryList" => "complementary",
        other => other,
    };
    cx.text("list", Some(word))
}
fn date(
    field: &str,
    value: Option<Field<String>>,
    cx: &mut Problems,
) -> Option<biodata::PartialDate> {
    let value = cx.field(field, value)?;
    cx.optional(field, Some(value.parse::<biodata::PartialDate>()))
}
fn atc(code: Code, cx: &mut Problems) -> Option<Text> {
    let system = cx.field("guidelineSubject.code", code.system);
    if system.as_deref() != Some("ATC") {
        return None;
    }
    cx.source_field("guidelineSubject.code", code.value)
}
impl Recommendation {
    /// One listing; `previous` holds the last committee text, so a repeat is omitted.
    pub(crate) fn map(
        self,
        id: u32,
        url: &str,
        asked: Option<&Text>,
        previous: &mut Option<Text>,
        cx: &mut Problems,
    ) -> Option<DrugEssentialMedicineListing> {
        let subject = cx.field("guidelineSubject", self.subject);
        let (codes, drugs, indication) = match subject {
            Some(s) => (s.code, s.drug, s.indication),
            None => (None, None, None),
        };
        let listed: Vec<Text> = cx
            .fields("guidelineSubject.drug", drugs)
            .into_vec()
            .into_iter()
            .filter_map(|drug| cx.source_field("guidelineSubject.drug", drug.name))
            .collect();
        let alternative_to = listed
            .first()
            .filter(|listed| {
                asked.is_some_and(|asked| !asked.as_str().eq_ignore_ascii_case(listed.as_str()))
            })
            .cloned();
        let indication = cx.field("guidelineSubject.indication", indication);
        let (indication, code) = match indication {
            Some(i) => (i.name, i.code),
            None => (None, None),
        };
        let icd11 = cx
            .field("guidelineSubject.indication.code", code)
            .and_then(|code| cx.source_field("guidelineSubject.indication.code", code.value));
        let decisions: Box<[DrugEssentialMedicineDecision]> = cx
            .fields("history", self.history)
            .into_vec()
            .into_iter()
            .filter_map(|decision| {
                Some(DrugEssentialMedicineDecision {
                    decision: cx.required_source_field("history.@type", decision.kind)?,
                    date: date("history.guidelineDate", decision.date, cx),
                    list: list_word(decision.list, cx),
                })
            })
            .collect();
        let added = decisions
            .iter()
            .filter(|d| d.decision.as_str() == "DecisionToAdd")
            .filter_map(|d| d.date)
            .min_by_key(|d| (d.year(), d.month(), d.day()));
        let committee = cx
            .field("evidenceOrigin", self.evidence)
            .and_then(|parts| parts.into_iter().next().flatten())
            .and_then(|text| cx.source_field("evidenceOrigin", Some(text)));
        let committee = if committee.is_some() && committee == *previous {
            None
        } else {
            previous.clone_from(&committee);
            committee
        };
        Some(DrugEssentialMedicineListing {
            id: crate::mapped::record(id)?,
            url: cx.text("url", Some(url))?,
            alternative_to,
            list: list_word(self.list, cx),
            section: cx
                .field("section", self.section)
                .and_then(|s| cx.source_field("section.name", s.name)),
            indication: cx.source_field("guidelineSubject.indication.name", indication),
            icd11,
            status: cx.source_field("statusForIndication", self.status),
            added,
            decisions,
            children: cx.field("includedInEmlc", self.children),
            alternatives: cx
                .fields("therapeuticEquivalents", self.equivalents)
                .into_vec()
                .into_iter()
                .filter_map(|e| {
                    let drug = cx.field("therapeuticEquivalents.drug", e.drug)?;
                    cx.source_field("therapeuticEquivalents.drug", drug.name)
                })
                .collect(),
            atc: cx
                .fields("guidelineSubject.code", codes)
                .into_vec()
                .into_iter()
                .filter_map(|code| atc(code, cx))
                .collect(),
            formulations: cx
                .fields("drugUnit", self.units)
                .into_vec()
                .into_iter()
                .map(Into::into)
                .collect(),
            committee,
        })
    }
}

/// Each medicine on an eEML search page: the name in its heading's `title` and its eEML number.
pub(crate) fn search_hits(page: &str) -> Vec<(String, String)> {
    const HEADING: &str = "<h5 class=\"medicine-name\" title=\"";
    const LINK: &str = "/medicines/";
    let mut hits = Vec::new();
    for part in page.split(HEADING).skip(1) {
        let Some((title, rest)) = part.split_once('"') else {
            continue;
        };
        let heading = rest.split("</h5>").next().unwrap_or(rest);
        let Some((_, after)) = heading.split_once(LINK) else {
            continue;
        };
        let number: String = after.chars().take_while(char::is_ascii_digit).collect();
        if !number.is_empty() {
            hits.push((unescape(title), number));
        }
    }
    hits
}
fn unescape(value: &str) -> String {
    value
        .replace("&#039;", "'")
        .replace("&#39;", "'")
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}
