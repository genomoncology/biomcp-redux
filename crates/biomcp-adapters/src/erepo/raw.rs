use biodata::{Attribution, Text, VariantRef};

use biomcp_fetch::{BadReply, Reply, RequestError};
use biomcp_types::{
    AssertionCondition, AssertionDetail, AssertionDomain, AssertionKind, Source, VariantAssertion,
};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct ReplyBody {
    pub data: Option<Vec<Record>>,
    status: Status,
}
#[derive(Deserialize)]
struct Status {
    code: u16,
    name: String,
    msg: Option<String>,
}
impl Reply for ReplyBody {
    fn check(&self) -> Result<(), BadReply> {
        match self.status.code {
            200 if self.data.as_ref().is_some_and(|rows| {
                rows.iter().all(|r| {
                    r.ca_id.parse::<biodata::VariantId>().is_ok()
                        && !r.id.is_empty()
                        && !r.classification.is_empty()
                        // Mapping validates every field that reaches the output.
                        // Do this before cache admission, using the same rules.
                        && r.clone().map().is_ok()
                })
            }) =>
            {
                Ok(())
            }
            404 if self.status.name == "Not Found"
                && self.status.msg.as_deref() == Some("No records were found for given query")
                && self.data.as_ref().is_none_or(Vec::is_empty) =>
            {
                Ok(())
            }
            _ => Err(BadReply),
        }
    }
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if status == self.status.code {
            self.check()
        } else {
            Err(BadReply)
        }
    }
}
#[derive(Clone, Deserialize)]
pub(super) struct Record {
    #[serde(rename = "caId")]
    pub ca_id: String,
    #[serde(rename = "PCERDocID")]
    pub id: String,
    #[serde(rename = "docVersion")]
    pub version: String,
    classification: String,
    pub uuid: Option<String>,
    condition: Option<String>,
    #[serde(rename = "mondoId")]
    mondo: Option<String>,
    ep: Option<String>,
    gene: Option<String>,
    #[serde(rename = "metCodes")]
    criteria: Option<Vec<String>>,
    #[serde(rename = "unMetCodes")]
    unmet: Option<Vec<String>>,
    #[serde(rename = "summaryDesc")]
    summary: Option<String>,
    #[serde(rename = "approvedDate")]
    approved: Option<String>,
    #[serde(rename = "publishedDate")]
    published: Option<String>,
    #[serde(rename = "versionsList")]
    pub versions: Option<Vec<String>>,
}
impl Record {
    pub(super) fn map(self) -> Result<VariantAssertion, RequestError> {
        let subject = VariantRef::allele(self.ca_id.parse().map_err(|_| super::bad())?);
        let attribution = Attribution {
            source: Source::Erepo.text(),
            record: Some(text(&self.id)?),
            version: Some(text(&self.version)?),
            release: None,
            submitter: self.ep.as_deref().map(text).transpose()?,
            evaluated: None,
            url: self
                .uuid
                .as_ref()
                .map(|uuid| {
                    text(&format!(
                        "https://erepo.clinicalgenome.org/evrepo/ui/classification/{uuid}?version={}",
                        self.version
                    ))
                })
                .transpose()?,
        };
        let conditions = match self.condition {
            Some(label) => vec![AssertionCondition {
                database: None,
                accession: None,
                id: self
                    .mondo
                    .map(|v| v.parse().map_err(|_| super::bad()))
                    .transpose()?,
                label: Some(text(&label)?),
            }],
            None => vec![],
        };
        let detail = AssertionDetail {
            conditions: conditions.into(),
            summary: self.summary.as_deref().map(text).transpose()?,
            unmet_criteria: strings(self.unmet.unwrap_or_default())?,
            published: self
                .published
                .map(|s| s.parse().map_err(|_| super::bad()))
                .transpose()?,
            approved: self
                .approved
                .map(|s| s.parse().map_err(|_| super::bad()))
                .transpose()?,
            versions: strings(self.versions.unwrap_or_default())?,
            ..Default::default()
        };
        let mut row = crate::assertion::row(
            AssertionKind::Erepo,
            AssertionDomain::Germline,
            text(&self.classification)?,
            attribution,
            subject,
            detail,
            None,
        );
        if let Some(statement) = &mut row.statement {
            statement.criteria = strings(self.criteria.unwrap_or_default())?;
            statement.gene_context = self
                .gene
                .map(|s| {
                    biodata::GeneRef::new(text(&s)?, None, None, None).map_err(|_| super::bad())
                })
                .transpose()?;
        }
        Ok(row)
    }
}
pub(super) fn text(value: &str) -> Result<Text, RequestError> {
    Text::new(value).map_err(|_| super::bad())
}
fn strings(values: Vec<String>) -> Result<Box<[Text]>, RequestError> {
    values.iter().map(|s| text(s)).collect()
}
