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
            &format!("pub static SPEC: SourceSpec = {declaration};"),
        )?;
    }
    check(tree.path())?;
    let auxiliary = "crates/biomcp-adapters/src/directory/other.rs";
    put(
        tree.path(),
        auxiliary,
        &format!("pub static EXTRA: SourceSpec = {declaration};"),
    )?;
    let failure = check(tree.path())
        .err()
        .ok_or("unregistered auxiliary source accepted")?
        .to_string();
    assert!(
        failure.contains("&crate::directory::other::EXTRA"),
        "{failure}"
    );
    put(
        tree.path(),
        "crates/biomcp-adapters/src/registry.rs",
        "&crate::directory::SPEC, &crate::single_file::SPEC, &crate::directory::other::EXTRA",
    )?;
    check(tree.path())?;
    put(
        tree.path(),
        auxiliary,
        &format!(
            "pub static EXTRA: SourceSpec = {declaration};\nfn hidden() {{\n    let other = {declaration};\n}}\n"
        ),
    )?;
    let failure = check(tree.path())
        .err()
        .ok_or("dynamic source escaped the registry")?
        .to_string();
    assert!(
        failure.contains("source declarations require named statics"),
        "{failure}"
    );
    put(
        tree.path(),
        auxiliary,
        &format!("pub static EXTRA: SourceSpec = {declaration};"),
    )?;
    let path = "crates/biomcp-adapters/src/missing.rs";
    put(
        tree.path(),
        path,
        &format!("pub static SPEC: SourceSpec = {declaration};"),
    )?;
    let failure = check(tree.path())
        .err()
        .ok_or("unregistered single-file source accepted")?
        .to_string();
    assert!(failure.contains(path), "{failure}");
    assert!(failure.contains("&crate::missing::SPEC"), "{failure}");
    Ok(())
}

#[test]
fn rust_lines_obey_width_limit() -> Check {
    for path in ["crates/biodata/src/lib.rs", "crates/biodata/tests/width.rs"] {
        let tree = tree()?;
        put(tree.path(), path, &format!("//{}\n", "x".repeat(158)))?;
        check(tree.path())?;
        rejects(
            path,
            &format!("//{}\n", "x".repeat(159)),
            "line width limit",
        )?;
    }
    Ok(())
}

#[test]
fn request_reason_mapping_cannot_be_copied() -> Check {
    let root = tempfile::tempdir()?;
    let source = root.path().join("crates/biomcp-query/src");
    fs::create_dir_all(&source)?;
    for copied in [
        "fn reason(error: RequestError) -> Reason { match error { RequestError::Source(e) => e.reason, _ => Reason::Unavailable } }",
        "match reply { Err(RequestError::Source(e)) => Section::unavailable(source, e.reason), Err(_) => \
        Section::unavailable(source, Reason::Unavailable) }",
        "match error { RequestError::Source(SourceError { reason, .. }) => reason, _ => Reason::Unavailable }",
        "let reason = if let biomcp_fetch::RequestError::Source(e) = error { e.reason } else { Reason::Unavailable };",
        "fn reason(error: RequestError) -> Reason { if let RequestError::Source(e) = error { return e.reason; } Reason::Unavailable }",
        "if let RequestError::Source(e) = error { Some(e.reason) } else { None }",
        "fn reason(error: RequestError) -> Option<Reason> { if let RequestError::Source(e) = error { return Some(e.reason); } None }",
    ] {
        fs::write(source.join("section.rs"), copied)?;
        if super::reasons::check(root.path()).is_ok() {
            return Err("a copied request reason mapping passed the repository gate".into());
        }
    }
    fs::write(
        source.join("section.rs"),
        "if matches!(error, RequestError::Source(e) if e.reason == Reason::NotFound) { absent() }\nif let \
            RequestError::Source(e) = error { if e.reason == Reason::NotFound { absent() } }",
    )?;
    super::reasons::check(root.path())
}

#[test]
fn generated_exception_cannot_hide_product_code() -> Check {
    let tree = tree()?;
    let path = "crates/biomcp-fetch/src/grpc/proto.rs";
    let wrapper = r#"#![expect(dead_code, reason="pinned generation")]
include!("generated/mod.rs");
"#;
    put(tree.path(), path, wrapper)?;
    check(tree.path())?;
    rejects(
        path,
        &(wrapper.to_owned() + "fn hidden() {}"),
        "generated exception requires only",
    )?;
    for modified in [
        wrapper.replace("dead_code", "dead_code, unused_variables"),
        wrapper.replace("#![expect", "#[expect"),
        format!("fn hidden() {{}}\n{wrapper}"),
    ] {
        rejects(path, &modified, "generated exception requires only")?;
    }
    rejects(
        path,
        &wrapper.replace("dead_code", "unused_imports"),
        "suppressions forbidden",
    )?;
    rejects(
        "crates/biodata/src/lib.rs",
        wrapper,
        "suppressions forbidden",
    )?;
    Ok(())
}

#[test]
fn adapter_mapping_helpers_require_one_owner() -> Check {
    for declaration in ["fn problem() {}", "fn text() {}"] {
        rejects(
            "crates/biomcp-adapters/src/example/map.rs",
            declaration,
            "mapping helpers belong in mapped.rs",
        )?;
    }
    Ok(())
}

#[test]
fn new_optional_drop_requires_its_own_classification() -> Check {
    let tree = tree()?;
    let path = "crates/biomcp-adapters/src/new_source/details.rs";
    put(
        tree.path(),
        path,
        "fn map() { /* .ok() */ let native = \".ok()\"; }",
    )?;
    check(tree.path())?;
    for source in [
        "fn map() { let title = input.parse().ok(); }",
        "fn map() { let title = input.parse() . ok ( ); }",
    ] {
        rejects(path, source, "classify new optional-value drop")?;
    }
    Ok(())
}

#[test]
fn test_fixture_paths_have_one_owner() -> Check {
    for read in [
        "env!(\"CARGO_MANIFEST_DIR\")",
        "env! { \"CARGO_MANIFEST_DIR\" }",
        "option_env!(\"CARGO_MANIFEST_DIR\")",
        "std::env::var(\"CARGO_MANIFEST_DIR\")",
        "std::env::var_os(\"CARGO_MANIFEST_DIR\")",
    ] {
        rejects(
            "crates/biomcp/tests/runner/planted.rs",
            &format!("fn fixture() {{ let root = {read}; }}"),
            "fixture paths belong in runner/launch.rs",
        )?;
    }
    Ok(())
}

#[test]
fn adapter_drop_forms_and_file_rules_require_pending_identity() -> Check {
    let root = tempfile::tempdir()?;
    let directory = root.path().join("crates/biomcp-adapters/src/example");
    fs::create_dir_all(&directory)?;
    let path = directory.join("details.rs");
    for source in [
        "fn value() { let native = input.parse().ok(); }",
        "fn value() { if let Ok(native) = input.parse() { use_value(native); } }",
        "fn value() { let Ok(native) = input.parse() else { return; }; }",
        "fn value() { input.filter_map(|v| v.parse().ok()); }",
        "fn value() { input.parse::<u64>().unwrap_or_default(); }",
        "fn value() { let parsed = input.parse::<u64>(); parsed.unwrap_or_default(); }",
        "fn value() { serde_json::from_value::<u64>(input).unwrap_or_default(); }",
        "fn value() { serde_json::from_reader::<_, u64>(input).unwrap_or_default(); }",
        "#[serde(default)] struct Raw;",
        "#[serde(deny_unknown_fields)] struct Raw;",
    ] {
        fs::write(&path, source)?;
        let failure = super::source::check(root.path())
            .expect_err("planted adapter violation passed")
            .to_string();
        assert!(failure.contains("example/details.rs"), "{failure}");
    }
    fs::write(&path, "// line\n".repeat(301))?;
    assert!(
        super::source::check(root.path())
            .expect_err("301-line adapter passed")
            .to_string()
            .contains("file line limit")
    );
    fs::write(&path, "// line\n".repeat(300))?;
    super::source::check(root.path())?;
    fs::write(
        &path,
        "fn value() { /* .ok() */ let note = \"if let Ok(v) .ok()\"; }",
    )?;
    super::source::check(root.path())
}

#[test]
fn inventory_allows_shrinking_and_reclassification_but_refuses_new_identities() -> Check {
    use super::source::adapter_values::validate;
    let original = include_str!("adapter_value_audit.json");
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    let sites = current["sites"]
        .as_array_mut()
        .ok_or("missing audit sites")?;
    sites.retain(|entry| entry["maximum"].as_u64().is_some_and(|n| n > 1));
    for entry in sites.iter_mut() {
        entry["maximum"] = 1.into();
        entry["class"] = "migrated".into();
    }
    validate(&current.to_string())?;
    validate("{\"sites\":[]}")?;
    let planted = serde_json::json!({"sites": [{
        "path": "crates/biomcp-adapters/src/example/details.rs",
        "site": "new pending identity",
        "maximum": 1,
        "class": "pending_audit",
        "owner": "example",
        "reason": "Planted new entry"
    }]});
    assert!(
        validate(&planted.to_string()).is_err(),
        "new entry in empty inventory passed"
    );
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    if current["sites"].as_array().is_some_and(Vec::is_empty) {
        return Ok(());
    }
    current["sites"][0]["site"] = "new pending identity".into();
    assert!(
        validate(&current.to_string()).is_err(),
        "same-count replacement passed"
    );
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    let maximum = current["sites"][0]["maximum"]
        .as_u64()
        .ok_or("missing count")?;
    current["sites"][0]["maximum"] = (maximum + 1).into();
    assert!(
        validate(&current.to_string()).is_err(),
        "increased ceiling passed"
    );
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    current["sites"][0]["owner"] = "other_adapter".into();
    assert!(
        validate(&current.to_string()).is_err(),
        "wrong adapter owner passed"
    );
    let mut current: serde_json::Value = serde_json::from_str(original)?;
    let duplicate = current["sites"][0].clone();
    current["sites"]
        .as_array_mut()
        .ok_or("missing audit sites")?
        .push(duplicate);
    assert!(
        validate(&current.to_string()).is_err(),
        "duplicate pending entry passed"
    );
    Ok(())
}

#[test]
fn audit_rejects_owned_underscores_and_old_tags_but_allows_native_values() {
    for text in [
        "#[serde(tag = \"annotationkind\")] enum Pgx {}",
        "#[serde(rename_all = \"snake_case\")] enum Owned {}",
        "wire_enum! { Reason { NoKey => \"no_key\" } }",
    ] {
        assert!(
            super::spellings::audit(&super::source::tokens(text), true).is_err(),
            "{text}"
        );
    }
    assert!(
        super::spellings::audit(
            &super::source::tokens("wire_enum! { Native { Value => \"source_native\" } }"),
            true
        )
        .is_ok()
    );
}

#[test]
fn enum_return_types_are_not_macro_declarations() {
    assert!(
        super::spellings::audit(
            &super::source::tokens(
                "fn kind() -> ArticleMatchKind { match self { X => ArticleMatchKind::Doi } }"
            ),
            true
        )
        .is_ok()
    );
}

#[test]
fn namespaced_error_codes_are_exempt_by_type() {
    for name in ["ErrorCode", "ProblemCode"] {
        for literal in ["usage.future_name", "wire_enum.unknown"] {
            let source = format!("wire_enum! {{ {name} {{ New => {literal:?} }} }}");
            assert!(super::spellings::audit(&super::source::tokens(&source), true).is_ok());
            let ordinary = source.replace(name, "Reason");
            assert!(super::spellings::audit(&super::source::tokens(&ordinary), true).is_err());
        }
    }
}

#[test]
fn crate_ownership_keeps_biodata_values_and_audits_biomcp_values() -> Check {
    for source in [
        "#[serde(rename_all = \"snake_case\")] enum Owned {}",
        "wire_enum! { DrugRecallCountUnit { Value => \"recall_records\" } }",
    ] {
        let tree = tree()?;
        put(tree.path(), "crates/biodata/src/lib.rs", source)?;
        check(tree.path())?;
        rejects("crates/biomcp-types/src/lib.rs", source, "kebab-case")?;
    }
    Ok(())
}
