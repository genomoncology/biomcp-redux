//! Open saved.sqlite and read or change its searches and lists.
use biodata::Text;
use biomcp_types::{Saved, SavedList, SavedListItem, SavedRef, SavedSearch, SavedSkipped, Source};
use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior, params};
use std::path::Path;

use super::MIGRATIONS;

/// The project every command uses until projects can be chosen.
pub const PROJECT: &str = "default";
/// Qualifier selecting an item saved without an explicit source.
pub const DEFAULT_ITEM_SOURCE: &str = "default";
/// Longest accepted name, in characters.
pub const NAME_CHARS: usize = 80;
/// Longest accepted note, in characters.
pub const NOTE_CHARS: usize = 2_000;

/// SHA-256 of the checked request in hex, so the same request always gives the same key.
pub fn request_key(request: &biomcp_types::Request) -> String {
    use sha2::{Digest, Sha256};
    let bytes = serde_json::to_vec(request).unwrap_or_default();
    Sha256::digest(&bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Why a saved-work command could not finish.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SavedError {
    /// A newer build wrote the file; it was not changed.
    Newer,
    /// The name already holds a different search, or a list.
    Taken,
    /// No saved search or list has the name.
    Unknown,
    /// The file could not be read or written.
    Database,
    /// A name is over 80 characters or a note over 2,000.
    TooLong,
    /// The list holds no item with this ID.
    ItemUnknown,
    /// The list holds the ID under more than one entity or source.
    Ambiguous,
    /// An item removal needs an entity to distinguish its matching records.
    RemoveEntity(Box<[Text]>),
    /// An item removal needs a source to distinguish its matching records.
    RemoveSource(Box<[Text]>),
}

/// Refuse a name or note over its limit in characters.
pub(super) fn within(value: &str, limit: usize) -> Result<(), SavedError> {
    if value.chars().count() > limit {
        Err(SavedError::TooLong)
    } else {
        Ok(())
    }
}
impl From<rusqlite::Error> for SavedError {
    fn from(error: rusqlite::Error) -> Self {
        biomcp_fetch::record_sqlite_error("query.saved", &error);
        Self::Database
    }
}

/// One item to keep in a list.
#[derive(Debug, Clone)]
pub struct NewItem {
    /// Entity word of the get command.
    pub entity: Text,
    /// Exact identifier the get command accepts.
    pub id: Text,
    /// Source the identifier belongs to.
    pub source: Option<Source>,
    /// The user's note.
    pub note: Option<Text>,
}

/// An open saved-work file.
#[derive(Debug)]
pub struct SavedStore {
    pub(super) connection: Connection,
}

/// Highest file compatibility this build reads. Raise it only for a change older 1.x builds
/// cannot read; additive migrations never raise it (sdlc/planning/saved-work.md, Compatibility).
pub const COMPATIBILITY: i64 = 1;

/// A meta value; a file without the meta table has none.
fn meta(connection: &Connection, key: &str) -> Result<Option<i64>, SavedError> {
    let has_meta: bool = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='meta')",
        [],
        |row| row.get(0),
    )?;
    if !has_meta {
        return Ok(None);
    }
    Ok(connection
        .query_row("SELECT value FROM meta WHERE key=?1", [key], |row| {
            row.get(0)
        })
        .optional()?)
}

/// How many migration statements the file has run; this only chooses which to run next.
fn version(connection: &Connection) -> Result<usize, SavedError> {
    usize::try_from(meta(connection, "schema")?.unwrap_or(0)).map_err(|_| SavedError::Database)
}

/// Refuse a file that says it needs a newer reader; a missing key means 1.
fn check_compatibility(connection: &Connection) -> Result<(), SavedError> {
    if meta(connection, "compatibility")?.unwrap_or(1) > COMPATIBILITY {
        Err(SavedError::Newer)
    } else {
        Ok(())
    }
}

fn remove_qualifier(keys: &[(String, String)]) -> SavedError {
    let values = |names: Vec<&str>| {
        names
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .filter_map(|name| Text::new(name).ok())
            .collect::<Box<[_]>>()
    };
    let entities = values(keys.iter().map(|(entity, _)| entity.as_str()).collect());
    if entities.len() > 1 {
        SavedError::RemoveEntity(entities)
    } else {
        SavedError::RemoveSource(values(
            keys.iter()
                .map(|(_, source)| {
                    if source.is_empty() {
                        DEFAULT_ITEM_SOURCE
                    } else {
                        source.as_str()
                    }
                })
                .collect(),
        ))
    }
}

impl SavedStore {
    /// Open or create the file and run the migrations it lacks. A file with more additive
    /// migrations than this build knows opens as it is; one needing a newer reader is refused.
    pub fn open(path: &Path) -> Result<Self, SavedError> {
        let mut connection = Connection::open(path)?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        check_compatibility(&connection)?;
        if version(&connection)? < MIGRATIONS.len() {
            let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
            check_compatibility(&tx)?;
            let applied = version(&tx)?;
            if applied < MIGRATIONS.len() {
                for statement in MIGRATIONS.iter().skip(applied) {
                    tx.execute_batch(statement)?;
                }
                tx.execute(
                    "INSERT INTO meta(key,value) VALUES('schema',?1) \
                     ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                    [i64::try_from(MIGRATIONS.len()).map_err(|_| SavedError::Database)?],
                )?;
            }
            tx.commit()?;
        }
        Ok(Self { connection })
    }

    pub(super) fn write<T>(
        &mut self,
        operation: impl FnOnce(&Transaction<'_>, u64) -> Result<T, SavedError>,
    ) -> Result<T, SavedError> {
        let now = u64::try_from(crate::store::Store::now()).unwrap_or(0);
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let result = operation(&tx, now)?;
        tx.commit()?;
        Ok(result)
    }

    /// Refuse a name held by a list or by a search with other words.
    pub fn check_search_name(&self, name: &str, args: &[String]) -> Result<(), SavedError> {
        search_name_free(&self.connection, name, args)
    }

    /// Save a search; saving the same words under the same name again changes nothing.
    pub fn save_search(
        &mut self,
        name: &Text,
        args: &[String],
        schema: &str,
        request_key: &str,
    ) -> Result<SavedSearch, SavedError> {
        within(name.as_str(), NAME_CHARS)?;
        self.write(|tx, now| {
            search_name_free(tx, name.as_str(), args)?;
            if search_id(tx, name.as_str())?.is_none() {
                tx.execute(
                    "INSERT INTO saved_search(id,project,name,args,schema,request_key,created) \
                     VALUES(lower(hex(randomblob(16))),?1,?2,?3,?4,?5,?6)",
                    params![
                        PROJECT,
                        name.as_str(),
                        serde_json::to_string(args).map_err(|_| SavedError::Database)?,
                        schema,
                        request_key,
                        seconds(now)?
                    ],
                )?;
            }
            let id = search_id(tx, name.as_str())?.ok_or(SavedError::Database)?;
            search(tx, &id)
        })
    }

    /// Add items to a list, making the list when it is new. Present items stay as they are.
    pub fn list_add(&mut self, name: &Text, items: &[NewItem]) -> Result<Saved, SavedError> {
        within(name.as_str(), NAME_CHARS)?;
        for note in items.iter().filter_map(|item| item.note.as_ref()) {
            within(note.as_str(), NOTE_CHARS)?;
        }
        self.write(|tx, now| {
            if search_id(tx, name.as_str())?.is_some() {
                return Err(SavedError::Taken);
            }
            if list_id(tx, name.as_str())?.is_none() {
                tx.execute(
                    "INSERT INTO saved_list(id,project,name,created,updated) \
                     VALUES(lower(hex(randomblob(16))),?1,?2,?3,?3)",
                    params![PROJECT, name.as_str(), seconds(now)?],
                )?;
            }
            let id = list_id(tx, name.as_str())?.ok_or(SavedError::Database)?;
            for item in items {
                tx.execute(
                    "INSERT INTO saved_list_item(list,entity,id,source,note,added) \
                     VALUES(?1,?2,?3,?4,?5,?6) ON CONFLICT DO NOTHING",
                    params![
                        id,
                        item.entity.as_str(),
                        item.id.as_str(),
                        item.source
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_default(),
                        item.note.as_ref().map(Text::as_str),
                        seconds(now)?
                    ],
                )?;
            }
            tx.execute(
                "UPDATE saved_list SET updated=?2 WHERE id=?1",
                params![id, seconds(now)?],
            )?;
            let mut saved = Saved::default();
            saved
                .lists
                .get_or_insert_default()
                .push(list(tx, &id, &mut saved.skipped)?);
            Ok(saved)
        })
    }

    /// Remove items with these identifiers from a list. Return the list and the identifiers
    /// it did not hold; a remove that removes nothing leaves `modified` as it was.
    pub fn list_remove(&mut self, name: &str, ids: &[Text]) -> Result<Saved, SavedError> {
        self.list_remove_qualified(name, ids, None, None)
    }

    /// Remove only uniquely matched items, using the given entity and source qualifiers.
    /// An empty source qualifies an item saved without a source.
    pub fn list_remove_qualified(
        &mut self,
        name: &str,
        ids: &[Text],
        entity: Option<&str>,
        source: Option<&str>,
    ) -> Result<Saved, SavedError> {
        self.write(|tx, now| {
            let id = list_id(tx, name)?.ok_or(SavedError::Unknown)?;
            let mut missing = Vec::new();
            let mut removed = Vec::new();
            for item in ids {
                let keys = super::notes::item_keys(tx, &id, item.as_str(), entity, source)?;
                match keys.as_slice() {
                    [(entity, source)] => {
                        tx.execute(
                            "DELETE FROM saved_list_item WHERE list=?1 AND id=?2 AND entity=?3 AND source=?4",
                            params![id, item.as_str(), entity, source],
                        )?;
                        super::notes::delete_item(tx, &id, item.as_str(), entity, source)?;
                        removed.push(item);
                    }
                    [] if !removed.contains(&item) && !missing.contains(item) => {
                        missing.push(item.clone());
                    }
                    [] => {}
                    _ => return Err(remove_qualifier(&keys)),
                }
            }
            if !removed.is_empty() {
                tx.execute(
                    "UPDATE saved_list SET updated=?2 WHERE id=?1",
                    params![id, seconds(now)?],
                )?;
            }
            let mut saved = Saved {
                missing,
                ..Saved::default()
            };
            saved
                .lists
                .get_or_insert_default()
                .push(list(tx, &id, &mut saved.skipped)?);
            Ok(saved)
        })
    }

    /// Every saved search and list, or the one with this name.
    pub fn show(&self, name: Option<&str>) -> Result<Saved, SavedError> {
        select(&self.connection, name)
    }

    /// Delete the search or list with this name, or everything, with their notes, and return
    /// what was deleted.
    pub fn delete(&mut self, name: Option<&str>) -> Result<Saved, SavedError> {
        self.write(|tx, _| {
            let deleted = select(tx, name)?;
            if name.is_none() {
                tx.execute("DELETE FROM saved_note", [])?;
            }
            for search in deleted.searches.iter().flatten() {
                super::notes::delete_record(tx, search.id.as_str())?;
                super::snapshot::delete_snapshots(tx, search.id.as_str())?;
                tx.execute("DELETE FROM saved_search WHERE id=?1", [search.id.as_str()])?;
            }
            for list in deleted.lists.iter().flatten() {
                super::notes::delete_record(tx, list.id.as_str())?;
                tx.execute(
                    "DELETE FROM saved_list_item WHERE list=?1",
                    [list.id.as_str()],
                )?;
                tx.execute("DELETE FROM saved_list WHERE id=?1", [list.id.as_str()])?;
            }
            Ok(deleted)
        })
    }
}

pub(super) fn seconds(now: u64) -> Result<i64, SavedError> {
    i64::try_from(now).map_err(|_| SavedError::Database)
}

fn search_name_free(
    connection: &Connection,
    name: &str,
    args: &[String],
) -> Result<(), SavedError> {
    if list_id(connection, name)?.is_some() {
        return Err(SavedError::Taken);
    }
    let stored: Option<String> = connection
        .query_row(
            "SELECT args FROM saved_search WHERE project=?1 AND name=?2 COLLATE NOCASE",
            params![PROJECT, name],
            |row| row.get(0),
        )
        .optional()?;
    let Some(stored) = stored else {
        return Ok(());
    };
    let words: Vec<String> = serde_json::from_str(&stored).map_err(|_| SavedError::Database)?;
    if words == args {
        Ok(())
    } else {
        Err(SavedError::Taken)
    }
}

pub(super) fn search_id(connection: &Connection, name: &str) -> Result<Option<String>, SavedError> {
    Ok(connection
        .query_row(
            "SELECT id FROM saved_search WHERE project=?1 AND name=?2 COLLATE NOCASE",
            params![PROJECT, name],
            |row| row.get(0),
        )
        .optional()?)
}

pub(super) fn list_id(connection: &Connection, name: &str) -> Result<Option<String>, SavedError> {
    Ok(connection
        .query_row(
            "SELECT id FROM saved_list WHERE project=?1 AND name=?2 COLLATE NOCASE",
            params![PROJECT, name],
            |row| row.get(0),
        )
        .optional()?)
}

/// Every saved search, list and note, or the search or list with this name and its notes.
fn select(connection: &Connection, name: Option<&str>) -> Result<Saved, SavedError> {
    use super::notes::{Scope, ids as note_ids, read as note};
    let mut saved = Saved::default();
    if let Some(name) = name {
        let record = if let Some(id) = search_id(connection, name)? {
            saved
                .searches
                .get_or_insert_default()
                .push(search(connection, &id)?);
            id
        } else if let Some(id) = list_id(connection, name)? {
            saved
                .lists
                .get_or_insert_default()
                .push(list(connection, &id, &mut saved.skipped)?);
            id
        } else {
            return Err(SavedError::Unknown);
        };
        for id in note_ids(connection, Scope::Record(&record))? {
            match note(connection, &id)? {
                Ok(note) => saved.user_notes.push(note),
                Err(skip) => saved.skipped.push(skip),
            }
        }
        return Ok(saved);
    }
    let ids = |sql: &str| -> Result<Vec<String>, SavedError> {
        let mut statement = connection.prepare(sql)?;
        let rows = statement.query_map([], |row| row.get(0))?;
        Ok(rows.collect::<Result<_, _>>()?)
    };
    for id in ids("SELECT id FROM saved_search ORDER BY project, name COLLATE NOCASE, id")? {
        saved
            .searches
            .get_or_insert_default()
            .push(search(connection, &id)?);
    }
    for id in ids("SELECT id FROM saved_list ORDER BY project, name COLLATE NOCASE, id")? {
        saved
            .lists
            .get_or_insert_default()
            .push(list(connection, &id, &mut saved.skipped)?);
    }
    for id in note_ids(connection, Scope::All)? {
        match note(connection, &id)? {
            Ok(note) => saved.user_notes.push(note),
            Err(skip) => saved.skipped.push(skip),
        }
    }
    Ok(saved)
}

pub(super) fn text(value: &str) -> Result<Text, SavedError> {
    Text::new(value).map_err(|_| SavedError::Database)
}
pub(super) fn unsigned(value: i64) -> Result<u64, SavedError> {
    u64::try_from(value).map_err(|_| SavedError::Database)
}

type SearchRow = (
    String,
    String,
    String,
    String,
    String,
    String,
    i64,
    Option<i64>,
);

pub(super) fn search(connection: &Connection, id: &str) -> Result<SavedSearch, SavedError> {
    let row: SearchRow = connection.query_row(
        "SELECT id,name,project,args,schema,request_key,created,checked FROM saved_search WHERE id=?1",
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
            ))
        },
    )?;
    let (id, name, project, args, schema, request_key, created, checked) = row;
    let args: Vec<String> = serde_json::from_str(&args).map_err(|_| SavedError::Database)?;
    Ok(SavedSearch {
        id: text(&id)?,
        name: text(&name)?,
        project: text(&project)?,
        args: args.into_iter().map(String::into_boxed_str).collect(),
        schema: text(&schema)?,
        request_key: text(&request_key)?,
        created: unsigned(created)?,
        checked: checked.map(unsigned).transpose()?,
    })
}

type ItemRow = (String, String, String, Option<String>, i64);

/// A list and its items; an item whose source a newer build wrote is skipped into `skipped`.
fn list(
    connection: &Connection,
    id: &str,
    skipped: &mut Vec<SavedSkipped>,
) -> Result<SavedList, SavedError> {
    let (list_id, name, project, created, updated): (String, String, String, i64, i64) = connection
        .query_row(
            "SELECT id,name,project,created,updated FROM saved_list WHERE id=?1",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )?;
    let mut statement = connection.prepare(
        "SELECT entity,id,source,note,added FROM saved_list_item WHERE list=?1 ORDER BY rowid",
    )?;
    let rows = statement.query_map([id], |r| {
        Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
    })?;
    let mut items = Vec::new();
    for row in rows {
        let (entity, item, source, note, added): ItemRow = row?;
        let added = unsigned(added)?;
        let known = match (!source.is_empty()).then(|| source.parse::<Source>()) {
            None => None,
            Some(Ok(known)) => Some(known),
            Some(Err(_)) => {
                skipped.push(SavedSkipped {
                    record: text(&item)?,
                    name: Some(text(&name)?),
                    field: "source".into(),
                    value: text(&source)?,
                });
                continue;
            }
        };
        items.push(SavedListItem {
            reference: SavedRef {
                entity: text(&entity)?,
                id: text(&item)?,
                source: known,
                as_of: added,
                source_date: None,
            },
            note: note.as_deref().map(text).transpose()?,
            added,
        });
    }
    Ok(SavedList {
        id: text(&list_id)?,
        name: text(&name)?,
        project: text(&project)?,
        created: unsigned(created)?,
        modified: unsigned(updated)?,
        items,
    })
}
