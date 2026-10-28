//! Only the cited source vocabulary maps into the approved common claim model.
use super::{variant::failure, variant_raw as raw};
use biodata::{
    Attribution, Concept, Direction, Predicate, Proposition, Statement, Term, Text, VariantRef,
};

use biomcp_fetch::RequestError;
use biomcp_types::{CivicStatement, Problem, ProblemCode, Reason, Source, SourceRecord};

struct Claim<'a> {
    id: u64,
    kind: &'a biodata::SourceText,
    status: &'a biodata::SourceText,
    significance: &'a biodata::SourceText,
    direction: &'a Text,
    level: Option<Term>,
    classification: Option<Term>,
    disease: Option<&'a raw::Disease>,
    therapies: &'a [raw::Therapy],
    origin: Option<&'a Text>,
    citations: Box<[biodata::Pmid]>,
    assertion: bool,
}
pub(super) fn evidence(
    raw: &raw::Evidence,
    subject: Option<&VariantRef>,
    problems: &mut Vec<Problem>,
) -> Result<CivicStatement, RequestError> {
    use biodata::CivicEvidenceLevelCode as L;
    let code = match raw.evidence_level.as_str() {
        "A" => Some(L::A),
        "B" => Some(L::B),
        "C" => Some(L::C),
        "D" => Some(L::D),
        "E" => Some(L::E),
        _ => None,
    };
    let citations: Box<[biodata::Pmid]> = if raw.source.source_type.as_str() == "PUBMED" {
        raw.source
            .citation_id
            .as_ref()
            .and_then(|id| id.as_str().parse().ok())
            .into_iter()
            .collect()
    } else {
        Box::new([])
    };
    build(
        Claim {
            id: raw.id,
            kind: &raw.evidence_type,
            status: &raw.status,
            significance: &raw.significance,
            direction: &raw.evidence_direction,
            level: Some(Term {
                label: raw.evidence_level.clone(),
                coding: code.map(biodata::Coding::CivicEvidenceLevel),
            }),
            classification: classification(&raw.significance),
            disease: raw.disease.as_ref(),
            therapies: &raw.therapies,
            origin: None,
            citations,
            assertion: false,
        },
        subject,
        problems,
    )
}
pub(super) fn assertion(
    raw: &raw::Assertion,
    subject: Option<&VariantRef>,
    problems: &mut Vec<Problem>,
) -> Result<CivicStatement, RequestError> {
    let (classification, level) = match amp(raw.amp_level.as_ref()) {
        Some((classification, level)) => (Some(classification), level),
        None => (classification(&raw.significance), None),
    };
    build(
        Claim {
            id: raw.id,
            kind: &raw.assertion_type,
            status: &raw.status,
            significance: &raw.significance,
            direction: &raw.assertion_direction,
            classification,
            level,
            disease: raw.disease.as_ref(),
            therapies: &raw.therapies,
            origin: Some(&raw.variant_origin),
            citations: Box::new([]),
            assertion: true,
        },
        subject,
        problems,
    )
}
fn build(
    row: Claim<'_>,
    subject: Option<&VariantRef>,
    problems: &mut Vec<Problem>,
) -> Result<CivicStatement, RequestError> {
    let prefix = if row.assertion { "AID" } else { "EID" };
    let path = if row.assertion {
        "assertions"
    } else {
        "evidence"
    };
    let id = Text::new(&format!("{prefix}{}", row.id)).map_err(|_| failure(Reason::BadReply))?;
    let url = Text::new(&format!("https://civicdb.org/{path}/{}", row.id))
        .map_err(|_| failure(Reason::BadReply))?;
    let kind = if row.assertion {
        biomcp_types::CivicKind::from_assertion(row.kind.as_str().to_owned())
    } else {
        biomcp_types::CivicKind::from(row.kind.as_str().to_owned())
    };
    let status = biomcp_types::CivicStatus::from(row.status.as_str().to_owned());
    let significance = if row.assertion {
        biomcp_types::CivicSignificance::from_assertion(row.significance.as_str().to_owned())
    } else {
        biomcp_types::CivicSignificance::from_evidence(row.significance.as_str().to_owned())
    };
    let native = native_claim(&row, &id, &url);
    let proposition = (!kind.is_unknown())
        .then(|| proposition(row.kind.as_str()))
        .flatten();
    let statement = subject
        .zip(proposition)
        .zip(native.classification.clone())
        .map(|((subject, proposition), classification)| Statement {
            subject: subject.clone(),
            proposition,
            predicate: predicate(row.kind.as_str(), row.significance.as_str()),
            direction: match row.direction.as_str() {
                "SUPPORTS" => Some(Direction::Supports),
                "DOES_NOT_SUPPORT" => Some(Direction::Disputes),
                _ => None,
            },
            classification,
            strength: native.strength.clone(),
            condition: native.condition.clone(),
            therapies: native.therapies.clone(),
            allele_origin: native.allele_origin.clone(),
            gene_context: None,
            criteria: Box::new([]),
            citations: native.citations.clone(),
            attribution: native.attribution.clone(),
        });
    let native = statement.is_none().then_some(native);
    if statement.is_none() {
        problems.push(Problem {
            source: Source::Civic,
            record: Some(id.clone()),
            field: "statement".into(),
            code: ProblemCode::ValueUnknown,
        });
    }
    for (unknown, field) in [
        (
            kind.is_unknown(),
            if row.assertion {
                "assertionType"
            } else {
                "evidenceType"
            },
        ),
        (status.is_unknown(), "status"),
        (significance.is_unknown(), "significance"),
    ] {
        if unknown {
            crate::mapped::push_problem(problems, Source::Civic, Some(id.clone()), field);
        }
    }
    Ok(CivicStatement {
        statement_unmapped: statement.is_none(),
        statement,
        native,
        record: SourceRecord { id, url: Some(url) },
        kind,
        status,
        direction: Some(row.direction.clone()),
        significance: Some(significance),
    })
}
fn native_claim(row: &Claim<'_>, id: &Text, url: &Text) -> biomcp_types::CivicNativeClaim {
    biomcp_types::CivicNativeClaim {
        classification: row.classification.clone(),
        strength: row.level.clone(),
        condition: row.disease.map(|d| Concept {
            id: d
                .doid
                .as_ref()
                .and_then(|id| format!("DOID:{id}").parse().ok()),
            label: d.name.clone(),
        }),
        therapies: row
            .therapies
            .iter()
            .map(|t| Concept {
                id: None,
                label: t.name.clone(),
            })
            .collect(),
        allele_origin: row.origin.map(|label| Term {
            label: label.clone(),
            coding: None,
        }),
        citations: row.citations.clone(),
        attribution: Attribution {
            source: Source::Civic.text(),
            record: Some(id.clone()),
            version: None,
            release: None,
            submitter: None,
            evaluated: None,
            url: Some(url.clone()),
        },
    }
}
fn proposition(kind: &str) -> Option<Proposition> {
    // https://docs.civicdb.org/en/latest/model/evidence/type.html
    match kind {
        "PREDICTIVE" => Some(Proposition::TherapeuticResponse),
        "DIAGNOSTIC" => Some(Proposition::Diagnostic),
        "PROGNOSTIC" => Some(Proposition::Prognostic),
        "PREDISPOSING" => Some(Proposition::Pathogenicity),
        "ONCOGENIC" => Some(Proposition::Oncogenicity),
        "FUNCTIONAL" => Some(Proposition::FunctionalImpact),
        _ => None,
    }
}
fn predicate(kind: &str, significance: &str) -> Option<Predicate> {
    // https://docs.civicdb.org/en/latest/model/evidence/significance.html
    match (kind, significance) {
        ("PREDICTIVE", "SENSITIVITYRESPONSE") => Some(Predicate::PredictsSensitivityTo),
        ("PREDICTIVE", "RESISTANCE") => Some(Predicate::PredictsResistanceTo),
        ("DIAGNOSTIC", "POSITIVE") => Some(Predicate::IsDiagnosticInclusionCriterionFor),
        ("DIAGNOSTIC", "NEGATIVE") => Some(Predicate::IsDiagnosticExclusionCriterionFor),
        ("PROGNOSTIC", "BETTER_OUTCOME") => Some(Predicate::AssociatedWithBetterOutcomeFor),
        ("PROGNOSTIC", "POOR_OUTCOME") => Some(Predicate::AssociatedWithWorseOutcomeFor),
        ("FUNCTIONAL", value) if value != "NA" => Some(Predicate::ImpactsFunctionOf),
        _ => None,
    }
}
fn amp(label: Option<&Text>) -> Option<(Term, Option<Term>)> {
    use biodata::{AmpAscoCap2017LevelCode as L, AmpAscoCap2017TierCode as T, Coding};

    let label = label?;
    let (tier, level) = match label.as_str() {
        "TIER_I_LEVEL_A" => (T::TierI, Some(L::LevelA)),
        "TIER_I_LEVEL_B" => (T::TierI, Some(L::LevelB)),
        "TIER_II_LEVEL_C" => (T::TierIi, Some(L::LevelC)),
        "TIER_II_LEVEL_D" => (T::TierIi, Some(L::LevelD)),
        "TIER_III" => (T::TierIii, None),
        "TIER_IV" => (T::TierIv, None),
        _ => return None,
    };
    Some((
        Term {
            label: label.clone(),
            coding: Some(Coding::AmpAscoCap2017Tier(tier)),
        },
        level.map(|level| Term {
            label: label.clone(),
            coding: Some(Coding::AmpAscoCap2017Level(level)),
        }),
    ))
}

fn classification(label: &biodata::SourceText) -> Option<Term> {
    Text::new(label.as_str()).ok().map(|label| Term {
        label,
        coding: None,
    })
}
