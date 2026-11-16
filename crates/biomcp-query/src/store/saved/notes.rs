//! Notes a user attaches to the project, a saved search, a list or a list item
//! (sdlc/planning/saved-work.md, Records: `biomcp.saved-note/1`). A note's ID is a hash
//! of its target and text, so writing the same note again changes nothing.
use biodata::Text;
use biomcp_types::{NoteTargetKind, SavedNote, SavedNoteTarget, SavedRef, Source};
use rusqlite::{Connection, OptionalExtension, params};

use super::store::{NOTE_CHARS, PROJECT, list_id, search_id, seconds, text, unsigned, within};
use super::{SavedError, SavedStore};

/// What a new note is attached to.
#[derive(Debug, Clone, Copy)]
pub enum NoteTarget<'a> {
    /// The project itself.
    Project,
    /// The saved search or list with this name.
    Named(&'a str),
    /// The item with this ID in the named list; the entity word picks one when the list
    /// holds the ID under more than one entity or source.
    Item {
        /// List name.
        list: &'a str,
        /// Item ID.
        id: &'a str,
        /// Entity word, when the ID alone is ambiguous.
        entity: Option<&'a str>,
    },
}

/// A note's target as stored: kind, the search or list ID, and the item's key.
struct Key {
    kind: NoteTargetKind,
    record: String,
    entity: String,
    item: String,
    source: String,
}

impl SavedStore {
    /// Attach a note; the same text on the same target again returns the note unchanged.
    pub fn note(&mut self, target: NoteTarget<'_>, note: &Text) -> Result<SavedNote, SavedError> {
        within(note.as_str(), NOTE_CHARS)?;
        self.write(|tx, now| {
            let key = resolve(tx, target)?;
            let id = note_id(&key, note.as_str());
            tx.execute(
                "INSERT INTO saved_note(id,project,kind,record,entity,item,source,text,created) \
                 VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9) ON CONFLICT DO NOTHING",
                params![
                    id,
                    PROJECT,
                    key.kind.to_string(),
                    key.record,
                    key.entity,
                    key.item,
                    key.source,
                    note.as_str(),
                    seconds(now)?
                ],
            )?;
            load(tx, &id)
        })
    }

    /// Delete one note by its ID and return it.
    pub fn delete_note(&mut self, id: &str) -> Result<SavedNote, SavedError> {
        self.write(|tx, _| {
            let exists: Option<String> = tx
                .query_row("SELECT id FROM saved_note WHERE id=?1", [id], |row| {
                    row.get(0)
                })
                .optional()?;
            let id = exists.ok_or(SavedError::Unknown)?;
            let note = load(tx, &id)?;
            tx.execute("DELETE FROM saved_note WHERE id=?1", [id.as_str()])?;
            Ok(note)
        })
    }
}

fn resolve(connection: &Connection, target: NoteTarget<'_>) -> Result<Key, SavedError> {
    let key = |kind, record: String| Key {
        kind,
        record,
        entity: String::new(),
        item: String::new(),
        source: String::new(),
    };
    match target {
        NoteTarget::Project => Ok(key(NoteTargetKind::Project, String::new())),
        NoteTarget::Named(name) => {
            if let Some(id) = search_id(connection, name)? {
                Ok(key(NoteTargetKind::Search, id))
            } else if let Some(id) = list_id(connection, name)? {
                Ok(key(NoteTargetKind::List, id))
            } else {
                Err(SavedError::Unknown)
            }
        }
        NoteTarget::Item { list, id, entity } => {
            let record = list_id(connection, list)?.ok_or(SavedError::Unknown)?;
            let mut statement = connection.prepare(
                "SELECT entity,source FROM saved_list_item WHERE list=?1 AND id=?2 \
                 AND (?3 IS NULL OR entity=?3) ORDER BY rowid",
            )?;
            let found: Vec<(String, String)> = statement
                .query_map(params![record, id, entity], |row| {
                    Ok((row.get(0)?, row.get(1)?))
                })?
                .collect::<Result<_, _>>()?;
            let [(entity, source)] = <[_; 1]>::try_from(found).map_err(|found| {
                if found.is_empty() {
                    SavedError::ItemUnknown
                } else {
                    SavedError::Ambiguous
                }
            })?;
            Ok(Key {
                kind: NoteTargetKind::Item,
                record,
                entity,
                item: id.to_owned(),
                source,
            })
        }
    }
}

/// First 16 bytes of SHA-256 over the project, target and text, in hex.
fn note_id(key: &Key, note: &str) -> String {
    use sha2::{Digest, Sha256};
    let identity = (
        PROJECT,
        key.kind.to_string(),
        &key.record,
        &key.entity,
        &key.item,
        &key.source,
        note,
    );
    let bytes = serde_json::to_vec(&identity).unwrap_or_default();
    Sha256::digest(&bytes)
        .iter()
        .take(16)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Notes on the project, on one saved search or list and its items, or every note.
pub(super) enum Scope<'a> {
    /// Every note.
    All,
    /// Notes on this search or list ID, and on its items.
    Record(&'a str),
}

/// Note IDs in the order they were written.
pub(super) fn ids(connection: &Connection, scope: Scope<'_>) -> Result<Vec<String>, SavedError> {
    let record = match scope {
        Scope::All => None,
        Scope::Record(record) => Some(record),
    };
    let mut statement = connection
        .prepare("SELECT id FROM saved_note WHERE ?1 IS NULL OR record=?1 ORDER BY rowid")?;
    let rows = statement.query_map([record], |row| row.get(0))?;
    Ok(rows.collect::<Result<_, _>>()?)
}

/// Delete the notes on a search or list and its items.
pub(super) fn delete_record(connection: &Connection, record: &str) -> Result<(), SavedError> {
    connection.execute("DELETE FROM saved_note WHERE record=?1", [record])?;
    Ok(())
}

/// Delete the notes on one list's items with this ID.
pub(super) fn delete_item(connection: &Connection, list: &str, id: &str) -> Result<(), SavedError> {
    connection.execute(
        "DELETE FROM saved_note WHERE record=?1 AND kind='item' AND item=?2",
        params![list, id],
    )?;
    Ok(())
}

type NoteRow = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    i64,
);

pub(super) fn load(connection: &Connection, id: &str) -> Result<SavedNote, SavedError> {
    let row: NoteRow = connection.query_row(
        "SELECT id,project,kind,record,entity,item,source,text,created FROM saved_note WHERE id=?1",
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
                r.get(7)?,
                r.get(8)?,
            ))
        },
    )?;
    let (id, project, kind, record, entity, item, source, note, created) = row;
    let kind: NoteTargetKind = kind.parse().map_err(|_| SavedError::Database)?;
    let table = match kind {
        NoteTargetKind::Search => "saved_search",
        _ => "saved_list",
    };
    let name: Option<String> = if kind == NoteTargetKind::Project {
        None
    } else {
        connection
            .query_row(
                &format!("SELECT name FROM {table} WHERE id=?1"),
                [record.as_str()],
                |row| row.get(0),
            )
            .optional()?
    };
    let reference = if kind == NoteTargetKind::Item {
        let added: Option<i64> = connection
            .query_row(
                "SELECT added FROM saved_list_item WHERE list=?1 AND entity=?2 AND id=?3 \
                 AND source=?4",
                params![record, entity, item, source],
                |row| row.get(0),
            )
            .optional()?;
        Some(SavedRef {
            entity: text(&entity)?,
            id: text(&item)?,
            source: (!source.is_empty())
                .then(|| source.parse::<Source>())
                .transpose()
                .map_err(|_| SavedError::Database)?,
            as_of: unsigned(added.unwrap_or(created))?,
            source_date: None,
        })
    } else {
        None
    };
    Ok(SavedNote {
        id: text(&id)?,
        target: SavedNoteTarget {
            kind,
            project: text(&project)?,
            name: name.as_deref().map(text).transpose()?,
            item: reference,
        },
        text: text(&note)?,
        created: unsigned(created)?,
    })
}
