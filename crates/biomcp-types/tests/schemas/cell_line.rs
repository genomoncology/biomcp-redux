use super::Contract;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        CellLineAssayCountQuery => ("cell-line-assay-count-query", "biomcp.cell-line-assay-count-query/1", query),
        CellLineSearch => ("cell-line-search-query", "biomcp.cell-line-search-query/2", query),
        CellLineGet => ("cell-line-get-query", "biomcp.cell-line-get-query/1", query),
    }
}
