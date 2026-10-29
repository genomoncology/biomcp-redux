use super::Contract;
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        CellLineAssayCountQuery => ("cell-line-assay-count-query", "biomcp.cell-line-assay-count-query/1", query),
        CellLineSearch => ("cell-line-search-query", "biomcp.cell-line-search-query/1", query),
        CellLineGet => ("cell-line-get-query", "biomcp.cell-line-get-query/1", query),
        Envelope<CellLinePage> => ("cell-line-page", "biomcp.cell-line-page/3"),
        Envelope<CellLineCard> => ("cell-line-card", "biomcp.cell-line-card/4"),
        Envelope<CellLineNames> => ("cell-line-names", "biomcp.cell-line-names/2"),
        Envelope<CellLineIdentityPage> => ("cell-line-identity", "biomcp.cell-line-identity/2"),
        Envelope<CellLineXrefs> => ("cell-line-xrefs", "biomcp.cell-line-xrefs/2"),
        Envelope<CellLineVariants> => ("cell-line-variants", "biomcp.cell-line-variants/3"),
        Envelope<CellLineChembl> => ("cell-line-chembl", "biomcp.cell-line-chembl/3"),
        Envelope<CellLineAssayCount> => ("cell-line-assay-count", "biomcp.cell-line-assay-count/3"),
    }
}
