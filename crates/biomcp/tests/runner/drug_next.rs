//! Feature drug-next-steps: wholly synthetic D05/D17/D19-shaped source contexts.
//! These fixtures test selector propagation, never clinical answers or licensed annotations.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
const ROOT: &str = "/result/candidates/0/sections/oncokb";
const NAMES: [&str; 8] = [
    "d05-explicit-context", "d17-group-context", "d19-no-allele",
    "missing-build", "missing-gene", "missing-tumor", "unsupported-build", "compound-profile",
];
pub(super) fn trials() -> Vec<Trial> {
    NAMES.into_iter().enumerate().map(|(n, name)| {
        Trial::test(format!("drug-next-steps/{name}"), move || run(n).map_err(Into::into))
    }).collect()
}
fn run(n: usize) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_sources(&["civicdb.org", "www.oncokb.org"], |_| "application/json", reply)?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca});
    env.config(&config).map_err(|e| e.to_string())?;
    let args = ["get", "drug", &(900+n).to_string(), "civic", "--source", "civic"]
        .map(str::to_owned);
    let recording = env.root().join("civic-recording");
    let mut record = args.to_vec();
    record.extend(["--record".into(), recording.display().to_string()]);
    let value = invoke(&env, record, 0)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    verify(n, &value)?;
    let globals = vec!["--replay".into(), recording.display().to_string(), "--fresh".into()];
    let mcp = super::mcp::call_with_globals(&env, &args, false, &globals)?;
    for key in ["result", "next", "problems"] {
        if value.get(key) != mcp.get(key) {return Err(format!("CLI/MCP {key} differs"));}
    }
    if n == 0 {
        follow(&env, &value, &mut config)?;
    }
    let requests = stub.finish()?;
    let expected = if n == 0 {2} else {1};
    if requests.len() != expected {return Err(format!("unexpected network calls: {}",requests.len()));}
    let comparable = super::source_terms::size(&value, "")?;
    use std::io::Write;
    writeln!(std::io::stdout(), "DRUG_NEXT_SIZE case={n} full_bytes={} comparable_bytes={comparable}", value.to_string().len()).map_err(|e|e.to_string())?;
    if comparable > 8000 {return Err("new case exceeds initial bounded size".into());}
    Ok(())
}
fn verify(n: usize, value: &Value) -> Result<(), String> {
    let steps = value.get("next").and_then(Value::as_array).ok_or("missing next")?;
    let annotation = steps.iter().filter_map(|s|s.get("args").and_then(Value::as_array))
        .find(|a|a.iter().any(|v|v=="oncokb"));
    if n == 0 {
        let args = annotation.ok_or("explicit source context omitted typed OncoKB step")?;
        if args != &json!(["get","variant","NC_000007.14:g.140753336A>T","oncokb","--tumor-type","Synthetic tumor","--build","GRCh38"]).as_array().ok_or("args")?.clone() {
            return Err(format!("wrong source context: {args:?}"));
        }
    } else {
        if annotation.is_some() {return Err("invented complete allele context".into());}
        let field = match n {1|2|7=>"oncokb.variant",3|6=>"oncokb.build",4=>"oncokb.gene",5=>"oncokb.tumor_type",_=>return Err("case".into())};
        if !value.get("problems").and_then(Value::as_array).is_some_and(|p|p.iter().any(|p|p.get("field")==Some(&json!(field)))) {
            return Err(format!("missing named context {field}: {value}"));
        }
    }
    if value.to_string().contains("LEVEL_1") {return Err("drug response gained therapy level".into());}
    Ok(())
}
fn follow(env: &Environment, value: &Value, config: &mut Value) -> Result<(), String> {
    let args = value.get("next").and_then(Value::as_array).ok_or("next")?.iter()
        .filter_map(|s|s.get("args").and_then(Value::as_array))
        .find(|a|a.iter().any(|v|v=="oncokb")).ok_or("annotation")?
        .iter().map(|v|v.as_str().map(str::to_owned).ok_or("arg".into())).collect::<Result<Vec<_>,String>>()?;
    let missing = invoke(env,args.clone(),0)?;
    if missing.pointer(&format!("{ROOT}/reason"))!=Some(&json!("no_key")) {return Err("missing key did not stop".into());}
    config["keys"]=json!({"oncokb_api_key":"synthetic-oncokb-variant-key"});
    env.config(config).map_err(|e|e.to_string())?;
    let recorded=env.root().join("annotation-recording");
    let mut keyed=args.clone();keyed.extend(["--record".into(),recorded.display().to_string()]);
    let output=invoke(env,keyed,0)?;
    if output.pointer(&format!("{ROOT}/state"))!=Some(&json!("data")) {return Err(format!("keyed route failed: {output}"));}
    let globals=vec!["--replay".into(),recorded.display().to_string(),"--fresh".into()];
    let mcp=super::mcp::call_with_globals(env,&args,false,&globals)?;
    if output.get("result")!=mcp.get("result") {return Err("keyed CLI/MCP differs".into());}
    for file in fs::read_dir(recorded.join("oncokb")).map_err(|e|e.to_string())? {
        let bytes=fs::read(file.map_err(|e|e.to_string())?.path()).map_err(|e|e.to_string())?;
        if bytes.windows(28).any(|b|b==b"synthetic-oncokb-variant-key") {return Err("key in recording".into());}
    }
    Ok(())
}
fn reply(request: &str) -> Result<String,String> {
    if request.starts_with("GET /api/v1/annotate/") {return super::oncokb::reply(request);}
    let (_,body)=request.split_once("\r\n\r\n").ok_or("POST")?;
    let query:Value=serde_json::from_str(body).map_err(|e|e.to_string())?;
    let id=query.pointer("/variables/id").and_then(Value::as_u64).ok_or("therapy ID")?;
    let n=id-900;
    let mut member=json!({"id":900,"name":"Synthetic allele","link":"/variants/900","feature":{"name":"BRAF"},"hgvsDescriptions":["NC_000007.14:g.140753336A>T"],"coordinates":{"referenceBuild":"GRCH38"}});
    match n {
        1=>member["hgvsDescriptions"]=json!(["NC_000007.14:g.140753336A>T","NC_000007.14:g.140753336A>C"]),
        2=>{member["name"]=json!("Synthetic expression category");member["hgvsDescriptions"]=json!(["N/A"]);},
        3=>member["coordinates"]=Value::Null,
        4=>member["feature"]["name"]=json!("synthetic invalid gene"),
        6=>member["coordinates"]["referenceBuild"]=json!("NCBI36"),
        _=>{},
    }
    let profile=json!({"id":900,"name":"Synthetic profile","link":"/molecular-profiles/900","isComplex":n==7,"variants":[member]});
    let disease=if n==5 {Value::Null} else {json!({"name":"Synthetic tumor","doid":"0000"})};
    Ok(json!({"data":{"therapy":{"id":id,"name":"Synthetic therapy","link":format!("/therapies/{id}")},"evidenceItems":{"totalCount":1,"pageInfo":{"hasNextPage":false,"endCursor":null},"nodes":[{"id":900,"status":"ACCEPTED","evidenceType":"PREDICTIVE","evidenceLevel":"A","evidenceDirection":"SUPPORTS","significance":"SENSITIVITYRESPONSE","disease":disease,"therapies":[{"id":id,"name":"Synthetic therapy"}],"source":{"citationId":"17611497","sourceType":"PUBMED"},"molecularProfile":profile}]},"assertions":{"totalCount":0,"pageInfo":{"hasNextPage":false,"endCursor":null},"nodes":[]}}}).to_string())
}
