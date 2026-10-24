//! Inline markup table: the parser boundary warrants direct syntax edge cases.
use crate::xml::MixedText;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Raw {
    #[serde(rename = "$value", default)]
    text: MixedText,
}
#[test]
fn inline_markup_preserves_variant_text_order_and_whitespace() -> Result<(), quick_xml::DeError> {
    for (xml, expected) in [
        ("<raw>BRAF<sup>V600E</sup></raw>", "BRAFV600E"),
        (
            "<raw> A <sub>2<sup>+</sup></sub> <i>x<b>y</b></i><unknown attr=\"ignored\">z</unknown> </raw>",
            " A 2+ xyz ",
        ),
        ("<raw><i> </i> <b> </b></raw>", "   "),
        (
            "<raw>a&#32;&#x56;&amp;&lt;&gt;&quot;&apos;<![CDATA[  <b>  ]]><!--ignored-->z</raw>",
            "a V&<>\"'  <b>  z",
        ),
        ("<raw attr=\"ignored\"/>", ""),
        ("<raw><unknown/></raw>", ""),
    ] {
        let raw: Raw = quick_xml::de::from_str(xml)?;
        assert_eq!(raw.text.as_str(), expected, "{xml}");
    }
    Ok(())
}
