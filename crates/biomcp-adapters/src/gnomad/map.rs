//! Map independent constraint metrics for each checked source gene and transcript.
use super::{GeneConstraint, raw};
use crate::{
    Mapped,
    mapped::{Problems, record as record_text},
};
use biodata::EnsemblGeneId;
use biomcp_fetch::BadReply;
use biomcp_types::{Constraint, Source};

pub(super) fn constraints(
    mut raw: raw::Aliases,
    ids: &[EnsemblGeneId],
) -> Result<Vec<GeneConstraint>, BadReply> {
    ids.iter()
        .enumerate()
        .map(|(index, id)| {
            let gene = raw
                .0
                .iter_mut()
                .find(|(name, _)| *name == format!("g{index}"))
                .and_then(|(_, gene)| gene.take());
            Ok(GeneConstraint {
                id: id.clone(),
                constraint: constraint(gene, id)?,
            })
        })
        .collect()
}
fn constraint(
    gene: Option<raw::Gene>,
    id: &EnsemblGeneId,
) -> Result<Mapped<Option<Constraint>>, BadReply> {
    let mut problems = Problems {
        source: Source::Gnomad,
        record: record_text(id.base()),
        list: Vec::new(),
    };
    let version = crate::mapped::text("v4")?;
    let value = gene.and_then(|gene| {
        if gene.gene_id != id.base() {
            problems.required::<()>("gene_id", None);
            return None;
        }
        let transcript = gene.transcript()?;
        let raw = gene.gnomad_constraint?;
        let flags = problems
            .fields("flags", raw.flags)
            .iter()
            .filter_map(|value| problems.text("flags", Some(value)))
            .collect();
        Some(Constraint {
            pli: problems.field("pli", raw.pli),
            loeuf: problems.field("oe_lof_upper", raw.oe_lof_upper),
            oe_lof: problems.field("oe_lof", raw.oe_lof),
            mis_z: problems.field("mis_z", raw.mis_z),
            syn_z: problems.field("syn_z", raw.syn_z),
            flags,
            transcript,
            version,
        })
    });
    Ok(Mapped {
        value,
        problems: problems.list,
    })
}
