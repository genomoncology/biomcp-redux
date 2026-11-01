//! Consumer checks for the reviewed shared value changes.
use biodata::{AccessionBase, RcvId, ScvId, VcvId};
use std::collections::HashSet;

#[test]
fn clinvar_versions_preserve_distinct_identity() -> Result<(), Box<dyn std::error::Error>> {
    macro_rules! versions {
        ($ty:ty, $prefix:literal) => {{
            let first: $ty = concat!($prefix, "000000001.1").parse()?;
            let second: $ty = concat!($prefix, "000000001.2").parse()?;
            assert_ne!(first, second, "ClinVar versions identify distinct records");
            assert!(
                first.same_accession(second),
                "the named accession comparison ignores version"
            );
            assert_eq!(
                HashSet::from([first, second]).len(),
                2,
                "hashing retains both versions"
            );
        }};
    }
    versions!(VcvId, "VCV");
    versions!(RcvId, "RCV");
    versions!(ScvId, "SCV");
    Ok(())
}

#[test]
fn accessions_write_canonical_variable_width_numbers() -> Result<(), Box<dyn std::error::Error>> {
    for (input, output) in [("LRG_0001t01", "LRG_1t1"), ("NM_000546", "NM_000546")] {
        let accession: AccessionBase = input.parse()?;
        assert_eq!(
            accession.as_str(),
            output,
            "the accession scheme controls digit width"
        );
        assert_eq!(
            serde_json::to_string(&accession)?,
            format!("\"{output}\""),
            "JSON uses canonical output"
        );
    }
    Ok(())
}
