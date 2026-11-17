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
    /// The recommendations that list this medicine; every eEML medicine page carries it, so a
    /// page without it is a changed site, not a medicine with no listings.
    guideline: Field<Links>,
    /// The recommendations that name this medicine as a therapeutic alternative; required too.
    #[serde(rename = "equivalentFor")]
    equivalent_for: Field<Links>,
}
impl Reply for Medicine {
    fn check(&self) -> Result<(), BadReply> {
        for links in [&self.guideline, &self.equivalent_for] {
            numbers(links)?;
        }
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
/// The medicine's listings: each recommendation number, and whether the medicine is only the
/// listing's therapeutic alternative there.
pub(crate) struct Listed {
    pub(crate) name: Option<Text>,
    pub(crate) numbers: Vec<(u32, bool)>,
}
impl Medicine {
    /// Recommendations listing the medicine first, then those naming it as an alternative, each
    /// in number order; a link that is not a recommendation is a bad reply, as `check` says.
    pub(crate) fn map(self, cx: &mut Problems) -> Result<Listed, BadReply> {
        let name = cx.source_field("name", self.name);
        let listed = numbers(&self.guideline)?;
        let mut found: Vec<(u32, bool)> = listed.iter().map(|n| (*n, false)).collect();
        for number in numbers(&self.equivalent_for)? {
            if !listed.contains(&number) {
                found.push((number, true));
            }
        }
        Ok(Listed {
            name,
            numbers: found,
        })
    }
}
/// Every linked recommendation number, sorted once each; a bad reply when any link is not one.
fn numbers(links: &Field<Links>) -> Result<Vec<u32>, BadReply> {
    let links: Vec<&Field<Link>> = match links.as_ref().ok_or(BadReply)? {
        Links::List(links) => links.iter().collect(),
        Links::Keyed(links) => links.values().collect(),
    };
    let mut numbers = Vec::new();
    for link in links {
        let id = link
            .as_ref()
            .and_then(|link| link.id.as_ref())
            .and_then(Field::as_ref)
            .ok_or(BadReply)?;
        let number = id.strip_prefix("/recommendations/").ok_or(BadReply)?;
        numbers.push(number.parse::<u32>().map_err(|_| BadReply)?);
    }
    numbers.sort_unstable();
    numbers.dedup();
    Ok(numbers)
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
    /// Every recommendation page states its status, which decides whether it is a listing.
    fn check(&self) -> Result<(), BadReply> {
        self.status
            .as_ref()
            .and_then(Field::as_ref)
            .map(|_| ())
            .ok_or(BadReply)
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
/// Each distinct Committee decision, in the eEML's order. The eEML repeats a decision once per
/// formulation it covers; each is kept once.
fn decisions(
    history: Option<Field<Vec<Field<Decision>>>>,
    cx: &mut Problems,
) -> Box<[DrugEssentialMedicineDecision]> {
    let mut decisions: Vec<DrugEssentialMedicineDecision> = Vec::new();
    for decision in cx.fields("history", history) {
        let Some(kind) = cx.required_source_field("history.@type", decision.kind) else {
            continue;
        };
        let decision = DrugEssentialMedicineDecision {
            decision: kind,
            date: date("history.guidelineDate", decision.date, cx),
            list: list_word(decision.list, cx),
        };
        if !decisions.contains(&decision) {
            decisions.push(decision);
        }
    }
    decisions.into()
}
/// The Committee's recommendation, the first of the eEML's evidence texts.
fn committee(
    evidence: Option<Field<Vec<Option<Field<NativeText>>>>>,
    cx: &mut Problems,
) -> Option<Text> {
    cx.field("evidenceOrigin", evidence)
        .and_then(|parts| parts.into_iter().next().flatten())
        .and_then(|text| cx.source_field("evidenceOrigin", Some(text)))
}
/// The eEML status of a listing on the Model List; the medicine page also links applications
/// the Committee rejected, which are not listings.
const ADDED: &str = "AddedToEml";
impl Recommendation {
    /// One listing, or `None` when the recommendation is not on the Model List (its status is not
    /// `AddedToEml`, such as a rejected application). `alternative` says the medicine asked for
    /// is only the listing's therapeutic alternative.
    pub(crate) fn map(
        self,
        id: u32,
        url: &str,
        alternative: bool,
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
        let alternative_to = listed.first().filter(|_| alternative).cloned();
        let indication = cx.field("guidelineSubject.indication", indication);
        let (indication, code) = match indication {
            Some(i) => (i.name, i.code),
            None => (None, None),
        };
        let icd11 = cx
            .field("guidelineSubject.indication.code", code)
            .and_then(|code| cx.source_field("guidelineSubject.indication.code", code.value));
        let decisions = decisions(self.history, cx);
        let added = decisions
            .iter()
            .filter(|d| d.decision.as_str() == "DecisionToAdd")
            .filter_map(|d| d.date)
            .min_by_key(|d| (d.year(), d.month(), d.day()));
        let status = cx.source_field("statusForIndication", self.status);
        if status.as_ref().map(Text::as_str) != Some(ADDED) {
            return None;
        }
        let committee = committee(self.evidence, cx);
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
            status,
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
