use super::{Contract, generate};
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        CellLineAssayCountQuery => ("cell-line-assay-count-query", "biomcp.cell-line-assay-count-query/1", query),
        CellLineSearch => ("cell-line-search-query", "biomcp.cell-line-search-query/1", query),
        CellLineGet => ("cell-line-get-query", "biomcp.cell-line-get-query/1", query),
        Envelope<CellLinePage> => ("cell-line-page", "biomcp.cell-line-page/2"),
        Envelope<CellLineCard> => ("cell-line-card", "biomcp.cell-line-card/3"),
        Envelope<CellLineNames> => ("cell-line-names", "biomcp.cell-line-names/1"),
        Envelope<CellLineIdentityPage> => ("cell-line-identity", "biomcp.cell-line-identity/1"),
        Envelope<CellLineXrefs> => ("cell-line-xrefs", "biomcp.cell-line-xrefs/1"),
        Envelope<CellLineVariants> => ("cell-line-variants", "biomcp.cell-line-variants/2"),
        Envelope<CellLineChembl> => ("cell-line-chembl", "biomcp.cell-line-chembl/2"),
        Envelope<CellLineAssayCount> => ("cell-line-assay-count", "biomcp.cell-line-assay-count/2"),
    }
}
