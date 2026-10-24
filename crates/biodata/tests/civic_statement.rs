//! Shared public CIViC fixture exercises the consumer's checked statement contract.
use biodata::{AmpAscoCap2017LevelCode, AmpAscoCap2017TierCode, Coding, Source, Statement};
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;
#[test]
fn public_civic_tier_and_level_remain_separate() -> Check {
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
