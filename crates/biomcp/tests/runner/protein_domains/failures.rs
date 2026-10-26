use super::*;
use std::path::Path;
pub(super) fn run()->Result<(),String>{
    for command in ["domains","complexes"] {
        let stub=Stub::for_source("www.ebi.ac.uk","application/json",|request| {
            Ok(if request.contains("/interpro/"){json!({"count":0,"next":null,"results":[]})}else{json!({"totalNumberOfResults":0,"elements":[]})}.to_string())
        })?;
        let seed=setup(&stub)?;let record=seed.root().join("recorded");
        call(&seed,&["protein",command,"P38398","--record",record.to_str().ok_or("record path")?],0)?;
        stub.finish()?;
        for (kind,reason) in [("429","rate_limit"),("500","server"),("timeout","deadline"),("broken","bad_reply"),("error","bad_reply")] {
            let env=Environment::new().map_err(|e|e.to_string())?;let replay=env.root().join("recordings");copy(&record,&replay,kind)?;
            let mut words=env.globals(Some(&replay),false);words.extend(["protein",command,"P38398"].map(str::to_owned));let value=invoke(&env,words,4)?;
            if value.pointer("/status")!=Some(&json!("failed")) || value.pointer("/sources/0/reason")!=Some(&json!(reason)) || value.pointer("/sources/0/terms")!=Some(&json!("CC0")) || value.get("result").is_some(){return Err(format!("{command}/{kind} became an empty collection: {value}"));}
        }
    }
    Ok(())
}
fn copy(original:&Path,target:&Path,kind:&str)->Result<(),String>{
    for source in fs::read_dir(original).map_err(|e|e.to_string())? {
        let source=source.map_err(|e|e.to_string())?;let directory=target.join(source.file_name());fs::create_dir_all(&directory).map_err(|e|e.to_string())?;
        for path in fs::read_dir(source.path()).map_err(|e|e.to_string())? {
            let path=path.map_err(|e|e.to_string())?;let mut record:Value=serde_json::from_slice(&fs::read(path.path()).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
            let mut exchange=record.pointer("/exchanges/0").cloned().ok_or("record exchange")?;
            match kind {
                "429"=>{exchange["status"]=json!(429);exchange["retry_after_ms"]=json!(0);},
                "500"=>exchange["status"]=json!(500),
                "timeout"=>exchange["delay_ms"]=json!(9000),
                "broken"=>exchange["body"]=json!({"kind":"text","value":"{"}),
                _=>exchange["body"]=json!({"kind":"json","value":{"message":"source error"}}),
            }
            record["exchanges"]=json!([exchange.clone(),exchange.clone(),exchange]);
            fs::write(directory.join(path.file_name()),serde_json::to_vec(&record).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        }
    }
    Ok(())
}
