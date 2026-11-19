//! PubMed wire terms preserve structured selectors and resolved gene names.
use biodata::Pmid;
use biomcp_fetch::{Arg, Endpoint, Freshness, Input, Leg, Read, RequestError, Secret};
use biomcp_types::{ArticleInput, ArticleSearch, ArticleType};
/// Search typed selectors and retain the exact provider term for disclosure.
pub fn search_query(
    leg: &Leg<'_>,
    query: &ArticleSearch,
    names: &[String],
    email: Option<&Secret>,
) -> (Result<super::SearchIds, RequestError>, String) {
    let words = query.words();
    let group = if names.len() > 1 {
        format!("({})", names.join(" OR "))
    } else {
        names.join("")
    };
    let words = if group.is_empty() {
        words
    } else if words.is_empty() {
        group
    } else {
        format!("{group} AND {words}")
    };
    let term = filters(query, words);
    (super::search_ids(leg, &term, query.sort(), email), term)
}
fn filters(query: &ArticleSearch, words: String) -> String {
    let mut parts: Vec<String> = (!words.is_empty()).then_some(words).into_iter().collect();
    if let Some(author) = query.author() {
        parts.push(format!("{author}[au]"));
    }
    if let Some(journal) = query.journal() {
        parts.push(format!("{journal}[ta]"));
    }
    // Quoted, so words such as OR stay part of the institution name.
    if let Some(affiliation) = query.affiliation() {
        parts.push(format!("\"{}\"[ad]", affiliation.as_str().replace('"', "")));
    }
    let (since, until) = query.dates();
    if since.is_some() || until.is_some() {
        let date = |value: Option<biodata::PartialDate>, default: &str| {
            value.map_or_else(|| default.to_owned(), |v| v.to_string().replace('-', "/"))
        };
        parts.push(format!(
            "{}:{}[dp]",
            date(since, "1800"),
            date(until, "3000")
        ));
    }
    if let Some(kind) = query.kind() {
        parts.push(
            match kind {
                ArticleType::Trial => "\"clinical trial\"[pt]",
                ArticleType::Randomized => "\"randomized controlled trial\"[pt]",
                ArticleType::Review => "review[pt]",
                ArticleType::SystematicReview => "\"systematic review\"[pt]",
                ArticleType::MetaAnalysis => "\"meta-analysis\"[pt]",
                ArticleType::Guideline => "(guideline[pt] OR \"practice guideline\"[pt])",
                ArticleType::CaseReport => "\"case reports\"[pt]",
            }
            .to_owned(),
        );
    }
    let mut term = parts.join(" AND ");
    if query.open_access() {
        term.push_str(" AND \"pubmed pmc open access\"[filter]");
    }
    if query.no_preprints() {
        term.push_str(" NOT preprint[pt]");
    }
    if query.exclude_retracted() {
        term.push_str(" NOT retracted publication[pt]");
    }
    term
}

/// Resolve an explicit DOI group with native PubMed syntax and request identities.
pub fn search_dois(
    leg: &biomcp_fetch::Leg<'_>,
    batch: &[biodata::Doi],
    email: Option<&biomcp_fetch::Secret>,
) -> Result<super::SearchIds, RequestError> {
    let term = batch
        .iter()
        .map(|doi| {
            format!(
                "\"{}\"[doi]",
                doi.to_string()
                    .to_ascii_lowercase()
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
            )
        })
        .collect::<Vec<_>>()
        .join(" OR ");
    let inputs = Some(biomcp_fetch::Identities {
        namespace: "doi".into(),
        ids: batch.iter().map(ToString::to_string).collect(),
    });
    super::search_ids_with_inputs(leg, &term, biomcp_types::ArticleSort::Rank, email, inputs)
}

const RESOLVE: Endpoint = super::SPEC.get("/esearch.fcgi").fresh(Freshness::Days(1));
/// Resolve one namespace and retain distinct source matches.
pub fn resolve_ids(
    leg: &Leg<'_>,
    input: &ArticleInput,
    email: Option<&Secret>,
) -> Result<Vec<Pmid>, RequestError> {
    let term = match input {
        ArticleInput::EuropePmc(_) => return Ok(Vec::new()),
        ArticleInput::Pmid(id) => return Ok(vec![*id]),
        ArticleInput::Pmcid(id) => format!("{id}[pmc]"),
        ArticleInput::Doi(id) => format!("{id}[doi]"),
    };
    let mut params = vec![
        ("db", Arg::Text("pubmed")),
        ("term", Arg::Text(&term)),
        ("retmax", Arg::Num(9999)),
        ("retmode", Arg::Text("json")),
        ("tool", Arg::Text("biomcp")),
    ];
    if let Some(email) = email {
        params.push(("email", Arg::Contact(email)));
    }
    Ok(leg
        .call(
            &RESOLVE,
            Input::Params(&params),
            Read::<super::raw::Search>::json(),
        )
        .map(|received| received.value)?
        .esearchresult
        .idlist)
}

/// PMIDs of `ids` in PubMed Central's open-access subset, from one ESearch with NCBI's
/// `"pubmed pmc open access"[filter]`. Rows not returned are not in that subset.
pub fn open_access(
    leg: &Leg<'_>,
    ids: &[Pmid],
    email: Option<&Secret>,
) -> Result<Vec<Pmid>, RequestError> {
    let group = ids
        .iter()
        .map(|id| format!("{id}[uid]"))
        .collect::<Vec<_>>()
        .join(" OR ");
    let term = format!("({group}) AND \"pubmed pmc open access\"[filter]");
    let inputs = Some(biomcp_fetch::Identities {
        namespace: "pmid".into(),
        ids: ids.iter().map(ToString::to_string).collect(),
    });
    let found =
        super::search_ids_with_inputs(leg, &term, biomcp_types::ArticleSort::Date, email, inputs)?;
    Ok(found
        .ids
        .into_iter()
        .filter(|id| ids.contains(id))
        .collect())
}
