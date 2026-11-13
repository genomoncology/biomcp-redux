//! Admit native gene–disease–MOI identities before optional curation fields.
//! Publisher SOP v11 pp. 10–11: <https://clinicalgenome.org/site/assets/files/9851/gene-disease_validity_standard_operating_procedures-_version_11_docx.pdf>
//! Export field names: <https://search.clinicalgenome.org/kb/gene-validity/download>
use crate::native_text::NativeText;
use biodata::{GeneSymbol, HgncId, Text};

use crate::{mapped::Problems, raw::Field};
use biomcp_fetch::{KeyKind, Keys, Row};
use biomcp_types::ClinGenValidity;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(super) struct Validity {
    #[serde(rename = "GENE ID (HGNC)")]
    id: Option<Field<HgncId>>,
    #[serde(rename = "GENE SYMBOL")]
    symbol: Option<Field<GeneSymbol>>,
    #[serde(rename = "DISEASE LABEL")]
    disease: Option<Field<NativeText>>,
    #[serde(rename = "DISEASE ID (MONDO)")]
    mondo_id: Option<Field<biodata::MondoId>>,
    #[serde(rename = "MOI")]
    mode_of_inheritance: Option<Field<Text>>,
    #[serde(rename = "CLASSIFICATION")]
    classification: Option<Field<Text>>,
    #[serde(rename = "CLASSIFICATION DATE")]
    classification_date: Option<Field<Text>>,
    #[serde(rename = "GCEP")]
    expert_panel: Option<Field<NativeText>>,
    #[serde(rename = "ONLINE REPORT")]
    report_url: Option<Field<Text>>,
}
impl Validity {
    pub(super) fn checked(self, problems: &mut Problems) -> Option<ClinGenValidity> {
        problems.record = None;
        let id = problems.required_field("GENE ID (HGNC)", self.id)?;
        problems.record = crate::mapped::record(id);
        Some(ClinGenValidity {
            mondo_id: problems.required_field("DISEASE ID (MONDO)", self.mondo_id)?,
            mode_of_inheritance: problems.required_field("MOI", self.mode_of_inheritance)?,
            disease: problems.source_field("DISEASE LABEL", self.disease),
            classification: problems.field("CLASSIFICATION", self.classification),
            classification_date: problems.field("CLASSIFICATION DATE", self.classification_date),
            expert_panel: problems.source_field("GCEP", self.expert_panel),
            report_url: problems.url("ONLINE REPORT", self.report_url),
        })
    }
}
impl Row for Validity {
    fn keys(&self) -> Keys {
        keys(self.id.as_ref(), self.symbol.as_ref())
    }
}
#[derive(Debug, Deserialize)]
pub(super) struct Dosage {
    #[serde(rename = "HGNC ID")]
    id: Option<Field<HgncId>>,
    #[serde(rename = "GENE SYMBOL")]
    symbol: Option<Field<GeneSymbol>>,
    #[serde(rename = "HAPLOINSUFFICIENCY")]
    pub haploinsufficiency: Option<Field<Text>>,
    #[serde(rename = "TRIPLOSENSITIVITY")]
    pub triplosensitivity: Option<Field<Text>>,
    #[serde(rename = "DATE")]
    pub date: Option<Field<Text>>,
}
impl Row for Dosage {
    fn keys(&self) -> Keys {
        keys(self.id.as_ref(), self.symbol.as_ref())
    }
}
fn keys(id: Option<&Field<HgncId>>, symbol: Option<&Field<GeneSymbol>>) -> Keys {
    let key = match id.and_then(Field::as_ref) {
        Some(id) => Some(id.to_string()),
        None => symbol.and_then(Field::as_ref).map(rejected_key),
    };
    Keys {
        exact: key
            .into_iter()
            .map(|key| (KeyKind::Id, key.into()))
            .collect(),
        words: Vec::new(),
    }
}
/// Rejection-only index key; a native symbol scopes disclosures without admitting an identity.
pub(super) fn rejected_key(symbol: &GeneSymbol) -> String {
    format!("rejected-hgnc:{symbol}")
}
impl Dosage {
    pub(super) fn checked(self, problems: &mut Problems) -> Option<super::Dosage> {
        problems.record = None;
        let id = problems.required_field("HGNC ID", self.id)?;
        problems.record = crate::mapped::record(id);
        Some(super::Dosage {
            haploinsufficiency: problems.field("HAPLOINSUFFICIENCY", self.haploinsufficiency),
            triplosensitivity: problems.field("TRIPLOSENSITIVITY", self.triplosensitivity),
            date: problems.field("DATE", self.date),
        })
    }
}
/// One row of ClinGen's region curation list. Field names: <https://ftp.clinicalgenome.org/ClinGen_region_curation_list_GRCh38.tsv>
#[derive(Debug, Deserialize)]
pub(super) struct Region {
    #[serde(rename = "ISCA ID")]
    id: Option<Field<Text>>,
    #[serde(rename = "ISCA Region Name")]
    name: Option<Field<NativeText>>,
    #[serde(rename = "cytoBand")]
    cytoband: Option<Field<Text>>,
    #[serde(rename = "Genomic Location")]
    location: Option<Field<Text>>,
    #[serde(rename = "Haploinsufficiency Score")]
    haplo_score: Option<Field<Text>>,
    #[serde(rename = "Haploinsufficiency Description")]
    haplo_description: Option<Field<NativeText>>,
    #[serde(rename = "Haploinsufficiency PMID1")]
    haplo_pmid1: Option<Field<Text>>,
    #[serde(rename = "Haploinsufficiency PMID2")]
    haplo_pmid2: Option<Field<Text>>,
    #[serde(rename = "Haploinsufficiency PMID3")]
    haplo_pmid3: Option<Field<Text>>,
    #[serde(rename = "Haploinsufficiency PMID4")]
    haplo_pmid4: Option<Field<Text>>,
    #[serde(rename = "Haploinsufficiency PMID5")]
    haplo_pmid5: Option<Field<Text>>,
    #[serde(rename = "Haploinsufficiency PMID6")]
    haplo_pmid6: Option<Field<Text>>,
    #[serde(rename = "Haploinsufficiency Disease ID")]
    haplo_disease: Option<Field<Text>>,
    #[serde(rename = "Triplosensitivity Score")]
    triplo_score: Option<Field<Text>>,
    #[serde(rename = "Triplosensitivity Description")]
    triplo_description: Option<Field<NativeText>>,
    #[serde(rename = "Triplosensitivity PMID1")]
    triplo_pmid1: Option<Field<Text>>,
    #[serde(rename = "Triplosensitivity PMID2")]
    triplo_pmid2: Option<Field<Text>>,
    #[serde(rename = "Triplosensitivity PMID3")]
    triplo_pmid3: Option<Field<Text>>,
    #[serde(rename = "Triplosensitivity PMID4")]
    triplo_pmid4: Option<Field<Text>>,
    #[serde(rename = "Triplosensitivity PMID5")]
    triplo_pmid5: Option<Field<Text>>,
    #[serde(rename = "Triplosensitivity PMID6")]
    triplo_pmid6: Option<Field<Text>>,
    #[serde(rename = "Triplosensitivity Disease ID")]
    triplo_disease: Option<Field<Text>>,
    #[serde(rename = "Date Last Evaluated")]
    date: Option<Field<Text>>,
}
/// Published GRCh38 location split into chromosome without `chr`, start and end.
pub(super) fn bounds(location: &str) -> Option<(&str, u64, u64)> {
    let (chromosome, range) = location.split_once(':')?;
    let (start, end) = range.split_once('-')?;
    let chromosome = chromosome.strip_prefix("chr").unwrap_or(chromosome);
    match (start.parse(), end.parse()) {
        (Ok(start), Ok(end)) => Some((chromosome, start, end)),
        // A malformed location cannot overlap any interval; the row still answers band and ID searches.
        _ => None,
    }
}
impl Row for Region {
    fn keys(&self) -> Keys {
        let mut exact = Vec::new();
        if let Some(id) = self.id.as_ref().and_then(Field::as_ref) {
            exact.push((KeyKind::Id, id.as_str().into()));
        }
        // Band and interval searches both read a chromosome's rows, then filter them.
        let chromosomes = [
            self.cytoband
                .as_ref()
                .and_then(Field::as_ref)
                .and_then(|band| super::band::parse(band.as_str()))
                .map(|band| band.chromosome),
            self.location
                .as_ref()
                .and_then(Field::as_ref)
                .and_then(|location| bounds(location.as_str()))
                .map(|(chromosome, _, _)| chromosome),
        ];
        let mut seen = Vec::new();
        for chromosome in chromosomes.into_iter().flatten() {
            let key = format!("chr{chromosome}");
            if !seen.contains(&key) {
                seen.push(key.clone());
                exact.push((KeyKind::Xref, key.into()));
            }
        }
        Keys {
            exact,
            words: Vec::new(),
        }
    }
}
impl Region {
    pub(super) fn checked(self, problems: &mut Problems) -> Option<biomcp_types::ClinGenRegion> {
        problems.record = None;
        let id = problems.required_field("ISCA ID", self.id)?;
        problems.record = Some(id.clone());
        let haploinsufficiency = dosage(
            problems,
            "Haploinsufficiency",
            self.haplo_score,
            self.haplo_description,
            [
                self.haplo_pmid1,
                self.haplo_pmid2,
                self.haplo_pmid3,
                self.haplo_pmid4,
                self.haplo_pmid5,
                self.haplo_pmid6,
            ],
            self.haplo_disease,
        );
        let triplosensitivity = dosage(
            problems,
            "Triplosensitivity",
            self.triplo_score,
            self.triplo_description,
            [
                self.triplo_pmid1,
                self.triplo_pmid2,
                self.triplo_pmid3,
                self.triplo_pmid4,
                self.triplo_pmid5,
                self.triplo_pmid6,
            ],
            self.triplo_disease,
        );
        Some(biomcp_types::ClinGenRegion {
            id,
            name: problems.source_field("ISCA Region Name", self.name),
            cytoband: problems.field("cytoBand", self.cytoband),
            location: problems.field("Genomic Location", self.location),
            haploinsufficiency,
            triplosensitivity,
            date: problems.field("Date Last Evaluated", self.date),
        })
    }
}
fn dosage(
    problems: &mut Problems,
    kind: &str,
    score: Option<Field<Text>>,
    description: Option<Field<NativeText>>,
    pmids: [Option<Field<Text>>; 6],
    disease: Option<Field<Text>>,
) -> Option<biomcp_types::ClinGenRegionDosage> {
    let field = format!("{kind} Score");
    let score = problems.field(&field, score)?;
    let score = problems.optional(&field, Some(score.as_str().parse::<u32>()))?;
    let field = format!("{kind} PMID");
    // A cell can hold several comma-separated PMIDs, as ISCA-37430's PMID1 does.
    let pmids = pmids
        .into_iter()
        .filter_map(|pmid| problems.field(&field, pmid))
        .collect::<Vec<_>>();
    let pmids = pmids
        .iter()
        .flat_map(|cell| cell.as_str().split(','))
        .map(str::trim)
        .filter(|pmid| !pmid.is_empty())
        .filter_map(|pmid| problems.optional(&field, Some(pmid.parse::<biodata::Pmid>())))
        .collect();
    Some(biomcp_types::ClinGenRegionDosage {
        score,
        description: problems.source_field(&format!("{kind} Description"), description),
        pmids,
        disease_id: problems.field(&format!("{kind} Disease ID"), disease),
    })
}
