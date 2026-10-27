//! Generate the pinned provider message closure outside the product build.
use std::{env, path::PathBuf};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args_os().skip(1);
    let root = PathBuf::from(args.next().ok_or("missing proto root")?);
    let output = PathBuf::from(args.next().ok_or("missing output directory")?);
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    let version = std::process::Command::new(&protoc)
        .arg("--version")
        .output()?;
    println!("{}", String::from_utf8(version.stdout)?);
    prost_build::Config::new()
        .protoc_executable(protoc)
        .out_dir(output)
        .compile_protos(
            &[root.join("alphagenome/protos/v1/dna_model_service.proto")],
            &[root],
        )?;
    Ok(())
}
