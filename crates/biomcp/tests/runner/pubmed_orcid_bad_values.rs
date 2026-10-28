//! Features article-get, article-search and author-orcid: invalid optional fields retain source records.
use super::super::{configuration::invoke, network_stub::Stub, response_cases::checked_case};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("author-orcid/invalid-optional-fields-are-disclosed", || {
            orcid()
                .and_then(|_| person())
                .and_then(|_| stored_work_problems())
                .map_err(Into::into)
        }),
        Trial::test(
            "article-get/pubmed-invalid-optional-fields-are-disclosed",
            || pubmed().map_err(Into::into),
        ),
        Trial::test(
            "article-search/pubmed-invalid-optional-fields-are-disclosed",
            || {
                summaries()
                    .and_then(|_| notices())
                    .and_then(|_| stored_notice_problems())
                    .map_err(Into::into)
            },
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
    for (path, invalid, field) in work_values().into_iter().chain(work_source_values()) {
        let (parent, key) = path.rsplit_once('/').ok_or("invalid work fixture path")?;
        let value = checked_case(
            "orcid/visibility",
            &["author", "papers", "orcid:0000-0002-4299-3018"],
            |source, capture| {
                if source == "orcid"
                    && let Some(assertion) =
                        capture.pointer_mut("/exchanges/0/body/value/group/0/work-summary/0")
                    && let Some(object) =
                        assertion.pointer_mut(parent).and_then(Value::as_object_mut)
                {
                    object.insert(key.into(), invalid.clone());
                }
            },
            json!({"/result/groups/0/assertions/0/put_code":1}),
            0,
        )
        .map_err(|e| e.to_string())?;
        problem(&value, "orcid", field, "/0000-0002-4299-3018/work/1")?;
        let path = if field == "external_ids.url" {
            "external_ids/0/url".into()
        } else if field == "external_ids.relationship" {
            "external_ids/0/relationship".into()
        } else {
            field.replace('.', "/")
        };
        if field != "external_ids"
            && value
                .pointer(&format!("/result/groups/0/assertions/0/{path}"))
                .is_some_and(|v| !v.is_null())
        {
            return Err(format!("invalid ORCID field survived: {value}"));
        }
    }
    Ok(())
}
fn work_values() -> Vec<(&'static str, Value, &'static str)> {
    vec![
        ("/type", json!(""), "work_type"),
        (
            "/external-ids/external-id/0/external-id-relationship",
            json!(""),
            "external_ids.relationship",
        ),
        (
            "/external-ids/external-id/0/external-id-type",
            json!(""),
            "external_ids",
        ),
        (
            "/external-ids/external-id/0/external-id-value",
            json!(""),
            "external_ids",
        ),
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
    ]
}
fn work_source_values() -> Vec<(&'static str, Value, &'static str)> {
    vec![
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
    ]
}
fn pubmed() -> Result<(), String> {
    for (field, extra) in article_values() {
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
                concat!(
                    "<MeshHeadingList><MeshHeading><DescriptorName MajorTopicYN=\"N\">Valid descriptor</DescriptorName>",
                    "<QualifierName MajorTopicYN=\"N\"> </QualifierName></MeshHeading></MeshHeadingList>"
                )
            }
            _ => "",
        };
        let title = if field == "title" { " " } else { "Valid title" };
        let xml = format!(
            concat!(
                "<PubmedArticleSet><PubmedArticle><MedlineCitation><PMID>123</PMID><Article>",
                "<ArticleTitle>{}</ArticleTitle>{}</Article>{}</MedlineCitation>",
                "<PubmedData><ArticleIdList>{}</ArticleIdList></PubmedData></PubmedArticle></PubmedArticleSet>"
            ),
            title, extra, citation, ids
        );
        let stub = Stub::for_sources(
            &["eutils.ncbi.nlm.nih.gov"],
            |_| "application/xml",
            move |_| Ok(xml.clone()),
        )?;
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
        let field = if field == "date_prose" { "date" } else { field };
        problem(&value, "pubmed", field, "123")?;
        if field == "title" {
            super::super::schema::Registry::embedded()?.validate(&value, &[], true)?;
            if value.pointer("/result/items/0/answer/card/title").is_some() {
                return Err("bad title survived".into());
            }
            let output = env
                .command(
                    env!("CARGO_BIN_EXE_biomcp"),
                    &["get", "article", "123", "--offline", "--format", "md"].map(str::to_owned),
                )
                .output()
                .map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&output.stdout);
            if !output.status.success() || !text.contains("123") || text.contains("Valid title") {
                return Err(format!("identity-only article Markdown lost: {text}"));
            }
        }
        if stub.finish()?.len() != 1 {
            return Err("unexpected PubMed requests".into());
        }
    }
    Ok(())
}
fn article_values() -> Vec<(&'static str, &'static str)> {
    vec![
        ("title", ""),
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
            "date_prose",
            "<Journal><JournalIssue><PubDate><MedlineDate>2024 Spring</MedlineDate></PubDate></JournalIssue></Journal>",
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
            concat!(
                "<AuthorList><Author><CollectiveName>Valid author</CollectiveName>",
                "<AffiliationInfo><Affiliation> </Affiliation></AffiliationInfo></Author></AuthorList>"
            ),
        ),
        (
            "orcids",
            "<AuthorList><Author><CollectiveName>Valid author</CollectiveName><Identifier Source=\"ORCID\"> </Identifier></Author></AuthorList>",
        ),
    ]
}
fn person() -> Result<(), String> {
    for field in ["credit-name", "given-names", "family-name", "all"] {
        let mut name = json!({
            "path":"0000-0002-4299-3018", "visibility":"public", "credit-name":{"value":"Credit"},
            "given-names":{"value":"Given"}, "family-name":{"value":"Family"}
        });
        if field == "all" {
            for key in ["credit-name", "given-names", "family-name"] {
                *name.get_mut(key).ok_or("missing name fixture field")? = json!({"value":""});
            }
        } else {
            *name.get_mut(field).ok_or("missing name fixture field")? = json!({"value":""});
        }
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
        if field == "all" {
            super::super::schema::Registry::embedded()?.validate(&value, &[], true)?;
            if value.pointer("/result/items/0/card/header/id")
                != Some(&json!("orcid:0000-0002-4299-3018"))
                || value.pointer("/result/items/0/card/header/name").is_some()
            {
                return Err(format!("identity-only author lost: {value}"));
            }
        } else {
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
        }
        stub.finish()?;
    }
    Ok(())
}
fn summaries() -> Result<(), String> {
    for (field, invalid, problem_field) in [
        ("title", json!(""), "title"),
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
        row.as_object_mut()
            .ok_or("invalid summary fixture")?
            .insert(field.into(), invalid);
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
fn stored_work_problems() -> Result<(), String> {
    let value = super::super::response_cases::checked_steps(
        "orcid/visibility",
        |_, capture| {
            if let Some(title) = capture
                .pointer_mut("/exchanges/0/body/value/group/0/work-summary/0/title/title/value")
            {
                *title = json!("");
            }
        },
        json!([
            {"run":["author","papers","orcid:0000-0002-4299-3018"], "exit":0,
             "expect":{"/problems/0/field":"title"}},
            {"run":["author","papers","orcid:0000-0002-4299-3018"], "offline":true, "exit":0,
             "expect":{"/problems/0/field":"title", "/result/groups/0/assertions/0/put_code":1}}
        ]),
    )
    .map_err(|e| e.to_string())?;
    problem(&value, "orcid", "title", "/0000-0002-4299-3018/work/1")
}
fn notices() -> Result<(), String> {
    for (extra, field) in [
        (
            json!({"errorlist":{"phrasesnotfound":[""]}}),
            "notices.not_found_words",
        ),
        (
            json!({"errorlist":{"phrasesnotfound":[{}]}}),
            "notices.not_found_words",
        ),
        (
            json!({"errorlist":{"phrasesnotfound":{}}}),
            "notices.not_found_words",
        ),
        (json!({"errorlist":[]}), "notices.not_found_words"),
        (json!({"warninglist":[]}), "notices.pubmed_warnings"),
        (
            json!({"warninglist":{"quotedphrasesnotfound":{}}}),
            "notices.pubmed_warning.phrases",
        ),
        (
            json!({"warninglist":{"quotedphrasesnotfound":[{}]}}),
            "notices.pubmed_warning.phrases",
        ),
    ] {
        let mut result = json!({"count":"0","idlist":[]});
        let extra = extra.as_object().ok_or("invalid notice fixture fields")?;
        result
            .as_object_mut()
            .ok_or("invalid notice fixture result")?
            .extend(extra.clone());
        let body = json!({"esearchresult":result}).to_string();
        let stub = Stub::for_sources(
            &["eutils.ncbi.nlm.nih.gov"],
            |_| "application/json",
            move |_| Ok(body.clone()),
        )?;
        let env = super::super::variant::source_environment(&stub)?;
        let value = invoke(
            &env,
            ["search", "article", "synthetic", "--source", "pubmed"]
                .map(str::to_owned)
                .to_vec(),
            0,
        )?;
        if !value
            .get("problems")
            .and_then(Value::as_array)
            .is_some_and(|ps| {
                ps.iter().any(|p| {
                    p.get("field") == Some(&json!(field))
                        && p.get("source") == Some(&json!("pubmed"))
                })
            })
        {
            return Err(format!(
                "notice text disappeared without a problem: {value}"
            ));
        }
        if stub.finish()?.len() != 1 {
            return Err("empty PubMed search sent another request".into());
        }
    }
    Ok(())
}
fn stored_notice_problems() -> Result<(), String> {
    for count in [0, 1] {
        let stub = Stub::for_sources(
            &["eutils.ncbi.nlm.nih.gov"],
            |_| "application/json",
            move |request| {
                Ok(if request.contains("esearch.fcgi") {
                    let ids: Vec<_> = if count == 0 { vec![] } else { vec!["123"] };
                    json!({"esearchresult":{"count":count.to_string(), "idlist":ids,
                    "errorlist":{"phrasesnotfound":[{}]}}})
                } else {
                    json!({"result":{"uids":["123"],"123":{"uid":"123","title":"Valid title"}}})
                }
                .to_string())
            },
        )?;
        let env = super::super::variant::source_environment(&stub)?;
        let args = ["search", "article", "synthetic", "--source", "pubmed"]
            .map(str::to_owned)
            .to_vec();
        let first = invoke(&env, args.clone(), 0)?;
        let mut offline = args;
        offline.push("--offline".into());
        let second = invoke(&env, offline, 0)?;
        for pointer in [
            "/problems",
            "/page/pool",
            "/page/total",
            "/status",
            "/result",
        ] {
            if first.pointer(pointer) != second.pointer(pointer) {
                return Err(format!(
                    "stored notice changed {pointer}: {first} -> {second}"
                ));
            }
        }
        if second.pointer("/problems/0/field") != Some(&json!("notices.not_found_words"))
            || second.pointer("/problems/0/record").is_some()
            || second.pointer("/page/total") != Some(&json!(count))
        {
            return Err(format!("pool-scoped notice problem lost: {second}"));
        }
        if stub.finish()?.len() != (count + 1) as usize {
            return Err("cached notice requested the provider".into());
        }
    }
    Ok(())
}
