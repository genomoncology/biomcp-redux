//! GenCC submissions remain separate records keyed by HGNC identity.
use crate::quotas;
use biodata::{HgncId, Text};
use biomcp_fetch::{
    Bundle, Fetch, Find, Freshness, KeyKind, KeyUse, Keys, Leg, RequestError, Row, RowFormat,
    SourceSpec,
};
use biomcp_types::{GenccGene, GenccSubmission, Source, SyncResult};
use serde::Deserialize;
use std::{marker::PhantomData, time::Duration};

/// Public GenCC export and its publisher terms.
pub static SPEC: SourceSpec = SourceSpec {
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "CC0 1.0",
        url: "https://thegencc.org/download",
        usage: biomcp_types::Use::Open,
    },
    source: Source::Gencc,
    base: "https://thegencc.org",
    key: KeyUse::None,
    quota: &quotas::GENCC,
    deadline: Duration::from_secs(8),
};
const SUBMISSIONS: Bundle<Submission> = Bundle {
    spec: &SPEC,
    name: "gencc-submissions",
    path: "/download/action/submissions-export-csv?format=new",
    format: RowFormat::Csv { banner_lines: 0 },
    fresh: Freshness::Days(7),
    row: PhantomData,
};
/// Explicitly synchronize the source-owned submissions file.
pub fn sync(fetch: &Fetch) -> Result<SyncResult, RequestError> {
    let info = fetch.sync(&SUBMISSIONS)?;
    Ok(SyncResult {
        snapshot: None,
        etag: info.etag.map(Into::into),
        last_modified: info.last_modified.map(Into::into),
        duplicate_identities: None,
        rows: info.rows,
        bytes: info.bytes,
    })
}
/// Read each source submission for the checked HGNC identity.
pub fn gene(leg: &Leg<'_>, id: HgncId) -> Result<Option<GenccGene>, RequestError> {
    let (rows, _) = leg.stored_rows(&SUBMISSIONS, Find::Exact(KeyKind::Id, &id.to_string()))?;
    if rows.is_empty() {
        return Ok(None);
    }
    Ok(Some(GenccGene {
        submissions: rows.into_iter().map(Submission::into_record).collect(),
    }))
}
/// Publisher landing page; health never downloads the submissions file.
pub(crate) const HEALTH_PATH: &str = "/";
/// Inspect the stored export without a network request.
pub(crate) fn bundle_state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    leg.stored_bundle_state(&SUBMISSIONS)
}

#[derive(Deserialize)]
struct Submission {
    gene_curie: HgncId,
    sgc_id: Text,
    version_number: Text,
    disease_curie: Text,
    disease_title: Text,
    classification_curie: Text,
    classification_title: Text,
    moi_curie: Text,
    moi_title: Text,
    submitter_curie: Text,
    submitter_title: Text,
    submitted_as_date: Option<Text>,
    submitted_run_date: Option<Text>,
}
impl Row for Submission {
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![(KeyKind::Id, self.gene_curie.to_string().into())],
            words: Vec::new(),
        }
    }
}
impl Submission {
    fn into_record(self) -> GenccSubmission {
        GenccSubmission {
            id: self.sgc_id,
            version: self.version_number,
            disease_id: self.disease_curie,
            disease: self.disease_title,
            classification_id: self.classification_curie,
            classification: self.classification_title,
            mode_of_inheritance_id: self.moi_curie,
            mode_of_inheritance: self.moi_title,
            submitter_id: self.submitter_curie,
            submitter: self.submitter_title,
            evaluated_date: self.submitted_as_date,
            submitted_date: self.submitted_run_date,
        }
    }
}
