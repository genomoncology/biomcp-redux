//! BioMCP's PDF reader on real FDA PDFs: text drawn by form XObjects is read, and text that
//! decodes to gibberish is reported as garbled, never returned (1068).
use biomcp_adapters::pdf::embedded;
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
