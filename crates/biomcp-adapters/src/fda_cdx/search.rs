use biodata::CompanionDiagnosticRelation;
use biomcp_fetch::{BundleInfo, Find, KeyKind, Leg, RequestError};
use biomcp_types::{DiagnosticSearch, Problem, ProblemCode, Source};
/// Native physical association page from one stored complete snapshot.
#[derive(Debug)]
pub struct Search {
    /// Whole physical rows in source order.
    pub items: Box<[CompanionDiagnosticRelation]>,
    /// Matching physical rows, not tests or unique devices.
    pub total: u64,
    /// Exact local snapshot identity and retrieval metadata.
    pub info: BundleInfo,
    /// Identity conflicts and unsupported group-table biomarker evaluation.
    pub problems: Vec<Problem>,
    /// The group table has no biomarker column; only table zero was evaluated.
    pub group_unsupported: bool,
}
/// Search source-owned fields without synchronizing or parsing clinical text.
pub fn search(leg: &Leg<'_>, query: &DiagnosticSearch) -> Result<Search, RequestError> {
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
        return Err(biomcp_fetch::SourceError {
            source: Source::FdaCdx,
            reason: biomcp_types::Reason::Rejected,
            status: None,
            retry_after: None,
        }
        .into());
    }
    let snapshot = info
        .content_revision
        .as_deref()
        .ok_or_else(|| biomcp_fetch::bad_reply(biomcp_types::Source::FdaCdx, None))?;
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
            matching.push(native);
        }
    }
    let total = matching.len() as u64;
    let mut problems = Vec::new();
    let items = matching
        .into_iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
        .map(|row| {
            let (table, index) = match &row.relation {
                CompanionDiagnosticRelation::Individual { index, .. } => (0, index),
                CompanionDiagnosticRelation::Group { index, .. } => (1, index),
            };
            for (conflict, field) in [(row.device_conflict, "device"), (row.drug_conflict, "drug")]
            {
                if conflict {
                    problems.push(Problem {
                        source: Source::FdaCdx,
                        record: biodata::Text::new(&format!("{snapshot}:{table}:{index}")).ok(),
                        field: field.into(),
                        code: ProblemCode::IdentityConflict,
                    });
                }
            }
            row.relation
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
        group_unsupported,
    })
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
