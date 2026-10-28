//! Exact DataHub directory metadata under the existing restricted source declaration.
use super::super::SPEC;
use biodata::{StudyBlobSha, StudyFile, StudyFileKind, StudyId, Text};
use biomcp_fetch::{BadReply, Freshness, Leg, Reply, RequestError};
use serde::Deserialize;
use std::collections::BTreeSet;
/// Complete source directory and its acquisition metadata, without file contents.
#[derive(Debug)]
pub struct Directory {
    /// Native ordered references, absent when the exact directory returned 404.
    pub rows: Option<Vec<StudyFile>>,
    /// Publisher LICENSE link, discovered across the whole directory.
    pub license_notice: Option<Text>,
    /// Original response acquisition time.
    pub acquired: u64,
}
#[derive(Deserialize)]
struct Entry {
    name: Text,
    path: Text,
    html_url: Text,
    download_url: Option<Text>,
    sha: StudyBlobSha,
    size: u64,
    #[serde(rename = "type")]
    kind: StudyFileKind,
}
#[derive(Deserialize)]
#[serde(transparent)]
struct Entries(Vec<Entry>);
impl Reply for Entries {
    fn check(&self) -> Result<(), BadReply> {
        let mut paths = BTreeSet::new();
        if self.0.iter().all(|r| paths.insert(r.path.as_str())) {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
impl Entry {
    fn matches(&self, id: &StudyId) -> bool {
        let path = format!("public/{id}/{}", self.name);
        self.path.as_str() == path
            && !self.name.as_str().contains('/')
            && !matches!(self.name.as_str(), "." | "..")
            && self.html_url.as_str()
                == format!(
                    "https://github.com/cBioPortal/datahub/{}/master/{path}",
                    match self.kind {
                        StudyFileKind::File => "blob",
                        StudyFileKind::Directory => "tree",
                    }
                )
            && self.download_url.as_ref().is_none_or(|url| {
                url.as_str()
                    == format!("https://raw.githubusercontent.com/cBioPortal/datahub/master/{path}")
            })
    }
    fn mapped(self) -> StudyFile {
        StudyFile {
            name: self.name,
            path: self.path,
            html_url: self.html_url,
            download_url: self.download_url,
            sha: self.sha,
            repository_blob_size: self.size,
            kind: self.kind,
        }
    }
}
/// Read only one exact contents directory. No entry download URL is requested.
pub fn files(leg: &Leg<'_>, id: &StudyId) -> Result<Directory, RequestError> {
    let url = format!(
        "{}/repos/cBioPortal/datahub/contents/public/{id}",
        super::super::GITHUB_ORIGIN
    );
    let endpoint = SPEC
        .download(&url)?
        .fresh(Freshness::Days(1))
        .bytes(100_000);
    let result = leg.json_received_checked::<Entries>(&endpoint, &[], |r| {
        if r.0.iter().all(|v| v.matches(id)) {
            Ok(())
        } else {
            Err(BadReply)
        }
    });
    let (reply, metadata) = match result {
        Ok(value) => value,
        Err(RequestError::Source(e))
            if e.reason == biomcp_types::Reason::NotFound && e.status == Some(404) =>
        {
            return Ok(Directory {
                rows: None,
                license_notice: None,
                acquired: 0,
            });
        }
        Err(error) => return Err(error),
    };
    let license_notice = reply
        .0
        .iter()
        .find(|r| r.name.as_str() == "LICENSE" && r.kind == StudyFileKind::File)
        .map(|r| r.html_url.clone());
    Ok(Directory {
        rows: Some(reply.0.into_iter().map(Entry::mapped).collect()),
        license_notice,
        acquired: u64::try_from(metadata.fetched).map_err(|_| bad())?,
    })
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: biomcp_types::Source::Cbioportal,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
