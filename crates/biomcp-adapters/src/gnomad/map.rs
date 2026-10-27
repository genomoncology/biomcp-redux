use super::{GeneConstraint, raw};
use crate::Mapped;
use biodata::{EnsemblGeneId, Text};

use biomcp_types::{Constraint, Source};

pub(super) fn constraints(raw: raw::Aliases, ids: &[EnsemblGeneId]) -> Vec<GeneConstraint> {
    ids.iter()
        .enumerate()
        .map(|(index, id)| {
            let gene = raw
                .0
                .iter()
                .find(|(name, _)| *name == format!("g{index}"))
                .and_then(|(_, gene)| gene.as_ref());
            let mut problems = Vec::new();
            let constraint = gene.and_then(|gene| {
                if gene.gene_id != id.base() {
                    crate::mapped::push_problem(
                        &mut problems,
                        Source::Gnomad,
                        Text::new(id.base()).ok(),
                        "gene_id",
                    );
                    return None;
                }
                let raw = gene.gnomad_constraint.as_ref()?;
                let transcript = gene.transcript()?;
                let flags = match &raw.flags {
                    None => Vec::new(),
                    Some(crate::raw::Field::Value(flags)) => flags
                        .iter()
                        .filter_map(|flag| {
                            if let crate::raw::Field::Value(text) = flag
                                && let Ok(text) = Text::new(text)
                            {
                                return Some(text);
                            }
                            crate::mapped::push_problem(
                                &mut problems,
                                Source::Gnomad,
                                Text::new(id.base()).ok(),
                                "flags",
                            );
                            None
                        })
                        .collect(),
                    Some(_) => {
                        crate::mapped::push_problem(
                            &mut problems,
                            Source::Gnomad,
                            Text::new(id.base()).ok(),
                            "flags",
                        );
                        Vec::new()
                    }
                };
                Some(Constraint {
                    pli: raw.pli,
                    loeuf: raw.oe_lof_upper,
                    oe_lof: raw.oe_lof,
                    mis_z: raw.mis_z,
                    syn_z: raw.syn_z,
                    flags: flags.into(),
                    transcript,
                    version: Text::new("v4").ok()?,
                })
            });
            GeneConstraint {
                id: id.clone(),
                constraint: Mapped {
                    value: constraint,
                    problems,
                },
            }
        })
        .collect()
}
