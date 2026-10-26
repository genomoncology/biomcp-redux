use super::{Check, check};
use std::{fs, path::Path};

fn tree() -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let tree = tempfile::tempdir()?;
    fs::write(
        tree.path().join("Cargo.toml"),
        "[workspace]\n[workspace.dependencies]\nhttp = { package = 'ureq', version = '3' }\n",
    )?;
    put(
        tree.path(),
        "crates/biodata/Cargo.toml",
        "[package]\nname = 'biodata'\n[lints]\nworkspace = true\n",
    )?;
    put(tree.path(), "crates/biodata/src/lib.rs", "//! Values.\n")?;
    Ok(tree)
}

fn put(root: &Path, path: &str, text: &str) -> Check {
    let path = root.join(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)?;
    Ok(())
}

#[test]
fn isolated_trees_reject_structural_violations() -> Check {
    let valid = tree()?;
    check(valid.path())?;
    let cases = [
        (
            "crates/biodata/Cargo.toml",
            "[package]\nname='biodata'\n[lints]\nworkspace=true\n[dependencies]\nbiomcp-types={path='../biomcp-types'}\n",
            "forbidden dependencies dependency",
        ),
        (
            "crates/biodata/Cargo.toml",
            "[lints]\nworkspace=true\n[build-dependencies]\nhttp={package='ureq',version='3'}\n",
            "forbidden build-dependencies dependency",
        ),
        (
            "crates/biodata/Cargo.toml",
            "[lints]\nworkspace=true\n[target.'cfg(unix)'.dependencies]\nhttp.workspace=true\n",
            "forbidden dependencies dependency",
        ),
        (
            "crates/biodata/Cargo.toml",
            "[package]\nname='biodata'\n",
            "workspace lint inheritance",
        ),
        (
            "crates/biodata/src/lib.rs",
            "#[test]\nfn hidden() {}",
            "inline test",
        ),
        (
            "crates/biodata/src/lib.rs",
            "#[expect(dead_code, reason=\"unused\")]\nfn unused() {}",
            "suppressions forbidden",
        ),
        (
            "crates/biodata/src/lib.rs",
            "#[expect(clippy::panic, reason=\"test\")]\nfn unused() {}",
            "lint exception",
        ),
        (
            "crates/biomcp-adapters/src/example/raw.rs",
            "#[serde(default)]\nstruct Raw;",
            "raw serde default",
        ),
        (
            "crates/biodata/src/lib.rs",
            "// Quirk: missing.yaml\n",
            "Quirk requires",
        ),
        (
            "crates/biomcp-query/src/lib.rs",
            "use rusqlite as sql;",
            "SQLite references",
        ),
        (
            "crates/biomcp-adapters/src/lib.rs",
            "SourceKey::declare(\"provider\");",
            "quota declarations",
        ),
        (
            "crates/biomcp-adapters/src/quotas.rs",
            "SourceKey::declare(\"provider\");\nSourceKey::declare(\"provider\");",
            "duplicate source key",
        ),
    ];
    for (path, content, rule) in cases {
        rejects(path, content, rule)?;
    }
    rejects(
        "crates/biodata/src/lib.rs",
        &"// line\n".repeat(601),
        "file line limit",
    )?;
    rejects(
        "crates/biomcp-adapters/src/example/map.rs",
        &"// line\n".repeat(301),
        "file line limit",
    )?;
    Ok(())
}

fn rejects(path: &str, content: &str, rule: &str) -> Check {
    let tree = tree()?;
    put(tree.path(), path, content)?;
    let failure = match check(tree.path()) {
        Ok(()) => return Err("planted violation passed".into()),
        Err(error) => error.to_string(),
    };
    assert!(
        failure.contains(path) && failure.contains(rule),
        "{failure}"
    );
    Ok(())
}

#[test]
fn quirks_require_cases_inside_case_directory() -> Check {
    let tree = tree()?;
    put(
        tree.path(),
        "tests/cases/gene/example.yaml",
        "case: example",
    )?;
    let source = "crates/biodata/src/lib.rs";
    put(tree.path(), source, "// Quirk: gene/example.yaml\n")?;
    check(tree.path())?;
    put(tree.path(), "tests/outside.yaml", "case: outside")?;
    for reference in [
        "../outside.yaml".to_owned(),
        tree.path().join("tests/outside.yaml").display().to_string(),
    ] {
        put(tree.path(), source, &format!("// Quirk: {reference}\n"))?;
        let failure = check(tree.path()).expect_err("outside case accepted");
        assert!(failure.to_string().contains("Quirk requires"));
    }
    Ok(())
}

#[test]
fn static_quota_references_are_not_quota_declarations() -> Check {
    let tree = tree()?;
    put(
        tree.path(),
        "crates/biodata/src/lib.rs",
        "pub struct SourceSpec { pub quota: &'static Quota }\n",
    )?;
    check(tree.path())?;
    rejects(
        "crates/biodata/src/lib.rs",
        "static QUOTA: Quota = todo!();",
        "quota declarations",
    )?;
    Ok(())
}

#[test]
fn case_parser_stays_outside_runtime_dependencies() -> Check {
    rejects(
        "crates/biomcp/Cargo.toml",
        "[package]\nname='biomcp'\n[lints]\nworkspace=true\n[dependencies]\nserde-saphyr='2'\n",
        "forbidden dependencies dependency",
    )
}

#[test]
fn rejects_unlisted_integration_test_folders() -> Check {
    rejects(
        "crates/biomcp-adapters/tests/unlisted/hidden.rs",
        "#[test]\nfn hidden() {}",
        "test outside listed locations",
    )
}

#[test]
fn rejects_conditional_integration_test_attributes() -> Check {
    rejects(
        "crates/biomcp-adapters/tests/unlisted/conditional.rs",
        "#[cfg_attr(test, test)]\nfn hidden() {}",
        "test outside listed locations",
    )
}

#[test]
fn source_without_publisher_terms_is_rejected() -> Check {
    rejects(
        "crates/biomcp-adapters/src/planted.rs",
        "pub static SPEC: SourceSpec = SourceSpec { source: Source::Hgnc };",
        "publisher terms",
    )
}

#[test]
fn test_files_obey_the_file_line_limit() -> Check {
    let tree = tree()?;
    let path = "crates/biodata/tests/nested/size.rs";
    put(tree.path(), path, &"// line\n".repeat(600))?;
    check(tree.path())?;
    rejects(path, &"// line\n".repeat(601), "file line limit")
}

#[test]
fn source_outcomes_require_registry_terms() -> Check {
    let tree = tree()?;
    put(
        tree.path(),
        "crates/biomcp-query/src/example.rs",
        "fn response() { SourceOutcome { source: Source::Hgnc } }",
    )?;
    let error = check(tree.path())
        .err()
        .ok_or("unlabeled source outcome accepted")?;
    assert!(
        error
            .to_string()
            .contains("source outcomes require registry terms")
    );
    Ok(())
}

#[test]
fn source_registry_accepts_directory_and_file_modules_and_names_missing_entry() -> Check {
    let tree = tree()?;
    put(
        tree.path(),
        "crates/biomcp-adapters/src/registry.rs",
        "&crate::directory::SPEC, &crate::single_file::SPEC",
    )?;
    let declaration = r#"SourceSpec { terms: Terms { name: "Publisher", url: "https://example.org/terms", usage: Open } }"#;
    for path in ["directory/mod.rs", "single_file.rs"] {
        put(
            tree.path(),
            &format!("crates/biomcp-adapters/src/{path}"),
            declaration,
        )?;
    }
    check(tree.path())?;
    let path = "crates/biomcp-adapters/src/missing.rs";
    put(tree.path(), path, declaration)?;
    let failure = check(tree.path())
        .err()
        .ok_or("unregistered single-file source accepted")?
        .to_string();
    assert!(failure.contains(path), "{failure}");
    assert!(failure.contains("&crate::missing::SPEC"), "{failure}");
    Ok(())
}

#[test]
fn request_reason_mapping_cannot_be_copied() -> Check {
    let root = tempfile::tempdir()?;
    let source = root.path().join("crates/biomcp-query/src");
    fs::create_dir_all(&source)?;
    for copied in [
        "fn reason(error: RequestError) -> Reason { match error { RequestError::Source(e) => e.reason, _ => Reason::Unavailable } }",
        "match reply { Err(RequestError::Source(e)) => Section::unavailable(source, e.reason), Err(_) => Section::unavailable(source, Reason::Unavailable) }",
        "match error { RequestError::Source(SourceError { reason, .. }) => reason, _ => Reason::Unavailable }",
    ] {
        fs::write(source.join("section.rs"), copied)?;
        if super::reasons::check(root.path()).is_ok() {
            return Err("a copied request reason mapping passed the repository gate".into());
        }
    }
    fs::write(
        source.join("section.rs"),
        "if matches!(error, RequestError::Source(e) if e.reason == Reason::NotFound) { absent() }",
    )?;
    super::reasons::check(root.path())
}
