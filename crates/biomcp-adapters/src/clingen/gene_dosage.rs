//! Evidence for one gene, indexed by the publisher's NCBI Gene ID.
use super::{REGION_SPEC, raw};
use crate::{Mapped, Refusal, mapped::Problems, raw::Field};
use biodata::{NcbiGeneId, Text};
use biomcp_fetch::{Bundle, Find, Freshness, KeyKind, Keys, Leg, RequestError, Row, RowFormat};
use biomcp_types::{ClinGenGeneDosage, Source};
use serde::Deserialize;
use std::marker::PhantomData;

pub(super) const BUNDLE: Bundle<GeneDosage> = Bundle {
    spec: &REGION_SPEC,
    name: "clingen-gene-dosage-evidence",
    path: "/ClinGen_gene_curation_list_GRCh38.tsv",
    format: RowFormat::CommentedTsv,
    fresh: Freshness::Days(1),
    row: PhantomData,
};

#[derive(Debug, Deserialize)]
pub(super) struct GeneDosage {
    #[serde(rename = "Gene ID")]
    id: NcbiGeneId,
    #[serde(rename = "Haploinsufficiency Score")]
    haplo_score: Option<Field<Text>>,
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
    #[serde(rename = "Triplosensitivity Score")]
    triplo_score: Option<Field<Text>>,
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
}

impl Row for GeneDosage {
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![(KeyKind::Id, self.id.to_string().into())],
            words: Vec::new(),
        }
    }
}

/// Published gene dosage evidence for the two independent directions.
#[derive(Debug, Default)]
pub struct Evidence {
    /// Copy-loss evidence.
    pub haploinsufficiency: Option<ClinGenGeneDosage>,
    /// Copy-gain evidence.
    pub triplosensitivity: Option<ClinGenGeneDosage>,
}

/// Read the gene-level evidence using the resolved NCBI ID, never the symbol.
pub fn evidence(leg: &Leg<'_>, id: NcbiGeneId) -> Result<Mapped<Evidence>, RequestError> {
    let key = id.to_string();
    let mut rows = leg.rows(&BUNDLE, Find::Exact(KeyKind::Id, &key))?;
    let mut problems = Problems {
        source: Source::ClinGen,
        record: crate::mapped::record(id),
        list: Vec::new(),
    };
    let mut refusals = Refusal::None;
    let value = match rows.len() {
        0 => Evidence::default(),
        1 => rows
            .pop()
            .map_or_else(Evidence::default, |row| row.checked(&mut problems)),
        _ => {
            refusals = Refusal::Refused;
            problems
                .required::<Evidence>("Gene ID", None)
                .unwrap_or_default()
        }
    };
    Ok(Mapped {
        value,
        problems: problems.list,
        refusals,
    })
}

impl GeneDosage {
    fn checked(self, problems: &mut Problems) -> Evidence {
        Evidence {
            haploinsufficiency: direction(
                problems,
                "Haploinsufficiency",
                self.haplo_score,
                [
                    self.haplo_pmid1,
                    self.haplo_pmid2,
                    self.haplo_pmid3,
                    self.haplo_pmid4,
                    self.haplo_pmid5,
                    self.haplo_pmid6,
                ],
            ),
            triplosensitivity: direction(
                problems,
                "Triplosensitivity",
                self.triplo_score,
                [
                    self.triplo_pmid1,
                    self.triplo_pmid2,
                    self.triplo_pmid3,
                    self.triplo_pmid4,
                    self.triplo_pmid5,
                    self.triplo_pmid6,
                ],
            ),
        }
    }
}

fn direction(
    problems: &mut Problems,
    kind: &str,
    score: Option<Field<Text>>,
    cells: [Option<Field<Text>>; 6],
) -> Option<ClinGenGeneDosage> {
    let field = format!("{kind} Score");
    let score = problems
        .field(&field, score)
        // The publisher uses text or -1 for unscored assessments.
        .filter(|score| !matches!(score.as_str(), "Not yet evaluated" | "-1"))
        .and_then(|score| problems.optional(&field, Some(score.as_str().parse::<u32>())));
    let pmids = raw::dosage_pmids(problems, kind, cells);
    (score.is_some() || !pmids.is_empty()).then_some(ClinGenGeneDosage { score, pmids })
}
