//! One stored FDA companion row as a reply row that states each cell's text once.
use biodata::{CompanionDiagnosticRelation, DiagnosticCell, PartialDate};
use biomcp_types::{
    CompanionColumn, CompanionDiagnosticRow, CompanionLinks, CompanionOwned, CompanionShared,
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

/// FDA's original row with its date, each cell's text under one name, and its links by cell.
pub(super) fn reply(
    native: super::raw::Native,
    date: Option<PartialDate>,
    (name, biomarker): (Option<&str>, Option<&str>),
) -> CompanionDiagnosticRow {
    let name_match = name
        .map(|asked| name_match(&native.relation, asked))
        .unwrap_or_default();
    let negated = |cell: &DiagnosticCell| biomarker.is_some_and(|b| super::biomarker::negated(&cell.text, b));
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
                device_ids,
                drug_applications: drug_applications.iter().map(|v| v.id.clone()).collect(),
            },
        },
    }
}
