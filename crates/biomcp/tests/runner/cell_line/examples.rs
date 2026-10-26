//! Produce executable examples from the same unchanged native bodies as identity cases.
use super::*;
use std::path::Path;
pub(in crate::runner) fn generate(directory: &Path) -> Result<(), String> {
    if directory.exists() {return Err("example output already exists".into());}
    fs::create_dir_all(directory).map_err(|e|e.to_string())?;
    let stub=recorded_stub()?;
    let env=setup(&stub)?;
    for (index,words) in [
        ["search","cell-line","MOLM13"],
        ["search","cell-line","MV4;11"],
        ["get","cell-line","CVCL_2119"],
        ["get","cell-line","CVCL_0004"],
    ].into_iter().enumerate() {
        let recording=env.root().join(format!("example-{index}"));
        let mut args=words.to_vec();args.extend(["--record",recording.to_str().ok_or("record path")?]);
        let value=invoke(&env,&args,0)?;
        let replay=["--replay".into(),recording.to_str().ok_or("record path")?.into()];
        let mcp=super::super::mcp::call_with_globals(&env,&words.map(String::from),false,&replay)?;
        if value!=mcp{return Err("example CLI/MCP mismatch".into());}
        for (surface,value) in [("cli",&value),("mcp",&mcp)] {
            let bytes=serde_json::to_vec(value).map_err(|e|e.to_string())?.len()+usize::from(surface=="cli");
            writeln!(std::io::stderr(),"CELL_LINE_EXAMPLE {index} [{surface}] {bytes}").map_err(|e|e.to_string())?;
        }
        merge(&recording,directory)?;
    }
    stub.finish()?;
    Ok(())
}
fn merge(recording: &Path, target: &Path) -> Result<(),String> {
    for source in fs::read_dir(recording).map_err(|e|e.to_string())? {
        let source=source.map_err(|e|e.to_string())?;
        let destination=target.join(source.file_name());
        fs::create_dir_all(&destination).map_err(|e|e.to_string())?;
        for file in fs::read_dir(source.path()).map_err(|e|e.to_string())? {
            let file=file.map_err(|e|e.to_string())?;
            let destination=destination.join(file.file_name());
            if !destination.exists(){fs::copy(file.path(),destination).map_err(|e|e.to_string())?;}
        }
    }
    Ok(())
}
