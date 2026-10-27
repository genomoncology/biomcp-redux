//! Variant identifier lists qualify namespaces while published input grammar stays stable.
use biodata::{Variant, VariantId};
use serde_json::json;

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
    Ok(())
}
