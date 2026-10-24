use crate::{Ctx,Outcome,Action,gene,status};
use biodata::{ArticleSearch,GeneResolution,GeneInput,Source,Status,Request,StepKind};
use biomcp_fetch::{Deadline,RequestError};
use biomcp_adapters::hgnc;
pub(super) fn resolve(cx:&Ctx<'_>,query:&ArticleSearch,deadline:Deadline<'_>,out:&mut Outcome)->Result<Option<String>,RequestError> {
    let words=query.words();
    let Some(input)=query.gene() else {return Ok(Some(words));};
    let leg=cx.fetch.leg(&hgnc::SPEC,deadline).fresh(cx.fresh);
    let resolved=gene::resolve(&leg,input);
    out.sources.push(status::source(Source::Hgnc,leg.finish(),resolved.as_ref().err().copied()));
    let mapped=resolved?;status::append(&mut out.problems,mapped.problems);
    match mapped.value {
        GeneResolution::Resolved{gene,..}=>{
            let mut names=vec![gene.symbol.to_string()];
            let typed=match input {GeneInput::Text(text)=>Some(text.as_str()),_=>None};
            for name in typed.into_iter().map(str::to_owned).chain(gene.previous_symbols.iter().map(ToString::to_string)) {
                if !names.iter().any(|n|n.eq_ignore_ascii_case(&name)){names.push(name);}
            }
            let group=if names.len()>1 {format!("({})",names.join(" OR "))}else{names.join("")};
            Ok(Some(if words.is_empty(){group}else{format!("{group} AND {words}")}))
        },
        GeneResolution::Ambiguous{candidates}=>{
            for candidate in candidates.iter().take(5) {
                let value=serde_json::to_value(query).map_err(|_|biomcp_fetch::CacheError::Database)?;
                let mut value=value;value["gene"]=candidate.hgnc_id.to_string().into();
                let checked=serde_json::from_value(value).map_err(|_|biomcp_fetch::CacheError::Database)?;
                out.next.push((StepKind::Disambiguate,Action::Run{request:Request::SearchArticle(checked),fresh:cx.fresh}));
            }
            Ok(None)
        },
        _=>{out.status=Status::Absent;Ok(None)}
    }
}
