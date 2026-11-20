//! Character maps written here: every way BioMCP's code count and lopdf's map parser could
//! tokenise a map differently either counts the codes lopdf would read or refuses the map,
//! never skips part of it (1069 review), and codes a map leaves unmapped mark the read partial.
use biomcp_adapters::pdf::embedded;
use biomcp_types::{PdfMissing, PdfTextState};
use lopdf::{Dictionary, Document, Object, Stream, dictionary};

const SENTENCE: &str = "There is a risk of serious infections in patients treated with the drug.";
/// A header lopdf accepts.
const HEADER: &str = "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n\
    /CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n\
    /CMapName /Synthetic def\n";
const FOOTER: &str = "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";
/// The range that names the sentence's glyphs: codes 29 below the Latin characters.
const GLYPHS: &str = "<0003> <005D> <0020>";
/// A range over 196,608 three-byte codes, past the 131,072 one map may cover.
const HIDDEN: &str = "<000000> <02FFFF> <0041>";
/// A header lopdf refuses (`def` inside `/CIDSystemInfo`), whose maps are rebuilt.
const REFUSED_HEADER: &str = "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n\
    /CIDSystemInfo <<\n/Registry (Synthetic) def\n/Ordering (T1UV) def\n/Supplement 0 def\n>> def\n\
    /CMapName /Synthetic def\n";

/// A one-page PDF of the sentence in an Identity-H font whose character map is `map`.
fn glyph_pdf(map: &str) -> Vec<u8> {
    let words: Vec<String> = SENTENCE
        .split(' ')
        .map(|word| {
            word.chars()
                .map(|c| format!("{:04X}", u32::from(c) - 29))
                .collect()
        })
        .collect();
    let line = format!("[<{}>] TJ T*\n", words.join("> -400 <"));
    let content = format!("BT /F1 9 Tf 11 TL 36 760 Td\n{}ET\n", line.repeat(40));
    let mut document = Document::with_version("1.7");
    let pages = document.new_object_id();
    let descendant = document.add_object(dictionary! {
        "Type" => "Font", "Subtype" => "CIDFontType2", "BaseFont" => "Synthetic", "DW" => 500,
        "CIDSystemInfo" => dictionary! { "Registry" => Object::string_literal("Adobe"),
            "Ordering" => Object::string_literal("Identity"), "Supplement" => 0 },
    });
    let map = document.add_object(Stream::new(Dictionary::new(), map.as_bytes().to_vec()));
    let font = document.add_object(dictionary! {
        "Type" => "Font", "Subtype" => "Type0", "BaseFont" => "Synthetic",
        "Encoding" => "Identity-H", "DescendantFonts" => vec![descendant.into()], "ToUnicode" => map,
    });
    let contents = document.add_object(Stream::new(Dictionary::new(), content.into_bytes()));
    let page = document.add_object(dictionary! {
        "Type" => "Page", "Parent" => pages, "Contents" => contents,
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Resources" => dictionary! { "Font" => dictionary! { "F1" => font } },
    });
    document.objects.insert(
        pages,
        Object::Dictionary(
            dictionary! { "Type" => "Pages", "Kids" => vec![page.into()], "Count" => 1 },
        ),
    );
    let catalog = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    document.trailer.set("Root", catalog);
    let mut bytes = Vec::new();
    document.save_to(&mut bytes).expect("synthetic PDF");
    bytes
}

fn state(header: &str, body: &str) -> PdfTextState {
    let bytes = glyph_pdf(&format!("{header}{body}{FOOTER}"));
    embedded(&bytes, 100, &|| false)
        .expect("readable PDF")
        .state
}

/// The map is not read: the font decodes without it, as glyph numbers, so the read is garbled.
fn refused(header: &str, body: &str, why: &str) {
    assert_eq!(state(header, body), PdfTextState::Garbled, "{why}");
}

#[test]
fn a_comment_cannot_hide_a_range_from_the_count() {
    // The review's map: a comment in the code-space section names a range section and opens
    // a `[` that never closes, so the count skipped the real range section; lopdf read it.
    let body = format!(
        "1 begincodespacerange\n<0000> <FFFF>\n%1 beginbfrange <00> <01> [\nendcodespacerange\n\
         2 beginbfrange\n{GLYPHS}\n{HIDDEN}\nendbfrange\n"
    );
    refused(HEADER, &body, "the commented map's hidden range is counted");
    refused(
        REFUSED_HEADER,
        &body,
        "and a rebuilt copy is counted the same",
    );
}

#[test]
fn comments_between_entries_leave_a_bounded_map_readable() {
    let body = format!(
        "% leading comment ( [ <\n1 begincodespacerange % ]\n<0000> <FFFF> % >>\nendcodespacerange\n\
         1 beginbfrange % count\n{GLYPHS} % [ unclosed\nendbfrange\n"
    );
    assert_eq!(
        state(HEADER, &body),
        PdfTextState::Text,
        "comments are dropped, not misread"
    );
    assert_eq!(
        state(REFUSED_HEADER, &body),
        PdfTextState::Text,
        "the copy drops them too"
    );
}

#[test]
fn strings_holding_brackets_or_percent_signs_are_read_as_strings() {
    let header = HEADER.replace("(Adobe)", "(Ad[o%be\\) (x)");
    let body = format!("1 beginbfrange\n{GLYPHS}\n{HIDDEN}\nendbfrange\n");
    refused(&header, &body, "a string cannot hide the range after it");
    let unclosed = HEADER.replace("(Adobe)", "(Adobe");
    refused(
        &unclosed,
        &format!("1 beginbfrange\n{GLYPHS}\nendbfrange\n"),
        "an unclosed string",
    );
}

#[test]
fn hexadecimal_strings_the_count_cannot_read_with_certainty_refuse_the_map() {
    for (body, why) in [
        (
            "<0003> <005D> <0020 % comment\n>",
            "a comment inside a hex string",
        ),
        ("<0003> <005D> <00G0>", "a character that is not hex"),
        ("<0003> <005D <0020>", "a hex string that does not close"),
        ("<0003> <005D> [[<0020>]]", "a nested array"),
        ("<0003> <005D> [<0020> /Name]", "a name inside an array"),
        ("<0003> <005D> /Name", "a name in a section"),
        ("<0003> <005D> (string)", "a string in a section"),
    ] {
        refused(
            HEADER,
            &format!("1 beginbfrange\n{body}\nendbfrange\n"),
            why,
        );
    }
}

#[test]
fn sections_that_do_not_close_or_disagree_with_their_count_are_never_skipped() {
    refused(
        HEADER,
        &format!("1 beginbfrange\n{GLYPHS}\nendbf\nrange\n"),
        "an end split across lines",
    );
    refused(
        HEADER,
        &format!("1 beginbfrange\n{GLYPHS}\n"),
        "a section with no end",
    );
    refused(
        HEADER,
        &format!("1 beginbfrange\n{GLYPHS}\n{HIDDEN}\nendbfrange\n"),
        "a count of 1 over two entries",
    );
    refused(
        HEADER,
        &format!("endbfchar\n1 beginbfrange\n{GLYPHS}\n{HIDDEN}\nendbfrange\n"),
        "a stray end",
    );
    let used = format!("/Identity-H usecmap\n1 beginbfrange\n{GLYPHS}\n{HIDDEN}\nendbfrange\n");
    refused(
        HEADER,
        &used,
        "usecmap, which lopdf does not read, still counts the ranges",
    );
    let odd = format!("1 beginbfrange\n{GLYPHS}\n<00000> <2FFFF> <0041>\nendbfrange\n");
    refused(HEADER, &odd, "an odd-length code is counted at its value");
}

#[test]
fn codes_a_map_leaves_unmapped_mark_the_read_partial() {
    // Only "T", "h" and "e" are mapped; lopdf gives U+FFFD for every other glyph.
    let body = "1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n\
        3 beginbfchar\n<0037> <0054>\n<004B> <0068>\n<0048> <0065>\nendbfchar\n";
    let bytes = glyph_pdf(&format!("{HEADER}{body}{FOOTER}"));
    let read = embedded(&bytes, 100, &|| false).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Partial,
        "unmapped glyphs are missing text"
    );
    assert_eq!(read.missing, Some(PdfMissing::Skipped));
}
