//! Group entity trials without growing the case entry point.
use libtest_mimic::Trial;
#[path = "diagnostic_recall.rs"]
mod diagnostic_recall;
#[path = "native_vocabulary.rs"]
mod native_vocabulary;

#[path = "adapter_bad_values.rs"]
mod adapter_bad_values;

pub(super) fn drug_trials() -> Vec<Trial> {
    let mut trials = super::gwas_search::trials();
    trials.extend(super::gwas_get::trials());
    trials.extend(super::study_catalog::trials());
    trials.extend(super::drug_failures::trials());
    trials.extend(super::adverse_event_failures::trials());
    trials.extend(super::vaers::trials());
    trials.extend(super::drug_identity::trials());
    trials.extend(super::drug_registry::trials());
    trials.extend(super::adverse_event_registry::trials());
    trials.extend(super::faers_search::trials());
    trials.extend(super::maude::trials());
    trials.extend(super::faers_get::trials());
    trials.extend(diagnostic_recall::trials());
    trials.extend(super::diagnostic_registry::trials());
    trials.extend(super::diagnostic_retirements::trials());
    trials.extend(super::diagnostic_bundles::trials());
    trials.extend(super::diagnostic_gtr::trials());
    trials.extend(super::diagnostic_who::trials());
    trials.extend(super::diagnostic_pma::trials());
    trials.extend(super::diagnostic_510k::trials());
    trials.extend(super::diagnostic_cdx::trials());
    trials.extend(super::pgx_registry::trials());
    trials.extend(super::pathway_registry::trials());
    trials.extend(super::pathway_commands::trials());
    trials.extend(super::pathway_sections::trials());
    trials.extend(super::wikipathways::trials());
    trials.extend(super::wikipathways_genes::trials());
    trials.extend(super::hpo_registry::trials());
    trials.extend(super::protein_registry::trials());
    trials.extend(super::pgx_retirements::trials());
    trials.extend(super::cpic::trials());
    trials.extend(super::clinpgx::trials());
    trials.extend(super::drug_retirements::trials());
    trials.extend(super::adverse_event_retirements::trials());
    trials.extend(super::drug_civic::trials());
    trials.extend(super::drug_next::trials());
    trials.extend(super::drug_ema::trials());
    trials.extend(adapter_bad_values::trials());
    trials.extend(native_vocabulary::trials());
    trials.extend(super::drug_who::trials());
    trials.extend(super::drug_ddinter::trials());
    trials.extend(super::drug_fda::trials());
    trials.extend(super::drug_mychem::trials());
    trials.extend(super::drug_native::trials());
    trials.extend(super::drug_native_identity::trials());
    trials.extend(super::drug_label::trials());
    trials
}
