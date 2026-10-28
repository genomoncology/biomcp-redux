//! Features article-get and author-orcid: invalid optional fields retain source records.
use super::super::{configuration::invoke, network_stub::Stub, response_cases::checked_case};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("author-orcid/invalid-optional-fields-are-disclosed", || {
            orcid().and_then(|_| person()).map_err(Into::into)
        }),
        Trial::test(
            "article-get/pubmed-invalid-optional-fields-are-disclosed",
            || pubmed().and_then(|_| summaries()).map_err(Into::into),
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
        (
            "date",
            "<Journal><JournalIssue><PubDate><Year>bad</Year></PubDate></JournalIssue></Journal>",
        ),
        (
            "electronic_date",
            "<ArticleDate DateType=\"Electronic\"><Year>2024</Year><Month>Feb</Month><Day>31</Day></ArticleDate>",
        ),
        (
            "abstract_parts.text",
            "<Abstract><AbstractText> </AbstractText></Abstract>",
        ),
        ("mesh.descriptor", ""),
        ("mesh.qualifiers.name", ""),
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
            "mesh.descriptor" => {
                "<MeshHeadingList><MeshHeading><DescriptorName MajorTopicYN=\"N\"> </DescriptorName></MeshHeading></MeshHeadingList>"
            }
            "mesh.qualifiers.name" => {
                "<MeshHeadingList><MeshHeading><DescriptorName MajorTopicYN=\"N\">Valid descriptor</DescriptorName><QualifierName MajorTopicYN=\"N\"> </QualifierName></MeshHeading></MeshHeadingList>"
            }
            _ => "",
        };
        let xml = format!(
            "<PubmedArticleSet><PubmedArticle><MedlineCitation><PMID>123</PMID><Article><ArticleTitle>Valid title</ArticleTitle>{extra}</Article>{citation}</MedlineCitation><PubmedData><ArticleIdList>{ids}</ArticleIdList></PubmedData></PubmedArticle></PubmedArticleSet>"
        );
        let stub = Stub::for_sources("eutils.ncbi.nlm.nih.gov", "application/xml", move |_| {
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
fn person() -> Result<(), String> {
    for field in ["credit-name", "given-names", "family-name"] {
        let mut name = json!({"path":"0000-0002-4299-3018", "visibility":"public", "credit-name":{"value":"Credit"}, "given-names":{"value":"Given"}, "family-name":{"value":"Family"}});
        name[field] = json!({"value":""});
        let body = json!({"path":"/0000-0002-4299-3018/person", "name":name}).to_string();
        let stub = Stub::for_sources(
            &["pub.orcid.org"],
            |_| "application/json",
            move |_| Ok(body.clone()),
        )?;
        let env = super::super::variant::source_environment(&stub)?;
        let value = invoke(
            &env,
            ["get", "author", "orcid:0000-0002-4299-3018"]
                .map(str::to_owned)
                .to_vec(),
            0,
        )?;
        problem(
            &value,
            "orcid",
            &field.replace('-', "_"),
            "/0000-0002-4299-3018/person",
        )?;
        let expected = if field == "credit-name" {
            "Given Family"
        } else {
            "Credit"
        };
        if value.pointer("/result/items/0/card/header/name") != Some(&json!(expected)) {
            return Err(format!("name lost: {value}"));
        }
        stub.finish()?;
    }
    Ok(())
}
fn summaries() -> Result<(), String> {
    for (field, invalid, problem_field) in [
        ("fulljournalname", json!(""), "journal"),
        ("pubdate", json!("bad"), "date"),
        ("epubdate", json!("2024 Feb 31"), "electronic_date"),
        ("authors", json!([{"name":""}]), "first_author"),
        ("authors", json!({}), "authors"),
        ("pubtype", json!([""]), "types"),
        (
            "articleids",
            json!([{"idtype":"doi", "value":"bad"}]),
            "doi",
        ),
        (
            "articleids",
            json!([{"idtype":"pmc", "value":"bad"}]),
            "pmcid",
        ),
        ("articleids", json!({}), "articleids"),
    ] {
        let mut row = json!({"uid":"123", "title":"Valid title", "pubdate":"2024 Jan 2", "authors":[{"name":"Valid author"}]});
        row[field] = invalid;
        let summary = json!({"result":{"uids":["123"],"123":row}}).to_string();
        let stub = Stub::for_sources(
            &["eutils.ncbi.nlm.nih.gov"],
            |_| "application/json",
            move |request| {
                Ok(if request.contains("esearch.fcgi") {
                    json!({"esearchresult":{"count":"1","idlist":["123"]}}).to_string()
                } else {
                    summary.clone()
                })
            },
        )?;
        let env = super::super::variant::source_environment(&stub)?;
        let value = invoke(
            &env,
            ["search", "article", "synthetic", "--source", "pubmed"]
                .map(str::to_owned)
                .to_vec(),
            0,
        )?;
        problem(&value, "pubmed", problem_field, "123")?;
        if value.pointer("/result/items/0/header/pmid") != Some(&json!("123")) {
            return Err(format!("summary lost: {value}"));
        }
        if stub.finish()?.len() != 2 {
            return Err("unexpected PubMed search requests".into());
        }
    }
    Ok(())
}
