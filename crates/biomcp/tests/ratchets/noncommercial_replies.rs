//! Replies from noncommercial sources are synthetic, and a README beside them says so (0695, 0920).
//! The marker is a line `Synthetic: <source>` in README.md in the source folder or its case folder.
//! Outside the replies, real ORCID, DDInter and Semantic Scholar author IDs are invented too (0973).
//! Orphadata's replies are synthetic too, and each Orphadata file body, inline or in a sidecar,
//! says so and names only "Synthetic" disorders (0990, 0996).
use super::Check;
use biomcp_types::{Source, Use};
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

/// Ian, 2026-11-16 (`sdlc/decisions/2026-11-16-review-follow-ups.md`, item 1): the public
/// Orphadata captures are replaced by synthetic ones, though its terms are not noncommercial.
const SYNTHETIC_ONLY: &[Source] = &[Source::Orphadata];
/// Real Orphadata files name Orphanet's database server; authored ones state this instead.
const ORPHADATA_AUTHORED: &str = "dbserver=\"synthetic\"";

pub(super) fn check(root: &Path) -> Check {
    let replies = root.join("tests/replies");
    if !replies.exists() {
        return Ok(());
    }
    // The registry's terms decide which sources and carried fields are noncommercial.
    let sources: BTreeSet<String> = Source::ALL
        .iter()
        .filter(|&&source| {
            SYNTHETIC_ONLY.contains(&source)
                || biomcp_adapters::registry::terms(source)
                    .is_some_and(|terms| terms.usage == Use::Noncommercial)
        })
        .map(ToString::to_string)
        .collect();
    let components: Vec<(String, &str)> = biomcp_adapters::registry::components()
        .filter(|component| component.terms.usage == Use::Noncommercial)
        .map(|component| (component.source.to_string(), component.name))
        .collect();
    let mut unmarked = Vec::new();
    for path in json_files(&replies)? {
        // Other JSON under tests/replies (license lists, projections) is not a recording.
        let Ok(record) = serde_json::from_slice::<Value>(&fs::read(&path)?) else {
            continue;
        };
        let Some(source) = record.get("source").and_then(Value::as_str) else {
            continue;
        };
        let carried = components.iter().any(|(owner, name)| {
            owner == source
                && record
                    .get("exchanges")
                    .is_some_and(|value| has_key(value, name))
        });
        let relative = path.strip_prefix(root)?.to_string_lossy().into_owned();
        if (sources.contains(source) || carried) && !marked(&path, source)? {
            unmarked.push(format!("{relative} ({source})"));
        }
        if source == <&str>::from(Source::Orphadata)
            && let Some(problem) = real_orphadata(&path, &record)
        {
            unmarked.push(format!("{relative} ({source}: {problem})"));
        }
    }
    if unmarked.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "replies from noncommercial sources and Orphadata need `Synthetic: <source>` in \
             README.md in their source or case folder, and Orphadata bodies need \
             {ORPHADATA_AUTHORED} and \"Synthetic\" disorder names:\n{}",
            unmarked.join("\n")
        )
        .into())
    }
}

/// Only the source folder and its case folder may mark a recording, never a broader ancestor.
fn marked(path: &Path, source: &str) -> Result<bool, std::io::Error> {
    let marker = format!("Synthetic: {source}");
    for folder in path.ancestors().skip(1).take(2) {
        let readme = folder.join("README.md");
        if readme.exists() {
            let text = fs::read_to_string(readme)?;
            if text.lines().any(|line| line.trim() == marker) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// A real-looking Orphadata file body, read inline or from its sidecar `.bin` file (0906).
/// Real files name Orphanet's database server; an authored one says `dbserver="synthetic"` and
/// names every disorder "Synthetic ...", so a real file with only its server changed still fails.
fn real_orphadata(path: &Path, record: &Value) -> Option<String> {
    let exchanges = record
        .get("exchanges")
        .and_then(Value::as_array)
        .into_iter()
        .flatten();
    for body in exchanges.filter_map(|exchange| exchange.get("body")) {
        let value = body.get("value").and_then(Value::as_str).unwrap_or_default();
        let text = match body.get("kind").and_then(Value::as_str) {
            Some("sidecar") => match fs::read(path.with_file_name(value)) {
                Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                Err(error) => return Some(format!("sidecar {value}: {error}")),
            },
            _ => value.to_owned(),
        };
        if !text.contains("<JDBOR") {
            continue;
        }
        if !text.contains(ORPHADATA_AUTHORED) {
            return Some(format!("body lacks {ORPHADATA_AUTHORED}"));
        }
        if let Some(name) = disorder_names(&text).find(|name| !name.starts_with("Synthetic")) {
            return Some(format!(
                "disorder name {name:?} does not start with \"Synthetic\""
            ));
        }
    }
    None
}

/// Each disorder's own name, the first `Name` after its `OrphaCode`.
fn disorder_names(text: &str) -> impl Iterator<Item = &str> {
    text.split("<OrphaCode>").skip(1).filter_map(|disorder| {
        let (_, rest) = disorder.split_once("<Name lang=\"en\">")?;
        rest.split_once("</Name>").map(|(name, _)| name)
    })
}

fn has_key(value: &Value, name: &str) -> bool {
    match value {
        Value::Object(map) => map
            .iter()
            .any(|(key, item)| key == name || has_key(item, name)),
        Value::Array(items) => items.iter().any(|item| has_key(item, name)),
        _ => false,
    }
}

/// ORCID's own published test record, a fictitious researcher.
const ORCID_EXAMPLE: &str = "0000-0002-1825-0097";

/// The ORCID health check reads one real public record; split so this file names no real iD.
const PROBE: (&str, &str) = (
    "crates/biomcp-adapters/src/orcid/decl.rs",
    concat!("0000-0002-", "4299-3018"),
);

/// A real ID may stay only in the ORCID health probe, or in a live check under tests/native or
/// a live capture under scripts whose `live_check` or `live_inputs` field names a "real record".
pub(super) fn ids(root: &Path) -> Check {
    let mut files = Vec::new();
    for folder in ["crates", "tests", "scripts", "sdlc/planning"] {
        let path = root.join(folder);
        if path.exists() {
            files.extend(all_files(&path)?.into_iter().filter(|file| {
                (folder != "crates" || file.extension().is_some_and(|extension| extension == "rs"))
                    && !file.starts_with(root.join("tests/replies"))
            }));
        }
    }
    let mut found = Vec::new();
    for path in files {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let relative = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        let ids: Vec<String> = real_ids(&text)
            .into_iter()
            .filter(|id| (relative.as_str(), id.as_str()) != PROBE)
            .collect();
        if !ids.is_empty() && !justified(&relative, &text) {
            found.push(format!("{relative} ({})", ids.join(", ")));
        }
    }
    if found.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "real noncommercial IDs outside tests/replies need invented values, or a live call's reason naming a real record:\n{}",
            found.join("\n")
        )
        .into())
    }
}

/// Only live checks and live captures may give a reason; code comments never excuse a real ID.
fn justified(relative: &str, text: &str) -> bool {
    (relative.starts_with("tests/native/") || relative.starts_with("scripts/"))
        && serde_json::from_str::<Value>(text).is_ok_and(|value| {
            ["live_check", "live_inputs"].iter().any(|key| {
                value
                    .get(key)
                    .and_then(Value::as_str)
                    .is_some_and(|reason| reason.contains("real record"))
            })
        })
}

/// Checksum-valid ORCID iDs outside the invented 0000-0000 block, DDInter IDs below the
/// invented 9000 range, and Semantic Scholar author IDs of real length.
fn real_ids(text: &str) -> Vec<String> {
    let mut ids = BTreeSet::new();
    let bytes = text.as_bytes();
    for start in 0..bytes.len() {
        if let Some(id) = orcid_at(bytes, start) {
            ids.insert(id);
        }
    }
    for (start, _) in text.match_indices("DDInter") {
        let digits = digits_after(text, start + "DDInter".len());
        if digits.parse::<u32>().is_ok_and(|number| number < 9000) {
            ids.insert(format!("DDInter{digits}"));
        }
    }
    for prefix in [
        "semantic-scholar:",
        "semanticscholar.org/author/",
        "/graph/v1/author/",
    ] {
        for (start, _) in text.match_indices(prefix) {
            let digits = digits_after(text, start + prefix.len());
            if (4..=17).contains(&digits.len()) {
                ids.insert(format!("{prefix}{digits}"));
            }
        }
    }
    ids.into_iter().collect()
}

fn digits_after(text: &str, start: usize) -> &str {
    let rest = text.get(start..).unwrap_or_default();
    let end = rest
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(rest.len());
    rest.get(..end).unwrap_or_default()
}

fn orcid_at(bytes: &[u8], start: usize) -> Option<String> {
    let token = bytes.get(start..start + 19)?;
    let before = start.checked_sub(1).and_then(|index| bytes.get(index));
    if before.is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
        || bytes.get(start + 19).is_some_and(u8::is_ascii_alphanumeric)
    {
        return None;
    }
    let mut digits = Vec::with_capacity(16);
    for (index, &byte) in token.iter().enumerate() {
        match (index, byte) {
            (4 | 9 | 14, b'-') => {}
            (18, b'X') => digits.push(10),
            (index, byte) if ![4, 9, 14].contains(&index) && byte.is_ascii_digit() => {
                digits.push(u32::from(byte - b'0'));
            }
            _ => return None,
        }
    }
    // ISO 7064 MOD 11-2 over the first fifteen digits gives the last one.
    let (check, body) = digits.split_last()?;
    let total = body.iter().fold(0, |total, digit| (total + digit) * 2);
    let token = std::str::from_utf8(token).ok()?;
    if (12 - total % 11) % 11 != *check || token.starts_with("0000-0000") || token == ORCID_EXAMPLE
    {
        return None;
    }
    Some(token.to_owned())
}

fn all_files(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(all_files(&path)?);
        } else {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn json_files(path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(json_files(&path)?);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

#[test]
fn unmarked_noncommercial_replies_fail() -> Check {
    let root = tempfile::tempdir()?;
    let case = root.path().join("tests/replies/area/case");
    for (source, body) in [
        ("semantic-scholar", serde_json::json!({"data": []})),
        ("myvariant", serde_json::json!({"hits": [{"cadd": {}}]})),
    ] {
        fs::create_dir_all(case.join(source))?;
        let record = serde_json::json!({
            "source": source,
            "exchanges": [{"body": {"kind": "json", "value": body}}]
        });
        fs::write(case.join(source).join("a.json"), record.to_string())?;
    }
    let open = serde_json::json!({"source": "pubmed", "exchanges": []});
    fs::write(case.join("open.json"), open.to_string())?;
    let error = check(root.path()).err().ok_or("unmarked replies passed")?;
    for source in ["(semantic-scholar)", "(myvariant)"] {
        if !error.to_string().contains(source) {
            return Err(format!("{source} was not reported: {error}").into());
        }
    }
    if error.to_string().contains("(pubmed)") {
        return Err("an open source was reported".into());
    }
    fs::write(
        root.path().join("tests/replies/area/README.md"),
        "Synthetic: semantic-scholar\nSynthetic: myvariant\n",
    )?;
    if check(root.path()).is_ok() {
        return Err("an ancestor README above the case folder covered it".into());
    }
    fs::write(
        case.join("README.md"),
        "These rows are synthetic in another sense.\nSynthetic: pubmed\n",
    )?;
    if check(root.path()).is_ok() {
        return Err("a README without the source's marker passed".into());
    }
    fs::write(case.join("README.md"), "Synthetic: semantic-scholar\n")?;
    let error = check(root.path())
        .err()
        .ok_or("an unmarked source passed")?;
    if !error.to_string().contains("(myvariant)")
        || error.to_string().contains("(semantic-scholar)")
    {
        return Err(format!("the case marker covered the wrong sources: {error}").into());
    }
    fs::write(
        case.join("myvariant").join("README.md"),
        "Synthetic: myvariant\n",
    )?;
    check(root.path())
}

#[test]
fn real_orphadata_replies_fail() -> Check {
    let root = tempfile::tempdir()?;
    let case = root.path().join("tests/replies/disease/case");
    fs::create_dir_all(case.join("orphadata"))?;
    let record = |server: &str| {
        serde_json::json!({
            "source": "orphadata",
            "exchanges": [{"body": {"kind": "text", "value": format!("<JDBOR dbserver=\"{server}\">")}}]
        })
        .to_string()
    };
    let file = case.join("orphadata").join("a.json");
    fs::write(&file, record("synthetic"))?;
    let error = check(root.path())
        .err()
        .ok_or("an unmarked Orphadata reply passed")?;
    if !error.to_string().contains("(orphadata)") {
        return Err(format!("the unmarked Orphadata reply was not reported: {error}").into());
    }
    fs::write(case.join("README.md"), "Synthetic: orphadata\n")?;
    check(root.path())?;
    fs::write(&file, record("jdbc:sybase:Tds:example"))?;
    let error = check(root.path())
        .err()
        .ok_or("a real Orphadata body passed under the marker")?;
    if !error.to_string().contains("body lacks") {
        return Err(format!("the real Orphadata body was not reported: {error}").into());
    }
    // A hand-cut real file with only its server changed keeps a real disorder name.
    let named = |name: &str| {
        format!(
            "<JDBOR dbserver=\"synthetic\"><Disorder><OrphaCode>1</OrphaCode>\
             <Name lang=\"en\">{name}</Name></Disorder></JDBOR>"
        )
    };
    let text = |body: &str| {
        serde_json::json!({
            "source": "orphadata",
            "exchanges": [{"body": {"kind": "text", "value": body}}]
        })
        .to_string()
    };
    fs::write(&file, text(&named("Synthetic disorder")))?;
    check(root.path())?;
    fs::write(&file, text(&named("Kabuki syndrome")))?;
    let error = check(root.path())
        .err()
        .ok_or("a hand-cut real Orphadata file passed")?;
    if !error.to_string().contains("\"Kabuki syndrome\"") {
        return Err(format!("the real disorder name was not reported: {error}").into());
    }
    // A recorded multi-file sync stores each file as a sidecar `.bin` beside its record (0906).
    let sidecar = serde_json::json!({
        "source": "orphadata",
        "exchanges": [{"body": {"kind": "sidecar", "value": "a-0.bin"}}]
    });
    fs::write(&file, sidecar.to_string())?;
    let bin = case.join("orphadata").join("a-0.bin");
    fs::write(&bin, named("Synthetic disorder"))?;
    check(root.path())?;
    fs::write(&bin, "<JDBOR dbserver=\"jdbc:sybase:Tds:example\">")?;
    let error = check(root.path())
        .err()
        .ok_or("a real Orphadata sidecar passed")?;
    if !error.to_string().contains("body lacks") {
        return Err(format!("the real Orphadata sidecar was not reported: {error}").into());
    }
    Ok(())
}

#[test]
fn real_ids_outside_replies_fail() -> Check {
    let root = tempfile::tempdir()?;
    let tests = root.path().join("crates/area/tests");
    fs::create_dir_all(&tests)?;
    // Built at run time, so this file names no real-looking ID itself.
    let orcid = format!("0000-0001-0000-{}", "0009");
    let ddinter = format!("DDInter{}", 12);
    let scholar = format!("semantic-scholar:{}", 2_000_000);
    fs::write(
        tests.join("a.rs"),
        format!("\"{orcid}\" \"{ddinter}\" \"{scholar}\"\n"),
    )?;
    let error = ids(root.path()).err().ok_or("real IDs passed")?.to_string();
    for id in [&orcid, &ddinter, &scholar] {
        if !error.contains(id.as_str()) {
            return Err(format!("{id} was not reported: {error}").into());
        }
    }
    let invented = format!(
        "\"0000-0000-0000-0925\" \"{ORCID_EXAMPLE}\" \"{orcid}0\" \"DDInter{}\" \"semantic-scholar:{}\"\n",
        9001, 123
    );
    fs::write(tests.join("a.rs"), invented)?;
    ids(root.path())?;
    let planning = root.path().join("sdlc/planning");
    fs::create_dir_all(&planning)?;
    fs::write(
        planning.join("a.md"),
        format!("# A real record\n\n{orcid}\n"),
    )?;
    if ids(root.path()).is_ok() {
        return Err("a Markdown heading excused a real ID".into());
    }
    fs::remove_file(planning.join("a.md"))?;
    let native = root.path().join("tests/native");
    fs::create_dir_all(&native)?;
    let case =
        |reason: &str| serde_json::json!({"live_check": reason, "run": [&scholar]}).to_string();
    fs::write(native.join("a.yaml"), case("live call"))?;
    if ids(root.path()).is_ok() {
        return Err("a live check without its reason passed".into());
    }
    fs::write(
        native.join("a.yaml"),
        case("a live call needs a real record"),
    )?;
    ids(root.path())?;
    let cases = root.path().join("tests/cases");
    fs::create_dir_all(&cases)?;
    fs::write(
        cases.join("a.yaml"),
        case("a live call needs a real record"),
    )?;
    if ids(root.path()).is_ok() {
        return Err("a live check outside tests/native excused a real ID".into());
    }
    fs::remove_file(cases.join("a.yaml"))?;
    fs::write(
        tests.join("a.rs"),
        format!("// a live call needs a real record\n\"{orcid}\"\n"),
    )?;
    if ids(root.path()).is_ok() {
        return Err("a Rust comment excused a real ID".into());
    }
    fs::remove_file(tests.join("a.rs"))?;
    let probe = root.path().join(PROBE.0);
    fs::create_dir_all(probe.parent().ok_or("probe folder")?)?;
    fs::write(&probe, format!("\"{}\"\n", PROBE.1))?;
    ids(root.path())?;
    fs::write(&probe, format!("\"{}\" \"{orcid}\"\n", PROBE.1))?;
    if ids(root.path()).is_ok() {
        return Err("the health probe file excused a second real ID".into());
    }
    Ok(())
}
