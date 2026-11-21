//! Numbers read from saved FDA PDF pages keep the boundaries shown on the page.
use biomcp_adapters::pdf::embedded;
use biomcp_types::PdfTextState;

fn read(bytes: &[u8]) -> String {
    let read = embedded(bytes, 1, &|| false).expect("FDA page opens");
    assert_eq!(read.state, PdfTextState::Text, "FDA page is readable");
    read.text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn raised_table_notes_stay_separate_from_week_numbers() {
    let text = read(include_bytes!("fixtures/pdf/numbers-rinvoq.pdf"));
    for phrase in ["Week 26 2", "Week 24 4", "0.78 (0.1)", "-0.85, -0.20"] {
        assert!(text.contains(phrase), "{phrase:?} in {text:?}");
    }
    for joined in ["Week 262", "Week 244"] {
        assert!(!text.contains(joined), "{joined:?} in {text:?}");
    }
}

#[test]
fn decimal_glyphs_drawn_in_two_passes_read_as_one_number() {
    let text = read(include_bytes!("fixtures/pdf/numbers-lipitor.pdf"));
    for phrase in ["p=0.0005", "p=0.01"] {
        assert!(text.contains(phrase), "{phrase:?} in {text:?}");
    }
    assert!(!text.contains("0 0005"), "split decimal in {text:?}");
}

#[test]
fn a_phone_number_wrapped_at_its_hyphen_reads_whole() {
    let text = read(include_bytes!("fixtures/pdf/numbers-fda-letter.pdf"));
    assert!(text.contains("(301) 796-5073."), "phone number in {text:?}");
}
