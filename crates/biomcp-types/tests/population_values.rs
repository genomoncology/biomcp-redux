//! Published population value contracts.
use biodata::*;
use biomcp_types::Source;
use serde_json::json;
type Check = Result<(), Box<dyn std::error::Error>>;
fn population_base() -> Result<PopulationFrequency, Box<dyn std::error::Error>> {
    let attribution = Attribution {
        source: Source::Gnomad.text(),
        record: None,
        version: None,
        release: None,
        submitter: None,
        evaluated: None,
        url: None,
    };
    Ok(PopulationFrequency::new(
        VariantRef::allele("CA123643".parse()?),
        Build::GRCh38,
        Text::new("gnomad")?,
        Text::new("synthetic-release")?,
        attribution,
    ))
}

#[test]
fn population_strata_enforce_counts_and_frequencies() -> Check {
    let base = population_base()?;
    let value = base
        .clone()
        .with_stratum(
            Some(PopulationDataType::Exomes),
            Some(Text::new("synthetic-group")?),
            Some(PopulationSex::Xy),
            Some(Text::new("synthetic-subset")?),
        )?
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
    for invalid in [f64::NAN, f64::INFINITY, -0.01, 1.5] {
        assert_eq!(
            Fraction::try_from(invalid)
                .expect_err("fraction is bounded")
                .code(),
            "population.frequency.range"
        );
    }
    for (field, invalid) in [
        ("allele_count", json!(5)),
        ("allele_frequency", json!(-0.01)),
        ("faf95", json!(1.5)),
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
fn population_union_preserves_published_fields_and_refuses_misplacement() -> Check {
    let mut union = serde_json::to_value(population_base()?)?;
    for (field, source_value) in [
        ("heterozygote_count", json!(2)),
        ("grpmax_group", json!("synthetic-group")),
        ("genotype_filter", json!("raw")),
        ("homoplasmic_count", json!(1)),
        ("heteroplasmic_count", json!(3)),
        ("homoplasmic_frequency", json!(0.25)),
        ("heteroplasmic_frequency", json!(0.75)),
    ] {
        union[field] = source_value;
    }
    assert_eq!(
        serde_json::to_value(serde_json::from_value::<PopulationFrequency>(
            union.clone()
        )?)?,
        union
    );
    for (field, bad) in [
        ("genotype_filter", json!("unknown")),
        ("ancestry_group", json!("synthetic")),
        ("homoplasmic_frequency", json!(1.5)),
        ("heteroplasmic_frequency", json!(-0.1)),
    ] {
        let mut invalid = union.clone();
        invalid[field] = bad;
        assert!(
            serde_json::from_value::<PopulationFrequency>(invalid).is_err(),
            "{field}"
        );
    }
    union["subject"] = json!({"level":"allele", "hgvs":"NC_012920.1:m.1A>T"});
    assert!(serde_json::from_value::<PopulationFrequency>(union.clone()).is_ok());
    union["subject"] = json!({"level":"allele", "hgvs":"NC_000007.14:g.1A>T"});
    assert!(serde_json::from_value::<PopulationFrequency>(union).is_err());
    Ok(())
}
