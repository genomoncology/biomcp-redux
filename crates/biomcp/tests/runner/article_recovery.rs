//! Article Markdown presents dropped-word notices before article records.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::json;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("article-search/dropped-words-precede-articles", || {
            check().map_err(Into::into)
        }),
        transcript_trial("article-fold/failed-pubmed-keeps-europepmc-answer", || {
            secondary_answer().map_err(Into::into)
        }),
    ]
}

fn check() -> Result<(), String> {
    warnings()?;
    let format = "md";
    let environment = Environment::new().map_err(|error| error.to_string())?;
    let replay = super::launch::fixtures().join("replies/article/search/synthetic-dropped-words");
    let mut args = environment.globals(Some(&replay), false);
    args.extend(["search", "article", "AKT1 E17K", "--format", format].map(str::to_owned));
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|error| error.to_string())?;
    let text = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    let (notice, articles) = ("PubMed dropped supplied words", "## Result");
    let notice = text
        .find(notice)
        .ok_or_else(|| format!("{format}: notice missing"))?;
    let articles = text
        .find(articles)
        .ok_or_else(|| format!("{format}: articles missing"))?;
    if !output.status.success() || notice >= articles {
        return Err(format!(
            "{format}: dropped-word notice must precede articles"
        ));
    }
    Ok(())
}

fn warnings() -> Result<(), String> {
    for phrase in [
        "ERCC2 Ala717Gly",
        "GATA2 Thr176Pro",
        "POT1 Gly404Val",
        "GALC Thr109Ala",
        "ACD Pro457=",
    ] {
        let environment = Environment::new().map_err(|e| e.to_string())?;
        let replay =
            super::launch::fixtures().join("replies/article/search/synthetic-pubmed-warnings");
        let mut args = environment.globals(Some(&replay), false);
        args.extend([
            "search".into(),
            "article".into(),
            format!("\"{phrase}\""),
            "--source".into(),
            "pubmed".into(),
            "--format".into(),
            "md".into(),
        ]);
        let output = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        let result = text.find("## Result").ok_or("article result missing")?;
        let warning = text
            .find("**PubMed warning quotedphrasesnotfound:**")
            .ok_or("quoted phrase warning missing")?;
        if !output.status.success()
            || warning >= result
            || !text.starts_with("**PubMed warning ")
            || !text.contains(phrase)
        {
            return Err(format!(
                "{phrase}: source warning must lead Markdown: {text}"
            ));
        }
    }
    Ok(())
}

fn secondary_answer() -> Result<(), String> {
    super::response_cases::checked_case(
        "article/fold/synthetic-fold",
        &[
            "search",
            "article",
            "AKT1",
            "--author",
            "Carpten",
            "--source",
            "pubmed,europepmc",
        ],
        |source, record| {
            if source == "pubmed"
                && record["url"]
                    .as_str()
                    .is_some_and(|url| url.contains("/esearch.fcgi"))
                && let Some(exchanges) = record["exchanges"].as_array_mut()
            {
                for exchange in exchanges.iter_mut() {
                    exchange["status"] = json!(500);
                }
                // Each transport attempt receives an actual provider failure.
                if let Some(exchange) = exchanges.first().cloned() {
                    exchanges.resize(3, exchange);
                }
            }
        },
        json!({
            "/status": "partial",
            "/sources/0/source": "pubmed",
            "/sources/0/state": "failed",
            "/sources/0/reason": "unavailable",
            "/sources/1/source": "europepmc",
            "/sources/1/state": "ok",
            "/result/items/0/key": {"namespace":"europepmc", "id":"PPR8888"},
            "/result/items/0/header/title": "Synthetic source-local citation"
        }),
        0,
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}
