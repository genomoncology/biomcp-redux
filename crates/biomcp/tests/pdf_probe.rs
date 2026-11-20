//! One-off probe (ticket 1075): time PDF reads in process and through the child, and compare
//! their answers. Removed before landing.
use std::{
    fs,
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect(&path, out);
        } else if fs::read(&path).unwrap().starts_with(b"%PDF-") {
            out.push(path);
        }
    }
}

fn child(bytes: &[u8]) -> Vec<u8> {
    let mut stdin = tempfile::tempfile().unwrap();
    stdin.write_all(bytes).unwrap();
    stdin.rewind().unwrap();
    let mut stdout = tempfile::tempfile().unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_biomcp"))
        .arg("__pdf-read")
        .arg((1024_u64 << 20).to_string())
        .env_clear()
        .stdin(stdin)
        .stdout(stdout.try_clone().unwrap())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut pause = Duration::from_millis(1);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        std::thread::sleep(pause);
        pause = (pause * 2).min(Duration::from_millis(20));
    };
    assert!(status.success());
    stdout.rewind().unwrap();
    let mut answer = Vec::new();
    stdout.read_to_end(&mut answer).unwrap();
    answer
}

#[test]
#[ignore = "one-off probe"]
fn pdf_probe() {
    let mut paths = Vec::new();
    for dir in std::env::var("PDF_PROBE_DIRS").unwrap().split(':') {
        collect(Path::new(dir), &mut paths);
    }
    paths.sort();
    let (mut same, mut added) = (0, Vec::new());
    let (mut total_in, mut total_child) = (0.0, 0.0);
    for path in &paths {
        let bytes = fs::read(path).unwrap();
        let mut best_in = f64::MAX;
        let mut best_child = f64::MAX;
        let mut inside = Vec::new();
        let mut through = Vec::new();
        for _ in 0..3 {
            let start = Instant::now();
            inside = biomcp_query::pdf_child::serve(&bytes);
            best_in = best_in.min(start.elapsed().as_secs_f64() * 1000.0);
            let start = Instant::now();
            through = child(&bytes);
            best_child = best_child.min(start.elapsed().as_secs_f64() * 1000.0);
        }
        if inside == through {
            same += 1;
        } else {
            println!("DIFFERS {}", path.display());
        }
        total_in += best_in;
        total_child += best_child;
        added.push(best_child - best_in);
        println!(
            "PROBE {} bytes={} in={best_in:.1}ms child={best_child:.1}ms",
            path.display(),
            bytes.len()
        );
    }
    added.sort_by(f64::total_cmp);
    let n = added.len();
    println!(
        "SUMMARY files={n} same={same} in_total={total_in:.0}ms child_total={total_child:.0}ms added_median={:.1}ms added_p90={:.1}ms added_max={:.1}ms added_mean={:.1}ms",
        added[n / 2],
        added[n * 9 / 10],
        added[n - 1],
        added.iter().sum::<f64>() / n as f64
    );
}
