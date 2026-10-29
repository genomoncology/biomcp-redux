//! Repository structure is part of the build contract.
#[path = "ratchets/dependencies.rs"]
mod dependencies;
#[path = "ratchets/placement.rs"]
mod placement;
#[path = "ratchets/proof.rs"]
mod proof;
#[path = "ratchets/reasons.rs"]
mod reasons;
#[path = "ratchets/source.rs"]
mod source;

use std::{fs, path::Path};

type Check = Result<(), Box<dyn std::error::Error>>;

fn check(root: &Path) -> Check {
    dependencies::check(root)?;
    reasons::check(root)?;
    source::check(root)
}

fn rust_files(path: &Path) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    if path.exists() {
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if path.is_dir() {
                files.extend(rust_files(&path)?);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "The repository contract test owns its Cargo working directory boundary"
)]
fn repository_obeys_structural_contract() -> Check {
    check(std::env::current_dir()?.join("../..").as_path())
}

#[test]
fn outbound_registry_lists_only_request_hosts_and_explicit_publisher_policy() -> Check {
    use biomcp_fetch::Destinations;
    let mut rows = std::collections::BTreeSet::new();
    let mut publisher_policies = 0;
    for spec in biomcp_adapters::registry::requests() {
        for host in spec.request_hosts()? {
            rows.insert(format!("{} {host}", spec.source));
        }
        if matches!(spec.destinations, Destinations::PublisherPdf) {
            assert!(spec.base.is_empty());
            assert!(matches!(spec.key, biomcp_fetch::KeyUse::None));
            rows.insert(format!(
                "{} publisher-pdf-public-address-policy",
                spec.source
            ));
            publisher_policies += 1;
        } else {
            assert!(!spec.request_hosts()?.is_empty());
        }
    }
    assert_eq!(publisher_policies, 1);
    assert!(rows.contains("ctgov cdn.clinicaltrials.gov"));
    assert!(rows.contains("cbioportal api.github.com"));
    assert!(
        !rows
            .iter()
            .any(|row| row.ends_with(" raw.githubusercontent.com"))
    );
    use std::io::Write;
    let mut output = std::io::stdout().lock();
    for row in rows {
        writeln!(output, "{row}")?;
    }
    Ok(())
}
