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
    for text in [
        "NM_004333.4(BRAF):c.1799T>A",
        "NC_000007.13(NM_004333.4):c.1799T>A",
    ] {
        let hgvs: HgvsText = text.parse().expect("published inner reference");
        assert_eq!(hgvs.to_string(), text);
        assert_eq!(
            hgvs.accession().expect("outer accession").to_string(),
            text.split('(').next().expect("reference")
        );
    }
}

#[test]
fn identifier_levels_are_checked_and_never_promoted() {
    use biodata::{CaId, ClinVarAlleleId, CosmicId, RcvId, RsId, ScvId, VariantId};
    assert!(RsId::new(0).is_err());
    assert!(CaId::new(0).is_err());
    for text in ["RCV000014992.12", "RCV000014992"] {
        let value: RcvId = text.parse().expect("aggregate id");
        assert_eq!(value.to_string(), text);
        assert!(text.parse::<VariantId>().is_err());
    }
    assert_eq!(
        "SCV000001234.1"
            .parse::<ScvId>()
            .expect("submission")
            .to_string(),
        "SCV000001234.1"
    );
    assert_eq!(
        "15041"
            .parse::<ClinVarAlleleId>()
            .expect("record allele")
            .to_string(),
        "15041"
    );
    for text in ["COSV55386424", "COSM476", "COSN123"] {
        assert_eq!(
            text.parse::<CosmicId>().expect("COSMIC kind").to_string(),
            text
        );
    }
    for text in ["CA12345", "CA0000434", "RCV000014992.01", "COSV0"] {
        assert!(text.parse::<VariantId>().is_err());
    }
}
#[test]
fn gene_references_keep_only_stated_ids_and_require_a_label() {
    use biodata::GeneRef;
    let text = r#"{"label":"BRAF","ncbi_gene_id":"673"}"#;
    let value: GeneRef = serde_json::from_str(text).expect("source ids");
    assert!(value.hgnc_id().is_none());
    assert_eq!(value.ncbi_gene_id().expect("stated id").get(), 673);
    assert_eq!(
        serde_json::to_value(value).expect("serialize"),
        serde_json::from_str::<serde_json::Value>(text).expect("json")
    );
    assert!(serde_json::from_str::<GeneRef>(r#"{"label":" "}"#).is_err());
}
#[test]
fn interval_comparison_requires_both_source_builds() {
    use biodata::{Build, GenomicInterval, Span0};
    let mut region = GenomicInterval {
        build: Some(Build::GRCh37),
        chromosome: "7".parse().expect("chromosome"),
        span: Span0::new(0, 2).expect("bounds"),
        strand: None,
    };
    assert_eq!(region.contains(&region), Ok(true));
    let other = region.clone();
    region.build = None;
    assert_eq!(
        region
            .contains(&other)
            .expect_err("no guessed assembly")
            .code(),
        "coord.interval.build_absent"
    );
    assert!(other.contains(&region).is_err());
}
#[test]
fn shared_refusals_use_the_settled_codes() {
    for (text, code) in [
        ("NM_004333.04", "accession.version.leading_zero"),
        ("NM_1234567.1", "accession.digits.width"),
        ("NM_004333", "accession.version.missing"),
    ] {
        assert_eq!(
            text.parse::<Accession>()
                .expect_err("invalid grammar")
                .code(),
            code
        );
    }
    assert_eq!(
        "U".parse::<biodata::Base>().expect_err("DNA only").code(),
        "sequence.base.rna"
    );
}

#[test]
fn mane_statuses_keep_the_source_pair_and_round_trip() {
    for status in ["select", "plus_clinical"] {
        let value = serde_json::json!({"status":status,"refseq":"NM_004333.4","ensembl":"ENST00000288602.11"});
        let pair: biodata::Mane = serde_json::from_value(value.clone()).expect("stated pair");
        assert_eq!(serde_json::to_value(pair).expect("serialize"), value);
    }
}
