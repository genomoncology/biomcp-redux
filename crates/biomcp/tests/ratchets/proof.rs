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
            "[package]\nname='biodata'\n[lints]\nworkspace=true\n[dependencies]\nureq='3'\n",
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
