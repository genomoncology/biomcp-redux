//! Inline markup table: the parser boundary warrants direct syntax edge cases.
use crate::xml::MixedText;
#[test]
fn inline_markup_preserves_variant_text_order_and_whitespace() -> Result<(), crate::BadReply> {
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
        let text = MixedText::from_xml(xml)?;
        assert_eq!(text.as_str(), expected, "{xml}");
    }
    let deep = format!("<raw>{}text{}</raw>", "<i>".repeat(64), "</i>".repeat(64));
    assert_eq!(MixedText::from_xml(&deep), Err(crate::BadReply));
    Ok(())
}
