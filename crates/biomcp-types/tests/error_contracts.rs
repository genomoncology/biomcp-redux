//! Rejected library inputs never echo caller text.
#[test]
fn unknown_wire_enum_value_is_private() {
    let input = "synthetic-private-enum-value";
    let error = serde_json::from_value::<biodata::TrialSiteStatus>(serde_json::json!(input))
        .expect_err("unknown enum value must be refused");
    assert!(!error.to_string().contains(input), "decode errors must omit input");
}

#[test]
fn unrelated_value_failures_have_distinct_codes() {
    assert_ne!("future-license".parse::<biodata::License>().unwrap_err().to_string(), "source.unknown");
    assert_ne!("future-match".parse::<biomcp_types::ArticleMatchKind>().unwrap_err().to_string(), "source.unknown");
    assert_ne!(biodata::Bound::new(None, None).unwrap_err().to_string(), "coord.span.order");
}
