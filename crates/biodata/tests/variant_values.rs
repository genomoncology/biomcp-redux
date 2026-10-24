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
        "rs113488022",
        "CA123643",
        "CA003404",
        "CA000434",
        "clinvar:13961",
        "clinvar.allele:15041",
        "VCV000013961.98",
        "RCV000014992",
        "SCV000001234.1",
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
    assert_eq!(statement.attribution.source, Source::Civic);
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
        let mut statement = json!({
            "subject":{"level":"allele","id":subject},
            "proposition":"pathogenicity",
            "classification":{"label":row["classification"]},
            "attribution":{"source":if clinvar {"clinvar"} else {"erepo"},"record":row["record"],"submitter":row["submitter"]}
        });
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
