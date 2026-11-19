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
        let start = Instant::now();
        let other = std::panic::catch_unwind(|| pdf_extract::extract_text_from_mem(&bytes));
        let other_secs = start.elapsed().as_secs_f64();
        let other_state = match other {
            Ok(Ok(text)) => {
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("x");
                fs::write(format!("{out}/{stem}.pe.txt"), &text).expect("write");
                format!("ok {}", text.chars().count())
            }
            Ok(Err(error)) => format!("error {error:?}").replace('"', "'"),
            Err(_) => "panic".to_owned(),
        };
        println!(
            "OTHER {{\"file\":\"{}\",\"state\":\"{other_state}\",\"secs\":{other_secs:.3}}}",
            path.display()
        );
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
