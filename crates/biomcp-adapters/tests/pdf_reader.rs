//! BioMCP's PDF reader on real FDA PDFs: text drawn by form XObjects is read, and text that
//! decodes to gibberish is reported as garbled, never returned (1068).
use biomcp_adapters::pdf::{embedded, garbled};
use biomcp_types::PdfTextState;

const FORM_OBJECT_CARTON: &[u8] = include_bytes!("fixtures/pdf/form-object-carton.pdf");
const GARBLED_LETTER: &[u8] = include_bytes!("fixtures/pdf/garbled-letter.pdf");

#[test]
fn text_inside_form_objects_is_read() {
    let read = embedded(FORM_OBJECT_CARTON, 100, &|| false).expect("readable PDF");
    assert_eq!(read.state, PdfTextState::Text, "the carton text is read");
    assert!(
        read.text.contains("Use cutting tool"),
        "form text is present: {}",
        read.text
    );
    assert!(
        read.text.contains("RINVOQ"),
        "form text is present: {}",
        read.text
    );
}

#[test]
fn text_that_decodes_to_gibberish_is_garbled_and_not_returned() {
    let read = embedded(GARBLED_LETTER, 100, &|| false).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Garbled,
        "the letter's font decodes wrongly"
    );
    assert!(read.text.is_empty(), "gibberish is never returned as text");
    assert_eq!(read.pages, Some(5), "the letter still opened");
}

#[test]
fn english_is_never_garbled_and_misdecoded_fonts_are() {
    let english = "There is a risk of serious infections in patients treated with the drug.\n";
    let shifted = "7KHUH LV D ULVN RI VHULRXV LQIHFWLRQV LQ SDWLHQWV WUHDWHG ZLWK WKH GUXJ\n";
    let symbols = "3)(!%-F%(+(,) 6,3( ,3%%+3 # @((>(%S1/TD+3D(( +3>(+() +31F! %-BF!\n";
    let stamp = "This label may not be the latest approved by FDA.\n";
    let numbered =
        |line: &str, n: usize| -> String { (0..n).map(|i| format!("{i} {line}")).collect() };
    assert!(
        !garbled(&numbered(english, 30)),
        "English prose is not garbled"
    );
    assert!(
        garbled(&numbered(shifted, 30)),
        "shifted glyphs are garbled"
    );
    assert!(
        garbled(&numbered(symbols, 60)),
        "punctuation glyphs are garbled"
    );
    let stamped = format!("{}{}", stamp.repeat(30), numbered(shifted, 30));
    assert!(
        garbled(&stamped),
        "a repeated English stamp does not hide gibberish"
    );
    assert!(!garbled(&numbered(shifted, 2)), "too little text to judge");
    assert!(!garbled(""), "no text is not garbled");
}
