//! BioMCP's PDF reader on small PDFs written byte by byte: before lopdf loads a document, every
//! object its tables list is measured, so a table or object stream that lists one object many
//! times, a document holding more than lopdf may build, or a damaged file is refused, and a
//! damaged file is reported as damaged (1068).
use biomcp_adapters::pdf::embedded;
use biomcp_types::{PdfTextState, Reason};
use lopdf::{Stream, dictionary};

const SENTENCE: &str = "There is a risk of serious infections in patients treated with the drug.";

fn read(bytes: &[u8]) -> Result<biomcp_adapters::pdf::Embedded, Reason> {
    embedded(bytes, 100, &|| false)
}

/// The bodies of a one-page PDF that draws [`SENTENCE`] in Helvetica, objects 1 to 5.
fn page_bodies() -> Vec<Vec<u8>> {
    let content = format!("BT /F1 9 Tf 72 700 Td ({SENTENCE}) Tj ET");
    [
        "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_owned(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R \
         /Resources << /Font << /F1 5 0 R >> >> >>"
            .to_owned(),
        format!(
            "<< /Length {} >>\nstream\n{content}\nendstream",
            content.len()
        ),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"
            .to_owned(),
    ]
    .map(String::into_bytes)
    .to_vec()
}

/// A PDF written byte by byte, so its table can list what lopdf's writer never would. Object
/// `n` is `bodies[n - 1]`; `listed` maps the file so far and each object's offset to the
/// table's entries, object number and offset.
fn raw_pdf(
    bodies: &[Vec<u8>],
    listed: impl FnOnce(&[u8], &[usize]) -> Vec<(usize, usize)>,
) -> Vec<u8> {
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (index, body) in bodies.iter().enumerate() {
        offsets.push(out.len());
        out.extend(format!("{} 0 obj\n", index + 1).bytes());
        out.extend(body);
        out.extend(b"\nendobj\n");
    }
    let entries: std::collections::BTreeMap<usize, usize> =
        listed(&out, &offsets).into_iter().collect();
    let size = entries.keys().max().map_or(1, |number| number + 1);
    let xref = out.len();
    out.extend(format!("xref\n0 {size}\n").bytes());
    for number in 0..size {
        let entry = entries.get(&number).map_or_else(
            || "0000000000 65535 f\r\n".to_owned(),
            |offset| format!("{offset:010} 00000 n\r\n"),
        );
        out.extend(entry.bytes());
    }
    out.extend(
        format!("trailer\n<< /Size {size} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").bytes(),
    );
    out
}

/// Whether the reader refuses `bytes` as a damaged file, read but not too large or garbled.
fn damaged(bytes: &[u8]) -> bool {
    read(bytes).is_ok_and(|read| read.state == PdfTextState::Damaged)
}

/// Each object listed once, at its own offset.
fn as_written(_: &[u8], offsets: &[usize]) -> Vec<(usize, usize)> {
    offsets
        .iter()
        .enumerate()
        .map(|(index, offset)| (index + 1, *offset))
        .collect()
}

#[test]
fn a_table_listing_one_object_many_times_is_refused_before_loading() {
    // lopdf parses every entry at its offset. 200 entries at one 4 MB array took 17 s and
    // 50 GB; here 1,000 entries at a 200 KB array would build 100 million objects.
    let mut bodies = page_bodies();
    bodies.push(format!("[{}]", "0 ".repeat(100_000)).into_bytes());
    let fine = read(&raw_pdf(&bodies, as_written)).expect("readable PDF");
    assert_eq!(fine.state, PdfTextState::Text, "the PDF built here reads");
    let repeated = raw_pdf(&bodies, |_, offsets| {
        let mut listed = as_written(&[], offsets);
        listed.extend((7..1007).map(|number| (number, offsets[5])));
        listed
    });
    assert!(damaged(&repeated), "refused as damaged");
    // An entry inside another object's string parses that object's rest again.
    bodies.push(format!("(7 0 obj [{}])", "0 ".repeat(100_000)).into_bytes());
    let inside = raw_pdf(&bodies, |file, offsets| {
        let mut listed = as_written(&[], offsets);
        let nested = file
            .windows(8)
            .rposition(|w| w == b"(7 0 obj")
            .expect("nested header");
        listed.push((8, nested + 1));
        listed
    });
    assert!(damaged(&inside));
}

/// An object stream, object 6, holding a 200 KB array that its index lists `times` times.
fn object_stream_pdf(times: usize, kind: &str) -> Vec<u8> {
    let index: String = (10..10 + times)
        .map(|number| format!("{number} 0 "))
        .collect();
    let content = format!("{index}[{}]", "0 ".repeat(100_000));
    let mut bodies = page_bodies();
    bodies.push(
        format!(
            "<< /Type /{kind} /N {times} /First {} /Length {} >>\nstream\n{content}\nendstream",
            index.len(),
            content.len()
        )
        .into_bytes(),
    );
    raw_pdf(&bodies, as_written)
}

#[test]
fn an_object_stream_listing_one_object_many_times_is_refused_before_loading() {
    // 100 listings of one array in an object stream took 3 GB from 1,271 bytes.
    let fine = read(&object_stream_pdf(1, "ObjStm")).expect("readable PDF");
    assert_eq!(fine.state, PdfTextState::Text);
    assert!(damaged(&object_stream_pdf(1000, "ObjStm")));
    // lopdf reads names with their #xx escapes decoded, and so does the measuring.
    assert!(damaged(&object_stream_pdf(1000, "Obj#53tm")));
}

#[test]
fn documents_holding_too_much_for_lopdf_to_build_are_refused_before_loading() {
    // About 120 bytes each: over a million objects, or 18 MiB of object streams.
    let mut bodies = page_bodies();
    bodies.push(format!("[{}]", "0 ".repeat(1 << 20)).into_bytes());
    let objects = read(&raw_pdf(&bodies, as_written)).expect("refused, not failed");
    assert_eq!(objects.state, PdfTextState::TooLarge);
    let mut bodies = page_bodies();
    for _ in 0..18 {
        let mut stream = Stream::new(dictionary! {}, vec![b' '; (1 << 20) - 16]);
        stream.compress().expect("compressible");
        let mut body = format!(
            "<< /Type /ObjStm /N 0 /First 0 /Filter /FlateDecode /Length {} >>\nstream\n",
            stream.content.len()
        )
        .into_bytes();
        body.extend(&stream.content);
        body.extend(b"\nendstream");
        bodies.push(body);
    }
    let streams = read(&raw_pdf(&bodies, as_written)).expect("refused, not failed");
    assert_eq!(streams.state, PdfTextState::TooLarge);
}

#[test]
fn a_damaged_file_is_refused_as_damaged_not_repaired() {
    let whole = raw_pdf(&page_bodies(), as_written);
    assert_eq!(
        read(&whole).expect("readable PDF").state,
        PdfTextState::Text
    );
    let cut = whole.get(..whole.len() / 2).expect("half");
    assert!(damaged(cut), "a file cut short is damaged");
    let (head, rest) = whole.split_at(b"%PDF-1.7\n".len());
    let shifted = [head, b"0000".as_slice(), rest].concat();
    assert!(
        damaged(&shifted),
        "a table whose offsets miss their objects is not repaired"
    );
}

#[test]
fn a_chain_of_too_many_cross_reference_sections_is_refused() {
    let written = raw_pdf(&page_bodies(), as_written);
    let table = written
        .windows(6)
        .rposition(|w| w == b"\nxref\n")
        .expect("table")
        + 1;
    let end = written
        .windows(9)
        .rposition(|w| w == b"startxref")
        .expect("startxref");
    let (mut file, mut previous) = (written.get(..end).expect("head").to_vec(), table);
    for _ in 0..40 {
        let at = file.len();
        let section = format!(
            "xref\n0 1\n0000000000 65535 f\r\ntrailer\n<< /Size 6 /Root 1 0 R /Prev {previous} >>\n"
        );
        file.extend(section.bytes());
        previous = at;
    }
    file.extend(format!("startxref\n{previous}\n%%EOF\n").bytes());
    let read = read(&file).expect("refused, not failed");
    assert_eq!(read.state, PdfTextState::TooLarge);
}
