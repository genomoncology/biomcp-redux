//! CIViC accepts one native level per query; concatenate distinct level streams.
use super::variant::failure;
use super::variant_raw::{Connection, Page};
use biodata::Text;
use biomcp_fetch::RequestError;
use biomcp_types::{CivicEvidenceFilter, CivicEvidenceLevel, Reason};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
struct Cursor {
    level: usize,
    after: Option<Text>,
}

pub(super) fn arguments(
    filter: &CivicEvidenceFilter,
    level: Option<CivicEvidenceLevel>,
    default_status: &str,
) -> String {
    let status = filter.evidence_status.map_or_else(
        || default_status.to_owned(),
        |status| format!("status:{}", status.as_str().to_ascii_uppercase()),
    );
    match level {
        Some(level) => format!("{status},evidenceLevel:{}", level.as_str()),
        None => status,
    }
}

/// Preserve source totals and cursors without filtering any returned record.
pub(super) fn page<T>(
    filter: &CivicEvidenceFilter,
    cursor: Option<&Text>,
    count: u32,
    mut fetch: impl FnMut(
        Option<CivicEvidenceLevel>,
        Option<&Text>,
        u32,
    ) -> Result<Connection<T>, RequestError>,
) -> Result<Connection<T>, RequestError> {
    let cursor: Cursor = cursor
        .map(|cursor| serde_json::from_str(cursor.as_str()))
        .transpose()
        .map_err(|_| failure(Reason::BadReply))?
        .unwrap_or_default();
    let levels = filter.levels();
    if cursor.level >= levels.len() {
        return Err(failure(Reason::BadReply));
    }
    let mut total_count = 0_u64;
    let mut nodes = Vec::new();
    let mut next = None;
    for (index, level) in levels.into_iter().enumerate() {
        let remaining = count.saturating_sub(nodes.len() as u32);
        let active = index >= cursor.level && remaining > 0 && next.is_none();
        let after = (index == cursor.level)
            .then_some(cursor.after.as_ref())
            .flatten();
        let rows = fetch(
            level,
            if active { after } else { None },
            if active { remaining } else { 0 },
        )?;
        total_count = total_count
            .checked_add(rows.total_count)
            .ok_or_else(|| failure(Reason::TooLarge))?;
        if !active {
            if index > cursor.level && next.is_none() && rows.total_count > 0 {
                next = Some(Cursor {
                    level: index,
                    after: None,
                });
            }
            continue;
        }
        if rows.nodes.len() > remaining as usize || !rows.progresses(after) {
            return Err(failure(Reason::BadReply));
        }
        if rows.page_info.has_next_page {
            next = Some(Cursor {
                level: index,
                after: rows.page_info.end_cursor.clone(),
            });
        }
        nodes.extend(rows.nodes);
    }
    let end_cursor = next
        .as_ref()
        .map(|cursor| {
            let value = serde_json::to_string(cursor).map_err(|_| failure(Reason::BadReply))?;
            Text::new(&value).map_err(|_| failure(Reason::BadReply))
        })
        .transpose()?;
    Ok(Connection {
        total_count,
        nodes,
        page_info: Page {
            has_next_page: next.is_some(),
            end_cursor,
        },
    })
}
