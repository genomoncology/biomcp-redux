//! Features article-get and author-orcid: invalid optional fields retain source records.
use super::super::{configuration::invoke, network_stub::Stub, response_cases::checked_case};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("author-orcid/invalid-optional-fields-are-disclosed", || {
            orcid().map_err(Into::into)
        }),
        Trial::test(
            "article-get/pubmed-invalid-optional-fields-are-disclosed",
            || pubmed().map_err(Into::into),
        ),
    ]
}
fn problem(value: &Value, source: &str, field: &str, record: &str) -> Result<(), String> {
    if !value
        .get("problems")
        .and_then(Value::as_array)
        .is_some_and(|ps| {
            ps.iter().any(|p| {
                p.get("source") == Some(&json!(source))
                    && p.get("field") == Some(&json!(field))
                    && p.get("record") == Some(&json!(record))
            })
        })
    {
        return Err(format!(
            "missing {source} {field} problem for {record}: {value}"
        ));
    }
    Ok(())
}
fn orcid() -> Result<(), String> {
    for (path, invalid, field) in [
        ("/title/title", json!({"value":""}), "title"),
        ("/title/subtitle", json!({"value":""}), "subtitle"),
        (
            "/title/translated-title",
            json!({"content":""}),
            "translated_title",
        ),
        ("/journal-title", json!({"value":""}), "journal_title"),
        ("/url", json!({"value":""}), "url"),
        ("/display-index", json!(""), "display_index"),
        (
            "/publication-date",
            json!({"year":{"value":"bad"}}),
            "publication_date",
        ),
        (
            "/source/source-name",
            json!({"value":""}),
            "source.source_name",
        ),
        (
            "/source/assertion-origin-name",
            json!({"value":""}),
            "source.assertion_origin_name",
        ),
        (
            "/source/source-orcid",
            json!({"path":"invalid"}),
            "source.source_orcid",
        ),
        (
            "/source/source-client-id",
            json!({"path":"invalid"}),
            "source.source_client_id",
        ),
        (
            "/source/assertion-origin-orcid",
            json!({"path":"invalid"}),
            "source.assertion_origin_orcid",
        ),
        (
            "/source/assertion-origin-client-id",
            json!({"path":"invalid"}),
            "source.assertion_origin_client_id",
        ),
        (
            "/external-ids/external-id/0/external-id-url",
            json!({"value":""}),
            "external_ids.url",
        ),
    ] {
        let value = checked_case(
            "orcid/visibility",
            &["author", "papers", "orcid:0000-0002-4299-3018"],
            |source, capture| {
                if source == "orcid"
                    && let Some(assertion) =
                        capture.pointer_mut("/exchanges/0/body/value/group/0/work-summary/0")
                {
                    let (parent, key) = path.rsplit_once('/').unwrap();
                    if let Some(object) =
                        assertion.pointer_mut(parent).and_then(Value::as_object_mut)
                    {
                        object.insert(key.into(), invalid.clone());
                    }
                }
            },
            json!({"/result/groups/0/assertions/0/put_code":1}),
            0,
        )
        .map_err(|e| e.to_string())?;
        problem(&value, "orcid", field, "/0000-0002-4299-3018/work/1")?;
    }
    Ok(())
}
fn pubmed() -> Result<(), String> {
    for (field, extra) in [
        ("journal", "<Journal><Title></Title></Journal>"),
        ("doi", ""),
        ("pmcid", ""),
        (
            "authors",
            "<AuthorList><Author><CollectiveName> </CollectiveName></Author></AuthorList>",
        ),
        (
            "abstract_parts.label",
            "<Abstract><AbstractText Label=\"\">Valid abstract</AbstractText></Abstract>",
        ),
        ("corrections.pmid", ""),
        ("keywords", ""),
        (
            "affiliations",
            "<AuthorList><Author><CollectiveName>Valid author</CollectiveName><AffiliationInfo><Affiliation> </Affiliation></AffiliationInfo></Author></AuthorList>",
        ),
        (
            "orcids",
            "<AuthorList><Author><CollectiveName>Valid author</CollectiveName><Identifier Source=\"ORCID\"> </Identifier></Author></AuthorList>",
        ),
    ] {
        let ids = match field {
            "doi" => "<ArticleId IdType=\"doi\">bad</ArticleId>",
            "pmcid" => "<ArticleId IdType=\"pmc\">bad</ArticleId>",
            _ => "",
        };
        let citation = match field {
            "corrections.pmid" => {
                "<CommentsCorrectionsList><CommentsCorrections RefType=\"ErratumIn\"><PMID>bad</PMID></CommentsCorrections></CommentsCorrectionsList>"
            }
            "keywords" => "<KeywordList><Keyword> </Keyword></KeywordList>",
            _ => "",
        };
        let xml = format!(
            "<PubmedArticleSet><PubmedArticle><MedlineCitation><PMID>123</PMID><Article><ArticleTitle>Valid title</ArticleTitle>{extra}</Article>{citation}</MedlineCitation><PubmedData><ArticleIdList>{ids}</ArticleIdList></PubmedData></PubmedArticle></PubmedArticleSet>"
        );
        let stub = Stub::for_source("eutils.ncbi.nlm.nih.gov", "application/xml", move |_| {
            Ok(xml.clone())
        })?;
        let env = super::super::variant::source_environment(&stub)?;
        let value = invoke(
            &env,
            ["get", "article", "123", "indexing"]
                .map(str::to_owned)
                .to_vec(),
            0,
        )?;
        if value.pointer("/result/items/0/answer/card/pmid") != Some(&json!("123")) {
            return Err(format!("article lost: {value}"));
        }
        problem(&value, "pubmed", field, "123")?;
        if stub.finish()?.len() != 1 {
            return Err("unexpected PubMed requests".into());
        }
    }
    Ok(())
}
