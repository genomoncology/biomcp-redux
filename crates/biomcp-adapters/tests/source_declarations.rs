//! Source declaration contracts shared by every command interface.
use std::collections::BTreeSet;
#[test]
fn bundle_names_are_unique_and_keep_the_public_command_contract() {
    let names: Vec<_> = biomcp_adapters::decl::bundles()
        .map(|bundle| bundle.name())
        .collect();
    assert_eq!(
        names,
        [
            "hgnc",
            "clingen",
            "gencc",
            "wikipathways",
            "cbioportal-studies",
            "ema",
            "ddinter",
            "who-pq",
            "gtr",
            "who-ivd",
            "fda-cdx"
        ],
        "source declarations keep every public sync word"
    );
    assert_eq!(
        names.len(),
        names.iter().collect::<BTreeSet<_>>().len(),
        "each sync word selects exactly one bundle"
    );
}
