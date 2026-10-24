use super::{
    Withdrawn,
    fields::{self, Fields},
    raw,
};
use crate::Mapped;
use biodata::{Cytoband, Gene, GeneStatus, GeneSymbol, HgncId, Text};

pub(super) fn genes(rows: Vec<raw::Gene>) -> Mapped<Vec<Gene>> {
    let mut problems = Vec::new();
    let value = rows
        .into_iter()
        .filter_map(|row| {
            let mut fields = Fields {
                id: row.hgnc_id.parse().ok(),
                problems: Vec::new(),
            };
            let gene = gene(row, &mut fields);
            problems.extend(fields.problems);
            gene
        })
        .collect();
    Mapped { value, problems }
}
fn gene(row: raw::Gene, f: &mut Fields) -> Option<Gene> {
    let id = f.required("hgnc_id", f.id);
    let symbol = f.required("symbol", GeneSymbol::new(&row.symbol).ok());
    let name = f.required("name", Text::new(&row.name).ok());
    let locus_group = f.required("locus_group", fields::group(&row.locus_group));
    let status = f.required(
        "status",
        (row.status == "Approved").then_some(GeneStatus::Approved),
    );
    // All required fields are checked before dropping a record.
    let (hgnc_id, symbol, name, locus_group, status) = (id?, symbol?, name?, locus_group?, status?);
    let locus_type = if row.locus_type.is_empty() {
        None
    } else {
        f.required("locus_type", fields::locus(&row.locus_type))
    };
    let mane_select = if row.mane_select.is_empty() {
        None
    } else {
        f.required("mane_select", fields::mane(&row.mane_select))
    };
    Some(Gene {
        hgnc_id,
        symbol,
        name,
        locus_group,
        status,
        locus_type,
        mane_select,
        location: Cytoband::new(&row.location).ok(),
        entrez_id: f.optional("entrez_id", &row.entrez_id),
        ensembl_id: f.optional("ensembl_id", &row.ensembl_gene_id),
        previous_symbols: f.list("previous_symbols", &row.prev_symbol),
        aliases: f.list("aliases", &row.alias_symbol),
        uniprot: f.list("uniprot", &row.uniprot_ids),
        omim: f.list("omim", &row.omim_id),
        spans: Box::new([]),
        summary: None,
    })
}
pub(super) fn withdrawn(rows: Vec<raw::Withdrawn>) -> Mapped<Vec<Withdrawn>> {
    let mut problems = Vec::new();
    let value = rows
        .into_iter()
        .filter_map(|row| {
            let mut f = Fields {
                id: row.hgnc_id.parse().ok(),
                problems: Vec::new(),
            };
            let id = f.required("hgnc_id", f.id);
            let symbol = f.required("symbol", GeneSymbol::new(&row.symbol).ok());
            let hgnc_status = f.required("hgnc_status", row.status.parse().ok());
            let merged_into = if row.merged.is_empty() {
                Vec::new()
            } else {
                row.merged
                    .split(", ")
                    .filter_map(|part| {
                        let id = part
                            .split('|')
                            .next()
                            .and_then(|id| id.parse::<HgncId>().ok());
                        f.required("merged_into", id)
                    })
                    .collect()
            };
            problems.extend(f.problems);
            Some(Withdrawn {
                hgnc_id: id?,
                symbol: symbol?,
                hgnc_status: hgnc_status?,
                merged_into: merged_into.into(),
            })
        })
        .collect();
    Mapped { value, problems }
}
