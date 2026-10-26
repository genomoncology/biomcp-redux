use super::{Contract, generate};
use biomcp_types::*;
pub(super) fn registry() -> Vec<Contract> {
    contracts! {
        CellLineSearch => ("cell-line-search-query", "biomcp.cell-line-search-query/1"),
        CellLineGet => ("cell-line-get-query", "biomcp.cell-line-get-query/1"),
        Envelope<CellLinePage> => ("cell-line-page", "biomcp.cell_line_page/2"),
        Envelope<CellLineCard> => ("cell-line-card", "biomcp.cell_line_card/3"),
        Envelope<CellLineNames> => ("cell-line-names", "biomcp.cell_line_names/1"),
        Envelope<CellLineIdentityPage> => ("cell-line-identity", "biomcp.cell_line_identity/1"),
        Envelope<CellLineXrefs> => ("cell-line-xrefs", "biomcp.cell_line_xrefs/1"),
        Envelope<CellLineVariants> => ("cell-line-variants", "biomcp.cell_line_variants/2"),
        Envelope<CellLineChembl> => ("cell-line-chembl", "biomcp.cell_line_chembl/1"),
        Envelope<CellLineAssayCount> => ("cell-line-assay-count", "biomcp.cell_line_assay_count/1"),
    }
}
