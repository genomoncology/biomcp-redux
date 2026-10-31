use super::{Action, GwasAssociation, GwasGet, GwasLocus, Outcome, Request, StepKind, action};
use serde_json::json;
pub(super) fn association(row: &GwasAssociation, out: &mut Outcome) {
    if let Some(study) = &row.study
        && let Ok(id) = study.as_str().parse()
    {
        out.next
            .push((StepKind::Pivot, action(GwasGet::study(id), false)));
    }
    for allele in &row.alleles {
        if allele.rs_id.as_str().parse::<biodata::RsId>().is_ok()
            && let Ok(q) = serde_json::from_value(json!({"input":allele.rs_id}))
        {
            out.next.push((
                StepKind::Pivot,
                Action::Run {
                    request: Request::GetVariant(q),
                    fresh: false,
                },
            ));
        }
    }
    if let Some(publication) = &row.publication
        && let Ok(q) = serde_json::from_value(json!({"input":[publication]}))
    {
        out.next.push((
            StepKind::Pivot,
            Action::Run {
                request: Request::GetArticle(q),
                fresh: false,
            },
        ));
    }
}
pub(super) fn loci(rows: &[GwasLocus], out: &mut Outcome) {
    let mut seen = std::collections::BTreeSet::new();
    for gene in rows.iter().flat_map(|r| &r.author_reported_genes) {
        for id in gene.ensembl_ids.iter().chain(&gene.entrez_ids) {
            let input = if id.as_str().bytes().all(|b| b.is_ascii_digit()) {
                id.as_str()
                    .parse::<biodata::NcbiGeneId>()
                    .ok()
                    .map(|id| id.to_string())
            } else {
                id.as_str()
                    .parse::<biodata::EnsemblGeneId>()
                    .ok()
                    .map(|id| id.to_string())
            };
            let Some(input) = input else { continue };
            if seen.insert(input.clone())
                && let Ok(q) = serde_json::from_value(json!({"input":[input]}))
            {
                out.next.push((
                    StepKind::Pivot,
                    Action::Run {
                        request: Request::GetGene(q),
                        fresh: false,
                    },
                ));
            }
        }
    }
}
