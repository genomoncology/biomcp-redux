//! Repository structure is part of the build contract.
#[path = "ratchets/dependencies.rs"]
mod dependencies;
#[path = "ratchets/lints.rs"]
mod lints;
#[path = "ratchets/placement.rs"]
mod placement;
#[path = "ratchets/proof.rs"]
mod proof;
#[path = "ratchets/reachability.rs"]
mod reachability;
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
fn repository_obeys_structural_contract() -> Check {
    lints::check(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))?;
    reachability::check(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))?;
    check(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .as_path(),
    )
}

#[test]
fn outbound_registry_lists_only_request_hosts_and_explicit_publisher_policy() -> Check {
    use biomcp_fetch::Destinations;
    let mut rows = std::collections::BTreeSet::new();
    let mut publisher_policies = 0;
    for spec in biomcp_adapters::registry::requests() {
        for host in request_hosts(spec)? {
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
            assert!(!request_hosts(spec)?.is_empty());
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

// Inventory the declarations that the product admission boundary validates.
fn request_hosts(
    spec: &biomcp_fetch::SourceSpec,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut origins = Vec::new();
    if !spec.base.is_empty() {
        origins.push(spec.base);
    }
    if let biomcp_fetch::Destinations::Additional(additional) = spec.destinations {
        origins.extend_from_slice(additional);
    }
    let mut hosts = Vec::new();
    for origin in origins {
        let parsed = url::Url::parse(origin)?;
        hosts.push(
            parsed
                .host_str()
                .ok_or("declared origin needs a host")?
                .to_owned(),
        );
    }
    hosts.sort();
    hosts.dedup();
    Ok(hosts)
}
