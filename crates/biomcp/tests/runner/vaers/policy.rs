use super::{Check, both, environment, words};
use crate::runner::{configuration::invoke, network_stub::Stub};
use serde_json::json;
pub(super) fn run() -> Check {
    let stub = Stub::for_source("wonder.cdc.gov", "application/xml", |_| Ok(super::fixture::table("MMR","D8.V13-level2")))?;
    let env = environment(&stub)?;
    let recording = env.root().join("recording");
    let mut args = vec!["--record".into(),recording.to_string_lossy().into_owned()]; args.extend(words("MMR")); args.extend(["--limit".into(),"2".into()]);
    let value = invoke(&env,args,0)?;
    let page = value.get("next").and_then(|v|v.as_array()).and_then(|v|v.iter().find(|v|v.get("kind")==Some(&json!("page")))).and_then(|v|v.get("args"));
    for switch in ["vaers","restricted"] {
        for replay in [false,true] {
            let mut args = vec!["--disable-sources".into(),switch.into()];
            if replay { args.extend(["--replay".into(),recording.to_string_lossy().into_owned()]); }
            args.extend(words("MMR"));
            let value = both(&env,args,4)?;
            if value.get("result").is_some() || value.pointer("/sources/0/reason")!=Some(&json!("disabled")) { return Err("restricted VAERS data escaped policy admission".into()); }
        }
        if let Some(page) = page {
            let mut args = vec!["--disable-sources".into(),switch.into()];
            let next:Vec<String>=serde_json::from_value(page.clone()).map_err(|e|e.to_string())?;args.extend(next);
            both(&env,args,4)?;
        }
    }
    if stub.finish()?.len()!=1 { return Err("disabled policy or replay fetched native table".into()); }
    Ok(())
}
pub(super) fn default_agent() -> Check {
    let stub = Stub::for_source("api.fda.gov","application/json",|request|{
        if request.to_ascii_lowercase().contains("wget/1.21.4") { return Err("WONDER user agent leaked to FDA".into()); }
        Ok("synthetic broken JSON".into())
    })?;
    let env=environment(&stub)?;
    invoke(&env,["search","adverse-event","Synthetic drug"].map(str::to_owned).to_vec(),4)?;
    if stub.finish()?.len()!=1 { return Err("default source header test did not send exactly once".into()); }
    Ok(())
}
