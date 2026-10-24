use biodata::{
    AssertionCondition, AssertionDetail, AssertionDomain, AssertionKind, Attribution, Source, Text,
    VariantAssertion, VariantRef,
};
use biomcp_fetch::{BadReply, Reply, RequestError};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct ReplyBody {
    #[serde(default)]
    pub data: Option<Vec<Record>>,
    status: Status,
}
#[derive(Deserialize)]
struct Status {
    code: u16,
    name: String,
}
impl Reply for ReplyBody {
    fn check(&self) -> Result<(), BadReply> {
        match self.status.code {
            200 if self.data.as_ref().is_some_and(|rows| {
                rows.iter().all(|r| {
                    r.ca_id.parse::<biodata::VariantId>().is_ok()
                        && !r.id.is_empty()
                        && !r.classification.is_empty()
                })
            }) =>
            {
                Ok(())
            }
            404 if self.status.name == "No Records Found"
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
#[derive(Deserialize)]
pub(super) struct Record {
    #[serde(rename = "caId")]
    pub ca_id: String,
    #[serde(rename = "PCERDocID")]
    id: String,
    #[serde(rename = "docVersion")]
    version: String,
    classification: String,
    #[serde(default)]
    uuid: Option<String>,
    #[serde(default)]
    condition: Option<String>,
    #[serde(default, rename = "mondoId")]
    mondo: Option<String>,
    #[serde(default)]
    ep: Option<String>,
    #[serde(default)]
    gene: Option<String>,
    #[serde(default, rename = "metCodes")]
    criteria: Vec<String>,
    #[serde(default, rename = "unMetCodes")]
    unmet: Vec<String>,
    #[serde(default, rename = "summaryDesc")]
    summary: Option<String>,
    #[serde(default, rename = "approvedDate")]
    approved: Option<String>,
    #[serde(default, rename = "publishedDate")]
    published: Option<String>,
    #[serde(default, rename = "versionsList")]
    versions: Vec<String>,
}
impl Record {
    pub(super) fn map(self) -> Result<VariantAssertion, RequestError> {
        let subject = VariantRef::allele(self.ca_id.parse().map_err(|_| super::bad())?);
        let attribution = Attribution {
            source: Source::Erepo,
            record: Some(text(&self.id)?),
            version: Some(text(&self.version)?),
            release: None,
            submitter: self.ep.as_deref().map(text).transpose()?,
            evaluated: None,
            url:self.uuid.as_ref().map(|uuid|text(&format!("https://erepo.clinicalgenome.org/evrepo/ui/classification/{uuid}?version={}",self.version))).transpose()?,
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
            unmet_criteria: strings(self.unmet)?,
            published: self
                .published
                .map(|s| s.parse().map_err(|_| super::bad()))
                .transpose()?,
            approved: self
                .approved
                .map(|s| s.parse().map_err(|_| super::bad()))
                .transpose()?,
            versions: strings(self.versions)?,
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
            statement.criteria = strings(self.criteria)?;
            statement.gene_context = self
                .gene
                .map(|s| s.parse().map_err(|_| super::bad()))
                .transpose()?;
        }
        Ok(row)
    }
}
fn text(value: &str) -> Result<Text, RequestError> {
    Text::new(value).map_err(|_| super::bad())
}
fn strings(values: Vec<String>) -> Result<Box<[Text]>, RequestError> {
    values.iter().map(|s| text(s)).collect()
}
