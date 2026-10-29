//! Feature: variant-profile-reference. Source profile values retain identities and relations.
use biodata::{Error, ProfileExpression, ProfileNode, Text, VariantRef};
use serde_json::json;
type Check = Result<(), Box<dyn std::error::Error>>;
fn member(id: u32, feature: &str) -> Result<VariantRef, Error> {
    VariantRef::source(
        format!("civic:{id}").parse()?,
        Text::new(feature)?,
        Box::default(),
    )
}
fn profile(source: &str, members: Vec<VariantRef>) -> Result<VariantRef, Error> {
    VariantRef::profile_expression(
        Text::new("1")?,
        Text::new(source)?,
        members.into(),
        ProfileExpression::civic(Text::new(source)?)?,
    )
}
#[test]
fn native_fusion_profile_does_not_require_an_allele() -> Check {
    let value = profile("#VID1", vec![member(1, "BCR::ABL1")?])?;
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
    let value = profile(
        source,
        vec![member(1, "A")?, member(2, "B")?, member(3, "C")?],
    )?;
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
fn profile_refuses_missing_members_and_invalid_relations() -> Check {
    assert_eq!(
        profile("#VID1 AND #VID2", vec![member(1, "A")?]),
        Err(Error::ProfileMemberMissing)
    );
    assert_eq!(
        profile("#VID1", vec![member(1, "A")?, member(1, "A")?]),
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
            ProfileExpression::civic(Text::new(source)?).is_err(),
            "{source}"
        );
    }
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

#[test]
fn profile_keeps_additional_source_members_and_native_feature_identity() -> Check {
    let source = VariantRef::source_with_feature(
        "civic:1".parse()?,
        Text::new("exact feature")?,
        Some("civic:42".parse()?),
        Box::default(),
    )?;
    let value = profile("#VID1", vec![source, member(2, "other feature")?])?;
    let members = value.members().collect::<Vec<_>>();
    assert_eq!(members.len(), 2);
    assert_eq!(
        members[0].feature_record().map(|record| record.as_str()),
        Some("civic:42")
    );
    assert_eq!(
        serde_json::to_value(value)?["members"][0]["feature_record"],
        "civic:42"
    );
    Ok(())
}

#[test]
fn wire_expression_requires_authoritative_machine_source() {
    for source in ["unrelated label", "A AND B", "VID1", "civic:1"] {
        let value = json!({"source":source,"tree":{"operator":"member","record":"civic:1"}});
        assert!(
            serde_json::from_value::<ProfileExpression>(value).is_err(),
            "{source}"
        );
    }
}

#[test]
fn expression_depth_and_size_limits_match_on_construction_and_roundtrip() -> Check {
    for (depth, tail, valid) in [
        (64, "#VID1", true),
        (63, "NOT #VID1", true),
        (64, "NOT #VID1", false),
        (65, "#VID1", false),
    ] {
        let source = format!(
            "{}{}{}",
            "#VID1 AND (".repeat(depth),
            tail,
            ")".repeat(depth)
        );
        let expression = ProfileExpression::civic(Text::new(&source)?);
        assert_eq!(expression.is_ok(), valid, "depth={depth}, tail={tail}");
        if let Ok(expression) = expression {
            assert_eq!(
                serde_json::from_value::<ProfileExpression>(serde_json::to_value(&expression)?)?,
                expression
            );
        }
    }
    for (members, valid) in [(4096, true), (4097, false)] {
        let source = vec!["#VID1"; members].join(" AND ");
        let expression = ProfileExpression::civic(Text::new(&source)?);
        assert_eq!(expression.is_ok(), valid, "members={members}");
        if let Ok(expression) = expression {
            assert_eq!(
                serde_json::from_value::<ProfileExpression>(serde_json::to_value(&expression)?)?,
                expression
            );
        }
    }
    Ok(())
}

#[test]
fn profile_refuses_nested_native_haplotypes() -> Check {
    let compound = VariantRef::clinvar_haplotype(biodata::ClinvarHaplotype::new(
        "clinvar:441268".parse()?,
        "VCV000441268".parse()?,
        Text::new("5")?,
        Text::new("1")?,
        vec![biodata::ClinvarHaplotypeMember {
            id: "clinvar:17864".parse()?,
            allele_id: "32903".parse()?,
        }]
        .into(),
    )?)?;
    assert_eq!(
        VariantRef::profile(
            Text::new("profile")?,
            Text::new("native profile")?,
            vec![compound.clone()].into()
        ),
        Err(Error::VariantRefInvalid)
    );
    assert!(
        serde_json::from_value::<VariantRef>(json!({
            "level":"profile", "record":"profile", "label":"native profile", "members":[compound]
        }))
        .is_err()
    );
    Ok(())
}
