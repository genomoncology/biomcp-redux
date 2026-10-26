//! Feature pathway-wikipathways: explicit atomic catalog replacement and stored-only reads.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::{Path, PathBuf}};
type Check = Result<(), String>;
const SEARCH: &str = "https://www.wikipathways.org/json/findPathwaysByText.json";
const GET: &str = "https://www.wikipathways.org/json/getPathwayInfo.json";
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("wikipathways/stored-identity-pages-and-replacement", || run().map_err(Into::into)), Trial::test("wikipathways/independent-union-legs-and-round-robin",||union().map_err(Into::into))]
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
    let mut failed=args.to_vec();failed.extend(["--disable-sources","reactome"]);
    let partial=call(&env,&replay,&failed,0)?;
    if partial.get("status")!=Some(&json!("partial")) || partial.pointer("/sources/0/reason")!=Some(&json!("disabled")) || partial.pointer("/result/items/0/id/source")!=Some(&json!("wikipathways")){return Err(format!("failed leg beside successful leg: {partial}"));}
    Ok(())
}
