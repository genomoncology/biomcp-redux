use crate::Action;
use crate::next::Step;
use biodata::ProteinHeader;
use biomcp_types::{ProteinCardItem, ProteinGet, ProteinInput, ProteinSearch, Request, StepKind};
pub(super) fn accession(
    header: &ProteinHeader,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<Step> {
    let q = crate::next::checked(
        ProteinGet::new(vec![ProteinInput::Accession(header.protein.accession.clone())].into()),
        biomcp_types::Source::UniProt,
        problems,
    )?;
    Some(crate::next::Step::new(
        StepKind::Disambiguate,
        &[biomcp_types::Source::UniProt],
        Action::Run {
            request: Request::GetProtein(q),
            fresh: false,
        },
    ))
}
pub(super) fn cards(
    items: &[ProteinCardItem],
    problems: &mut Vec<biomcp_types::Problem>,
) -> Vec<Step> {
    let mut steps = Vec::new();
    for item in items {
        for candidate in &item.candidates {
            if let Some(step) = accession(candidate, problems)
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
            if let Some(q) = crate::next::checked(
                ProteinGet::new(
                    vec![ProteinInput::Accession(header.protein.accession.clone())].into(),
                ),
                biomcp_types::Source::UniProt,
                problems,
            ) {
                for section in &item.more {
                    if steps.len() >= 5 {
                        break;
                    }
                    steps.push(crate::next::Step::new(
                        StepKind::Section,
                        &[section_source(*section)],
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
                if let Some(input) = crate::next::checked(
                    biomcp_types::GeneInput::new(name.as_str()),
                    biomcp_types::Source::UniProt,
                    problems,
                ) && let Some(q) = crate::next::checked(
                    biomcp_types::GeneGet::new(vec![input].into()),
                    biomcp_types::Source::UniProt,
                    problems,
                ) {
                    let step = crate::next::Step::new(
                        StepKind::Pivot,
                        &[biomcp_types::Source::Hgnc],
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
    problems: &mut Vec<biomcp_types::Problem>,
) -> Vec<Step> {
    let mut steps: Vec<_> = rows
        .iter()
        .filter_map(|row| accession(row, problems))
        .collect();
    if more {
        steps.push(crate::next::Step::new(
            StepKind::Page,
            &[biomcp_types::Source::UniProt],
            Action::Run {
                request: Request::SearchProtein(q.at_page(pool, offset)),
                fresh: false,
            },
        ));
    }
    if partial {
        steps.push(crate::next::Step::new(
            StepKind::Fresh,
            &[biomcp_types::Source::UniProt],
            Action::Run {
                request: Request::SearchProtein(q.restart()),
                fresh: true,
            },
        ));
    }
    steps
}

fn section_source(section: biomcp_types::ProteinSection) -> biomcp_types::Source {
    match section {
        biomcp_types::ProteinSection::Domains => biomcp_types::Source::InterPro,
        biomcp_types::ProteinSection::Complexes => biomcp_types::Source::ComplexPortal,
        _ => biomcp_types::Source::UniProt,
    }
}
