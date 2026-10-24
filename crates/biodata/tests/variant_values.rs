//! Public common-value contracts, including the copied reference boundaries.
use biodata::*;
use serde_json::json;
type Check = Result<(), Box<dyn std::error::Error>>;
#[test]
fn copied_reference_values_keep_canonical_spellings() -> Check {
    for text in [
        "NC_000007.14",
        "NM_004333.4",
        "NP_004324.2",
        "ENST00000288602.11",
        "LRG_199p1",
    ] {
        let accession: Accession = text.parse()?;
        assert_eq!(accession.to_string(), text);
        assert_eq!(
            serde_json::from_str::<Accession>(&serde_json::to_string(&accession)?)?,
            accession
        );
    }
    for text in ["", "NC_7.14", "NM_004333", "NC_000007.14 extra"] {
        assert!(text.parse::<Accession>().is_err());
    }
    for text in ["1", "X", "Y", "MT"] {
        let chromosome: Chromosome = text.parse()?;
        for build in [Build::GRCh37, Build::GRCh38] {
            assert_eq!(
                Chromosome::from_accession(chromosome.accession(build), build)?,
                chromosome
            );
        }
    }
    for text in ["chr1", "chrM", "M", "23", "01", "26"] {
        assert!(text.parse::<Chromosome>().is_err());
    }
    Ok(())
}
#[test]
fn shared_allele_shape_allows_reference_alleles_and_refuses_rna() -> Check {
    // Copied owner edge table, adapted to SequenceLocation; normalization is absent.
    for (end, reference, alternate, valid) in [
        (0, "", "", false),
        (0, "", "A", true),
        (1, "A", "", true),
        (1, "A", "A", true),
        (2, "A", "T", false),
        (1, "U", "A", false),
        (1, "A", "U", false),
    ] {
        let value = json!({"location":{"sequence":"NC_000001.11","span":{"start":0,"end":end}},"reference":reference,"alternate":alternate});
        let allele = serde_json::from_value::<Allele>(value.clone());
        assert_eq!(allele.is_ok(), valid);
        if let Ok(allele) = allele {
            assert_eq!(serde_json::to_value(allele)?, value);
        }
    }
    assert!("U".parse::<Base>().is_err());
    assert!(serde_json::from_value::<Sequence>(json!("AU")).is_err());
    Ok(())
}
#[test]
fn published_variant_identifiers_round_trip_without_text_inference() -> Check {
    for text in [
        "rs113488022",
        "CA123643",
        "CA003404",
        "CA000434",
        "clinvar:13961",
        "VCV000013961.98",
        "ga4gh:VA.abcdefghijklmnopqrstuvwxyz012345",
    ] {
        let id: VariantId = text.parse()?;
        assert_eq!(id.to_string(), text);
        assert_eq!(serde_json::from_value::<VariantId>(json!(text))?, id);
    }
    for text in [
        "rs0",
        "rs01",
        "CA0",
        "RCV000014992",
        "SCV000001234.1",
        "clinvar.allele:15041",
        "clinvar:0",
        "clinvar:013961",
        "VCV1234",
        "rs18446744073709551616",
    ] {
        assert!(text.parse::<VariantId>().is_err());
    }
    for text in [
        "NM_004333.4:c.1799T>A",
        "LRG_321p1:p.Arg273His",
        "NC_012920.1:o.100A>G",
    ] {
        let hgvs: HgvsText = text.parse()?;
        assert_eq!(hgvs.as_str(), text);
        assert_eq!(Expression::hgvs(Text::new(text)?)?.value().as_str(), text);
    }
    for text in [
        "BRAF V600E",
        "V600E",
        "NM_004333:c.1799T>A",
        "NM_004333.4:x.1A>G",
    ] {
        assert!(text.parse::<HgvsText>().is_err());
    }
    Ok(())
}
#[test]
fn shared_public_fixture_checks_identity_and_source_alleles() -> Check {
    let rows: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("../../../tests/fixtures/variant/common.json"))?;
    let mut mitochondrial = Vec::new();
    for row in rows {
        // Clinical source rows are preserved for the ClinVar, ClinGen and CIViC section owners.
        if row["kind"] == "statement" {
            continue;
        }
        if row["kind"] == "refusal" {
            assert!(
                row["input"]
                    .as_str()
                    .ok_or("refusal input")?
                    .parse::<Sequence>()
                    .is_err()
            );
            continue;
        }
        for text in row["hgvs"].as_array().into_iter().flatten() {
            let text = text.as_str().ok_or("HGVS text")?;
            assert_eq!(text.parse::<HgvsText>()?.as_str(), text);
        }
        for id in row["identifiers"].as_array().into_iter().flatten() {
            let id = id.as_str().ok_or("identifier text")?;
            assert_eq!(id.parse::<VariantId>()?.to_string(), id);
        }
        if let Some(value) = row.get("allele") {
            let allele: Allele = serde_json::from_value(value.clone())?;
            assert_eq!(serde_json::to_value(&allele)?, *value);
            let build: Build = serde_json::from_value(row["build"].clone())?;
            if row["name"]
                .as_str()
                .is_some_and(|s| s.starts_with("mitochondrial-"))
            {
                let chromosome: Chromosome = "MT".parse()?;
                assert_eq!(chromosome.accession(build), allele.location().sequence);
                mitochondrial.push((build, allele));
            }
        }
        for expression in row["expressions"].as_array().into_iter().flatten() {
            let parsed: Expression = serde_json::from_value(expression.clone())?;
            assert_eq!(serde_json::to_value(parsed)?, *expression);
        }
        for candidate in row["candidates"].as_array().into_iter().flatten() {
            let _: Build = serde_json::from_value(candidate["build"].clone())?;
            let _: Expression = serde_json::from_value(candidate["expression"].clone())?;
        }
    }
    let [(first_build, first), (second_build, second)] = mitochondrial.as_slice() else {
        return Err("one mitochondrial row per build is required".into());
    };
    assert_ne!(first_build, second_build);
    assert_eq!(first, second);
    Ok(())
}

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
