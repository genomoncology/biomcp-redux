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
    destinations: biomcp_fetch::Destinations::Primary,
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
    format: RowFormat::NativeCsv {
        max_bytes: biomcp_fetch::MAX_BUNDLE_BYTES,
    },
    fresh: Freshness::Days(7),
    row: PhantomData,
};
/// Explicitly synchronize the source-owned submissions file.
pub fn sync(fetch: &Fetch) -> Result<SyncResult, RequestError> {
    let info = fetch.sync(&SUBMISSIONS)?;
    Ok(SyncResult {
        snapshot: None,
        etag: info.etag,
        last_modified: info.last_modified,
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
#[serde(deny_unknown_fields)]
struct Submission {
    gene_curie: HgncId,
    #[serde(rename = "gene_symbol")]
    _gene_symbol: String,
    #[serde(rename = "disease_original_curie")]
    _disease_original_curie: String,
    #[serde(rename = "disease_original_title")]
    _disease_original_title: String,
    #[serde(rename = "submitted_as_hgnc_id")]
    _submitted_as_hgnc_id: String,
    #[serde(rename = "submitted_as_hgnc_symbol")]
    _submitted_as_hgnc_symbol: String,
    #[serde(rename = "submitted_as_disease_id")]
    _submitted_as_disease_id: String,
    #[serde(rename = "submitted_as_disease_name")]
    _submitted_as_disease_name: String,
    #[serde(rename = "submitted_as_moi_id")]
    _submitted_as_moi_id: String,
    #[serde(rename = "submitted_as_moi_name")]
    _submitted_as_moi_name: String,
    #[serde(rename = "submitted_as_submitter_id")]
    _submitted_as_submitter_id: String,
    #[serde(rename = "submitted_as_submitter_name")]
    _submitted_as_submitter_name: String,
    #[serde(rename = "submitted_as_classification_id")]
    _submitted_as_classification_id: String,
    #[serde(rename = "submitted_as_classification_name")]
    _submitted_as_classification_name: String,
    #[serde(rename = "submitted_as_public_report_url")]
    _submitted_as_public_report_url: String,
    #[serde(rename = "submitted_as_notes")]
    _submitted_as_notes: String,
    #[serde(rename = "submitted_as_pmids")]
    _submitted_as_pmids: String,
    #[serde(rename = "submitted_as_assertion_criteria_url")]
    _submitted_as_assertion_criteria_url: String,
    #[serde(rename = "submitted_as_submission_id")]
    _submitted_as_submission_id: String,

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
    #[serde(deserialize_with = "optional_text")]
    submitted_as_date: Option<Text>,
    #[serde(deserialize_with = "optional_text")]
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

fn optional_text<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<Text>, D::Error> {
    let value = String::deserialize(d)?;
    if value.is_empty() {
        Ok(None)
    } else {
        Text::new(&value)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}
