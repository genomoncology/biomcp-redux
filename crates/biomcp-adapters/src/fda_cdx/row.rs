//! One stored FDA companion row as a reply row that states each cell's text once.
use biodata::{CompanionDiagnosticRelation, DiagnosticCell, FdaDrugApplication, PartialDate};
use biomcp_types::{
    CompanionColumn, CompanionDiagnosticRow, CompanionLinks, CompanionMatch, CompanionOwned,
    CompanionShared, DiagnosticSearch,
};

/// FDA's tables break and pad one test's name differently; read it as one line (0994).
fn line(t: &str) -> Box<str> {
    t.split_whitespace().collect::<Vec<_>>().join(" ").into()
}

/// The row's diagnostic and drug cells, the two `--name` searches.
fn named(row: &CompanionDiagnosticRelation) -> [(CompanionColumn, &DiagnosticCell); 2] {
    let (diagnostic, drug) = match row {
        CompanionDiagnosticRelation::Individual { cells, .. } => (&cells.diagnostic, &cells.drug),
        CompanionDiagnosticRelation::Group { cells, .. } => (&cells.diagnostic, &cells.drug_group),
    };
    [
        (CompanionColumn::Diagnostic, diagnostic),
        (CompanionColumn::Drug, drug),
    ]
}

/// The cells whose text names the asked test or drug whole, in any case; a name FDA breaks
/// across lines or pads with no-break spaces still matches (1084).
pub(super) fn name_match(row: &CompanionDiagnosticRelation, asked: &str) -> Box<[CompanionColumn]> {
    let asked = line(asked);
    named(row)
        .into_iter()
        .filter(|(_, cell)| super::biomarker::names(&line(&cell.text), &asked))
        .map(|(column, _)| column)
        .collect()
}

/// Match only names attached to explicit combination wording in the drug cell.
pub(super) fn regimen_partner(row: &CompanionDiagnosticRelation, asked: &str) -> bool {
    let asked = line(asked).to_lowercase();
    match row {
        CompanionDiagnosticRelation::Individual { cells, .. } => {
            partner_in(&cells.drug.text, &asked)
        }
        // The HTML reader puts each group-table list entry on its own line.
        CompanionDiagnosticRelation::Group { cells, .. } => cells
            .drug_group
            .text
            .lines()
            .any(|entry| partner_in(entry, &asked)),
    }
}

fn partner_in(entry: &str, asked: &str) -> bool {
    let entry = line(entry).to_lowercase();
    entry
        .split(" or ")
        .filter_map(|clause| {
            clause.strip_prefix("in combination with ").or_else(|| {
                clause
                    .split_once(" in combination with ")
                    .map(|(_, partners)| partners)
            })
        })
        .flat_map(|partners| partners.split(','))
        .flat_map(|part| part.split(" and "))
        .any(|part| {
            let name = without_application(part.trim());
            let Some((trade, generic)) = name.split_once('(') else {
                return name == asked;
            };
            trade.trim_end() == asked
                || generic
                    .strip_suffix(')')
                    .is_some_and(|name| name.trim() == asked)
        })
}

/// Application numbers sometimes follow a partner name in the same cell.
fn without_application(part: &str) -> &str {
    let Some((before, last)) = part.rsplit_once(' ') else {
        return part;
    };
    let id = last
        .split('/')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let attached = id.parse::<FdaDrugApplication>().is_ok();
    let separate = before.rsplit_once(' ').filter(|(_, kind)| {
        matches!(*kind, "nda" | "bla")
            && format!("{kind}{id}")
                .to_ascii_uppercase()
                .parse::<FdaDrugApplication>()
                .is_ok()
    });
    let name = if attached {
        before
    } else {
        separate.map_or(part, |(name, _)| name)
    };
    name.trim_end_matches([' ', '-', '–', '—'])
}

/// FDA's original row with its date, each cell's text under one name, and its links by cell.
pub(super) fn reply(
    native: super::raw::Native,
    date: Option<PartialDate>,
    query: &DiagnosticSearch,
) -> CompanionDiagnosticRow {
    let name_match = query
        .name()
        .map(|asked| name_match(&native.relation, asked.as_str()))
        .unwrap_or_default();
    let matched = [
        query
            .drug_application()
            .is_some()
            .then_some(CompanionMatch::DrugApplication),
        query
            .regimen_partner()
            .is_some()
            .then_some(CompanionMatch::RegimenPartner),
    ]
    .into_iter()
    .flatten()
    .collect();
    let biomarker = query.biomarker();
    let negated = |cell: &DiagnosticCell| {
        biomarker.is_some_and(|asked| super::biomarker::negated(&cell.text, asked.as_str()))
    };
    let text = |cell: &DiagnosticCell| Some(cell.text.clone());
    match native.relation {
        CompanionDiagnosticRelation::Individual {
            index,
            cells,
            device_ids,
            drug_applications,
            ..
        } => CompanionDiagnosticRow::Individual {
            shared: CompanionShared {
                row_date: date,
                diagnostic: Some(line(&cells.diagnostic.text)),
                indication: Some(line(&cells.indication.text)),
                device: text(&cells.device),
                drug: text(&cells.drug),
            },
            biomarker: text(&cells.biomarker),
            biomarker_details: text(&cells.details),
            biomarker_negated: negated(&cells.biomarker),
            owned: CompanionOwned {
                index,
                links: CompanionLinks {
                    diagnostic: cells.diagnostic.links,
                    indication: cells.indication.links,
                    device: cells.device.links,
                    drug: cells.drug.links,
                    biomarker: cells.biomarker.links,
                    biomarker_details: cells.details.links,
                },
                name_match,
                matched,
                device_ids,
                drug_applications: drug_applications.iter().map(|v| v.id.clone()).collect(),
            },
        },
        CompanionDiagnosticRelation::Group {
            index,
            cells,
            device_ids,
            drug_applications,
            ..
        } => CompanionDiagnosticRow::Group {
            shared: CompanionShared {
                row_date: date,
                diagnostic: Some(line(&cells.diagnostic.text)),
                indication: Some(line(&cells.indication.text)),
                device: text(&cells.device),
                drug: text(&cells.drug_group),
            },
            owned: CompanionOwned {
                index,
                links: CompanionLinks {
                    diagnostic: cells.diagnostic.links,
                    indication: cells.indication.links,
                    device: cells.device.links,
                    drug: cells.drug_group.links,
                    ..CompanionLinks::default()
                },
                name_match,
                matched,
                device_ids,
                drug_applications: drug_applications.iter().map(|v| v.id.clone()).collect(),
            },
        },
    }
}
