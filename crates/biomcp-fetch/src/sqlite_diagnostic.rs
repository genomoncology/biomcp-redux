//! Opt-in proof diagnostics retain only source operation coordinates and numeric codes.
use std::{fs::File, io::Write, path::Path};

/// Internal proof-owned destination; it must already exist and is never created here.
pub const SQLITE_DIAGNOSTICS: &str = "BIOMCP_SQLITE_DIAGNOSTICS";
const MAX_BYTES: u64 = 4096;

/// Record a SQLite error without changing its classification or the command result.
#[track_caller]
#[expect(
    clippy::disallowed_methods,
    reason = "The proof diagnostic boundary alone reads its opt-in destination"
)]
pub fn record_sqlite_error(component: &'static str, error: &rusqlite::Error) {
    let Some(path) = std::env::var_os(SQLITE_DIAGNOSTICS) else {
        return;
    };
    let caller = std::panic::Location::caller();
    // Only a compiled source basename is retained, never a runtime path or SQL.
    let source = caller
        .file()
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("unknown");
    let operation = format!("{component}::{source}:{}", caller.line());
    append(Path::new(&path), &operation, error);
}
fn append(path: &Path, operation: &str, error: &rusqlite::Error) {
    let Ok(mut file) = File::options().append(true).open(path) else {
        return;
    };
    let Ok(metadata) = file.metadata() else {
        return;
    };
    let code = error.sqlite_error().map(|error| error.extended_code);
    let value = serde_json::json!({"operation": operation,
        "primary_code": code.map(|code| code & 0xff), "extended_code": code});
    let Ok(mut bytes) = serde_json::to_vec(&value) else {
        return;
    };
    bytes.push(b'\n');
    if metadata.len().saturating_add(bytes.len() as u64) <= MAX_BYTES {
        // Match the existing opt-in observation sink: reporting cannot replace a result.
        let _ = file.write_all(&bytes);
    }
}

#[cfg(test)]
mod tests {
    // This inner test owns the diagnostic privacy and nonfatal I/O boundary.
    #[test]
    fn numeric_records_exclude_error_text_and_ignore_unavailable_files() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("sqlite.jsonl");
        let error = rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT_TRIGGER),
            Some("private SQL, binding and database path".into()),
        );
        super::append(&path, "fixture.insert", &error);
        assert!(!path.exists(), "diagnostics must not create a destination");
        std::fs::write(&path, []).unwrap();
        super::append(&path, "fixture.insert", &error);
        let record: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(
            record,
            serde_json::json!({"operation":"fixture.insert",
            "primary_code":19,"extended_code":1811})
        );
        super::append(root.path(), "fixture.insert", &error);
    }
}
