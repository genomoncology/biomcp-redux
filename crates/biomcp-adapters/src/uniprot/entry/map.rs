use super::{Entry, raw};
use crate::Mapped;
use biodata::{Protein, ProteinHeader, Text, UniProtIsoform};
use biomcp_types::{Problem, ProblemCode, Source};
pub(in crate::uniprot) fn entry(raw: raw::Entry, id: &UniProtIsoform) -> Mapped<Option<Entry>> {
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
    let audit: biodata::ProteinAudit = raw.entry_audit.unwrap_or_default().into();
    let mut problems = Vec::new();
    let features =
        super::sections_map::features(raw.features.unwrap_or_default(), id, &audit, &mut problems);
    let (isoforms, function) = super::sections_map::comments(raw.comments.unwrap_or_default(), id, &mut problems);
    let structures =
        super::sections_map::structures(raw.uni_prot_kb_cross_references.unwrap_or_default());
    let sequence = raw.sequence.value.map(|sequence| biodata::ProteinSequence {
        accession: id.clone(),
        sequence,
        length: raw.sequence.length,
        crc64: raw.sequence.crc64,
        md5: raw.sequence.md5,
        audit: audit.clone(),
    });
    let value = header(
        raw.primary_accession,
        raw.uni_protkb_id,
        raw.protein_description.recommended_name.full_name.value,
        raw.sequence.length,
        raw.genes.unwrap_or_default(),
    )
    .map(|header| Entry {
        header,
        audit,
        features,
        isoforms,
        function,
        structures,
        sequence,
    });
    Mapped { value, problems }
}
pub(in crate::uniprot) fn header(
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
                .chain(g.synonyms.into_iter().flatten())
                .chain(g.ordered_locus_names.into_iter().flatten())
                .chain(g.orf_names.into_iter().flatten())
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
