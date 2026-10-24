//! Selected SEPIO documents supply their own facts; summary rows supply routing only.
use biodata::{
    AssertionAssessment, AssertionCondition, AssertionDetail, AssertionDocument, AssertionDomain,
    AssertionKind, Attribution, Source, VariantAssertion, VariantRef,
};
use biomcp_fetch::{BadReply, Freshness, Leg, PathSegment, Reply, RequestError};
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
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    })?;
    reply.data.map(caid, record, version)
}
#[derive(Deserialize)]
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
#[derive(Deserialize)]
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
#[derive(Deserialize)]
struct Metadata {
    version: String,
}
#[derive(Deserialize)]
struct Identifier {
    id: String,
}
#[derive(Deserialize)]
struct Label {
    label: String,
    id: Option<String>,
    #[serde(rename = "@id")]
    iri: Option<String>,
}
#[derive(Deserialize)]
struct Line {
    #[serde(rename = "evidenceItem")]
    items: Option<Vec<Assessment>>,
}
#[derive(Deserialize)]
struct Assessment {
    #[serde(rename = "type")]
    kind: String,
    criterion: Option<Criterion>,
    #[serde(rename = "statementOutcome")]
    outcome: Option<Label>,
}
#[derive(Deserialize)]
struct Criterion {
    label: String,
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
        let text = super::raw::text;
        let attribution = Attribution {
            source: Source::Erepo,
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
            label: Some(text(&self.condition.label)?),
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
            text(&self.outcome.label)?,
            attribution,
            VariantRef::allele(caid.parse().map_err(|_| super::bad())?),
            detail,
            None,
        ))
    }
}
impl Assessment {
    fn map(self) -> Result<AssertionAssessment, RequestError> {
        let text = super::raw::text;
        let criterion = self.criterion.ok_or_else(super::bad)?;
        let outcome = self.outcome.ok_or_else(super::bad)?;
        Ok(AssertionAssessment {
            criterion: text(&criterion.label)?,
            default_strength: criterion.strength.map(|s| text(&s.label)).transpose()?,
            outcome: text(&outcome.label)?,
            outcome_id: outcome.id.as_deref().map(text).transpose()?,
            outcome_iri: outcome.iri.as_deref().map(text).transpose()?,
        })
    }
}
