//! One mapping shared by association search and the existing variant section.
use biomcp_adapters::gwas;
use biomcp_types::{GwasAssociation, GwasEffectAllele};
pub(crate) fn association(row: gwas::Association) -> GwasAssociation {
    let exact_p_value = row.exact_p_value();
    GwasAssociation {
        id: row.association_id,
        study: row.accession_id,
        traits: row.reported_trait.into_iter().map(Into::into).collect(),
        ontology_traits: row
            .efo_traits
            .into_iter()
            .map(|t| biomcp_types::GwasTrait {
                id: t.efo_id,
                label: t.efo_trait,
            })
            .collect(),
        alleles: row
            .snp_allele
            .into_iter()
            .map(|a| GwasEffectAllele {
                rs_id: a.rs_id,
                allele: a.effect_allele,
            })
            .collect(),
        p_value: row.p_value,
        exact_p_value,
        author: optional_text(row.author.clone()),
        mapped_genes: row
            .mapped_gene
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect(),
        locations: row
            .locations
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(Into::into)
            .collect(),
        ci_lower: row.ci_lower,
        ci_upper: row.ci_upper,
        beta: optional_text(row.beta),
        odds_ratio: row.or_per_copy_num,
        risk_frequency: optional_text(row.risk_frequency),
        interval: optional_text(row.range),
        publication: optional_text(row.pubmed_id),
    }
}

fn optional_text(value: Option<String>) -> Option<biodata::Text> {
    value.and_then(|v| v.parse().ok())
}
