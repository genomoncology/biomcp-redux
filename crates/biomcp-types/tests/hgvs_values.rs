//! HGVS boundary edge cases protect syntax checks without a molecular parser.
use biodata::{Expression, ExpressionSyntax, HgvsText, Text};

type Check = Result<(), Box<dyn std::error::Error>>;

#[test]
fn hgvs_reference_and_syntax_checks_preserve_positionless_forms() -> Check {
    // HGVS 21.1: recommendations/uncertain, RNA/splicing and background/refseq.
    for (text, valid) in [
        ("NM_005228.5:c.???", false),
        ("NM_005228.5:c.nonsense", false),
        ("NM_005228.5:c.2369C>T word", false),
        ("NM_005228.5:c.2369 C>T", false),
        ("NM_005228.5:c.2369C>T\t", false),
        ("NP_005219.2:c.2369C>T", false),
        ("XP_005219.2:r.2369c>u", false),
        ("ENSP00000288602.1:g.100A>T", false),
        ("LRG_199p1:n.100A>T", false),
        ("NC_000007.14(NP_005219.2):c.2369C>T", false),
        ("NP_005219.2:p.???", false),
        ("NP_005219.2:p.?", true),
        ("NP_005219.2:p.0", true),
        ("NP_005219.2:p.0?", true),
        ("NP_005219.2:p.=", true),
        ("NP_005219.2:p.(=)", true),
        ("NP_005219.2:p.(?)", true),
        ("NP_005219.2:p.(Thr790Met)", true),
        ("NM_005228.5:r.?", true),
        ("NM_005228.5:r.(?)", true),
        ("NM_005228.5:r.=", true),
        ("NM_005228.5:r.(=)", true),
        ("NM_005228.5:r.spl", true),
        ("NM_005228.5:r.spl?", true),
        ("NM_005228.5:r.(spl?)", true),
        ("NM_005228.5:c.2369C>T", true),
    ] {
        assert_eq!(text.parse::<HgvsText>().is_ok(), valid, "{text}");
        assert_eq!(Expression::hgvs(Text::new(text)?).is_ok(), valid, "{text}");
    }
    for text in ["chr7:g.???", "chr7:g.100A>T word"] {
        assert!(Expression::new(ExpressionSyntax::HgvsG, Text::new(text)?, None).is_err(), "{text}");
    }
    Ok(())
}
