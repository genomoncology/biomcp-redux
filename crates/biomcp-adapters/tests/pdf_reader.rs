//! BioMCP's PDF reader on real FDA PDFs and on small PDFs built here: text drawn by form
//! XObjects is read, hostile forms and maps cannot buy unbounded work, a bad form is skipped,
//! and text decoded without a character map that reads as gibberish is reported as garbled,
//! never returned (1068).
use biomcp_adapters::pdf::{embedded, garbled};
use biomcp_types::PdfTextState;
use lopdf::{Dictionary, Document, Object, Stream, dictionary};

const FORM_OBJECT_CARTON: &[u8] = include_bytes!("fixtures/pdf/form-object-carton.pdf");
const GARBLED_LETTER: &[u8] = include_bytes!("fixtures/pdf/garbled-letter.pdf");
const SENTENCE: &str = "There is a risk of serious infections in patients treated with the drug.";

/// A one-page PDF drawing `content` with the resources `build` adds to the document.
fn pdf(content: &[u8], build: impl FnOnce(&mut Document) -> Dictionary) -> lopdf::Result<Vec<u8>> {
    let mut document = Document::with_version("1.7");
    let pages = document.new_object_id();
    let resources = build(&mut document);
    let contents = document.add_object(Stream::new(Dictionary::new(), content.to_vec()));
    let page = document.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages,
        "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => contents,
        "Resources" => resources,
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
    document.save_to(&mut bytes)?;
    Ok(bytes)
}

/// An Identity-H composite font whose codes are glyph IDs 29 below the Latin character codes,
/// as the FDA labels that misread were, with `map` as its character map when given.
fn composite(document: &mut Document, map: Option<Vec<u8>>) -> Object {
    let descendant = document.add_object(dictionary! {
        "Type" => "Font", "Subtype" => "CIDFontType2", "BaseFont" => "Synthetic", "DW" => 500,
        "CIDSystemInfo" => dictionary! { "Registry" => Object::string_literal("Adobe"),
            "Ordering" => Object::string_literal("Identity"), "Supplement" => 0 },
    });
    let mut font = dictionary! {
        "Type" => "Font", "Subtype" => "Type0", "BaseFont" => "Synthetic",
        "Encoding" => "Identity-H", "DescendantFonts" => vec![descendant.into()],
    };
    if let Some(map) = map {
        font.set(
            "ToUnicode",
            document.add_object(Stream::new(Dictionary::new(), map)),
        );
    }
    document.add_object(font).into()
}

/// Lines of `text` in the composite font, each word its own string so word gaps show.
fn glyph_lines(text: &str, lines: usize) -> Vec<u8> {
    let words: Vec<String> = text
        .split(' ')
        .map(|word| {
            word.chars()
                .map(|c| format!("{:04X}", u32::from(c) - 29))
                .collect()
        })
        .collect();
    let line = format!("[<{}>] TJ T*\n", words.join("> -400 <"));
    format!("BT /F1 9 Tf 11 TL 36 760 Td\n{}ET\n", line.repeat(lines)).into_bytes()
}

fn read(bytes: &[u8]) -> Result<biomcp_adapters::pdf::Embedded, biomcp_types::Reason> {
    embedded(bytes, 100, &|| false)
}

#[test]
fn text_inside_form_objects_is_read() {
    let read = read(FORM_OBJECT_CARTON).expect("readable PDF");
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
    let read = read(GARBLED_LETTER).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Garbled,
        "the letter's fonts decode wrongly"
    );
    assert!(read.text.is_empty(), "gibberish is never returned as text");
    assert_eq!(read.pages, Some(5), "the letter still opened");
}

#[test]
fn glyphs_without_a_character_map_are_garbled() {
    let bytes = pdf(&glyph_lines(SENTENCE, 40), |document| {
        let font = composite(document, None);
        dictionary! { "Font" => dictionary! { "F1" => font } }
    })
    .expect("synthetic PDF");
    let read = read(&bytes).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Garbled,
        "nothing names the glyphs"
    );
    assert!(read.text.is_empty(), "gibberish is never returned as text");
}

#[test]
fn gibberish_beside_readable_text_is_cut_out_and_the_rest_is_partial() {
    let english: String = (0..60)
        .map(|n| format!("({n} {SENTENCE}) Tj T* "))
        .collect();
    let mut content = format!("BT /F2 9 Tf 11 TL 36 760 Td {english}ET\n").into_bytes();
    content.extend(glyph_lines(SENTENCE, 30));
    let bytes = pdf(&content, |document| {
        let glyphs = composite(document, None);
        let font = document.add_object(dictionary! {
            "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica", "Encoding" => "WinAnsiEncoding",
        });
        dictionary! { "Font" => dictionary! { "F1" => glyphs, "F2" => font } }
    })
    .expect("synthetic PDF");
    let read = read(&bytes).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Partial,
        "the gibberish stretch is missing"
    );
    assert!(
        read.text.contains("59 There is a risk"),
        "readable text is kept"
    );
    assert!(
        !read.text.contains("7KHUH"),
        "gibberish is never returned: {}",
        read.text
    );
}

#[test]
fn text_in_a_declared_encoding_is_never_judged_garbled() {
    let table: String = (0..600)
        .map(|n| format!("{n}.{} {}-{:03} ", n % 7, n * 13, n % 997))
        .collect();
    let content = format!("BT /F1 9 Tf 36 760 Td ({table}) Tj ET");
    let bytes = pdf(content.as_bytes(), |document| {
        let font = document.add_object(dictionary! {
            "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica", "Encoding" => "WinAnsiEncoding",
        });
        dictionary! { "Font" => dictionary! { "F1" => font } }
    })
    .expect("synthetic PDF");
    let read = read(&bytes).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Text,
        "a numeric table is text, not gibberish"
    );
}

#[test]
fn a_form_drawn_without_end_and_a_huge_map_cannot_stall_the_reader() {
    // One outer form draws a one-glyph inner form 100,000 times; the inner form's font carries a
    // character map of about 245 KB. Fonts are read once and draws stop at the page's cap.
    let entries: String = (0..15_000u32)
        .map(|code| format!("<{code:04X}> <0041>\n"))
        .collect();
    let map = format!(
        "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapName /Hostile def\n\
         1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n15000 beginbfchar\n{entries}endbfchar\n\
         endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n"
    );
    let bytes = pdf(b"/Outer Do", |document| {
        let font = composite(document, Some(map.into_bytes()));
        let inner = document.add_object(Stream::new(
            dictionary! { "Type" => "XObject", "Subtype" => "Form", "BBox" => vec![0.into(), 0.into(), 1.into(), 1.into()],
                "Resources" => dictionary! { "Font" => dictionary! { "F1" => font } } },
            b"BT /F1 9 Tf <0001> Tj ET".to_vec(),
        ));
        let outer = document.add_object(Stream::new(
            dictionary! { "Type" => "XObject", "Subtype" => "Form", "BBox" => vec![0.into(), 0.into(), 1.into(), 1.into()],
                "Resources" => dictionary! { "XObject" => dictionary! { "Inner" => inner } } },
            b"/Inner Do\n".repeat(100_000),
        ));
        dictionary! { "XObject" => dictionary! { "Outer" => outer } }
    })
    .expect("synthetic PDF");
    let read = read(&bytes).expect("the hostile PDF still opens");
    assert_eq!(
        read.state,
        PdfTextState::Partial,
        "draws past the page's cap are skipped"
    );
}

/// A one-glyph-pair page in a composite font whose `/Encoding` entry is `encoding` and whose
/// character map has one range over 16,777,216 codes mapped from "A".
fn huge_map_pdf(encoding: impl FnOnce(&mut Document) -> Object) -> Vec<u8> {
    let map = "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapName /Huge def\n\
        1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n\
        1 beginbfrange\n<00000000> <00FFFFFF> <0041>\nendbfrange\n\
        endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";
    pdf(b"BT /F1 9 Tf 36 760 Td <00000001> Tj ET", |document| {
        let encoding = encoding(document);
        let descendant = document.add_object(dictionary! {
            "Type" => "Font", "Subtype" => "CIDFontType2", "BaseFont" => "Huge",
            "CIDSystemInfo" => dictionary! { "Registry" => Object::string_literal("Adobe"),
                "Ordering" => Object::string_literal("Identity"), "Supplement" => 0 },
        });
        let map = document.add_object(Stream::new(Dictionary::new(), map.as_bytes().to_vec()));
        let font = document.add_object(dictionary! {
            "Type" => "Font", "Subtype" => "Type0", "BaseFont" => "Huge", "Encoding" => encoding,
            "DescendantFonts" => vec![descendant.into()], "ToUnicode" => map,
        });
        dictionary! { "Font" => dictionary! { "F1" => font } }
    })
    .expect("synthetic PDF")
}

#[test]
fn a_huge_map_is_never_read_however_the_encoding_is_named() {
    // The second review's bypasses: an encoding that refers to /Identity-H, and an encoding
    // dictionary based on it; lopdf reads the map in both, so it must be counted first.
    let by_reference = huge_map_pdf(|document| {
        document
            .add_object(Object::Name(b"Identity-H".to_vec()))
            .into()
    });
    let by_dictionary = huge_map_pdf(|_| dictionary! { "BaseEncoding" => "Identity-H" }.into());
    let direct = huge_map_pdf(|_| Object::Name(b"Identity-H".to_vec()));
    for bytes in [by_reference, by_dictionary, direct] {
        let read = read(&bytes).expect("readable PDF");
        assert!(
            !read.text.contains("AB"),
            "the 16,777,216-code map was not read: {:?}",
            read.text
        );
    }
}

/// A one-page PDF whose resources carry `count` copies of the name `name`.
fn names_pdf(name: &str, count: usize) -> Vec<u8> {
    let names: Vec<Object> = (0..count)
        .map(|_| Object::Name(name.as_bytes().to_vec()))
        .collect();
    pdf(b"BT ET", |_| dictionary! { "Names" => names }).expect("synthetic PDF")
}

#[test]
fn documents_naming_too_many_load_time_streams_are_refused_before_loading() {
    let objects = names_pdf("Obj#53tm", 1025);
    assert_eq!(
        read(&objects).expect("refused, not failed").state,
        PdfTextState::TooLarge,
        "escaped names count"
    );
    let xrefs = names_pdf("XRef", 33);
    assert_eq!(
        read(&xrefs).expect("refused, not failed").state,
        PdfTextState::TooLarge
    );
    let fine = names_pdf("ObjStm", 10);
    assert_eq!(
        read(&fine).expect("readable PDF").state,
        PdfTextState::NoText
    );
}

#[test]
fn a_form_that_cannot_be_read_is_skipped_and_the_page_keeps_its_text() {
    let words = SENTENCE.split(' ').collect::<Vec<_>>().join(") Tj ( ");
    let content = format!("BT /F1 9 Tf 36 760 Td ({words}) Tj ET /Broken Do");
    let bytes = pdf(content.as_bytes(), |document| {
        let font = document.add_object(dictionary! {
            "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica", "Encoding" => "WinAnsiEncoding",
        });
        let broken = document.add_object(Stream::new(
            dictionary! { "Type" => "XObject", "Subtype" => "Form", "Filter" => "BrotliDecode",
            "BBox" => vec![0.into(), 0.into(), 1.into(), 1.into()] },
            b"not brotli data".to_vec(),
        ));
        dictionary! { "Font" => dictionary! { "F1" => font }, "XObject" => dictionary! { "Broken" => broken } }
    })
    .expect("synthetic PDF");
    let read = read(&bytes).expect("readable PDF");
    assert_eq!(
        read.state,
        PdfTextState::Partial,
        "text may be missing where the form was"
    );
    assert!(
        read.text.contains("serious"),
        "the page's own text is kept: {}",
        read.text
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
