//! Snapshots of saved searches and the change reports derived from two of them
//! (sdlc/planning/saved-work.md, Change detection). A snapshot keeps identifiers and row
//! fingerprints, never source content, and each search keeps its two newest.
use biodata::Text;
use biomcp_types::{
    ChangeReport, ChangeSnapshot, ChangeState, SavedRef, SavedSearch, SavedSkipped, SavedSnapshot,
    SavedSnapshotRow, SavedSnapshotSource, Source,
};
use rusqlite::{Connection, OptionalExtension, Transaction, params};
use std::collections::{BTreeMap, BTreeSet};

use super::store::{seconds, text, unsigned};
use super::{SavedError, SavedStore};

/// A saved search reads its source at most once an hour; inside the floor a check is not due.
pub const FLOOR_SECONDS: u64 = 3_600;
/// Most matching records a snapshot holds; a search matching more is incomplete.
pub const SNAPSHOT_ROWS: usize = 500;
/// Snapshots kept per saved search.
const KEPT: usize = 2;

/// One matching record of a check.
#[derive(Debug, Clone)]
pub struct SnapshotRow {
    /// Entity word of the get command.
    pub entity: Text,
    /// Exact identifier the get command accepts.
    pub id: Text,
    /// Source the identifier belongs to.
    pub source: Option<Source>,
    /// SHA-256 of the search row in hex.
    pub fingerprint: Text,
}

/// What one check of a saved search read.
#[derive(Debug, Clone)]
pub struct NewSnapshot {
    /// Provider matching count, or the rows held when it stated none.
    pub total: u64,
    /// Whether the rows hold every matching record.
    pub complete: bool,
    /// Reply schema the fingerprints were computed under.
    pub row_schema: Text,
    /// Sources read.
    pub sources: Vec<SavedSnapshotSource>,
    /// Matching records in source order.
    pub rows: Vec<SnapshotRow>,
}

impl SavedStore {
    /// Saved searches, or the one with this name; a list's name is unknown here.
    pub fn searches(&self, name: Option<&str>) -> Result<Vec<SavedSearch>, SavedError> {
        let saved = self.show(name)?;
        if name.is_some() && saved.searches.is_empty() {
            return Err(SavedError::Unknown);
        }
        Ok(saved.searches)
    }

    /// A search's kept snapshots, newest first; rows this build cannot read go to `skipped`.
    pub fn snapshots(
        &self,
        search: &str,
        skipped: &mut Vec<SavedSkipped>,
    ) -> Result<Vec<SavedSnapshot>, SavedError> {
        kept(&self.connection, search)?
            .iter()
            .map(|id| snapshot(&self.connection, id, skipped))
            .collect()
    }

    /// Every kept snapshot of every saved search, by search name and then oldest first; rows
    /// this build cannot read go to `skipped`.
    pub fn all_snapshots(
        &self,
        skipped: &mut Vec<SavedSkipped>,
    ) -> Result<Vec<SavedSnapshot>, SavedError> {
        let mut statement = self.connection.prepare(
            "SELECT s.id FROM saved_snapshot s JOIN saved_search q ON q.id=s.search \
             ORDER BY q.project, q.name COLLATE NOCASE, q.id, s.taken, s.rowid",
        )?;
        let ids = statement
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        ids.iter()
            .map(|id| snapshot(&self.connection, id, skipped))
            .collect()
    }

    /// Keep a new snapshot, drop all but the two newest, mark the search checked, and
    /// report what changed since the snapshot before it.
    pub fn record_check(
        &mut self,
        search: &SavedSearch,
        new: &NewSnapshot,
    ) -> Result<ChangeReport, SavedError> {
        self.write(|tx, now| {
            let id = search.id.as_str();
            let previous = kept(tx, id)?
                .first()
                // `saved changed` reports the previous snapshot's skips when it reads it.
                .map(|previous| snapshot(tx, previous, &mut Vec::new()))
                .transpose()?;
            let sources = serde_json::to_string(&new.sources).map_err(|_| SavedError::Database)?;
            tx.execute(
                "INSERT INTO saved_snapshot(id,search,taken,total,complete,row_schema,sources) \
                 VALUES(lower(hex(randomblob(16))),?1,?2,?3,?4,?5,?6)",
                params![
                    id,
                    seconds(now)?,
                    i64::try_from(new.total).map_err(|_| SavedError::Database)?,
                    new.complete,
                    new.row_schema.as_str(),
                    sources
                ],
            )?;
            let taken: String = tx.query_row(
                "SELECT id FROM saved_snapshot WHERE rowid=last_insert_rowid()",
                [],
                |row| row.get(0),
            )?;
            for (position, row) in new.rows.iter().enumerate() {
                tx.execute(
                    "INSERT INTO saved_snapshot_row(snapshot,position,entity,id,source,fingerprint) \
                     VALUES(?1,?2,?3,?4,?5,?6)",
                    params![
                        taken,
                        i64::try_from(position).map_err(|_| SavedError::Database)?,
                        row.entity.as_str(),
                        row.id.as_str(),
                        row.source
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_default(),
                        row.fingerprint.as_str()
                    ],
                )?;
            }
            let older = kept(tx, id)?;
            for old in older.iter().skip(KEPT) {
                delete_snapshot(tx, old)?;
            }
            tx.execute(
                "UPDATE saved_search SET checked=?2 WHERE id=?1",
                params![id, seconds(now)?],
            )?;
            let current = snapshot(tx, &taken, &mut Vec::new())?;
            Ok(compare(&search.name, previous.as_ref(), &current))
        })
    }
}

/// Unix seconds now, as the saved file records times.
pub fn now() -> u64 {
    u64::try_from(crate::store::Store::now()).unwrap_or(0)
}

/// The report for a search checked inside the floor, which runs nothing; None when due.
pub fn not_due(
    search: &SavedSearch,
    newest: Option<&SavedSnapshot>,
    now: u64,
) -> Option<ChangeReport> {
    let next_due = search.checked?.saturating_add(FLOOR_SECONDS);
    (now < next_due).then(|| ChangeReport {
        search: search.name.clone(),
        from: None,
        to: newest.map(mark),
        state: ChangeState::NotDue,
        new: Vec::new(),
        gone: Vec::new(),
        changed: Vec::new(),
        next_due,
    })
}

/// The report for a due search whose check could not finish; nothing was kept, so it stays due.
pub fn failed(search: &SavedSearch, newest: Option<&SavedSnapshot>, now: u64) -> ChangeReport {
    ChangeReport {
        search: search.name.clone(),
        from: None,
        to: newest.map(mark),
        state: ChangeState::Failed,
        new: Vec::new(),
        gone: Vec::new(),
        changed: Vec::new(),
        next_due: now,
    }
}

/// What `to` gained, lost and changed since `from`. IDs compare only when both snapshots are
/// complete; rows compare only when they also share a row schema.
pub fn compare(name: &Text, from: Option<&SavedSnapshot>, to: &SavedSnapshot) -> ChangeReport {
    let mut report = ChangeReport {
        search: name.clone(),
        from: from.map(mark),
        to: Some(mark(to)),
        state: ChangeState::Unchanged,
        new: Vec::new(),
        gone: Vec::new(),
        changed: Vec::new(),
        next_due: to.taken.saturating_add(FLOOR_SECONDS),
    };
    let Some(from) = from else {
        report.state = ChangeState::First;
        return report;
    };
    if !from.complete || !to.complete {
        report.state = if same_rows(from, to) {
            ChangeState::Incomplete
        } else {
            ChangeState::Rebaselined
        };
        return report;
    }
    // IDs do not depend on the row format, so a rebaseline still lists new and gone.
    let rebaselined = !same_rows(from, to);
    let before: BTreeMap<_, _> = from.rows.iter().map(|row| (key(row), row)).collect();
    let after: BTreeSet<_> = to.rows.iter().map(key).collect();
    for row in &to.rows {
        match before.get(&key(row)) {
            None => report.new.push(row.reference.clone()),
            Some(old) if !rebaselined && old.fingerprint != row.fingerprint => {
                report.changed.push(row.reference.clone());
            }
            Some(_) => {}
        }
    }
    report.gone = from
        .rows
        .iter()
        .filter(|row| !after.contains(&key(row)))
        .map(|row| row.reference.clone())
        .collect();
    if rebaselined {
        report.state = ChangeState::Rebaselined;
    } else if !(report.new.is_empty() && report.gone.is_empty() && report.changed.is_empty()) {
        report.state = ChangeState::Changed;
    }
    report
}

/// Whether two snapshots share a row format. Only the row digest after `#` counts, so a new
/// reply schema ID whose rows are unchanged, such as an envelope-only major, compares rows.
fn same_rows(from: &SavedSnapshot, to: &SavedSnapshot) -> bool {
    let digest = |schema: &str| {
        schema
            .rsplit_once('#')
            .map_or(schema, |(_, digest)| digest)
            .to_owned()
    };
    digest(from.row_schema.as_str()) == digest(to.row_schema.as_str())
}

fn key(row: &SavedSnapshotRow) -> (&str, &str, Option<String>) {
    (
        row.reference.entity.as_str(),
        row.reference.id.as_str(),
        row.reference.source.as_ref().map(ToString::to_string),
    )
}

fn mark(snapshot: &SavedSnapshot) -> ChangeSnapshot {
    ChangeSnapshot {
        id: snapshot.id.clone(),
        taken: snapshot.taken,
        total: snapshot.total,
    }
}

/// Snapshot IDs of a search, newest first.
fn kept(connection: &Connection, search: &str) -> Result<Vec<String>, SavedError> {
    let mut statement = connection
        .prepare("SELECT id FROM saved_snapshot WHERE search=?1 ORDER BY taken DESC, rowid DESC")?;
    let ids = statement
        .query_map([search], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ids)
}

fn delete_snapshot(tx: &Transaction<'_>, id: &str) -> Result<(), SavedError> {
    tx.execute("DELETE FROM saved_snapshot_row WHERE snapshot=?1", [id])?;
    tx.execute("DELETE FROM saved_snapshot WHERE id=?1", [id])?;
    Ok(())
}

/// Delete every snapshot of a search.
pub(super) fn delete_snapshots(tx: &Transaction<'_>, search: &str) -> Result<(), SavedError> {
    for id in kept(tx, search)? {
        delete_snapshot(tx, &id)?;
    }
    Ok(())
}

type SnapshotHead = (String, String, i64, i64, bool, String, String);
type RowColumns = (String, String, String, String);

fn snapshot(
    connection: &Connection,
    id: &str,
    skipped: &mut Vec<SavedSkipped>,
) -> Result<SavedSnapshot, SavedError> {
    let (id, search, taken, total, complete, row_schema, sources): SnapshotHead = connection
        .query_row(
            "SELECT id,search,taken,total,complete,row_schema,sources FROM saved_snapshot \
             WHERE id=?1",
            [id],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                ))
            },
        )?;
    let taken = unsigned(taken)?;
    let name: Option<String> = connection
        .query_row(
            "SELECT name FROM saved_search WHERE id=?1",
            [&search],
            |row| row.get(0),
        )
        .optional()?;
    let mut skip = |record: &str, value: &str| -> Result<(), SavedError> {
        skipped.push(SavedSkipped {
            record: text(record)?,
            name: name.as_deref().map(text).transpose()?,
            field: "source".into(),
            value: text(value)?,
        });
        Ok(())
    };
    let mut statement = connection.prepare(
        "SELECT entity,id,source,fingerprint FROM saved_snapshot_row WHERE snapshot=?1 \
         ORDER BY position",
    )?;
    let columns = statement
        .query_map([&id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
        .collect::<Result<Vec<RowColumns>, _>>()?;
    let mut rows = Vec::with_capacity(columns.len());
    for (entity, record, source, fingerprint) in columns {
        let source = match (!source.is_empty()).then(|| source.parse::<Source>()) {
            None => None,
            Some(Ok(known)) => Some(known),
            Some(Err(_)) => {
                skip(&record, &source)?;
                continue;
            }
        };
        rows.push(SavedSnapshotRow {
            reference: SavedRef {
                entity: text(&entity)?,
                id: text(&record)?,
                source,
                as_of: taken,
                source_date: None,
            },
            fingerprint: text(&fingerprint)?,
        });
    }
    let sources = read_sources(&sources, |value| skip(&id, value))?;
    Ok(SavedSnapshot {
        id: text(&id)?,
        search: text(&search)?,
        taken,
        total: unsigned(total)?,
        complete,
        row_schema: text(&row_schema)?,
        sources,
        rows,
    })
}

/// A snapshot's stored sources; one a newer build wrote goes to `skip`.
fn read_sources(
    stored: &str,
    mut skip: impl FnMut(&str) -> Result<(), SavedError>,
) -> Result<Vec<SavedSnapshotSource>, SavedError> {
    let stored: Vec<serde_json::Value> =
        serde_json::from_str(stored).map_err(|_| SavedError::Database)?;
    let mut read = Vec::with_capacity(stored.len());
    for source in stored {
        let value = source
            .get("source")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown")
            .to_owned();
        match serde_json::from_value::<SavedSnapshotSource>(source) {
            Ok(source) => read.push(source),
            Err(_) => skip(&value)?,
        }
    }
    Ok(read)
}
