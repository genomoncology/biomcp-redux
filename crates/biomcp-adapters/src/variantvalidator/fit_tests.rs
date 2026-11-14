//! Transcript order for the grouped fit query. The accessions and MANE Plus Clinical flags
//! are VariantValidator's gene2transcripts_v2 answers for CAMK2D and SCN5A on GRCh38
//! (checked 2026-11-13); the other annotation fields are left out.
use super::{Genes, gene};
use crate::mapped::Problems;
use biomcp_types::Source;

type Checked<T> = Result<T, Box<dyn std::error::Error>>;

fn ordered(symbol: &str, transcripts: &[(&str, bool)], input: &str) -> Checked<Vec<String>> {
    let rows: Vec<serde_json::Value> = transcripts
        .iter()
        .map(|(reference, plus)| {
            serde_json::json!({
                "reference": reference,
                "annotations": {"mane_plus_clinical": plus},
            })
        })
        .collect();
    let reply = serde_json::json!([{"current_symbol": symbol, "transcripts": rows}]);
    let genes: Genes = serde_json::from_value(reply)?;
    let mut problems = Problems {
        source: Source::VariantValidator,
        record: None,
        list: Vec::new(),
    };
    let (_, list) = gene(genes, input, &mut problems).ok_or("no other transcripts")?;
    assert!(problems.list.is_empty());
    Ok(list)
}

#[test]
fn older_short_accessions_fall_in_the_first_group() -> Checked<()> {
    let camk2d = [
        "NM_001321573.2",
        "NM_001321566.2",
        "NM_001321587.2",
        "NM_001396965.1",
        "NM_001399861.1",
        "NM_001399865.1",
        "NM_001221.4",
        "NM_001399855.1",
        "NM_001399860.1",
        "NM_001321567.2",
        "NM_001321589.2",
        "NM_001321584.2",
        "NM_001321579.2",
        "NM_001396966.1",
        "NM_001399857.1",
        "NM_001321581.2",
        "NM_001396967.1",
        "NM_001321585.2",
        "NM_001321580.2",
        "NM_001399863.1",
        "NM_001321574.2",
        "NM_001321582.2",
        "NM_001321583.2",
        "NM_001321578.2",
        "NM_001399864.1",
        "NM_172128.3",
        "NM_172129.2",
        "NM_001321586.2",
        "NM_172114.2",
        "NM_001399859.1",
        "NM_001321576.2",
        "NM_001321571.2",
        "NM_001321577.2",
        "NM_172115.3",
        "NM_001321590.2",
        "NM_001321592.2",
        "NM_172127.3",
        "NM_001399856.1",
        "NM_001399858.1",
        "NM_001321575.2",
        "NM_001321569.2",
        "NM_001321572.2",
        "NM_001321568.2",
        "NM_001321591.2",
        "NM_001321588.2",
        "NM_001321570.2",
        "NM_001396964.1",
        "NM_001399862.1",
    ]
    .map(|reference| (reference, false));
    let list = ordered("CAMK2D", &camk2d, "NM_001321571.2")?;
    assert_eq!(list.len(), 47);
    assert_eq!(
        list.iter().take(6).map(String::as_str).collect::<Vec<_>>(),
        [
            "NM_001221.4",
            "NM_172114.2",
            "NM_172115.3",
            "NM_172127.3",
            "NM_172128.3",
            "NM_172129.2"
        ]
    );
    assert!(!list.iter().any(|name| name.starts_with("NM_001321571.")));
    Ok(())
}

#[test]
fn mane_plus_clinical_comes_first() -> Checked<()> {
    let scn5a = [
        ("NM_001099404.2", true),
        ("NR_176299.1", false),
        ("NM_001407186.1", false),
        ("NM_001160160.2", false),
        ("NM_001099405.2", false),
        ("NM_001407187.1", false),
        ("NM_000335.5", false),
        ("NM_198056.3", false),
        ("NM_001160161.2", false),
        ("NM_001407185.1", false),
        ("NM_001354701.2", false),
    ];
    let list = ordered("SCN5A", &scn5a, "NM_000335.5")?;
    assert_eq!(
        list,
        [
            "NM_001099404.2",
            "NM_198056.3",
            "NM_001099405.2",
            "NM_001160160.2",
            "NM_001160161.2",
            "NM_001354701.2",
            "NM_001407185.1",
            "NM_001407186.1",
            "NM_001407187.1"
        ]
    );
    Ok(())
}
