//! Native claim identity survives malformed optional clinical content.
use super::{claim_vocab as vocab, variant_map, variant_raw as raw};
use crate::mapped::Problems;
use biodata::{Attribution, Concept, Direction, Statement, Term, Text, VariantRef};
use biomcp_types::{CivicCondition, CivicKind, CivicNativeClaim, CivicSignificance, CivicStatement, CivicStatus, CivicTherapy, DrugHeader, DrugSource, Source, SourceRecord};

pub(super) struct MappedClaim {
    pub claim: CivicStatement,
    pub therapies: Box<[DrugHeader]>,
}
struct Claim {
    id: u64,
    kind: Option<CivicKind>,
    status: Option<CivicStatus>,
    significance: Option<CivicSignificance>,
    direction: Option<Text>,
    native: CivicNativeClaim,
    therapies: Box<[DrugHeader]>,
    assertion: bool,
    context_bad: bool,
}
pub(super) fn evidence(raw: raw::Evidence, subject: Option<&VariantRef>, context_bad: bool, cx: &mut Problems) -> Option<MappedClaim> {
    let id = variant_map::identity(raw.id, "id", cx)?;
    cx.record = Text::new(&format!("EID{id}")).into_iter().next();
    let before = cx.list.len();
    let kind = cx.field("evidenceType", raw.evidence_type).map(|v| CivicKind::from(v.as_str().to_owned()));
    let status = cx.field("status", raw.status);
    let significance = cx.field("significance", raw.significance).map(|v| CivicSignificance::from_evidence(v.as_str().to_owned()));
    let direction = cx.field("evidenceDirection", raw.evidence_direction);
    let condition = condition(raw.disease, cx);
    let therapies = therapies(raw.therapies, cx);
    let context_bad = context_bad || before != cx.list.len();
    let strength = cx.field("evidenceLevel", raw.evidence_level).map(|label| evidence_level(label, cx));
    let citations = citations(raw.source, cx);
    let native = native(significance.as_ref().and_then(|v| term(v.as_str())), strength, condition, &therapies.0, None, citations);
    build(Claim {id, kind, status, significance, direction, native, therapies: therapies.1, assertion: false, context_bad}, subject, cx)
}
pub(super) fn assertion(raw: raw::Assertion, subject: Option<&VariantRef>, context_bad: bool, cx: &mut Problems) -> Option<MappedClaim> {
    let id = variant_map::identity(raw.id, "id", cx)?;
    cx.record = Text::new(&format!("AID{id}")).into_iter().next();
    let before = cx.list.len();
    let kind = cx.field("assertionType", raw.assertion_type).map(|v| CivicKind::from_assertion(v.as_str().to_owned()));
    let status = cx.field("status", raw.status);
    let significance = cx.field("significance", raw.significance).map(|v| CivicSignificance::from_assertion(v.as_str().to_owned()));
    let direction = cx.field("assertionDirection", raw.assertion_direction);
    let condition = condition(raw.disease, cx);
    let therapies = therapies(raw.therapies, cx);
    let origin = cx.field("variantOrigin", raw.variant_origin).map(|label| Term {label, codings: Box::default()});
    let amp = cx.field("ampLevel", raw.amp_level);
    let (classification, strength) = if let Some(label) = amp {
        match vocab::amp(Some(&label)) {
            Some(value) => (Some(value.0), value.1),
            None => {cx.required::<()>("ampLevel", None); (Some(Term {label, codings: Box::default()}), None)}
        }
    } else { (significance.as_ref().and_then(|v| term(v.as_str())), None) };
    let native = native(classification, strength, condition, &therapies.0, origin, Box::default());
    build(Claim {id, kind, status, significance, direction, native, therapies: therapies.1, assertion: true, context_bad: context_bad || before != cx.list.len()}, subject, cx)
}
fn term(value: &str) -> Option<Term> {
    Text::from_source(value).into_iter().next().map(|label| Term {label, codings: Box::default()})
}
fn condition(raw: Option<crate::raw::Field<raw::Disease>>, cx: &mut Problems) -> Option<CivicCondition> {
    let raw = cx.field("disease", raw)?;
    let label = cx.source_field("disease.name", raw.name);
    let id = cx.field("disease.doid", raw.doid).and_then(|id| cx.optional("disease.doid", Some(format!("DOID:{id}").parse())));
    Some(CivicCondition {id, label})
}
fn therapies(raw: Option<crate::raw::Field<Vec<crate::raw::Field<raw::Therapy>>>>, cx: &mut Problems) -> (Box<[CivicTherapy]>, Box<[DrugHeader]>) {
    let mut native = Vec::new();
    let mut headers = Vec::new();
    for row in cx.fields("therapies", raw).into_vec() {
        let id = cx.field("therapies.id", row.id).and_then(|id| cx.optional("therapies.id", Some(if raw::id(id) { Ok(id) } else { Err(()) })));
        let label = cx.source_field("therapies.name", row.name);
        if let Some(id) = id {
            if let Some(id) = cx.optional("therapies.id", Some(id.to_string().parse())) {
                headers.push(DrugHeader {id, source: DrugSource::Civic, name: label.clone(), identity_level: biodata::DrugIdentityLevel::Unknown, matched_name: None, r#type: None, version: None, url: None});
            }
        }
        native.push(CivicTherapy {id, label});
    }
    (native.into(), headers.into())
}
fn citations(raw: Option<crate::raw::Field<raw::Citation>>, cx: &mut Problems) -> Box<[biodata::Pmid]> {
    let Some(raw) = cx.field("source", raw) else { return Box::default(); };
    let kind = cx.field("source.sourceType", raw.source_type);
    let id = cx.field("source.citationId", raw.citation_id);
    if kind.as_ref().is_some_and(|kind| kind.as_str() == "PUBMED") {
        cx.optional("source.citationId", id.map(|id| id.as_str().parse())).into_iter().collect()
    } else { Box::default() }
}
fn evidence_level(label: Text, cx: &mut Problems) -> Term {
    use biodata::CivicEvidenceLevelCode as L;
    let code = match label.as_str() {"A" => Some(L::A), "B" => Some(L::B), "C" => Some(L::C), "D" => Some(L::D), "E" => Some(L::E), _ => {cx.required::<()>("evidenceLevel", None); None}};
    Term {label, codings: code.map(biodata::Coding::CivicEvidenceLevel).into_iter().collect()}
}
fn native(classification: Option<Term>, strength: Option<Term>, condition: Option<CivicCondition>, therapies: &[CivicTherapy], allele_origin: Option<Term>, citations: Box<[biodata::Pmid]>) -> CivicNativeClaim {
    CivicNativeClaim {classification, strength, condition, therapies: therapies.into(), allele_origin, citations, attribution: Attribution {source: Source::Civic.text(), record: None, version: None, release: None, submitter: None, evaluated: None, url: None}}
}
fn build(mut row: Claim, subject: Option<&VariantRef>, cx: &mut Problems) -> Option<MappedClaim> {
    let id = cx.optional("id", Some(Text::new(&format!("{}{}", if row.assertion {"AID"} else {"EID"}, row.id))))?;
    let url = cx.optional("id", Some(Text::new(&format!("https://civicdb.org/{}/{}", if row.assertion {"assertions"} else {"evidence"}, row.id))))?;
    cx.record = Some(id.clone());
    row.native.attribution.record = Some(id.clone());
    row.native.attribution.url = Some(url.clone());
    for (unknown, field) in [(row.kind.as_ref().is_some_and(CivicKind::is_unknown), if row.assertion {"assertionType"} else {"evidenceType"}), (row.status.as_ref().is_some_and(CivicStatus::is_unknown), "status"), (row.significance.as_ref().is_some_and(CivicSignificance::is_unknown), "significance")] {
        if unknown {cx.required::<()>(field, None); row.context_bad = true;}
    }
    let direction = match row.direction.as_ref().map(Text::as_str) {Some("SUPPORTS") => Some(Direction::Supports), Some("DOES_NOT_SUPPORT") => Some(Direction::Disputes), Some(_) => {cx.required::<()>(if row.assertion {"assertionDirection"} else {"evidenceDirection"}, None); None}, None => None};
    let statement = (!row.context_bad).then_some(subject).flatten().zip(row.kind.as_ref().and_then(|kind| vocab::proposition(kind.as_str()))).zip(row.native.classification.clone()).filter(|_| direction.is_some() && row.significance.is_some()).map(|((subject, proposition), classification)| Statement {
        subject: subject.clone(), proposition, predicate: row.kind.as_ref().zip(row.significance.as_ref()).and_then(|(kind, significance)| vocab::predicate(kind.as_str(), significance.as_str())), direction, classification, strength: row.native.strength.clone(), condition: row.native.condition.as_ref().and_then(|c| c.label.clone().map(|label| Concept {id: c.id.clone(), label})), therapies: row.native.therapies.iter().filter_map(|t| t.label.clone().map(|label| Concept {id: None, label})).collect(), allele_origin: row.native.allele_origin.clone(), gene_context: None, criteria: Box::default(), citations: row.native.citations.clone(), attribution: row.native.attribution.clone()
    });
    if statement.is_none() && !row.context_bad {cx.required::<()>("statement", None);}
    Some(MappedClaim {claim: CivicStatement {statement_unmapped: statement.is_none(), native: statement.is_none().then_some(row.native), statement, record: SourceRecord {id, url: Some(url)}, kind: row.kind, status: row.status, direction: row.direction, significance: row.significance}, therapies: row.therapies})
}
