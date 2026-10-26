//! Feature pathway-wikipathways: explicit atomic catalog replacement and stored-only reads.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::{Path, PathBuf}};
type Check = Result<(), String>;
const SEARCH: &str = "https://www.wikipathways.org/json/findPathwaysByText.json";
const GET: &str = "https://www.wikipathways.org/json/getPathwayInfo.json";
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("wikipathways/stored-identity-pages-and-replacement", || run().map_err(Into::into)), Trial::test("wikipathways/independent-union-legs-and-round-robin",||union().map_err(Into::into)),Trial::test("wikipathways/concurrent-readers-during-pair-replacement",||concurrent().map_err(Into::into)),Trial::test("wikipathways/native-disclosure-measurements",||native().map_err(Into::into)).with_ignored_flag(!super::launch::fixtures().join("cases/wikipathways-native").is_dir())]
}
fn catalog(search: bool, changed: bool) -> Value {
    json!({"pathwayInfo":(0..13).map(|n| {
        let mut row = json!({"id":format!("WP{}",n+1),"url":format!("https://www.wikipathways.org/instance/WP{}",n+1),"name":if changed {format!("New Atlas {n}")} else {format!("Literal Atlas {n}")},"species":if n==12 {"Mus musculus"} else {"Homo sapiens"},"revision":"2025-05-22","authors":"Source Author","description":"Native source description...","citedIn":""});
        if search && let Some(map)=row.as_object_mut() { map.insert("datanodes".into(),json!("TP53, unchecked label")); map.insert("annotations".into(),json!("native annotation")); }
        row
    }).collect::<Vec<_>>()})
}
fn seed(env: &Environment, search: Value, get: Value, status: u16) -> Result<PathBuf,String> {
    use sha2::{Digest,Sha256};
    let root=env.root().join("replay"); let dir=root.join("wikipathways");
    fs::create_dir_all(&dir).map_err(|e|e.to_string())?;
    for (url,body,code) in [(SEARCH,search,200),(GET,get,status)] {
        let key=Sha256::digest(format!("GET\0{url}").as_bytes());
        let key=key.iter().map(|b|format!("{b:02x}")).collect::<String>();
        let wire=json!({"source":"wikipathways","method":"GET","url":url,"exchanges":[{"status":code,"content_type":"application/json","delay_ms":0,"retry_after_ms":null,"body":{"kind":"text","value":body.to_string()}}]});
        fs::write(dir.join(format!("{key}.json")),wire.to_string()).map_err(|e|e.to_string())?;
    }
    Ok(root)
}
fn call(env:&Environment,replay:&Path,args:&[&str],exit:i32)->Result<Value,String>{
    let mut command=env.globals(Some(replay),false); command.extend(args.iter().map(|s|(*s).into())); invoke(env,command,exit)
}
fn run()->Check {
    let env=Environment::new().map_err(|e|e.to_string())?;
    let replay=seed(&env,catalog(true,false),catalog(false,false),200)?;
    let receipt=call(&env,&replay,&["sync","wikipathways"],0)?;
    if receipt.pointer("/result/search/total_rows")!=Some(&json!(13)) || receipt.pointer("/result/get/human_rows")!=Some(&json!(12)) {return Err(format!("receipt: {receipt}"));}
    fs::remove_dir_all(replay.join("wikipathways")).map_err(|e|e.to_string())?;
    let first=call(&env,&replay,&["search","pathway","atlas","--source","wikipathways"],0)?;
    if first.pointer("/result/page/total")!=Some(&json!(12)) || first.pointer("/result/items/0/id/id")!=Some(&json!("WP1")) {return Err(format!("title page: {first}"));}
    let pool=first.pointer("/next/5/args").and_then(Value::as_array).ok_or("page action")?.iter().map(|v|v.as_str().ok_or("arg")).collect::<Result<Vec<_>,_>>()?;
    let second=call(&env,&replay,&pool,0)?;
    if second.pointer("/result/items/0/id/id")!=Some(&json!("WP6")){return Err("page order".into());}
    let card=call(&env,&replay,&["get","pathway","WP1","summary"],0)?;
    if card.pointer("/result/description_truncated")!=Some(&json!(true)) || card.pointer("/result/pathway/description")!=Some(&json!("Native source description...")){return Err(format!("source summary: {card}"));}
    call(&env,&replay,&["get","pathway","WP13"],1)?;
    call(&env,&replay,&["get","pathway","WP99999"],1)?;
    for section in ["genes","events"] {call(&env,&replay,&["get","pathway","WP1",section],2)?;}
    for fault in [json!({"unexpected":true}),json!({"pathwayInfo":[]})] {
        seed(&env,catalog(true,true),fault,200)?;
        call(&env,&replay,&["sync","wikipathways"],4)?;
        let retained=call(&env,&replay,&["search","pathway","Literal Atlas","--source","wikipathways"],0)?;
        if retained.pointer("/result/page/total")!=Some(&json!(12)){return Err("failed pair replaced old snapshot".into());}
    }
    seed(&env,catalog(true,true),catalog(false,true),500)?;
    call(&env,&replay,&["sync","wikipathways"],4)?;
    seed(&env,catalog(true,true),catalog(false,true),200)?;
    call(&env,&replay,&["sync","wikipathways"],0)?;
    let retained=call(&env,&replay,&pool,0)?;
    if retained.pointer("/result/items/0/name")!=Some(&json!("Literal Atlas 5")){return Err("pool changed with catalog replacement".into());}
    let disabled=call(&env,&replay,&["get","pathway","WP1","--disable-sources","wikipathways"],4)?;
    if disabled.pointer("/sources/0/reason")!=Some(&json!("disabled")){return Err("disabled read used stored rows".into());}
    Ok(())
}
fn union() -> Check {
    use super::launch::fixtures;
    let env=Environment::new().map_err(|e|e.to_string())?;
    let replay=seed(&env,catalog(true,false),catalog(false,false),200)?;
    call(&env,&replay,&["sync","wikipathways"],0)?;
    let source=fixtures().join("replies/pathway/native/reactome");
    fs::create_dir_all(replay.join("reactome")).map_err(|e|e.to_string())?;
    for path in fs::read_dir(source).map_err(|e|e.to_string())? {
        let path=path.map_err(|e|e.to_string())?.path();
        if path.extension().is_some_and(|s|s=="json") {
            fs::copy(&path,replay.join("reactome").join(path.file_name().ok_or("filename")?)).map_err(|e|e.to_string())?;
        }
    }
    let args=["search","pathway","MAPK signaling","--source","reactome","--source","wikipathways"];
    // The synthetic catalog has no title match for this phrase; the leg still has an outcome.
    let mut missing=args.to_vec();missing.extend(["--offset","5"]);
    let restarted=call(&env,&replay,&missing,3)?;
    if restarted.pointer("/next/0/kind")!=Some(&json!("restart")) {return Err(format!("missing mixed pool: {restarted}"));}
    let page=call(&env,&replay,&args,0)?;
    if page.pointer("/sources/0/source")!=Some(&json!("reactome")) || page.pointer("/sources/1/source")!=Some(&json!("wikipathways")) {return Err(format!("independent legs: {page}"));}
    let mut body=catalog(true,false);
    for row in body.get_mut("pathwayInfo").and_then(Value::as_array_mut).ok_or("rows")? {
        *row.get_mut("name").ok_or("name")?=json!("MAPK signaling");
    }
    seed(&env,body,catalog(false,false),200)?;
    call(&env,&replay,&["sync","wikipathways"],0)?;
    let first=call(&env,&replay,&args,0)?;
    let sources=first.pointer("/result/items").and_then(Value::as_array).ok_or("items")?.iter().map(|r|r.pointer("/id/source").and_then(Value::as_str)).collect::<Vec<_>>();
    if sources!=vec![Some("reactome"),Some("wikipathways"),Some("reactome"),Some("wikipathways"),Some("reactome")] {return Err(format!("round robin order: {first}"));}
    for path in fs::read_dir(fixtures().join("replies/pathway/mixed/reactome")).map_err(|e|e.to_string())? {
        let path=path.map_err(|e|e.to_string())?.path();
        fs::copy(&path,replay.join("reactome").join(path.file_name().ok_or("filename")?)).map_err(|e|e.to_string())?;
    }
    let mut mixed=args.to_vec();mixed.push("--fresh");
    let short=call(&env,&replay,&mixed,0)?;
    if short.pointer("/result/page/returned")!=Some(&json!(4)) || short.pointer("/result/page/next_offset")!=Some(&json!(4)) {return Err(format!("native admission turn: {short}"));}
    let next=short.get("next").and_then(Value::as_array).and_then(|a|a.iter().find(|s|s.get("kind")==Some(&json!("page")))).and_then(|s|s.get("args")).and_then(Value::as_array).ok_or("short continuation")?.iter().map(|v|v.as_str().ok_or("arg")).collect::<Result<Vec<_>,_>>()?;
    let resumed=call(&env,&replay,&next,0)?;
    if resumed.pointer("/result/items/0/id/source")!=Some(&json!("reactome")) || resumed.pointer("/result/items/1/id/id")!=Some(&json!("WP3")){return Err(format!("short page changed source turn: {resumed}"));}
    let mut failed=args.to_vec();failed.extend(["--disable-sources","reactome"]);
    let partial=call(&env,&replay,&failed,0)?;
    if partial.get("status")!=Some(&json!("partial")) || partial.pointer("/sources/0/reason")!=Some(&json!("disabled")) || partial.pointer("/result/items/0/id/source")!=Some(&json!("wikipathways")){return Err(format!("failed leg beside successful leg: {partial}"));}
    Ok(())
}
fn concurrent() -> Check {
    use sha2::{Digest,Sha256};
    use std::{process::Stdio,time::{Duration,Instant}};
    let env=Environment::new().map_err(|e|e.to_string())?;
    let replay=seed(&env,catalog(true,false),catalog(false,false),200)?;
    call(&env,&replay,&["sync","wikipathways"],0)?;
    seed(&env,catalog(true,true),catalog(false,true),200)?;
    let key=Sha256::digest(format!("GET\0{GET}").as_bytes()).iter().map(|b|format!("{b:02x}")).collect::<String>();
    let path=replay.join("wikipathways").join(format!("{key}.json"));
    let mut reply:Value=serde_json::from_slice(&fs::read(&path).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    *reply.pointer_mut("/exchanges/0/delay_ms").ok_or("delay")?=json!(1500);
    fs::write(path,reply.to_string()).map_err(|e|e.to_string())?;
    let observed=env.root().join("replacement-observations.jsonl");
    let mut args=env.globals(Some(&replay),false);
    args.extend(["sync".into(),"wikipathways".into(),"--observe".into(),observed.to_string_lossy().into_owned()]);
    let mut writer=env.command(env!("CARGO_BIN_EXE_biomcp"),&args).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(|e|e.to_string())?;
    let until=Instant::now()+Duration::from_secs(30);
    while fs::metadata(&observed).map_or(true,|m|m.len()==0) {
        if Instant::now()>until || writer.try_wait().map_err(|e|e.to_string())?.is_some() {
            let _=writer.kill(); let _=writer.wait(); return Err("replacement never admitted its first catalog".into());
        }
        std::thread::yield_now();
    }
    let results=std::thread::scope(|scope|{
        let one=scope.spawn(||call(&env,&replay,&["search","pathway","Atlas","--source","wikipathways"],0));
        let two=scope.spawn(||call(&env,&replay,&["search","pathway","Atlas","--source","wikipathways"],0));
        [one.join(),two.join()]
    });
    let finished=writer.wait_with_output().map_err(|e|e.to_string())?;
    if !finished.status.success(){return Err(format!("replacement failed: {}",String::from_utf8_lossy(&finished.stderr)));}
    for result in results {
        let result=result.map_err(|_|"reader panicked")??;
        let names=result.pointer("/result/items").and_then(Value::as_array).ok_or("items")?.iter().filter_map(|r|r.get("name").and_then(Value::as_str)).collect::<Vec<_>>();
        if names.len()!=5 || !(names.iter().all(|n|n.starts_with("Literal Atlas")) || names.iter().all(|n|n.starts_with("New Atlas"))) {return Err(format!("reader observed partial replacement: {result}"));}
    }
    Ok(())
}
fn native() -> Check {
    use std::io::Write;
    let root=super::launch::fixtures().join("cases/wikipathways-native");
    let registry=super::schema::Registry::embedded()?;
    let mut files=fs::read_dir(root).map_err(|e|e.to_string())?.map(|e|e.map(|e|e.path()).map_err(|e|e.to_string())).collect::<Result<Vec<_>,_>>()?;
    files.sort();
    for path in files {
        let value=super::run_case(&path,&registry)?;
        let full=serde_json::to_vec(&value).map_err(|e|e.to_string())?.len();
        let comparable=super::source_terms::size(&value,"")?;
        writeln!(std::io::stdout(),"WIKIPATHWAYS_MEASURE {}",json!({"case":path.file_name().and_then(|n|n.to_str()).ok_or("filename")?,"full_cli_bytes":full+1,"full_mcp_bytes":full,"comparable_cli_bytes":comparable+1,"comparable_mcp_bytes":comparable,"tokens":comparable.div_ceil(4)})).map_err(|e|e.to_string())?;
    }
    Ok(())
}
