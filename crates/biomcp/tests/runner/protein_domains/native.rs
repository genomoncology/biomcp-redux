use super::*;
use std::{collections::BTreeMap,io::Write,path::Path};
pub(super) fn run(root:&Path)->Result<(),String>{
    let requests:BTreeMap<String,String>=serde_json::from_slice(&fs::read(root.join("requests.json")).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    let stub=Stub::for_sources(&["www.ebi.ac.uk","rest.uniprot.org"],|_|"application/json",move |request|{
        let path=request.lines().next().and_then(|line|line.split_whitespace().nth(1)).ok_or("request path")?;
        requests.get(path).cloned().ok_or_else(||format!("no accepted native request for {path}"))
    })?;
    let replay=root.join("replay");fs::create_dir_all(&replay).map_err(|e|e.to_string())?;
    let output=root.join("outputs");fs::create_dir_all(&output).map_err(|e|e.to_string())?;
    let registry=super::super::schema::Registry::embedded()?;
    let mut paths=fs::read_dir(root.join("cases")).map_err(|e|e.to_string())?.map(|p|p.map(|p|p.path()).map_err(|e|e.to_string())).collect::<Result<Vec<_>,_>>()?;paths.sort();
    for path in paths {
        let case:Value=serde_json::from_slice(&fs::read(&path).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        let args=case.pointer("/steps/0/run").and_then(Value::as_array).ok_or("native command")?.iter().map(|v|v.as_str().ok_or("argument")).collect::<Result<Vec<_>,_>>()?;
        let name=path.file_stem().and_then(|s|s.to_str()).ok_or("case name")?;
        if root.join(format!("{name}.done")).is_file(){continue;}
        let env=setup(&stub)?;let mut record=args.clone();record.extend(["--record",replay.to_str().ok_or("native replay")?]);call(&env,&record,0)?;
        let value=super::super::run_case(&path,&registry)?;
        measure(name,&value,&output)?;
        if name.starts_with("complexes-") || name.starts_with("domains-") {walk(&env,value,name,&output)?;}
        fs::write(root.join(format!("{name}.done")),b"passed").map_err(|e|e.to_string())?;
    }
    let seen=stub.finish()?;
    fs::write(root.join("request-count.txt"),seen.len().to_string()).map_err(|e|e.to_string())?;
    Ok(())
}
fn measure(name:&str,value:&Value,output:&Path)->Result<(),String>{
    let bytes=serde_json::to_vec(value).map_err(|e|e.to_string())?;
    let comparable=super::super::source_terms::size(value,"")?;
    fs::write(output.join(format!("{name}.json")),bytes.as_slice()).map_err(|e|e.to_string())?;
    writeln!(std::io::stdout(),"PROTEIN_RELATIONS {name} CLI={} MCP={} comparable={comparable} ceiling={}",bytes.len()+1,bytes.len(),(comparable*115).div_ceil(100)).map_err(|e|e.to_string())
}
fn walk(env:&Environment,mut value:Value,name:&str,output:&Path)->Result<(),String>{
    let total=value.pointer("/page/total").and_then(Value::as_u64).ok_or("native total")?;
    let pool=value.pointer("/page/pool").cloned().ok_or("native pool")?;
    let mut ids=Vec::new();let mut page=0;
    loop {
        ids.extend(value.pointer("/result/items").and_then(Value::as_array).ok_or("native rows")?.iter().map(|v|v.get("id").cloned().ok_or("native row id")).collect::<Result<Vec<_>,_>>()?);
        if ids.len() as u64>=total{break;}
        let action=super::super::action(&value,"page")?;let words:Vec<_>=action.iter().map(String::as_str).collect();value=call(env,&words,0)?;page+=1;
        if value.pointer("/page/pool")!=Some(&pool){return Err("native continuation changed publication identity or lifetime".into());}
        measure(&format!("{name}-page-{page}"),&value,output)?;
    }
    let unique=ids.iter().map(Value::to_string).collect::<std::collections::BTreeSet<_>>();
    if ids.len() as u64!=total || unique.len()!=ids.len(){return Err("native walk has a gap or duplicate".into());}
    if name=="complexes-P15056" && ids.contains(&json!("CPX-26353")){return Err("BRAF description admitted an unrelated complex".into());}
    if name=="complexes-P38398" && !["CPX-715","CPX-845","CPX-955"].iter().all(|s|ids.contains(&json!(s))){return Err("BRCA1 native memberships lost".into());}
    Ok(())
}
