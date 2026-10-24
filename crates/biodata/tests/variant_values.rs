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
        for build in [Build::Grch37, Build::Grch38] {
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
        "rs113488022", "CA123643", "CA003404", "CA000434", "clinvar:13961", "clinvar.allele:15041", "VCV000013961.98", "RCV000014992", "SCV000001234.1", "ga4gh:VA.abcdefghijklmnopqrstuvwxyz012345",
    ] {
        let id: VariantId = text.parse()?;
        assert_eq!(id.to_string(), text);
        assert_eq!(serde_json::from_value::<VariantId>(json!(text))?, id);
    }
    for text in [
        "rs0",
        "rs01",
        "CA0",
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
