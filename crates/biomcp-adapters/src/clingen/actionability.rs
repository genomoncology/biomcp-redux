//! Published outcome–intervention scores indexed by exact gene symbol.
use crate::native_text::{NativeText, admit};
use biodata::{GeneSymbol, Text};
use biomcp_fetch::{
    BadReply, Bundle, Find, Freshness, KeyKind, Keys, Leg, RequestError, Row, RowFormat, SourceSpec,
};
use biomcp_types::{ClinGenActionabilityPair, ClinGenActionabilityRow, Source};
use serde::Deserialize;
use std::marker::PhantomData;

/// ClinGen Actionability Knowledge Repository.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::ClinGen,
    terms: super::SPEC.terms,
    base: "https://actionability.clinicalgenome.org",
    ..super::SPEC
};

/// Publisher's index of clinical actionability reports.
pub const REPORT_INDEX: &str = "https://search.clinicalgenome.org/kb/actionability/report-index";

const ADULT: Bundle<SummaryRow> = summary(
    "clingen-actionability-adult",
    "/ac/Adult/api/summ?format=tsv",
);
const PEDIATRIC: Bundle<SummaryRow> = summary(
    "clingen-actionability-pediatric",
    "/ac/Pediatric/api/summ?format=tsv",
);

const fn summary(name: &'static str, path: &'static str) -> Bundle<SummaryRow> {
    Bundle {
        spec: &SPEC,
        name,
        path,
        format: RowFormat::NativeTsv {
            gzip: false,
            max_decoded_bytes: 1024 * 1024,
        },
        fresh: Freshness::Days(1),
        row: PhantomData,
    }
}

/// Age context of a published summary.
#[derive(Clone, Copy, Debug)]
pub enum Context {
    /// Adult Actionability Working Group.
    Adult,
    /// Pediatric Actionability Working Group.
    Pediatric,
}

/// Return the gene-condition rows in publisher order, including unscored reports.
pub fn curations(
    leg: &Leg<'_>,
    symbol: &GeneSymbol,
    context: Context,
) -> Result<Box<[ClinGenActionabilityRow]>, RequestError> {
    let (bundle, name) = match context {
        Context::Adult => (&ADULT, "Adult"),
        Context::Pediatric => (&PEDIATRIC, "Pediatric"),
    };
    let (rows, info) = leg.rows_with_info(bundle, Find::Exact(KeyKind::Symbol, symbol.as_str()))?;
    let invalid = || biomcp_fetch::bad_reply(Source::ClinGen, None);
    if !info.rejections.is_empty() {
        return Err(invalid());
    }
    rows.into_iter()
        .map(|row| row.checked(name).map_err(|_| invalid()))
        .collect()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SummaryRow {
    #[serde(rename = "# docId")]
    doc_id: Text,
    context: Text,
    curation_type: Text,
    gene_or_variant: Text,
    disease: NativeText,
    #[serde(rename = "status-overall")]
    status: Text,
    #[serde(rename = "status-stg1")]
    early_rule_out: Text,
    outcome: String,
    intervention: String,
    severity: String,
    likelihood: String,
    nature_of_intervention: String,
    effectiveness: String,
    overall: String,
}

impl Row for SummaryRow {
    fn keys(&self) -> Keys {
        let mut exact = vec![(KeyKind::Id, self.doc_id.as_str().into())];
        if self.curation_type.as_str() == "Gene-Condition" {
            exact.extend(
                self.gene_or_variant
                    .as_str()
                    .split(',')
                    .map(|symbol| (KeyKind::Symbol, symbol.trim().into())),
            );
        }
        Keys {
            exact,
            words: Vec::new(),
        }
    }
}

impl SummaryRow {
    fn checked(self, context: &str) -> Result<ClinGenActionabilityRow, BadReply> {
        let doc = self.doc_id.as_str();
        let number = doc.strip_prefix("AC").ok_or(BadReply)?;
        if number.is_empty()
            || !number.bytes().all(|byte| byte.is_ascii_digit())
            || self.context.as_str() != context
        {
            return Err(BadReply);
        }
        let pair = self.pair()?;
        Ok(ClinGenActionabilityRow {
            disease: self.disease.into(),
            status: self.status,
            early_rule_out: self.early_rule_out,
            report_url: Text::new(&format!(
                "{}/ac/ui/{context}/ui/stg2SummaryRpt/{doc}",
                SPEC.base
            ))
            .map_err(|_| BadReply)?,
            pair,
        })
    }

    fn pair(&self) -> Result<Option<ClinGenActionabilityPair>, BadReply> {
        if self.overall.is_empty() && self.outcome.is_empty() && self.intervention.is_empty() {
            return Ok(None);
        }
        let text = |value: &str| admit(value).map_err(|_| BadReply);
        Ok(Some(ClinGenActionabilityPair {
            outcome: text(&self.outcome)?,
            intervention: text(&self.intervention)?,
            severity: text(&self.severity)?,
            likelihood: text(&self.likelihood)?,
            nature_of_intervention: text(&self.nature_of_intervention)?,
            effectiveness: text(&self.effectiveness)?,
            overall: text(&self.overall)?,
        }))
    }
}
