//! Throwaway (1068 second review): write worst-case hostile PDFs under the reader's bounds.
use lopdf::{Dictionary, Document, Object, SaveOptions, Stream, dictionary};

fn page_doc(contents: Vec<Vec<u8>>, objects: usize) -> Document {
    let mut document = Document::with_version("1.7");
    let pages = document.new_object_id();
    let font = document.add_object(dictionary! { "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica", "Encoding" => "WinAnsiEncoding" });
    let mut kids = Vec::new();
    for content in contents {
        let mut stream = Stream::new(Dictionary::new(), content);
        let _ = stream.compress();
        let contents = document.add_object(stream);
        kids.push(Object::from(document.add_object(dictionary! {
            "Type" => "Page", "Parent" => pages, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Contents" => contents, "Resources" => dictionary! { "Font" => dictionary! { "F1" => font } },
        })));
    }
    let count = kids.len() as i64;
    document.objects.insert(
        pages,
        Object::Dictionary(dictionary! { "Type" => "Pages", "Kids" => kids, "Count" => count }),
    );
    for n in 0..objects {
        document.add_object(Object::Integer(n as i64));
    }
    let catalog = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    document.trailer.set("Root", catalog);
    document
}

#[test]
#[ignore = "throwaway probe"]
fn write_hostile() {
    let Some(out) = std::env::var_os("HOSTILE_OUT") else {
        return;
    };
    let out = std::path::PathBuf::from(out);
    std::fs::create_dir_all(&out).unwrap();
    let mb4 = 4 << 20;
    let save = |mut document: Document, name: &str, streams: bool| {
        let mut bytes = Vec::new();
        if streams {
            let options = SaveOptions::builder()
                .use_object_streams(true)
                .use_xref_streams(true)
                .max_objects_per_stream(60_000)
                .build();
            document.save_with_options(&mut bytes, options).unwrap();
        } else {
            document.save_to(&mut bytes).unwrap();
        }
        println!("{name} {} bytes", bytes.len());
        std::fs::write(out.join(name), bytes).unwrap();
    };
    save(
        page_doc(vec![b"n\n".repeat(mb4 / 2 - 8)], 0),
        "ops-4mib.pdf",
        false,
    );
    let tj = format!("BT /F1 9 Tf [{}] TJ ET", "0 ".repeat(mb4 / 2 - 64)).into_bytes();
    save(page_doc(vec![tj], 0), "tj-4mib.pdf", false);
    let text = format!(
        "BT /F1 9 Tf {} ET",
        "(word word word word word word word word) Tj T* ".repeat(mb4 / 50)
    )
    .into_bytes();
    save(page_doc(vec![text; 100], 0), "text-100x4mib.pdf", false);
    save(
        page_doc(vec![b"BT ET".to_vec()], 600_000),
        "objstm-600k.pdf",
        true,
    );
    save(
        page_doc(vec![b"BT ET".to_vec()], 1_150_000),
        "plain-1150k.pdf",
        false,
    );
    save(maps_doc(40), "maps-40x131k.pdf", false);
    let mut objects = page_doc(vec![b"BT ET".to_vec()], 0);
    objects.add_object(Object::Array(vec![Object::Integer(0); 1_040_000]));
    save(objects, "objects-1m.pdf", false);
    let mut strings = page_doc(vec![b"BT ET".to_vec()], 0);
    strings.add_object(Object::Array(vec![
        Object::string_literal(
            "x".repeat(20 << 20)
        );
        1
    ]));
    save(strings, "string-20mib.pdf", false);
}

fn maps_doc(fonts: usize) -> Document {
    let mut document = Document::with_version("1.7");
    let pages = document.new_object_id();
    let map = b"/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CMapName /M def\n1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n1 beginbfrange\n<00000000> <0001FFFF> <0041>\nendbfrange\nendcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n".to_vec();
    let mut entries = Dictionary::new();
    let mut content = String::from("BT ");
    for n in 0..fonts {
        let descendant = document.add_object(dictionary! { "Type" => "Font", "Subtype" => "CIDFontType2", "BaseFont" => "M",
            "CIDSystemInfo" => dictionary! { "Registry" => Object::string_literal("Adobe"), "Ordering" => Object::string_literal("Identity"), "Supplement" => 0 } });
        let map = document.add_object(Stream::new(Dictionary::new(), map.clone()));
        let font = document.add_object(dictionary! { "Type" => "Font", "Subtype" => "Type0", "BaseFont" => "M", "Encoding" => "Identity-H",
            "DescendantFonts" => vec![descendant.into()], "ToUnicode" => map });
        entries.set(format!("F{n}"), font);
        content.push_str(&format!("/F{n} 9 Tf <00000001> Tj "));
    }
    content.push_str("ET");
    let contents = document.add_object(Stream::new(Dictionary::new(), content.into_bytes()));
    let page = document.add_object(dictionary! { "Type" => "Page", "Parent" => pages, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
        "Contents" => contents, "Resources" => dictionary! { "Font" => entries } });
    document.objects.insert(
        pages,
        Object::Dictionary(
            dictionary! { "Type" => "Pages", "Kids" => vec![page.into()], "Count" => 1 },
        ),
    );
    let catalog = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
    document.trailer.set("Root", catalog);
    document
}
