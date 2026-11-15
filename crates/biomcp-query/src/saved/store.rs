//! Open saved.sqlite and read or change its searches and lists.
use biodata::Text;
use biomcp_types::{Saved, SavedList, SavedListItem, SavedRef, SavedSearch, Source};
use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior, params};
use std::path::Path;

use super::MIGRATIONS;

/// The project every command uses until projects can be chosen.
pub const PROJECT: &str = "default";
/// Longest accepted name, in characters.
pub const NAME_CHARS: usize = 80;
/// Longest accepted item note, in characters.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedError {
    /// A newer build wrote the file; it was not changed.
    Newer,
    /// The name already holds a different search, or a list.
    Taken,
    /// No saved search or list has the name.
    Unknown,
    /// The file could not be read or written.
    Database,
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
    connection: Connection,
}

/// The schema version a file states: the number of migration statements it has run.
fn version(connection: &Connection) -> Result<usize, SavedError> {
    let has_meta: bool = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='meta')",
        [],
        |row| row.get(0),
    )?;
    if !has_meta {
        return Ok(0);
    }
    let value: Option<i64> = connection
        .query_row("SELECT value FROM meta WHERE key='schema'", [], |row| {
            row.get(0)
        })
        .optional()?;
    usize::try_from(value.unwrap_or(0)).map_err(|_| SavedError::Database)
}

impl SavedStore {
    /// Open or create the file and bring its schema up to this build.
    pub fn open(path: &Path) -> Result<Self, SavedError> {
        let mut connection = Connection::open(path)?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        if version(&connection)? > MIGRATIONS.len() {
            return Err(SavedError::Newer);
        }
        if version(&connection)? < MIGRATIONS.len() {
            let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
            let applied = version(&tx)?;
            if applied > MIGRATIONS.len() {
                return Err(SavedError::Newer);
            }
            for statement in MIGRATIONS.iter().skip(applied) {
                tx.execute_batch(statement)?;
            }
            tx.execute(
                "INSERT INTO meta(key,value) VALUES('schema',?1) \
                 ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                [i64::try_from(MIGRATIONS.len()).map_err(|_| SavedError::Database)?],
            )?;
            tx.commit()?;
        }
        Ok(Self { connection })
    }

    fn write<T>(
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
    pub fn list_add(&mut self, name: &Text, items: &[NewItem]) -> Result<SavedList, SavedError> {
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
            list(tx, &id)
        })
    }

    /// Remove items with these identifiers from a list.
    pub fn list_remove(&mut self, name: &str, ids: &[Text]) -> Result<SavedList, SavedError> {
        self.write(|tx, now| {
            let id = list_id(tx, name)?.ok_or(SavedError::Unknown)?;
            for item in ids {
                tx.execute(
                    "DELETE FROM saved_list_item WHERE list=?1 AND id=?2",
                    params![id, item.as_str()],
                )?;
            }
            tx.execute(
                "UPDATE saved_list SET updated=?2 WHERE id=?1",
                params![id, seconds(now)?],
            )?;
            list(tx, &id)
        })
    }

    /// Every saved search and list, or the one with this name.
    pub fn show(&self, name: Option<&str>) -> Result<Saved, SavedError> {
        select(&self.connection, name)
    }

    /// Delete the search or list with this name, or everything, and return what was deleted.
    pub fn delete(&mut self, name: Option<&str>) -> Result<Saved, SavedError> {
        self.write(|tx, _| {
            let deleted = select(tx, name)?;
            for search in &deleted.searches {
                tx.execute("DELETE FROM saved_search WHERE id=?1", [search.id.as_str()])?;
            }
            for list in &deleted.lists {
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

fn seconds(now: u64) -> Result<i64, SavedError> {
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
    match stored {
        Some(stored)
            if serde_json::from_str::<Vec<String>>(&stored).ok().as_deref() != Some(args) =>
        {
            Err(SavedError::Taken)
        }
        _ => Ok(()),
    }
}

fn search_id(connection: &Connection, name: &str) -> Result<Option<String>, SavedError> {
    Ok(connection
        .query_row(
            "SELECT id FROM saved_search WHERE project=?1 AND name=?2 COLLATE NOCASE",
            params![PROJECT, name],
            |row| row.get(0),
        )
        .optional()?)
}

fn list_id(connection: &Connection, name: &str) -> Result<Option<String>, SavedError> {
    Ok(connection
        .query_row(
            "SELECT id FROM saved_list WHERE project=?1 AND name=?2 COLLATE NOCASE",
            params![PROJECT, name],
            |row| row.get(0),
        )
        .optional()?)
}

fn select(connection: &Connection, name: Option<&str>) -> Result<Saved, SavedError> {
    let mut saved = Saved::default();
    if let Some(name) = name {
        if let Some(id) = search_id(connection, name)? {
            saved.searches.push(search(connection, &id)?);
        } else if let Some(id) = list_id(connection, name)? {
            saved.lists.push(list(connection, &id)?);
        } else {
            return Err(SavedError::Unknown);
        }
        return Ok(saved);
    }
    let ids = |sql: &str| -> Result<Vec<String>, SavedError> {
        let mut statement = connection.prepare(sql)?;
        let rows = statement.query_map([], |row| row.get(0))?;
        Ok(rows.collect::<Result<_, _>>()?)
    };
    for id in ids("SELECT id FROM saved_search ORDER BY project, name COLLATE NOCASE, id")? {
        saved.searches.push(search(connection, &id)?);
    }
    for id in ids("SELECT id FROM saved_list ORDER BY project, name COLLATE NOCASE, id")? {
        saved.lists.push(list(connection, &id)?);
    }
    Ok(saved)
}

fn text(value: &str) -> Result<Text, SavedError> {
    Text::new(value).map_err(|_| SavedError::Database)
}
fn unsigned(value: i64) -> Result<u64, SavedError> {
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

fn search(connection: &Connection, id: &str) -> Result<SavedSearch, SavedError> {
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

fn list(connection: &Connection, id: &str) -> Result<SavedList, SavedError> {
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
        items.push(SavedListItem {
            reference: SavedRef {
                entity: text(&entity)?,
                id: text(&item)?,
                source: (!source.is_empty())
                    .then(|| source.parse::<Source>())
                    .transpose()
                    .map_err(|_| SavedError::Database)?,
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
        updated: unsigned(updated)?,
        items,
    })
}
