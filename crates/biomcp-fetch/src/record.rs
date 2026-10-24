//! Bounded, credential-free captures and atomically consumed replay sequences.
#![expect(
    clippy::disallowed_types,
    reason = "Serialize capture publication and reserve replay cursors across scoped callers"
)]
use crate::{CacheError, Mode, RecordError, cache::Cache};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_METADATA_BYTES: usize = 8192;
const MAX_EXCHANGES: usize = 128;
const LOCK: &str = ".record-owner";

/// An opaque temporary cache shared by one replay case and its offline checks.
#[derive(Clone)]
pub struct ReplayCache(Arc<CaseCache>);
#[derive(Debug)]
struct CaseCache {
    cache: Arc<Cache>,
    _directory: tempfile::TempDir,
}
impl std::fmt::Debug for ReplayCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ReplayCache")
    }
}
impl ReplayCache {
    /// Allocate an isolated cache whose files live until its last owner drops.
    pub fn temporary() -> Result<Self, CacheError> {
        let directory = tempfile::tempdir().map_err(|_| CacheError::Database)?;
        let cache = Arc::new(Cache::open(&directory.path().join("replay.sqlite"))?);
        Ok(Self(Arc::new(CaseCache {
            cache,
            _directory: directory,
        })))
    }
    pub(crate) fn cache(&self) -> Arc<Cache> {
        self.0.cache.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Recording {
    method: String,
    source: biodata::Source,
    url: String,
    #[serde(flatten)]
    post: Option<crate::prepared::PostMetadata>,
    exchanges: Vec<Exchange>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Exchange {
    pub(crate) status: u16,
    pub(crate) content_type: String,
    pub(crate) delay_ms: u64,
    pub(crate) retry_after_ms: Option<u64>,
    pub(crate) body: Body,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub(crate) enum Body {
    Json(Box<serde_json::value::RawValue>),
    Text(String),
}
impl Body {
    pub(crate) fn capture(bytes: Vec<u8>) -> Result<Self, RecordError> {
        if bytes.len() as u64 > MAX_FILE_BYTES {
            return Err(RecordError::Limit);
        }
        let text = String::from_utf8(bytes).map_err(|_| RecordError::NonUtf8)?;
        Ok(
            match serde_json::value::RawValue::from_string(text.clone()) {
                Ok(value) => Self::Json(value),
                Err(_) => Self::Text(text),
            },
        )
    }
    pub(crate) fn bytes(&self) -> &[u8] {
        match self {
            Self::Json(value) => value.get().as_bytes(),
            Self::Text(text) => text.as_bytes(),
        }
    }
}
impl Exchange {
    fn valid(&self) -> bool {
        (100..=599).contains(&self.status)
            && self.content_type.len() <= MAX_METADATA_BYTES
            && self
                .content_type
                .bytes()
                .all(|b| b.is_ascii() && !b.is_ascii_control())
    }
}

#[derive(Debug)]
pub(crate) enum Recordings {
    None,
    Capture(Capture),
    Replay {
        dir: PathBuf,
        cursors: Mutex<BTreeMap<(biodata::Source, [u8; 32]), Cursor>>,
    },
}
#[derive(Debug)]
pub(crate) struct Capture {
    dir: PathBuf,
    sequences: Mutex<BTreeMap<(biodata::Source, [u8; 32]), Recording>>,
}
#[derive(Debug)]
pub(crate) struct Cursor {
    recording: Recording,
    next: usize,
}
impl Drop for Capture {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.dir.join(LOCK));
    }
}
impl Recordings {
    pub(crate) fn open(mode: &Mode) -> Result<Self, RecordError> {
        match mode {
            Mode::Record { dir } => {
                if let Err(error) = fs::create_dir(dir)
                    && error.kind() != std::io::ErrorKind::AlreadyExists
                {
                    return Err(RecordError::Io);
                }
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(dir.join(LOCK))
                    .map_err(|error| {
                        if error.kind() == std::io::ErrorKind::AlreadyExists {
                            RecordError::Conflict
                        } else {
                            RecordError::Io
                        }
                    })?;
                let capture = Capture {
                    dir: dir.clone(),
                    sequences: Mutex::new(BTreeMap::new()),
                };
                for entry in fs::read_dir(dir).map_err(|_| RecordError::Io)? {
                    if entry.map_err(|_| RecordError::Io)?.file_name() != LOCK {
                        return Err(RecordError::Conflict);
                    }
                }
                Ok(Self::Capture(capture))
            }
            Mode::Replay { dir } => Ok(Self::Replay {
                dir: dir.clone(),
                cursors: Mutex::new(BTreeMap::new()),
            }),
            _ => Ok(Self::None),
        }
    }
    pub(crate) fn replay(
        &self,
        source: biodata::Source,
        key: &[u8; 32],
        url: &str,
        post: Option<&crate::prepared::PostMetadata>,
        max_body: u64,
    ) -> Result<Option<Exchange>, RecordError> {
        let Self::Replay { dir, cursors } = self else {
            return Ok(None);
        };
        let mut cursors = cursors.lock().map_err(|_| RecordError::Io)?;
        let cursor = match cursors.entry((source, *key)) {
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::btree_map::Entry::Vacant(entry) => {
                let recording = read(&path(dir, source, key), source, url, post)?;
                entry.insert(Cursor { recording, next: 0 })
            }
        };
        let value = cursor
            .recording
            .exchanges
            .get(cursor.next)
            .ok_or(RecordError::Exhausted)?;
        cursor.next += 1;
        if value.body.bytes().len() as u64 > max_body {
            return Err(RecordError::Limit);
        }
        Ok(Some(value.clone()))
    }
    pub(crate) fn append(
        &self,
        source: biodata::Source,
        key: &[u8; 32],
        url: &str,
        post: Option<&crate::prepared::PostMetadata>,
        exchange: Exchange,
    ) -> Result<(), RecordError> {
        let Self::Capture(capture) = self else {
            return Ok(());
        };
        if url.len() > MAX_METADATA_BYTES || !exchange.valid() {
            return Err(RecordError::Limit);
        }
        let mut sequences = capture.sequences.lock().map_err(|_| RecordError::Io)?;
        let recording = sequences
            .entry((source, *key))
            .or_insert_with(|| Recording {
                method: if post.is_some() { "POST" } else { "GET" }.into(),
                post: post.cloned(),
                source,
                url: url.into(),
                exchanges: Vec::new(),
            });
        if recording.exchanges.len() >= MAX_EXCHANGES {
            return Err(RecordError::Limit);
        }
        recording.exchanges.push(exchange);
        let result = publish(&path(&capture.dir, source, key), recording);
        if result.is_err() {
            recording.exchanges.pop();
        }
        result
    }
}
fn path(dir: &Path, source: biodata::Source, key: &[u8; 32]) -> PathBuf {
    let key: String = key.iter().map(|byte| format!("{byte:02x}")).collect();
    dir.join(source.to_string()).join(format!("{key}.json"))
}
fn read(
    path: &Path,
    source: biodata::Source,
    url: &str,
    post: Option<&crate::prepared::PostMetadata>,
) -> Result<Recording, RecordError> {
    let file = fs::File::open(path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            RecordError::Missing
        } else {
            RecordError::Io
        }
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| RecordError::Io)?;
    if bytes.len() as u64 > MAX_FILE_BYTES {
        return Err(RecordError::Limit);
    }
    let value: Recording = serde_json::from_slice(&bytes).map_err(|_| RecordError::Malformed)?;
    if value.method != if post.is_some() { "POST" } else { "GET" }
        || value.post.as_ref() != post
        || value.source != source
        || value.url != url
        || value.url.len() > MAX_METADATA_BYTES
        || value.exchanges.is_empty()
        || value.exchanges.len() > MAX_EXCHANGES
        || value.exchanges.iter().any(|exchange| !exchange.valid())
    {
        return Err(RecordError::Malformed);
    }
    Ok(value)
}
fn publish(path: &Path, value: &Recording) -> Result<(), RecordError> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|_| RecordError::Malformed)?;
    if bytes.len() as u64 > MAX_FILE_BYTES {
        return Err(RecordError::Limit);
    }
    let parent = path.parent().ok_or(RecordError::Io)?;
    fs::create_dir_all(parent).map_err(|_| RecordError::Io)?;
    let mut file = tempfile::NamedTempFile::new_in(parent).map_err(|_| RecordError::Io)?;
    file.write_all(&bytes).map_err(|_| RecordError::Io)?;
    file.as_file().sync_all().map_err(|_| RecordError::Io)?;
    file.persist(path).map_err(|_| RecordError::Io)?;
    Ok(())
}
pub(crate) fn millis(duration: Duration) -> Result<u64, RecordError> {
    u64::try_from(duration.as_millis()).map_err(|_| RecordError::Limit)
}
