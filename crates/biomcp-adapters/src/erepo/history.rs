//! Selected SEPIO documents supply their own facts; summary rows supply routing only.
use biodata::{Attribution, VariantRef};

use biomcp_fetch::{BadReply, Freshness, Leg, PathSegment, Reply, RequestError};
use biomcp_types::{
    AssertionAssessment, AssertionCondition, AssertionDetail, AssertionDocument, AssertionDomain,
    AssertionKind, Source, VariantAssertion,
};
use serde::Deserialize;
const DOCUMENT: biomcp_fetch::Endpoint = super::SPEC
    .get("/evrepo/api/summary/classification")
    .fresh(Freshness::Days(1));
pub(super) fn fetch(
    leg: &Leg<'_>,
    caid: &str,
    record: &str,
    uuid: &str,
    version: &str,
) -> Result<VariantAssertion, RequestError> {
    let path = [uuid, "doc", "sepio", "version", version]
        .into_iter()
        .map(PathSegment::new)
        .collect::<Result<Vec<_>, _>>()
        .map_err(RequestError::Input)?;
    let endpoint = DOCUMENT.clone().with_segments(&path);
    let (reply, _) = leg.json_received_checked::<DocumentReply>(&endpoint, &[], |r| {
        let data = &r.data;
        if data.uuid == uuid
            && data.metadata.version == version
            && data.variant.id == format!("CAR:{caid}")
            && data.clone().map(caid, record, version).is_ok()
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    })?;
    reply.data.map(caid, record, version)
}
#[derive(Clone, Deserialize)]
struct DocumentReply {
    data: Document,
}
impl Reply for DocumentReply {
    fn check(&self) -> Result<(), BadReply> {
        if self.data.kind == "VariantPathogenicityInterpretation"
            && !self.data.id.is_empty()
            && !self.data.outcome.label.is_empty()
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
#[derive(Clone, Deserialize)]
struct Document {
    uuid: String,
    id: String,
    #[serde(rename = "type")]
    kind: String,
    metadata: Metadata,
    variant: Identifier,
    condition: Label,
    #[serde(rename = "statementOutcome")]
    outcome: Label,
    #[serde(rename = "evidenceLine")]
    lines: Option<Vec<Line>>,
}
#[derive(Clone, Deserialize)]
struct Metadata {
    version: String,
}
#[derive(Clone, Deserialize)]
struct Identifier {
    id: String,
}
#[derive(Clone, Deserialize)]
struct Label {
    label: String,
    id: Option<String>,
    #[serde(rename = "@id")]
    iri: Option<String>,
}
#[derive(Clone, Deserialize)]
struct Line {
    #[serde(rename = "evidenceItem")]
    items: Option<Vec<Assessment>>,
}
#[derive(Clone, Deserialize)]
struct Assessment {
    #[serde(rename = "type")]
    kind: String,
    criterion: Option<Criterion>,
    #[serde(rename = "statementOutcome")]
    outcome: Option<Label>,
}
#[derive(Clone, Deserialize)]
struct Criterion {
    label: String,
    id: Option<String>,
    #[serde(rename = "@id")]
    iri: Option<String>,
    #[serde(rename = "defaultStrength")]
    strength: Option<Label>,
}
impl Document {
    fn map(
        self,
        caid: &str,
        record: &str,
        version: &str,
    ) -> Result<VariantAssertion, RequestError> {
        let text = |value: &str| crate::mapped::text(value).map_err(|_| super::bad());
        let attribution = Attribution {
            source: Source::Erepo.text(),
            record: Some(text(record)?),
            version: Some(text(version)?),
            release: None,
            submitter: None,
            evaluated: None,
            url: Some(text(&format!(
                "https://erepo.clinicalgenome.org/evrepo/ui/classification/{}?version={version}",
                self.uuid
            ))?),
        };
        let condition = AssertionCondition {
            database: None,
            accession: None,
            id: self
                .condition
                .id
                .map(|v| v.parse().map_err(|_| super::bad()))
                .transpose()?,
            label: Some(
                crate::mapped::source_text(&self.condition.label).map_err(|_| super::bad())?,
            ),
        };
        let mut assessments = Vec::new();
        for line in self.lines.unwrap_or_default() {
            for item in line.items.unwrap_or_default() {
                if item.kind == "CriterionAssessment" {
                    assessments.push(item.map()?);
                }
            }
        }
        let detail = AssertionDetail {
            conditions: vec![condition].into(),
            document: Some(AssertionDocument {
                uuid: text(&self.uuid)?,
                native_id: text(&self.id)?,
                document_type: text(&self.kind)?,
                outcome_id: self.outcome.id.as_deref().map(text).transpose()?,
                outcome_iri: self.outcome.iri.as_deref().map(text).transpose()?,
                assessments: assessments.into(),
            }),
            ..Default::default()
        };
        Ok(crate::assertion::row(
            AssertionKind::Erepo,
            AssertionDomain::Germline,
            crate::mapped::source_text(&self.outcome.label).map_err(|_| super::bad())?,
            attribution,
            VariantRef::allele(caid.parse().map_err(|_| super::bad())?),
            detail,
            None,
        ))
    }
}
impl Assessment {
    fn map(self) -> Result<AssertionAssessment, RequestError> {
        let text = |value: &str| crate::mapped::text(value).map_err(|_| super::bad());
        let criterion = self.criterion.ok_or_else(super::bad)?;
        let outcome = self.outcome.ok_or_else(super::bad)?;
        Ok(AssertionAssessment {
            criterion: crate::mapped::source_text(&criterion.label).map_err(|_| super::bad())?,
            criterion_id: criterion.id.as_deref().map(text).transpose()?,
            criterion_iri: criterion.iri.as_deref().map(text).transpose()?,
            default_strength: criterion
                .strength
                .as_ref()
                .map(|s| crate::mapped::source_text(&s.label).map_err(|_| super::bad()))
                .transpose()?,
            default_strength_id: criterion
                .strength
                .as_ref()
                .and_then(|s| s.id.as_deref())
                .map(text)
                .transpose()?,
            default_strength_iri: criterion
                .strength
                .as_ref()
                .and_then(|s| s.iri.as_deref())
                .map(text)
                .transpose()?,
            outcome_conflict: outcome_conflict(&outcome),
            outcome: crate::mapped::source_text(&outcome.label).map_err(|_| super::bad())?,
            outcome_id: outcome.id.as_deref().map(text).transpose()?,
            outcome_iri: outcome.iri.as_deref().map(text).transpose()?,
        })
    }
}

// ClinGen's documented CriterionAssessment outcomes are 0000223 Met and 0000224 Not Met.
// Preserve contrary source values without silently repairing them.
// https://dataexchange.clinicalgenome.org/interpretation/entities/CriterionAssessment.html
fn outcome_conflict(outcome: &Label) -> bool {
    [outcome.id.as_deref(), outcome.iri.as_deref()]
        .into_iter()
        .flatten()
        .any(|id| {
            matches!(
                (id, outcome.label.as_str()),
                ("SEPIO:0000224", "Met") | ("SEPIO:0000223", "Not Met")
            )
        })
}
