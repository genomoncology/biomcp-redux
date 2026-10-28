//! Conservative product-name reachability, including local representation closures.
use super::{Check, rust_files, source::tokens};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

struct Item {
    name: String,
    file: usize,
    references: BTreeSet<String>,
}

fn item_end(words: &[String], start: usize) -> usize {
    let mut depth = 0;
    for (i, word) in words.iter().enumerate().skip(start) {
        match word.as_str() {
            "(" | "[" => depth += 1,
            ")" | "]" => depth -= 1,
            "{" if depth == 0 => return end(words, i),
            ";" if depth == 0 => return i + 1,
            _ => {}
        }
    }
    words.len()
}

fn end(words: &[String], start: usize) -> usize {
    let mut depth = 0;
    for (i, word) in words.iter().enumerate().skip(start) {
        match word.as_str() {
            "{" | "(" | "[" => depth += 1,
            "}" | ")" | "]" => {
                depth -= 1;
                if depth == 0 {
                    return i + 1;
                }
            }
            ";" if depth == 0 => return i + 1,
            _ => {}
        }
    }
    words.len()
}

fn declarations(words: &[String], file: usize) -> Vec<Item> {
    let mut items = Vec::new();
    let mut opaque_until = 0;
    for (i, word) in words.iter().enumerate() {
        if i < opaque_until {
            continue;
        }
        if word == "!" {
            let start = if words
                .get(i + 1)
                .is_some_and(|s| matches!(s.as_str(), "{" | "(" | "["))
            {
                i + 1
            } else {
                i + 2
            };
            if words
                .get(start)
                .is_some_and(|s| matches!(s.as_str(), "{" | "(" | "["))
            {
                opaque_until = end(words, start);
            }
        }
        if word != "pub" {
            continue;
        }
        let mut at = i + 1;
        while words
            .get(at)
            .is_some_and(|s| matches!(s.as_str(), "async" | "unsafe"))
        {
            at += 1;
        }
        if words.get(at).is_some_and(|s| s == "const")
            && words.get(at + 1).is_some_and(|s| s == "fn")
        {
            at += 1;
        }
        if !words.get(at).is_some_and(|s| {
            matches!(
                s.as_str(),
                "struct" | "enum" | "trait" | "type" | "fn" | "const" | "static"
            )
        }) {
            continue;
        }
        let Some(name) = words.get(at + 1).filter(|s| s.as_str() != "$") else {
            continue;
        };
        let references = words
            .get(at + 2..item_end(words, at + 2))
            .unwrap_or_default()
            .iter()
            .cloned()
            .collect();
        items.push(Item {
            name: name.clone(),
            file,
            references,
        });
    }
    items
}

fn unreferenced(files: &[(String, Vec<String>)]) -> Vec<String> {
    let items: Vec<_> = files
        .iter()
        .enumerate()
        .flat_map(|(i, (_, words))| declarations(words, i))
        .collect();
    let mut references: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    let mut aliases = Vec::new();
    for (file, (_, words)) in files.iter().enumerate() {
        let mut skip = 0;
        for (i, word) in words.iter().enumerate() {
            if words.get(i..i + 2).is_some_and(|s| s == ["pub", "use"]) {
                skip = words
                    .iter()
                    .enumerate()
                    .skip(i)
                    .find(|(_, s)| *s == ";")
                    .map_or(words.len(), |(i, _)| i + 1);
                for alias in words.get(i..skip).unwrap_or_default().windows(3) {
                    if alias.get(1).is_some_and(|s| s == "as") {
                        aliases.push((alias[0].clone(), alias[2].clone()));
                    }
                }
            }
            if i >= skip {
                references.entry(word.clone()).or_default().insert(file);
            }
        }
    }
    let mut live: BTreeSet<_> = items
        .iter()
        .filter(|item| {
            references
                .get(&item.name)
                .is_some_and(|sites| sites.iter().any(|file| *file != item.file))
        })
        .map(|item| item.name.clone())
        .collect();
    loop {
        let before = live.len();
        for (original, alias) in &aliases {
            if references.contains_key(alias) {
                live.insert(original.clone());
            }
        }
        for item in &items {
            if live.contains(&item.name) {
                live.extend(item.references.iter().cloned());
            }
        }
        implementation_closure(files, &mut live);
        if live.len() == before {
            break;
        }
    }
    items
        .iter()
        .filter(|item| !live.contains(&item.name))
        .map(|item| {
            format!(
                "{}: {} has no product reference outside its file or live representation closure",
                files[item.file].0, item.name
            )
        })
        .collect()
}

fn implementation_closure(files: &[(String, Vec<String>)], live: &mut BTreeSet<String>) {
    // Associated parse errors and private schema representations belong to the
    // live type's impl closure. A declaration alone never roots a method.
    for (_, words) in files {
        for (i, word) in words.iter().enumerate() {
            if word != "impl" {
                continue;
            }
            let Some(open) = words
                .iter()
                .enumerate()
                .skip(i)
                .find(|(_, s)| *s == "{")
                .map(|(i, _)| i)
            else {
                continue;
            };
            if words
                .get(i + 1..open)
                .unwrap_or_default()
                .iter()
                .any(|name| live.contains(name))
            {
                let body = words.get(open..end(words, open)).unwrap_or_default();
                for (j, name) in body.iter().enumerate() {
                    if j > 0
                        && body
                            .get(j - 1)
                            .is_some_and(|s| matches!(s.as_str(), "fn" | "type" | "const"))
                    {
                        continue;
                    }
                    live.insert(name.clone());
                }
            }
        }
    }
}

fn unbuilt_messages(files: &[(String, Vec<String>)]) -> Vec<String> {
    let Some((file, (_, words))) = files
        .iter()
        .enumerate()
        .find(|(_, (path, _))| path.ends_with("/response/error.rs"))
    else {
        return Vec::new();
    };
    let Some(start) = words
        .windows(2)
        .position(|row| row == ["ErrorMessage", "{"])
        .map(|i| i + 1)
    else {
        return Vec::new();
    };
    let stop = end(words, start);
    let variants: Vec<_> = words
        .get(start..stop)
        .unwrap_or_default()
        .windows(3)
        .filter(|row| row[1] == "=" && row[2] == ">")
        .map(|row| row[0].clone())
        .collect();
    let mut references = BTreeSet::new();
    for (i, (_, words)) in files.iter().enumerate() {
        for (at, word) in words.iter().enumerate() {
            if i != file || at < start || at >= stop {
                references.insert(word);
            }
        }
    }
    variants
        .into_iter()
        .filter(|name| !references.contains(name))
        .map(|name| format!("ErrorMessage::{name} is never built by product code"))
        .collect()
}

pub(super) fn check(root: &Path) -> Check {
    let mut files = Vec::new();
    for entry in fs::read_dir(root.join("crates"))? {
        for path in rust_files(&entry?.path().join("src"))? {
            let relative = path.strip_prefix(root)?.to_string_lossy().into_owned();
            if relative.contains("/generated/") || relative.ends_with("_tests.rs") {
                continue;
            }
            files.push((relative, tokens(&fs::read_to_string(path)?)));
        }
    }
    let mut missing: Vec<_> = unreferenced(&files)
        .into_iter()
        .filter(|message| {
            ["biodata", "biomcp-types", "biomcp-fetch"]
                .iter()
                .any(|name| message.starts_with(&format!("crates/{name}/")))
        })
        .collect();
    missing.extend(unbuilt_messages(&files));
    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing.join("\n").into())
    }
}

#[test]
fn planted_public_item_fails_without_misclassifying_representation_paths() {
    let files = [
        ("values.rs", "pub struct Root { child: Child } pub struct Child; pub struct Dead; pub use Dead as Exported; pub fn unused() {}"),
        ("product.rs", "fn run(value: Root) {}"),
        ("macro.rs", "macro_rules! generated { () => { pub struct Generated; }; } generated!();"),
    ].map(|(path, text)| (path.to_owned(), tokens(text)));
    let missing = unreferenced(&files);
    assert_eq!(
        missing.len(),
        2,
        "a re-export alone must not count as product use: {missing:?}"
    );
    assert!(
        missing.iter().any(|s| s.contains("Dead has")),
        "planted dead public type must fail"
    );
    assert!(
        missing.iter().any(|s| s.contains("unused has")),
        "planted dead public function must fail"
    );
}
