use super::Contract;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        PhenotypeSearch => ("phenotype-search-query", "biomcp.phenotype-search-query/1", query),
        PhenotypeGet => ("phenotype-get", "biomcp.phenotype-get/1", query),
    }
}
