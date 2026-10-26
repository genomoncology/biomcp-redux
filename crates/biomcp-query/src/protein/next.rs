use crate::Action;
use biodata::ProteinHeader;
use biomcp_types::{ProteinCardItem, ProteinGet, ProteinInput, ProteinSearch, Request, StepKind};
type Step = (StepKind, Action);
pub(super) fn accession(header: &ProteinHeader) -> Option<Step> {
    let q = ProteinGet::new(vec![ProteinInput::Accession(header.protein.accession.clone())].into())
        .ok()?;
    Some((
        StepKind::Disambiguate,
        Action::Run {
            request: Request::GetProtein(q),
            fresh: false,
        },
    ))
}
pub(super) fn cards(items: &[ProteinCardItem]) -> Vec<Step> {
    let mut steps = Vec::new();
    for item in items {
        for candidate in &item.candidates {
            if let Some(step) = accession(candidate)
                && !steps.contains(&step)
            {
                steps.push(step);
            }
        }
    }
    for item in items {
        if steps.len() >= 5 {
            break;
        }
        if let Some(header) = &item.header {
            if let Ok(q) = ProteinGet::new(
                vec![ProteinInput::Accession(header.protein.accession.clone())].into(),
            ) {
                for section in &item.more {
                    if steps.len() >= 5 {
                        break;
                    }
                    steps.push((
                        StepKind::Section,
                        Action::Run {
                            request: Request::GetProtein(q.with_section(*section)),
                            fresh: false,
                        },
                    ));
                }
            }
            if steps.len() >= 5 {
                break;
            }
            for name in &header.gene_names {
                if let Ok(input) = biomcp_types::GeneInput::new(name.as_str())
                    && let Ok(q) = biomcp_types::GeneGet::new(vec![input].into())
                {
                    let step = (
                        StepKind::Pivot,
                        Action::Run {
                            request: Request::GetGene(q),
                            fresh: false,
                        },
                    );
                    if !steps.contains(&step) {
                        steps.push(step);
                    }
                    break;
                }
            }
        }
    }
    steps
}
pub(super) fn search(
    q: &ProteinSearch,
    rows: &[ProteinHeader],
    pool: biodata::Text,
    offset: u32,
    more: bool,
    partial: bool,
) -> Vec<Step> {
    let mut steps: Vec<_> = rows.iter().filter_map(accession).collect();
    if more {
        steps.push((
            StepKind::Page,
            Action::Run {
                request: Request::SearchProtein(q.at_page(pool, offset)),
                fresh: false,
            },
        ));
    }
    if partial {
        steps.push((
            StepKind::Fresh,
            Action::Run {
                request: Request::SearchProtein(q.restart()),
                fresh: true,
            },
        ));
    }
    steps
}
