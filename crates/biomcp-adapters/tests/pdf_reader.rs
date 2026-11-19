//! BioMCP's PDF reader on real FDA PDFs and small synthetic ones: text drawn by form XObjects
//! is read (1068), a character map lopdf refuses is read again (1069), and text that still
//! decodes to gibberish is reported as garbled, never returned.
use biomcp_adapters::pdf::{embedded, garbled};
use biomcp_types::PdfTextState;
use std::collections::BTreeMap;

const FORM_OBJECT_CARTON: &[u8] = include_bytes!("fixtures/pdf/form-object-carton.pdf");
const REFUSED_MAP_LETTER: &[u8] = include_bytes!("fixtures/pdf/refused-map-letter.pdf");
const REFUSED_MAP_LETTER_REFERENCE: &str = include_str!("fixtures/pdf/refused-map-letter.txt");
const REFUSED_MAP_LABEL: &[u8] = include_bytes!("fixtures/pdf/refused-map-label-page.pdf");
const REFUSED_MAP_LABEL_REFERENCE: &str = include_str!("fixtures/pdf/refused-map-label-page.txt");
const GARBLED: &[u8] = include_bytes!("fixtures/pdf/garbled.pdf");
const HOSTILE_MAP: &[u8] = include_bytes!("fixtures/pdf/refused-map-hostile.pdf");
const OVER_BUDGET_MAP: &[u8] = include_bytes!("fixtures/pdf/refused-map-over-budget.pdf");

/// Lowercase words of two or more letters, counted.
fn words(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for word in text
        .split(|c: char| !c.is_ascii_alphabetic())
        .filter(|word| word.len() > 1)
    {
        *counts.entry(word.to_ascii_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// The shares of `read`'s words found in `reference`, and of `reference`'s found in `read`.
fn agreement(read: &str, reference: &str) -> (f64, f64) {
    let (read, reference) = (words(read), words(reference));
    let shared: usize = read
        .iter()
        .map(|(word, count)| (*count).min(reference.get(word).copied().unwrap_or(0)))
        .sum();
    let total = |counts: &BTreeMap<String, usize>| counts.values().sum::<usize>().max(1) as f64;
    (
        shared as f64 / total(&read),
        shared as f64 / total(&reference),
    )
}

fn flat(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn read_as_reference(pdf: &[u8], reference: &str, phrases: &[&str]) {
    let read = embedded(pdf, 100, &|| false).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Text,
        "the refused map is read again"
    );
    let (precision, recall) = agreement(&read.text, reference);
    assert!(
        precision >= 0.99 && recall >= 0.99,
        "words agree with the reference reading: {precision:.3} of the read, {recall:.3} of the reference"
    );
    let text = flat(&read.text);
    for phrase in phrases {
        assert!(text.contains(phrase), "{phrase:?} is read: {text}");
    }
}

#[test]
fn a_letter_whose_character_map_lopdf_refuses_reads_as_the_reference() {
    read_as_reference(
        REFUSED_MAP_LETTER,
        REFUSED_MAP_LETTER_REFERENCE,
        &["NDA 208065/S-21", "SUPPLEMENT APPROVAL", "TAGRISSO"],
    );
}

#[test]
fn a_label_page_whose_character_map_lopdf_refuses_reads_as_the_reference() {
    read_as_reference(
        REFUSED_MAP_LABEL,
        REFUSED_MAP_LABEL_REFERENCE,
        &[
            "WARNING: SERIOUS INFECTIONS, MALIGNANCY, AND THROMBOSIS",
            "WARNINGS AND PRECAUTIONS",
            "ADVERSE REACTIONS",
            "Thrombosis, including deep venous thrombosis, pulmonary embolism, and arterial thrombosis",
        ],
    );
}

#[test]
fn glyphs_without_a_character_map_are_garbled_and_not_returned() {
    let read = embedded(GARBLED, 100, &|| false).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Garbled,
        "nothing names the glyphs"
    );
    assert!(read.text.is_empty(), "gibberish is never returned as text");
    assert_eq!(read.pages, Some(1), "the PDF still opened");
}

#[test]
fn a_hostile_refused_map_is_read_within_its_budget_and_refused_beyond_it() {
    let read = embedded(HOSTILE_MAP, 100, &|| false).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Text,
        "a huge range and many sections still read"
    );
    assert!(
        flat(&read.text).contains("There is a risk of serious infections"),
        "{}",
        read.text
    );
    let read = embedded(OVER_BUDGET_MAP, 100, &|| false).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Garbled,
        "a map past the budget is not rebuilt"
    );
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
