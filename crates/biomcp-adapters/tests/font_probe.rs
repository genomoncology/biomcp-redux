//! Throwaway 1069 probe: which ToUnicode maps lopdf refuses, and what would make it accept them.
#![allow(clippy::all, clippy::pedantic, clippy::restriction)]
use lopdf::{Dictionary, Document, Encoding, Object, Stream};

fn kind(e: &lopdf::Result<Encoding>) -> &'static str {
    match e {
        Ok(Encoding::UnicodeMapEncoding(_)) => "unicode",
        Ok(Encoding::OneByteEncoding(_)) => "onebyte",
        Ok(Encoding::SimpleEncoding(_)) => "simple",
        Ok(Encoding::Differences(_)) => "differences",
        Err(_) => "error",
    }
}

fn rebuilt(raw: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(raw).replace('\r', "\n");
    let mut body = String::new();
    for (open, close) in [
        ("begincodespacerange", "endcodespacerange"),
        ("beginbfchar", "endbfchar"),
        ("beginbfrange", "endbfrange"),
    ] {
        let mut rest = text.as_str();
        while let Some(at) = rest.find(open) {
            let head = &rest[..at];
            let count = head
                .trim_end()
                .rsplit(|c: char| c.is_whitespace())
                .next()
                .unwrap_or("0");
            let after = &rest[at + open.len()..];
            let Some(end) = after.find(close) else { break };
            body.push_str(&format!(
                "{count} {open}\n{}\n{close}\n",
                after[..end].trim()
            ));
            rest = &after[end + close.len()..];
        }
    }
    format!("/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapName /Rebuilt def\n{body}endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n").into_bytes()
}

fn with(doc: &Document, font: &Dictionary, bytes: Vec<u8>) -> &'static str {
    let mut f = font.clone();
    f.set(
        "ToUnicode",
        Object::Stream(Stream::new(Dictionary::new(), bytes)),
    );
    let k = kind(&f.get_font_encoding(doc));
    k
}

#[test]
#[ignore]
fn probe() {
    let Some(dir) = std::env::var_os("FONT_PROBE_DIR") else {
        return;
    };
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    names.sort();
    for path in names {
        let Ok(doc) = Document::load(&path) else {
            continue;
        };
        for (id, object) in &doc.objects {
            let Ok(font) = object.as_dict() else { continue };
            if !font.has_type(b"Font") {
                continue;
            }
            let Ok(tu) = font
                .get_deref(b"ToUnicode", &doc)
                .and_then(Object::as_stream)
            else {
                continue;
            };
            let Ok(raw) = tu.get_plain_content() else {
                println!("FONT {} {id:?} undecodable-stream", path.display());
                continue;
            };
            let base = font
                .get(b"BaseFont")
                .and_then(Object::as_name)
                .map(|n| String::from_utf8_lossy(n).into_owned())
                .unwrap_or_default();
            let enc = font
                .get(b"Encoding")
                .and_then(Object::as_name)
                .map(|n| String::from_utf8_lossy(n).into_owned())
                .unwrap_or_default();
            let now = kind(&font.get_font_encoding(&doc));
            let lf: Vec<u8> = raw
                .iter()
                .map(|b| if *b == b'\r' { b'\n' } else { *b })
                .collect();
            let mut trimmed = raw.clone();
            while trimmed
                .last()
                .is_some_and(|b| *b == 0 || b.is_ascii_whitespace())
            {
                trimmed.pop();
            }
            trimmed.push(b'\n');
            let both: Vec<u8> = trimmed
                .iter()
                .map(|b| if *b == b'\r' { b'\n' } else { *b })
                .collect();
            let head = String::from_utf8_lossy(&raw[..raw.len().min(160)])
                .replace('\r', "\\r")
                .replace('\n', "\\n");
            let rb = with(&doc, font, rebuilt(&raw));
            println!(
                "FONT {} {id:?} {base} enc={enc} now={now} rebuilt={rb} lf={} trim={} both={} len={} head={head}",
                path.file_name().unwrap().to_string_lossy(),
                with(&doc, font, lf),
                with(&doc, font, trimmed),
                with(&doc, font, both),
                raw.len()
            );
        }
    }
}
