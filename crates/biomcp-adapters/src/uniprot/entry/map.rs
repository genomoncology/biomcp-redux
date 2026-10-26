use super::{Entry, raw};
use crate::Mapped;
use biodata::{Protein, ProteinHeader, Text, UniProtIsoform};
use biomcp_types::{Problem, ProblemCode, Source};
pub(super) fn entry(raw: raw::Entry, id: &UniProtIsoform) -> Mapped<Option<Entry>> {
    if !raw.admitted(id) {
        return Mapped {
            value: None,
            problems: vec![Problem {
                source: Source::UniProt,
                record: Text::new(id.as_str()).ok(),
                field: if raw.primary_accession != *id {
                    "primaryAccession"
                } else {
                    "reviewed_human"
                }
                .into(),
                code: ProblemCode::ValueUnknown,
            }],
        };
    }
    let audit = raw.entry_audit;
    let value = header(
        raw.primary_accession,
        raw.uni_protkb_id,
        raw.protein_description.recommended_name.full_name.value,
        raw.sequence.length,
        raw.genes,
    )
    .map(|header| Entry {
        header,
        audit: audit.into(),
    });
    Mapped {
        value,
        problems: Vec::new(),
    }
}
pub(super) fn header(
    accession: UniProtIsoform,
    entry_id: Text,
    name: Text,
    length: u32,
    genes: Vec<raw::Gene>,
) -> Option<ProteinHeader> {
    let url = Text::new(&format!(
        "https://www.uniprot.org/uniprotkb/{accession}/entry"
    ))
    .ok()?;
    let gene_names = genes
        .into_iter()
        .flat_map(|g| {
            g.gene_name
                .into_iter()
                .chain(g.synonyms)
                .chain(g.ordered_locus_names)
                .chain(g.orf_names)
        })
        .map(|v| v.value)
        .collect();
    Some(ProteinHeader {
        protein: Protein {
            accession,
            name,
            length,
            function: Box::default(),
            subcellular_locations: Box::default(),
        },
        entry_id,
        gene_names,
        gene_ids: Box::default(),
        taxon: 9606,
        reviewed: true,
        url,
    })
}
