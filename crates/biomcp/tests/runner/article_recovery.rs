//! Article Markdown presents dropped-word notices before article records.
use super::launch::Environment;
use libtest_mimic::Trial;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "article-search/dropped-words-precede-articles",
        || check().map_err(Into::into),
    )]
}

fn check() -> Result<(), String> {
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
