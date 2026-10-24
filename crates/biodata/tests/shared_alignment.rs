//! Public grammar boundaries shared by consumers.
use biodata::{Accession, HgvsText};
#[test]
fn accessions_refuse_noncanonical_versions_and_widths() {
    for value in ["NM_004333.04", "NM_004333.0", "NM_1234567.1"] {
        assert!(value.parse::<Accession>().is_err(), "{value}");
    }
}
#[test]
fn hgvs_keeps_its_inner_reference_and_outer_accession() {
    for text in ["NM_004333.4(BRAF):c.1799T>A", "NC_000007.13(NM_004333.4):c.1799T>A"] {
        let hgvs: HgvsText = text.parse().expect("published inner reference");
        assert_eq!(hgvs.to_string(), text);
        assert_eq!(hgvs.accession().expect("outer accession").to_string(), text.split('(').next().expect("reference"));
    }
}
