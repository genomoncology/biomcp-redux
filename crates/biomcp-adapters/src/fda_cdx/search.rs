use super::date::{key, row_date};
use biodata::{CompanionDiagnosticRelation, PartialDate};
use biomcp_fetch::{BundleInfo, Find, KeyKind, Leg, RequestError};
use biomcp_types::{
    CompanionDiagnosticRow, CompanionShared, DiagnosticSearch, Problem, ProblemCode, Source,
};
use std::cmp::Ordering;
/// Native physical association page from one stored complete snapshot.
#[derive(Debug)]
pub struct Search {
    /// Whole physical rows in source order, or newest first over the snapshot when asked.
    pub items: Box<[CompanionDiagnosticRow]>,
    /// Matching physical rows, not tests or unique devices.
    pub total: u64,
    /// Exact local snapshot identity and retrieval metadata.
    pub info: BundleInfo,
    /// Identity conflicts, undated rows and unsupported group-table biomarker evaluation.
    pub problems: Vec<Problem>,
}
/// Search source-owned fields without synchronizing or parsing clinical text.
pub fn search(leg: &Leg<'_>, query: &DiagnosticSearch) -> Result<Search, RequestError> {
    let invalid_reply = biomcp_fetch::bad_reply(biomcp_types::Source::FdaCdx, None);
    let find = if let Some(id) = query.device_id() {
        Find::Exact(KeyKind::Id, id.as_str())
    } else if let Some(id) = query.drug_application() {
        Find::Exact(KeyKind::Xref, id.as_str())
    } else if let Some(value) = query.biomarker() {
        Find::Exact(KeyKind::Symbol, value.as_str())
    } else {
        Find::Exact(KeyKind::Band, "all")
    };
    let (rows, info) = super::bundle::read(leg, find)?;
    if query
        .bundle_revision()
        .is_some_and(|v| Some(v.as_str()) != info.content_revision.as_deref())
    {
        return Err(changed_revision());
    }
    let snapshot = info.content_revision.as_deref().ok_or(invalid_reply)?;
    let mut matching = Vec::new();
    for row in rows {
        let mut native = row.value;
        if matches(&native.relation, query) {
            match &mut native.relation {
                CompanionDiagnosticRelation::Individual {
                    snapshot: value, ..
                }
                | CompanionDiagnosticRelation::Group {
                    snapshot: value, ..
                } => *value = snapshot.into(),
            }
            let date = row_date(&device_cell(&native.relation).text);
            matching.push((native, date));
        }
    }
    // The window and the order apply to every matching row of the snapshot, never to one page.
    let whole = query.sort().is_some() || query.date_from().is_some() || query.date_to().is_some();
    let mut conflicts = Vec::new();
    if whole {
        conflicts.extend(matching.iter().filter_map(undated));
    }
    matching.retain(|(_, date)| within(query, *date));
    if query.sort().is_some() {
        matching.sort_by(newest_first);
    }
    let total = matching.len() as u64;
    let items = matching
        .into_iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
        .map(|row| {
            conflicts.extend(conflicts_of(&row.0));
            if !whole {
                conflicts.extend(undated(&row));
            }
            shared_row(row)
        })
        .collect();
    if let Some(id) = query.drug_application() {
        // A row that mentions the asked application but joins nothing is named, not dropped.
        let digits = id.as_str().get(3..).unwrap_or_default();
        let (all, _) = super::bundle::read(leg, Find::Exact(KeyKind::Band, "all"))?;
        for row in all {
            conflicts.extend(unjoined(&row.value, digits));
        }
    }
    let mut problems: Vec<Problem> = conflicts
        .into_iter()
        .map(|(table, index, field, code)| Problem {
            source: Source::FdaCdx,
            record: biodata::Text::new(&format!("{snapshot}:{table}:{index}")).ok(),
            field: field.into(),
            code,
        })
        .collect();
    let group_unsupported = query.biomarker().is_some();
    if group_unsupported {
        problems.push(Problem {
            source: Source::FdaCdx,
            record: None,
            field: "table_1.biomarker".into(),
            code: ProblemCode::ValueUnknown,
        });
    }
    Ok(Search {
        items,
        total,
        info,
        problems,
    })
}
/// Problems for a row that mentions the application digits but joins no application.
fn unjoined(row: &super::raw::Native, digits: &str) -> Vec<(u8, u32, &'static str, ProblemCode)> {
    if !mentions(&row.relation, digits) {
        return Vec::new();
    }
    let mut found = conflicts_of(row);
    if found.is_empty() && applications(&row.relation).is_empty() {
        // The text names it but no link carries an application number.
        let (table, index) = place(&row.relation);
        found.push((table, index, "drug", ProblemCode::ValueUnknown));
    }
    found
}
/// Whether the row's drug cell text or a link address contains the application digits.
fn mentions(row: &CompanionDiagnosticRelation, digits: &str) -> bool {
    let cell = match row {
        CompanionDiagnosticRelation::Individual { cells, .. } => &cells.drug,
        CompanionDiagnosticRelation::Group { cells, .. } => &cells.drug_group,
    };
    !digits.is_empty()
        && (cell.text.contains(digits) || cell.links.iter().any(|l| l.href.contains(digits)))
}
/// Table and physical row position.
fn place(row: &CompanionDiagnosticRelation) -> (u8, u32) {
    match row {
        CompanionDiagnosticRelation::Individual { index, .. } => (0, *index),
        CompanionDiagnosticRelation::Group { index, .. } => (1, *index),
    }
}
fn applications(row: &CompanionDiagnosticRelation) -> &[biodata::DiagnosticDrugApplication] {
    match row {
        CompanionDiagnosticRelation::Individual {
            drug_applications, ..
        }
        | CompanionDiagnosticRelation::Group {
            drug_applications, ..
        } => drug_applications,
    }
}
/// Table, row, field and code of each identity cell this stored row could not read.
fn conflicts_of(row: &super::raw::Native) -> Vec<(u8, u32, &'static str, ProblemCode)> {
    let (table, index) = place(&row.relation);
    [(row.device_conflict, "device"), (row.drug_conflict, "drug")]
        .into_iter()
        .filter(|(conflict, _)| *conflict)
        .map(|(_, field)| (table, index, field, ProblemCode::IdentityConflict))
        .collect()
}
/// The row's own device cell.
fn device_cell(row: &CompanionDiagnosticRelation) -> &biodata::DiagnosticCell {
    match row {
        CompanionDiagnosticRelation::Individual { cells, .. } => &cells.device,
        CompanionDiagnosticRelation::Group { cells, .. } => &cells.device,
    }
}
/// A problem naming a row whose device cell gives no date.
fn undated(
    (row, date): &(super::raw::Native, Option<PartialDate>),
) -> Option<(u8, u32, &'static str, ProblemCode)> {
    let (table, index) = place(&row.relation);
    date.is_none()
        .then_some((table, index, "date", ProblemCode::ValueUnknown))
}
/// Whether the row date falls in the asked window; an undated row falls in no window.
fn within(query: &DiagnosticSearch, date: Option<PartialDate>) -> bool {
    if query.date_from().is_none() && query.date_to().is_none() {
        return true;
    }
    date.is_some_and(|d| {
        query.date_from().is_none_or(|from| key(d) >= key(from))
            && query.date_to().is_none_or(|to| key(d) <= key(to))
    })
}
/// Newest date first, undated rows last, then physical table and row order.
fn newest_first(
    a: &(super::raw::Native, Option<PartialDate>),
    b: &(super::raw::Native, Option<PartialDate>),
) -> Ordering {
    let by_date = match (a.1, b.1) {
        (Some(x), Some(y)) => key(y).cmp(&key(x)),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    };
    by_date.then_with(|| place(&a.0.relation).cmp(&place(&b.0.relation)))
}
/// FDA's original row with its date and the names both tables share.
fn shared_row((native, date): (super::raw::Native, Option<PartialDate>)) -> CompanionDiagnosticRow {
    let text = |cell: &biodata::DiagnosticCell| Some(cell.text.clone());
    match native.relation {
        CompanionDiagnosticRelation::Individual {
            snapshot,
            index,
            cells,
            device_ids,
            drug_applications,
        } => CompanionDiagnosticRow::Individual {
            shared: CompanionShared {
                date,
                diagnostic: text(&cells.diagnostic),
                indication: text(&cells.indication),
                device: text(&cells.device),
                drug: text(&cells.drug),
            },
            snapshot,
            index,
            cells,
            device_ids,
            drug_applications,
        },
        CompanionDiagnosticRelation::Group {
            snapshot,
            index,
            cells,
            device_ids,
            drug_applications,
        } => CompanionDiagnosticRow::Group {
            shared: CompanionShared {
                date,
                diagnostic: text(&cells.diagnostic),
                indication: text(&cells.indication),
                device: text(&cells.device),
                drug: text(&cells.drug_group),
            },
            snapshot,
            index,
            cells,
            device_ids,
            drug_applications,
        },
    }
}
fn changed_revision() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::FdaCdx,
        reason: biomcp_types::Reason::Rejected,
        status: None,
        tls_cause: None,
        retry_after: None,
    }
    .into()
}
fn matches(row: &CompanionDiagnosticRelation, query: &DiagnosticSearch) -> bool {
    let (devices, drugs, biomarker) = match row {
        CompanionDiagnosticRelation::Individual {
            device_ids,
            drug_applications,
            cells,
            ..
        } => (
            device_ids,
            drug_applications,
            Some(cells.biomarker.text.as_ref()),
        ),
        CompanionDiagnosticRelation::Group {
            device_ids,
            drug_applications,
            ..
        } => (device_ids, drug_applications, None),
    };
    query.device_id().is_none_or(|q| {
        devices.iter().any(|id| {
            id == q
                || !q.as_str().contains('/')
                    && id
                        .as_str()
                        .split_once('/')
                        .is_some_and(|(base, _)| base == q.as_str())
        })
    }) && query
        .drug_application()
        .is_none_or(|q| drugs.iter().any(|v| &v.id == q))
        && query
            .biomarker()
            .is_none_or(|q| biomarker == Some(q.as_str()))
}
