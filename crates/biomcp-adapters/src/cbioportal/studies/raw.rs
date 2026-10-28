use biodata::{
    CbioportalCancerTypeId, Pmid, Study, StudyHeader, StudyId, StudyImportTime, StudyResourceCount,
    Text,
};
use biomcp_fetch::{BadReply, KeyKind, Keys, Row, encode_bundle_row};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
/// Checked native catalog row, retaining card fields for later exact reads.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CatalogRow {
    study_id: StudyId,
    name: Text,
    cancer_type_id: CbioportalCancerTypeId,
    description: Box<str>,
    all_sample_count: u64,
    reference_genome: Text,
    pmid: Option<Box<str>>,
    citation: Option<Box<str>>,
    sequenced_sample_count: u64,
    cna_sample_count: u64,
    structural_variant_count: u64,
    treatment_count: u64,
    #[serde(deserialize_with = "crate::strict_input::deserialize")]
    resource_counts: Box<[StudyResourceCount]>,
    import_date: StudyImportTime,
    groups: Box<str>,
    read_permission: bool,
    public_study: bool,
}
impl CatalogRow {
    pub(super) fn header(&self) -> Result<StudyHeader, BadReply> {
        Ok(StudyHeader {
            study_id: self.study_id.clone(),
            name: self.name.clone(),
            cancer_type_id: self.cancer_type_id.clone(),
            all_sample_count: self.all_sample_count,
            pmids: self
                .pmid
                .as_deref()
                .map(|s| {
                    s.split(',')
                        .map(|p| p.trim().parse::<Pmid>().map_err(|_| BadReply))
                        .collect::<Result<Box<[_]>, _>>()
                })
                .transpose()?,
            citation: self.citation.clone(),
            reference_genome: self.reference_genome.clone(),
        })
    }
    pub(super) fn matches(&self, q: &biomcp_types::StudySearch) -> bool {
        q.cancer_type().is_none_or(|v| v == &self.cancer_type_id)
            && q.text().is_none_or(|v| {
                let needle = v.as_str().to_lowercase();
                [
                    self.study_id.as_str(),
                    self.name.as_str(),
                    &self.description,
                ]
                .iter()
                .any(|v| v.to_lowercase().contains(&needle))
            })
    }
    pub(super) fn card(self) -> Result<Study, BadReply> {
        Ok(Study {
            header: self.header()?,
            description: self.description,
            sequenced_sample_count: self.sequenced_sample_count,
            cna_sample_count: self.cna_sample_count,
            structural_variant_count: self.structural_variant_count,
            treatment_count: self.treatment_count,
            patient_count: None,
            resource_counts: self.resource_counts,
            import_date: self.import_date,
            groups: self.groups,
            read_permission: self.read_permission,
            public_study: self.public_study,
        })
    }
}
impl Row for CatalogRow {
    const MAX_BYTES: u64 = 1024 * 1024;
    fn decode_document(body: &[u8]) -> Result<Vec<String>, BadReply> {
        if body.len() > 1024 * 1024 {
            return Err(BadReply);
        }
        let rows: Vec<Self> = biomcp_fetch::decode_bundle_document(body)?;
        if rows.is_empty() {
            return Err(BadReply);
        }
        let mut ids = BTreeSet::new();
        rows.iter()
            .map(|r| {
                if !ids.insert(r.study_id.as_str())
                    || r.resource_counts
                        .iter()
                        .any(|v| v.cancer_study_identifier != r.study_id)
                {
                    return Err(BadReply);
                }
                r.header()?;
                encode_bundle_row(r)
            })
            .collect()
    }
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![
                (KeyKind::Id, self.study_id.to_string().into()),
                (KeyKind::Band, "catalog".into()),
            ],
            words: vec![],
        }
    }
}
