//! Feature: variant-profile-reference. Source profile values retain identities and relations.
use biodata::{Error, ProfileExpression, ProfileNode, Text, VariantRef};
use serde_json::json;
type Check = Result<(), Box<dyn std::error::Error>>;
fn member(id: u32, feature: &str) -> VariantRef {
    VariantRef::source(
        format!("civic:{id}").parse().unwrap(),
        Text::new(feature).unwrap(),
        Box::default(),
    )
    .unwrap()
}
fn profile(source: &str, members: Vec<VariantRef>) -> Result<VariantRef, Error> {
    VariantRef::profile_expression(
        Text::new("1").unwrap(),
        Text::new(source).unwrap(),
        members.into(),
        ProfileExpression::civic(Text::new(source).unwrap())?,
    )
}
#[test]
fn native_fusion_profile_does_not_require_an_allele() -> Check {
    let value = profile("#VID1", vec![member(1, "BCR::ABL1")])?;
    let json = serde_json::to_value(&value)?;
    assert_eq!(
        json["members"][0],
        json!({"level":"source","record":"civic:1","feature":"BCR::ABL1"})
    );
    assert_eq!(json["expression"]["source"], "#VID1");
    assert!(value.members().all(|member| member.id().is_none()));
    assert_eq!(serde_json::from_value::<VariantRef>(json)?, value);
    Ok(())
}
#[test]
fn boolean_tree_retains_grouping_negation_and_exact_source_text() -> Check {
    let source = "#VID1 AND ( #VID2 OR NOT #VID3 )";
    let value = profile(source, vec![member(1, "A"), member(2, "B"), member(3, "C")])?;
    let expression = value.expression().unwrap();
    assert_eq!(expression.source().as_str(), source);
    let ProfileNode::And { operands } = expression.tree() else {
        panic!("AND root")
    };
    assert!(matches!(operands[1], ProfileNode::Or { .. }));
    assert_eq!(
        serde_json::to_value(expression)?["tree"]["operands"][1]["operands"][1]["operator"],
        "not"
    );
    Ok(())
}
#[test]
fn profile_refuses_missing_members_and_invalid_relations() {
    assert_eq!(
        profile("#VID1 AND #VID2", vec![member(1, "A")]),
        Err(Error::ProfileMemberMissing)
    );
    assert_eq!(
        profile("#VID1", vec![member(1, "A"), member(1, "A")]),
        Err(Error::ProfileMemberDuplicate)
    );
    for source in [
        "#VID1 AND #VID2 OR #VID3",
        "NOT (#VID1)",
        "#VID1 AND",
        "(#VID1",
        "#VID0",
        "#VID01",
        "#VID1 tail",
    ] {
        assert!(
            ProfileExpression::civic(Text::new(source).unwrap()).is_err(),
            "{source}"
        );
    }
}
#[test]
fn display_parser_matches_complete_source_labels_before_boolean_syntax() -> Check {
    let labels = vec![
        ("civic:1".parse()?, Text::new("BCR::ABL1 Fusion")?),
        ("civic:2".parse()?, Text::new("EGFR p.(L858R)")?),
    ];
    let expression = ProfileExpression::civic_display(
        Text::new("BCR::ABL1 Fusion AND NOT EGFR p.(L858R)")?,
        &labels,
    )?;
    assert!(matches!(expression.tree(), ProfileNode::And { .. }));
    assert_eq!(
        ProfileExpression::civic_display(Text::new("missing label")?, &labels),
        Err(Error::ProfileMemberMissing)
    );
    let ambiguous = vec![
        ("civic:1".parse()?, Text::new("A")?),
        ("civic:2".parse()?, Text::new("A")?),
    ];
    assert_eq!(
        ProfileExpression::civic_display(Text::new("A")?, &ambiguous),
        Err(Error::ProfileExpressionInvalid)
    );
    Ok(())
}
#[test]
fn single_variant_json_keeps_its_existing_form() -> Check {
    let value = VariantRef::allele("VCV000013961.3".parse()?);
    assert_eq!(
        serde_json::to_value(value)?,
        json!({"level":"allele","id":"VCV000013961.3"})
    );
    Ok(())
}

#[test]
fn source_member_preserves_only_source_stated_leaf_forms() -> Check {
    let allele = VariantRef::allele("VCV000013961.3".parse()?);
    let source = VariantRef::source(
        "civic:1".parse()?,
        Text::new("feature")?,
        vec![allele.clone()].into(),
    )?;
    assert_eq!(source.forms().collect::<Vec<_>>(), vec![&allele]);
    assert!(source.id().is_none());
    assert!(
        VariantRef::source(
            "civic:2".parse()?,
            Text::new("feature")?,
            vec![source].into()
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn wire_expression_cannot_contradict_its_machine_source() {
    let value = json!({"source":"#VID1", "tree":{"operator":"member","record":"civic:2"}});
    assert!(serde_json::from_value::<ProfileExpression>(value).is_err());
}
