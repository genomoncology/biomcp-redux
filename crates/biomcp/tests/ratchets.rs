//! Repository structure is part of the build contract.
#[path = "ratchets/dependencies.rs"]
mod dependencies;
#[path = "ratchets/errors.rs"]
mod errors;
#[path = "runner/features.rs"]
mod feature_contract;
#[path = "ratchets/lints.rs"]
mod lints;
#[path = "ratchets/placement.rs"]
mod placement;
#[path = "ratchets/proof.rs"]
mod proof;
#[path = "ratchets/proof_support.rs"]
mod proof_support;
#[path = "ratchets/reasons.rs"]
mod reasons;
#[path = "ratchets/source.rs"]
mod source;
#[path = "ratchets/spellings.rs"]
mod spellings;
#[path = "ratchets/transport_sources.rs"]
mod transport_sources;

use std::{fs, path::Path};

type Check = Result<(), Box<dyn std::error::Error>>;

fn check(root: &Path) -> Check {
    dependencies::check(root)?;
    reasons::check(root)?;
    spellings::check(root)?;
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
    let root = std::env::current_dir()?.join("../..");
    lints::check(&root)?;
    check(&root)
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

#[test]
fn registry_declarations_reject_unsafe_additional_origins() -> Check {
    const INVALID: &[&[&str]] = &[
        &["https://user@cdn.example"],
        &["https://user:password@cdn.example"],
        &["https://cdn.example/path"],
        &["https://cdn.example?x=1"],
        &["https://cdn.example#fragment"],
        &["https://*.example"],
        &["ftp://cdn.example"],
    ];
    let template = biomcp_adapters::registry::requests()
        .find(|spec| matches!(spec.destinations, biomcp_fetch::Destinations::Additional(_)))
        .ok_or("missing registry declaration with additional origins")?;
    request_hosts(template)?;
    for &origins in INVALID {
        let changed = biomcp_fetch::SourceSpec {
            destinations: biomcp_fetch::Destinations::Additional(origins),
            ..*template
        };
        assert!(
            request_hosts(&changed).is_err(),
            "unsafe registry origins: {origins:?}"
        );
    }
    Ok(())
}

// Validate actual registry declarations before listing their admitted hosts.
fn request_hosts(
    spec: &biomcp_fetch::SourceSpec,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut origins = Vec::new();
    if !spec.base.is_empty() {
        origins.push((spec.base, false));
    }
    if let biomcp_fetch::Destinations::Additional(additional) = spec.destinations {
        origins.extend(additional.iter().map(|origin| (*origin, true)));
    }
    let mut hosts = Vec::new();
    for (origin, additional) in origins {
        let parsed = url::Url::parse(origin)?;
        let host = parsed.host_str().ok_or("declared origin needs a host")?;
        if !matches!(parsed.scheme(), "http" | "https")
            || !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.fragment().is_some()
            || host.is_empty()
            || host.contains('*')
            || additional && (parsed.path() != "/" || parsed.query().is_some())
        {
            return Err(format!("invalid request origin declaration for {}", spec.source).into());
        }
        hosts.push(host.to_owned());
    }
    hosts.sort();
    hosts.dedup();
    Ok(hosts)
}
