//! Reverse the same embedded query schema used by the parser.

use biomcp_query::{Action, Bundle};
use biomcp_types::{Request, Step, StepKind};
use serde_json::Value;

pub(crate) fn step(kind: StepKind, action: &Action) -> Result<Step, serde_json::Error> {
    Ok(Step {
        kind,
        args: action_args(action)?
            .into_iter()
            .map(String::into_boxed_str)
            .collect(),
    })
}
pub(crate) fn action_args(action: &Action) -> Result<Vec<String>, serde_json::Error> {
    match action {
        Action::PubmedSearch(q) => {
            let mut args = request_args(&Request::SearchArticle(q.clone()))?;
            if !args.iter().any(|arg| arg == "--source") {
                args.extend(["--source".into(), "pubmed".into()]);
            }
            Ok(args)
        }
        Action::AuthorPapers(q) => request_args(&Request::AuthorPapers(q.clone())),
        Action::QueryHelp(request) => Ok(std::iter::once("help".into())
            .chain(command_words(request))
            .collect()),
        Action::StoreClean => Ok(vec!["store".into(), "clean".into()]),
        Action::Health => Ok(vec!["health".into()]),
        Action::KeyHelp(source) => Ok(vec!["help".into(), "key".into(), source.to_string()]),
        Action::Sync { bundle } => Ok(vec![
            "sync".into(),
            match bundle {
                Bundle::WikiPathways => "wikipathways",
                Bundle::CbioportalStudies => "cbioportal-studies",
                Bundle::FdaCdx => "fda-cdx",
                Bundle::Gtr => "gtr",
                Bundle::WhoIvd => "who-ivd",
                Bundle::Ema => "ema",
                Bundle::WhoPq => "who-pq",
                Bundle::Ddinter => "ddinter",
                Bundle::Hgnc => "hgnc",
                Bundle::ClinGen => "clingen",
            }
            .into(),
        ]),
        Action::Run { request, fresh } => {
            let mut args = request_args(request)?;
            if *fresh {
                let position = args
                    .iter()
                    .position(|arg| arg == "--")
                    .unwrap_or(args.len());
                args.insert(position, "--fresh".into());
            }
            Ok(args)
        }
    }
}
fn request_args(request: &Request) -> Result<Vec<String>, serde_json::Error> {
    let schema: Value = serde_json::from_str(
        crate::schema::find(crate::grammar::query_schema(request)).unwrap_or("{}"),
    )?;
    let query = query_value(request)?;
    let mut args = command_words(request);
    let positional = fields(&schema, &query, &mut args);
    if positional.iter().any(|arg| arg.starts_with('-')) {
        args.push("--".into());
        args.extend(positional);
    } else {
        args.splice(2..2, positional);
    }
    Ok(args)
}

fn query_value(request: &Request) -> Result<Value, serde_json::Error> {
    match request {
        Request::AuthorPapers(q) => serde_json::to_value(q),
        Request::SearchAuthor(q) => serde_json::to_value(q),
        Request::GetAuthor(q) => serde_json::to_value(q),
        Request::CellLineAssayCount(q) => serde_json::to_value(q),
        Request::SearchCellLine(q) => serde_json::to_value(q),
        Request::GetCellLine(q) => serde_json::to_value(q),
        Request::GetGwasAssociation(q) => serde_json::to_value(q),
        Request::GetGwasStudy(q) => serde_json::to_value(q),
        Request::SearchGwas(q) => serde_json::to_value(q),
        Request::ProteinDomains(q) => serde_json::to_value(q),
        Request::ProteinComplexMembers(q) => serde_json::to_value(q),
        Request::ProteinFeatures(q) => serde_json::to_value(q),
        Request::ProteinIsoforms(q)
        | Request::ProteinStructures(q)
        | Request::ProteinComplexes(q) => serde_json::to_value(q),
        Request::ProteinSequence(q) => serde_json::to_value(q),
        Request::SearchProtein(q) => serde_json::to_value(q),
        Request::GetProtein(q) => serde_json::to_value(q),
        Request::SearchPhenotype(q) => serde_json::to_value(q),
        Request::GetPhenotype(q) => phenotype_value(q),
        Request::SearchPathway(q) => serde_json::to_value(q),
        Request::GetPathway(q) => serde_json::to_value(q),
        Request::PathwayGenes(q) => serde_json::to_value(q),
        Request::PathwayEvents(q) => serde_json::to_value(q),
        Request::SearchStudy(q) => serde_json::to_value(q),
        Request::GetStudy(q) => serde_json::to_value(q),
        Request::SearchPgx(query) => serde_json::to_value(query),
        Request::GetPgx(query) => {
            let mut value = serde_json::to_value(query)?;
            if query.section_limit() == query.default_section_limit()
                && let Some(fields) = value.as_object_mut()
            {
                fields.remove("section_limit");
            }
            Ok(value)
        }
        Request::GetPgxAnnotation(query) => {
            let mut value = serde_json::to_value(query)?;
            if let Some(object) = value.as_object_mut()
                && let Some(id) = object.remove("id")
            {
                object.insert("annotation_id".into(), id);
            }
            Ok(value)
        }
        Request::SearchDiagnostic(q) => serde_json::to_value(q),
        Request::GetDiagnostic(q) => serde_json::to_value(q),
        Request::SearchAdverseEvent(query) => serde_json::to_value(query),
        Request::GetAdverseEvent(query) => serde_json::to_value(query),
        Request::SearchDisease(query) => serde_json::to_value(query),
        Request::GetDisease(query) => serde_json::to_value(query),
        Request::SearchDrug(q) => serde_json::to_value(q),
        Request::GetDrug(q) => serde_json::to_value(q),
        Request::GetWhoMedicine(q) => serde_json::to_value(q),
        Request::VariantAssertions(query) => serde_json::to_value(query),
        Request::VariantPopulation(query) => serde_json::to_value(query),
        Request::VariantRecurrence(query) => serde_json::to_value(query),
        Request::SearchVariant(query) => serde_json::to_value(query),
        Request::VariantNormalize(query) => serde_json::to_value(query),
        Request::GetVariant(query) => serde_json::to_value(query),
        Request::TrialDocument(query) => serde_json::to_value(query),
        Request::GetTrial(query) => serde_json::to_value(query),
        Request::SearchTrial(query) => serde_json::to_value(query),
        Request::ArticlePdf(query) => serde_json::to_value(query),
        Request::ArticleEvidence(query) => serde_json::to_value(query),
        Request::ArticleRecommend(query) => serde_json::to_value(query),
        Request::ArticleSupplement(query) => serde_json::to_value(query),
        Request::ArticleText(query) => serde_json::to_value(query),
        Request::ArticleCitations(query) | Request::ArticleReferences(query) => {
            serde_json::to_value(query)
        }
        Request::GetArticle(query) => serde_json::to_value(query),
        Request::SearchArticle(query) => serde_json::to_value(query),
        Request::GetGene(query) => serde_json::to_value(query),
        Request::SearchGene(query) => serde_json::to_value(query),
    }
}

fn trailing_input(schema: &Value, query: &Value, property: &Value) -> bool {
    let item = property.get("items").unwrap_or(&Value::Null);
    let item = item
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|s| schema.pointer(s.trim_start_matches('#')))
        .unwrap_or(item);
    query
        .get("input")
        .and_then(Value::as_array)
        .is_some_and(|inputs| {
            inputs.iter().any(|input| {
                item.get("enum")
                    .and_then(Value::as_array)
                    .is_some_and(|words| words.contains(input))
            })
        })
}

pub(crate) fn command_words(request: &Request) -> Vec<String> {
    vec![command_verb(request).into(), command_entity(request).into()]
}
fn command_verb(request: &Request) -> &'static str {
    match request {
        Request::CellLineAssayCount(_) => "cell-line",
        Request::PathwayGenes(_) | Request::PathwayEvents(_) => "pathway",
        Request::ProteinFeatures(_)
        | Request::ProteinIsoforms(_)
        | Request::ProteinDomains(_)
        | Request::ProteinComplexes(_)
        | Request::ProteinComplexMembers(_)
        | Request::ProteinStructures(_)
        | Request::ProteinSequence(_) => "protein",
        Request::VariantAssertions(_)
        | Request::VariantPopulation(_)
        | Request::VariantRecurrence(_)
        | Request::VariantNormalize(_) => "variant",
        Request::AuthorPapers(_) => "author",
        Request::TrialDocument(_) => "trial",
        Request::ArticlePdf(_)
        | Request::ArticleEvidence(_)
        | Request::ArticleRecommend(_)
        | Request::ArticleSupplement(_)
        | Request::ArticleText(_)
        | Request::ArticleCitations(_)
        | Request::ArticleReferences(_) => "article",
        Request::GetAuthor(_)
        | Request::GetCellLine(_)
        | Request::GetGwasAssociation(_)
        | Request::GetGwasStudy(_)
        | Request::GetProtein(_)
        | Request::GetPhenotype(_)
        | Request::GetPathway(_)
        | Request::GetStudy(_)
        | Request::GetPgx(_)
        | Request::GetPgxAnnotation(_)
        | Request::GetDiagnostic(_)
        | Request::GetAdverseEvent(_)
        | Request::GetDisease(_)
        | Request::GetWhoMedicine(_)
        | Request::GetDrug(_)
        | Request::GetVariant(_)
        | Request::GetTrial(_)
        | Request::GetGene(_)
        | Request::GetArticle(_) => "get",
        Request::SearchAuthor(_)
        | Request::SearchCellLine(_)
        | Request::SearchGwas(_)
        | Request::SearchProtein(_)
        | Request::SearchPhenotype(_)
        | Request::SearchPathway(_)
        | Request::SearchStudy(_)
        | Request::SearchPgx(_)
        | Request::SearchDiagnostic(_)
        | Request::SearchAdverseEvent(_)
        | Request::SearchDisease(_)
        | Request::SearchDrug(_)
        | Request::SearchVariant(_)
        | Request::SearchTrial(_)
        | Request::SearchGene(_)
        | Request::SearchArticle(_) => "search",
    }
}
pub(crate) fn command_entity(request: &Request) -> &'static str {
    match request {
        Request::AuthorPapers(_) => "papers",
        Request::CellLineAssayCount(_) => "assay-count",
        Request::ProteinFeatures(_) => "features",
        Request::ProteinIsoforms(_) => "isoforms",
        Request::ProteinDomains(_) => "domains",
        Request::ProteinComplexes(_) => "complexes",
        Request::ProteinComplexMembers(_) => "complex-members",
        Request::ProteinStructures(_) => "structures",
        Request::ProteinSequence(_) => "sequence",
        Request::PathwayGenes(_) => "genes",
        Request::PathwayEvents(_) => "events",
        _ => base_entity(request),
    }
}
fn base_entity(request: &Request) -> &'static str {
    if matches!(request, Request::SearchAuthor(_) | Request::GetAuthor(_)) {
        return "author";
    }
    if matches!(
        request,
        Request::SearchCellLine(_) | Request::GetCellLine(_)
    ) {
        return "cell-line";
    }
    if matches!(
        request,
        Request::SearchGwas(_) | Request::GetGwasAssociation(_) | Request::GetGwasStudy(_)
    ) {
        return "gwas";
    }
    other_entity(request)
}
fn other_entity(request: &Request) -> &'static str {
    if matches!(request, Request::SearchProtein(_) | Request::GetProtein(_)) {
        "protein"
    } else if matches!(request, Request::SearchStudy(_) | Request::GetStudy(_)) {
        "study"
    } else if matches!(
        request,
        Request::SearchPhenotype(_) | Request::GetPhenotype(_)
    ) {
        "phenotype"
    } else if matches!(request, Request::SearchPathway(_) | Request::GetPathway(_)) {
        "pathway"
    } else if matches!(
        request,
        Request::SearchPgx(_) | Request::GetPgx(_) | Request::GetPgxAnnotation(_)
    ) {
        "pgx"
    } else if matches!(
        request,
        Request::SearchDiagnostic(_) | Request::GetDiagnostic(_)
    ) {
        "diagnostic"
    } else if matches!(
        request,
        Request::SearchAdverseEvent(_) | Request::GetAdverseEvent(_)
    ) {
        "adverse-event"
    } else if matches!(request, Request::SearchDisease(_) | Request::GetDisease(_)) {
        "disease"
    } else if matches!(
        request,
        Request::SearchDrug(_) | Request::GetWhoMedicine(_) | Request::GetDrug(_)
    ) {
        "drug"
    } else if matches!(request, Request::VariantAssertions(_)) {
        "assertions"
    } else if matches!(request, Request::VariantPopulation(_)) {
        "population"
    } else if matches!(request, Request::VariantRecurrence(_)) {
        "recurrence"
    } else if matches!(request, Request::VariantNormalize(_)) {
        "normalize"
    } else if matches!(request, Request::TrialDocument(_)) {
        "document"
    } else if matches!(request, Request::SearchTrial(_)) {
        "trial"
    } else if matches!(request, Request::GetVariant(_) | Request::SearchVariant(_)) {
        "variant"
    } else if matches!(request, Request::ArticlePdf(_)) {
        "pdf"
    } else if matches!(request, Request::ArticleEvidence(_)) {
        "citation-evidence"
    } else if matches!(request, Request::ArticleRecommend(_)) {
        "recommendations"
    } else if matches!(request, Request::ArticleSupplement(_)) {
        "supplement"
    } else if matches!(request, Request::ArticleText(_)) {
        "text"
    } else if matches!(request, Request::ArticleCitations(_)) {
        "citations"
    } else if matches!(request, Request::ArticleReferences(_)) {
        "references"
    } else if matches!(request, Request::GetTrial(_) | Request::SearchTrial(_)) {
        "trial"
    } else if matches!(request, Request::SearchArticle(_) | Request::GetArticle(_)) {
        "article"
    } else {
        "gene"
    }
}

fn fields(schema: &Value, query: &Value, args: &mut Vec<String>) -> Vec<String> {
    let mut positional = Vec::new();
    let mut trailing = Vec::new();
    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        let mut fields: Vec<_> = properties.iter().collect();
        fields.sort_by_key(|(_, property)| {
            property
                .get("x-positional")
                .and_then(Value::as_u64)
                .unwrap_or(u64::MAX)
        });
        for (name, property) in fields {
            let Some(value) = query
                .get(name)
                .filter(|value| !value.is_null() && property.get("default") != Some(*value))
            else {
                continue;
            };
            if value.is_boolean() {
                if value == &Value::Bool(true) {
                    args.push(format!("--{}", name.replace('_', "-")));
                }
                continue;
            }
            let is_trailing = property.get("x-trailing-section") == Some(&Value::Bool(true));
            let explicit = is_trailing && trailing_input(schema, query, property);
            if property.get("x-positional").is_none() && !is_trailing {
                args.push(format!("--{}", name.replace('_', "-")));
            }
            if property.get("x-comma-separated") == Some(&Value::Bool(true)) {
                args.push(
                    value
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join(","),
                );
                continue;
            }
            for value in value
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or(std::slice::from_ref(value))
            {
                let target = if is_trailing && !explicit {
                    &mut trailing
                } else if property.get("x-positional").is_some() {
                    &mut positional
                } else {
                    &mut *args
                };
                if explicit {
                    target.push(format!("--{}", name.replace('_', "-")));
                }
                target.push(
                    value
                        .as_str()
                        .map_or_else(|| value.to_string(), str::to_owned),
                );
            }
        }
    }
    positional.extend(trailing);
    positional
}

fn phenotype_value(q: &biomcp_types::PhenotypeGet) -> Result<Value, serde_json::Error> {
    let mut value = serde_json::to_value(q)?;
    if let Some(fields) = value.as_object_mut() {
        if q.source() == q.default_source() {
            fields.remove("source");
        }
        if q.section_limit() == q.default_section_limit() {
            fields.remove("section_limit");
        }
    }
    Ok(value)
}
