//! One-off probe (ticket 1066): read a folder of label PDFs with BioMCP's own reader.
//! Removed before landing.
use std::{fs, time::Instant};

#[test]
#[ignore = "one-off probe over local label PDFs"]
fn label_probe() {
    let dir = std::env::var("LABEL_PROBE_DIR").expect("LABEL_PROBE_DIR");
    let out = std::env::var("LABEL_PROBE_OUT").expect("LABEL_PROBE_OUT");
    fs::create_dir_all(&out).expect("out");
    let mut names: Vec<_> = fs::read_dir(&dir)
        .expect("dir")
        .map(|e| e.expect("entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "pdf"))
        .collect();
    names.sort();
    for path in names {
        let bytes = fs::read(&path).expect("read");
        let start = Instant::now();
        let read = biomcp_adapters::pdf::embedded(&bytes, 100_000, &|| false);
        let secs = start.elapsed().as_secs_f64();
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("x")
            .to_owned();
        match read {
            Ok(r) => {
                fs::write(format!("{out}/{name}.txt"), &r.text).expect("write");
                println!(
                    "PROBE {{\"file\":\"{name}\",\"bytes\":{},\"pages\":{},\"state\":\"{:?}\",\"chars\":{},\"secs\":{secs:.3}}}",
                    bytes.len(),
                    r.pages.unwrap_or(0),
                    r.state,
                    r.text.chars().count()
                );
            }
            Err(reason) => println!(
                "PROBE {{\"file\":\"{name}\",\"bytes\":{},\"error\":\"{reason:?}\",\"secs\":{secs:.3}}}",
                bytes.len()
            ),
        }
    }
}

#[test]
#[ignore = "one-off probe: does strict loading read the corpus as lenient loading does"]
fn strict_probe() {
    let dir = std::env::var("LABEL_PROBE_DIR").expect("LABEL_PROBE_DIR");
    let mut names: Vec<_> = fs::read_dir(&dir).expect("dir").map(|e| e.expect("entry").path()).collect();
    names.sort();
    for path in names.into_iter().filter(|p| p.extension().is_some_and(|x| x == "pdf")) {
        let bytes = fs::read(&path).expect("read");
        let mut options = lopdf::LoadOptions::with_max_decompressed_size(1 << 20);
        let lenient = lopdf::Document::load_mem_with_options(&bytes, options.clone());
        options.strict = true;
        let strict = lopdf::Document::load_mem_with_options(&bytes, options);
        let summary = |d: &lopdf::Result<lopdf::Document>| match d {
            Ok(d) => format!("ok objects {} pages {} recovery {:?}", d.objects.len(), d.get_pages().len(), d.xref_recovery),
            Err(e) => format!("err {e}"),
        };
        println!("STRICT {} | {} | {}", path.display(), summary(&lenient), summary(&strict));
    }
}
