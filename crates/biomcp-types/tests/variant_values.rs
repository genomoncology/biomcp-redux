//! Public common-value contracts, including the copied reference boundaries.
use biodata::*;
use biomcp_types::*;
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
    // HGVS 21.1: recommendations/uncertain, RNA/splicing and background/refseq.
    #[cfg(feature = "schema")]
    let validator =
        jsonschema::draft202012::new(&serde_json::to_value(schemars::schema_for!(HgvsText))?)?;
    for (text, valid) in [
        ("NM_005228.5:c.???", false),
        ("NM_005228.5:c.2369C>T word", false),
        ("NM_005228.5:c.2369C\t>T", false),
        ("NP_005219.2:c.2369C>T", false),
        ("XP_005219.2:r.2369c>u", false),
        ("ENSP00000288602.1:g.100A>T", false),
        ("LRG_199p1:n.100A>T", false),
        ("NC_000007.14(NP_005219.2):c.2369C>T", false),
        ("NM_005228.5(NP_label):c.2369C>T", true),
        ("NP_005219.2:p.???", false),
        ("NP_005219.2:p.?", true),
        ("NP_005219.2:p.0", true),
        ("NP_005219.2:p.0?", true),
        ("NP_005219.2:p.=", true),
        ("NP_005219.2:p.(=)", true),
        ("NP_005219.2:p.(?)", true),
        ("NM_005228.5:r.?", true),
        ("NM_005228.5:r.(?)", true),
        ("NM_005228.5:r.=", true),
        ("NM_005228.5:r.(=)", true),
        ("NM_005228.5:r.spl", true),
        ("NM_005228.5:r.spl?", true),
        ("NM_005228.5:r.(spl?)", true),
    ] {
        #[cfg(feature = "schema")]
        assert_eq!(validator.is_valid(&json!(text)), valid, "schema: {text}");
        assert_eq!(text.parse::<HgvsText>().is_ok(), valid, "{text}");
        assert_eq!(Expression::hgvs(Text::new(text)?).is_ok(), valid, "{text}");
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
fn allele_boundaries_refuse_unchecked_accession_construction() -> Check {
    for sequence in [
        Accession::RefSeq {
            prefix: RefSeqPrefix::Nm,
            number: 4333,
            version: 0,
        },
        Accession::Ensembl {
            feature: EnsemblFeature::Transcript,
            number: 288602,
            version: 0,
        },
        Accession::RefSeq {
            prefix: RefSeqPrefix::Nm,
            number: u32::MAX,
            version: 1,
        },
    ] {
        let location = SequenceLocation {
            sequence,
            span: Span0::new(0, 1)?,
        };
        assert!(
            serde_json::to_value(&location).is_err(),
            "unchecked location: {sequence}"
        );
        assert!(
            Allele::new(location, "A".parse()?, "T".parse()?).is_err(),
            "unchecked allele: {sequence}"
        );
    }
    Ok(())
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

#[test]
fn public_civic_tier_and_level_remain_separate() -> Check {
    use biodata::{AmpAscoCap2017LevelCode, AmpAscoCap2017TierCode};
    use serde_json::Value;
    let rows: Vec<Value> =
        serde_json::from_str(include_str!("../../../tests/fixtures/variant/common.json"))?;
    let row = rows
        .iter()
        .find(|r| r["name"] == "civic-tier-level-statement")
        .ok_or("missing public CIViC fixture")?;
    let value = json!({
        "subject":row["subject"], "proposition":"therapeutic_response", "predicate":"predicts_sensitivity_to",
        "classification":{"label":row["classification"],"coding":{"system":"amp_asco_cap_2017_tier","code":"tier_i"}},
        "strength":{"label":row["classification"],"coding":{"system":"amp_asco_cap_2017_level","code":"level_a"}},
        "condition":{"label":row["condition"]},
        "therapies":row["therapies"].as_array().ok_or("therapies")?.iter().map(|label|json!({"label":label})).collect::<Vec<_>>(),
        "attribution":{"source":"civic","record":row["record"]}
    });
    let statement: Statement = serde_json::from_value(value)?;
    assert_eq!(statement.attribution.source, Source::Civic.text());
    assert_eq!(
        statement.classification.coding,
        Some(Coding::AmpAscoCap2017Tier(AmpAscoCap2017TierCode::TierI))
    );
    assert_eq!(
        statement.strength.as_ref().and_then(|s| s.coding.as_ref()),
        Some(&Coding::AmpAscoCap2017Level(
            AmpAscoCap2017LevelCode::LevelA
        ))
    );
    assert!(statement.attribution.version.is_none());
    assert!(statement.attribution.evaluated.is_none());
    let output = serde_json::to_value(statement)?;
    assert_eq!(
        output.pointer("/classification/label"),
        Some(&row["classification"])
    );
    assert_eq!(output.pointer("/subject/level"), Some(&json!("protein")));
    let mut other_consumer = output.clone();
    other_consumer["attribution"]["source"] = json!("another-consumer");
    let neutral: Statement = serde_json::from_value(other_consumer)?;
    assert_eq!(neutral.attribution.source.as_str(), "another-consumer");
    Ok(())
}

#[test]
fn shared_clinvar_and_clingen_fixture_rows_become_statements() -> Check {
    let rows: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("../../../tests/fixtures/variant/common.json"))?;
    let mut checked = 0;
    for row in rows.iter().filter(|r| {
        matches!(
            r["owner"].as_str(),
            Some("clinvar-assertions" | "clingen-assertions")
        )
    }) {
        let clinvar = row["owner"] == "clinvar-assertions";
        let subject = if clinvar {
            &row["variation_id"]
        } else {
            &row["subject"]
        };
        let mut statement = json!(
            { "subject":{"level":"allele","id":subject},"proposition":"pathogenicity","classification":{"label":row["classification"]},
            "attribution":{"source":if clinvar {"clinvar"} else {"erepo"},"record":row["record"],"submitter":row["submitter"]} }
        );
        if clinvar {
            statement["classification"]["coding"] =
                json!({"system":"acmg_amp_2015","code":"pathogenic"});
            statement["attribution"]["evaluated"] = row["evaluated"].clone();
            statement["allele_origin"] = json!({"label":row["allele_origin"]});
            statement["strength"] = json!({"label":row["review_status"]});
            statement["citations"] = json!([row["method_citation"]]);
        } else {
            statement["attribution"]["version"] = row["version"].clone();
            statement["condition"] = row["condition"].clone();
            statement["criteria"] = row["criteria"].clone();
        }
        let parsed: Statement = serde_json::from_value(statement.clone())?;
        assert_eq!(serde_json::to_value(parsed)?, statement);
        checked += 1;
    }
    assert_eq!(checked, 2);
    Ok(())
}

#[test]
fn population_strata_enforce_counts_and_frequencies() -> Check {
    let attribution = Attribution {
        source: Source::Gnomad.text(),
        record: None,
        version: None,
        release: None,
        submitter: None,
        evaluated: None,
        url: None,
    };
    let base = PopulationFrequency::new(
        VariantRef::allele("CA123643".parse()?),
        Build::GRCh38,
        Text::new("gnomad")?,
        Text::new("synthetic-release")?,
        attribution,
    );
    let value = base
        .clone()
        .with_stratum(
            Some(PopulationDataType::Exomes),
            Some(Text::new("synthetic-group")?),
            Some(PopulationSex::Xy),
            Some(Text::new("synthetic-subset")?),
        )
        .with_counts(Some(2), Some(4), Some(0), Some(0))?
        .with_frequencies(Some(0.5.try_into()?), Some(0.25.try_into()?))?;
    let serialized = serde_json::to_value(&value)?;
    assert_eq!(serialized.get("hemizygote_count"), Some(&json!(0)));
    assert_eq!(serialized.get("allele_frequency"), Some(&json!(0.5)));
    assert_eq!(
        serde_json::from_value::<PopulationFrequency>(serialized.clone())?,
        value
    );
    assert_eq!(
        base.clone()
            .with_counts(Some(3), Some(2), None, None)
            .expect_err("count cannot exceed its denominator")
            .code(),
        "population.count.exceeds_number"
    );
    assert_eq!(
        base.clone()
            .with_frequencies(Some(1.01.try_into()?), None)
            .expect_err("frequency is bounded")
            .code(),
        "population.frequency.range"
    );
    assert!(
        base.with_frequencies(None, Some((-0.01).try_into()?))
            .is_err()
    );
    for (field, invalid) in [
        ("allele_count", json!(5)),
        ("allele_frequency", json!(-0.01)),
        ("faf95", json!(1.01)),
    ] {
        let mut changed = serialized.clone();
        changed
            .as_object_mut()
            .ok_or("population object required")?
            .insert(field.into(), invalid);
        assert!(serde_json::from_value::<PopulationFrequency>(changed).is_err());
    }
    Ok(())
}

#[test]
fn source_predictor_contract_keeps_zero_and_native_labels() {
    let source = json!(
        {"subject":{"level":"allele","hgvs":"NC_000007.13:g.140453136A>T"},"method":"cadd.sift.val","score":0.0,"label":"deleterious",
        "attribution":{"source":"myvariant","record":"chr7:g.140453136A>T"}}
    );
    let score: PredictorScore = serde_json::from_value(source.clone()).expect("valid source score");
    assert_eq!(serde_json::to_value(score).expect("serialize"), source);
    assert!(serde_json::from_value::<PredictorScore>(json!({"score":null})).is_err());
    let empty: VariantConservation = serde_json::from_value(
        json!({"subject_qualification":"adapter_derived_reference","scores":[],"components":[]}),
    )
    .expect("an absent record field has no fabricated score");
    assert!(empty.scores.is_empty());
}

#[test]
fn identifier_lists_qualify_each_namespace_and_roundtrip() -> Result<(), Box<dyn std::error::Error>>
{
    for (published, output) in [
        ("rs113488022", "rs:113488022"),
        ("CA123643", "clingen:CA123643"),
        ("PA123", "clingen:PA123"),
        ("VCV000013961.1", "clinvar:VCV000013961.1"),
        ("clinvar:13961", "clinvar:13961"),
        ("COSV60000001", "cosmic:COSV60000001"),
        (
            "ga4gh:VA.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "ga4gh:VA.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
    ] {
        let id: VariantId = published.parse()?;
        assert_eq!(id.to_string(), published);
        let variant = Variant {
            ids: Box::new([id]),
            representations: Box::default(),
            genes: Box::default(),
        };
        let value = serde_json::to_value(&variant)?;
        assert_eq!(value, json!({"ids":[output]}));
        assert_eq!(serde_json::from_value::<Variant>(value)?, variant);
        // Stored entities with the old published spelling remain readable.
        assert_eq!(
            serde_json::from_value::<Variant>(json!({"ids":[published]}))?,
            variant
        );
    }
    for invalid in ["clingen:rs123", "cosmic:CA123643", "rs:CA123643"] {
        assert!(serde_json::from_value::<Variant>(json!({"ids":[invalid]})).is_err());
    }
    Ok(())
}
