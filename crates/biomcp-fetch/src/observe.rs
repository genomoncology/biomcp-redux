//! Opt-in bounded wire summaries without response values or request URLs.
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::File, io::Write, path::Path, sync::Mutex};
/// Maximum attempts retained per command.
pub const MAX_OBSERVATIONS: usize = 4096;
const FIELDS: usize = 256;
/// Raw structure before adapter mapping.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Shape {
    /// Sampled JSON paths or XML names.
    pub fields: BTreeMap<String, bool>,
    /// Array or declared count is positive.
    pub positive: BTreeMap<String, bool>,
    /// Bounded tabular header row.
    pub header: Option<String>,
}
impl Shape {
    /// Inspect structure without retaining text or scalar values.
    pub fn inspect(body: &[u8], media: &str) -> Self {
        let mut shape = Self::default();
        if media.contains("json") {
            if let Ok(value) = serde_json::from_slice(body) { shape.json(&value, "", 0); }
        } else if media.contains("xml") {
            let mut reader = quick_xml::Reader::from_reader(body);
            while shape.fields.len() < FIELDS {
                match reader.read_event() {
                    Ok(quick_xml::events::Event::Start(e) | quick_xml::events::Event::Empty(e)) => {
                        let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                        if name.len() <= 128 { shape.fields.insert(name, true); }
                    }
                    Ok(quick_xml::events::Event::Eof) | Err(_) => break,
                    _ => {}
                }
            }
        } else if media.contains("csv") || media.contains("tab-separated") {
            shape.header = body.split(|b| *b == b'\n').next().filter(|line| line.len() <= 4096)
                .and_then(|line| std::str::from_utf8(line).ok()).map(str::to_owned);
        }
        shape
    }
    fn json(&mut self, value: &serde_json::Value, path: &str, depth: usize) {
        if depth >= 12 || self.fields.len() >= FIELDS { return; }
        match value {
            serde_json::Value::Object(map) => for (key, value) in map {
                if key.len() > 128 || self.fields.len() >= FIELDS { break; }
                let path = format!("{path}/{}", key.replace('~', "~0").replace('/', "~1"));
                self.fields.insert(path.clone(), true);
                if key.to_ascii_lowercase().contains("count") {
                    if let Some(n) = value.as_u64().or_else(|| value.as_str()?.parse().ok()) {
                        self.positive.insert(path.clone(), n > 0);
                    }
                }
                self.json(value, &path, depth + 1);
            },
            serde_json::Value::Array(items) => {
                self.positive.insert(path.to_owned(), !items.is_empty());
                for item in items.iter().take(3) { self.json(item, &format!("{path}/*"), depth + 1); }
            }
            _ => {}
        }
    }
}
/// One attempt, including failed and retried attempts.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Observation {
    /// Transport's prepared credential-free identity.
    pub identity: String,
    /// Provider source.
    pub source: biodata::Source,
    /// HTTP status, absent on transport failure.
    pub status: Option<u16>,
    /// Received bytes.
    pub bytes: u64,
    /// Wire duration in milliseconds.
    pub elapsed_ms: u64,
    /// Bounded raw response shape.
    pub shape: Shape,
}
/// Explicitly created command-owned sink.
#[expect(clippy::disallowed_types, reason = "Serialize bounded summaries across source workers")]
#[derive(Debug)]
pub struct Observations(Mutex<(File, usize)>);
impl Observations {
    /// Create exclusively; never overwrite an earlier report.
    pub fn create(path: &Path) -> std::io::Result<Self> {
        Ok(Self(Mutex::new((File::options().write(true).create_new(true).open(path)?, 0))))
    }
    pub(crate) fn push(&self, item: &Observation) {
        let mut state = self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.1 >= MAX_OBSERVATIONS { return; }
        state.1 += 1;
        if let Ok(mut line) = serde_json::to_vec(item) {
            line.push(b'\n');
            let _ = state.0.write_all(&line);
        }
    }
}
impl crate::Leg<'_> {
    pub(crate) fn observe(&self, identity: &[u8; 32], status: Option<u16>, body: &[u8], media: &str, elapsed: std::time::Duration) {
        if let Some(sink) = &self.fetch.config.observations {
            sink.push(&Observation {
                identity: identity.iter().map(|byte| format!("{byte:02x}")).collect(),
                source: self.spec.source, status, bytes: body.len() as u64,
                elapsed_ms: u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX),
                shape: Shape::inspect(body, media),
            });
        }
    }
}
