//! Keep association metadata failures separate from native row identity.
use super::raw::{EffectAllele, Trait};
use crate::native_text::NativeText;
use biodata::{FiniteNumber, Text};
use serde::Deserialize;
/// A source association retains publisher labels and numbers unchanged.
#[derive(Debug, Deserialize)]
pub(super) struct ReceivedAssociation {
    /// Stable association record identity.
    pub association_id: u64,
    /// Catalog study accession.
    pub accession_id: Text,
    /// Source-reported traits.
    pub reported_trait: Option<crate::raw::Field<Vec<crate::raw::Field<NativeText>>>>,
    /// Source mapped ontology terms.
    pub efo_traits: Option<crate::raw::Field<Vec<crate::raw::Field<Trait>>>>,
    /// Source effect alleles, never interpreted as a classification.
    pub snp_allele: Option<crate::raw::Field<Vec<crate::raw::Field<EffectAllele>>>>,
    /// Reported association p value.
    pub p_value: Option<crate::raw::Field<FiniteNumber>>,
    /// Source exact mantissa, independent of rounded p_value.
    #[serde(rename = "pvalue_mantissa")]
    pub p_value_mantissa: Option<crate::raw::Field<u64>>,
    /// Source exponent, including values below the double range.
    #[serde(rename = "pvalue_exponent")]
    pub p_value_exponent: Option<crate::raw::Field<i64>>,
    /// Source author attribution.
    #[serde(rename = "first_author")]
    pub author: Option<crate::raw::Field<String>>,
    /// Source gene mapping strings.
    #[serde(rename = "mapped_genes")]
    pub mapped_gene: Option<crate::raw::Field<Vec<crate::raw::Field<NativeText>>>>,
    /// Native mapping locations.
    pub locations: Option<crate::raw::Field<Vec<crate::raw::Field<NativeText>>>>,
    /// Source interval lower bound.
    pub ci_lower: Option<crate::raw::Field<FiniteNumber>>,
    /// Source interval upper bound.
    pub ci_upper: Option<crate::raw::Field<FiniteNumber>>,
    /// Reported effect beta, preserving units and prose.
    pub beta: Option<crate::raw::Field<String>>,
    /// Reported odds ratio where provided.
    pub or_per_copy_num: Option<crate::raw::Field<FiniteNumber>>,
    /// Source allele frequency text, such as NR.
    pub risk_frequency: Option<crate::raw::Field<String>>,
    /// Source-reported effect interval.
    pub range: Option<crate::raw::Field<String>>,
    /// Source publication identity.
    pub pubmed_id: Option<crate::raw::Field<String>>,
}
impl From<ReceivedAssociation> for super::raw::Association {
    fn from(row: ReceivedAssociation) -> Self {
        let mut problems = crate::mapped::Problems {
            source: biomcp_types::Source::Gwas,
            record: crate::mapped::record(row.association_id),
            list: Vec::new(),
        };
        let mut value = Self {
            association_id: row.association_id,
            accession_id: row.accession_id,
            reported_trait: problems
                .fields("reported_trait", row.reported_trait)
                .into_vec(),
            efo_traits: problems.fields("efo_traits", row.efo_traits).into_vec(),
            snp_allele: problems.fields("snp_allele", row.snp_allele).into_vec(),
            p_value: problems.required_field("p_value", row.p_value),
            p_value_mantissa: problems.field("pvalue_mantissa", row.p_value_mantissa),
            p_value_exponent: problems.field("pvalue_exponent", row.p_value_exponent),
            author: problems.field("first_author", row.author),
            mapped_gene: row
                .mapped_gene
                .map(|v| problems.fields("mapped_genes", Some(v)).into_vec()),
            locations: row
                .locations
                .map(|v| problems.fields("locations", Some(v)).into_vec()),
            ci_lower: problems.field("ci_lower", row.ci_lower),
            ci_upper: problems.field("ci_upper", row.ci_upper),
            beta: problems.field("beta", row.beta),
            or_per_copy_num: problems.field("or_per_copy_num", row.or_per_copy_num),
            risk_frequency: problems.field("risk_frequency", row.risk_frequency),
            range: problems.field("range", row.range),
            pubmed_id: problems.field("pubmed_id", row.pubmed_id),
            problems: Vec::new(),
        };
        if value.exact_p_value().is_none()
            && (value.p_value_mantissa.is_some() || value.p_value_exponent.is_some())
        {
            problems.required::<()>("pvalue_mantissa/pvalue_exponent", None);
            value.p_value_mantissa = None;
            value.p_value_exponent = None;
        }
        for trait_row in &mut value.efo_traits {
            for issue in trait_row.problems.drain(..) {
                crate::mapped::push_problem(
                    &mut problems.list,
                    problems.source,
                    problems.record.clone(),
                    &format!("efo_traits.{}", issue.field),
                );
            }
        }
        for allele in &mut value.snp_allele {
            for issue in allele.problems.drain(..) {
                crate::mapped::push_problem(
                    &mut problems.list,
                    problems.source,
                    problems.record.clone(),
                    &format!("snp_allele.{}", issue.field),
                );
            }
        }
        value.problems = problems.list;
        value
    }
}
