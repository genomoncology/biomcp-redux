use super::{
    Withdrawn,
    fields::{self, Fields},
    raw,
};
use crate::Mapped;
use biodata::{Gene, GeneStatus, GeneSymbol, HgncId, Text};

pub(super) fn genes(rows: Vec<raw::Gene>) -> Mapped<Vec<Gene>> {
    let mut problems = Vec::new();
    let value = rows
        .into_iter()
        .filter_map(|row| {
            let mut fields = Fields {
                id: raw::cell(&row.hgnc_id).parse().ok(),
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
    let symbol = f.required("symbol", GeneSymbol::new(raw::cell(&row.symbol)).ok());
    let name = f.required("name", Text::new(raw::cell(&row.name)).ok());
    let locus_group = f.required("locus_group", fields::group(raw::cell(&row.locus_group)));
    let status = f.required(
        "status",
        (raw::cell(&row.status) == "Approved").then_some(GeneStatus::Approved),
    );
    // All required fields are checked before dropping a record.
    let (hgnc_id, symbol, name, locus_group, status) = (id?, symbol?, name?, locus_group?, status?);
    let locus_type = if raw::cell(&row.locus_type).is_empty() {
        None
    } else {
        f.required("locus_type", fields::locus(raw::cell(&row.locus_type)))
    };
    let mane_select = if raw::cell(&row.mane_select).is_empty() {
        None
    } else {
        f.required("mane_select", fields::mane(raw::cell(&row.mane_select)))
    };
    Some(Gene {
        hgnc_id,
        symbol,
        name,
        locus_group,
        status,
        locus_type,
        mane: mane_select.into_iter().collect(),
        location: (!raw::cell(&row.location).is_empty()).then(|| raw::cell(&row.location).into()),
        entrez_id: f.optional("entrez_id", raw::cell(&row.entrez_id)),
        ensembl_id: f.optional("ensembl_id", raw::cell(&row.ensembl_gene_id)),
        previous_symbols: f.list("previous_symbols", raw::cell(&row.prev_symbol)),
        aliases: f.list("aliases", raw::cell(&row.alias_symbol)),
        uniprot: f.list("uniprot", raw::cell(&row.uniprot_ids)),
        omim: f.list("omim", raw::cell(&row.omim_id)),
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
                id: raw::cell(&row.hgnc_id).parse().ok(),
                problems: Vec::new(),
            };
            let id = f.required("hgnc_id", f.id);
            let symbol = f.required("symbol", GeneSymbol::new(raw::cell(&row.symbol)).ok());
            let hgnc_status = f.required(
                "hgnc_status",
                match raw::cell(&row.status) {
                    "Entry Withdrawn" => Some(biodata::HgncStatus::EntryWithdrawn),
                    "Merged/Split" => Some(biodata::HgncStatus::MergedSplit),
                    _ => None,
                },
            );
            let merged_into = if raw::cell(&row.merged).is_empty() {
                Vec::new()
            } else {
                raw::cell(&row.merged)
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
