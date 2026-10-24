//! Schema-fed command parsing shared by CLI and MCP.
use biodata::{ArticleEdges, ArticleGet, ArticleSearch, ArticleText, GeneGet, GeneSearch, Request};
use biomcp_query::{Action, Bundle};
use clap::{Arg, ArgAction, Command};
use serde_json::{Map, Value};
use std::{ffi::OsString, path::PathBuf};

#[derive(Clone, Debug, Default)]
pub(crate) struct Globals {
    pub(crate) config: Option<PathBuf>,
    pub(crate) replay: Option<PathBuf>,
    pub(crate) observe: Option<PathBuf>,
    pub(crate) record: Option<PathBuf>,
    pub(crate) case_cache: Option<PathBuf>,
    pub(crate) offline: bool,
    pub(crate) fresh: bool,
    pub(crate) format: Option<Format>,
    pub(crate) disable_sources: Vec<String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Format {
    Json,
    Md,
}
#[derive(Debug)]
pub(crate) struct Parsed {
    pub(crate) key: Option<crate::keys::Command>,
    pub(crate) action: Option<Action>,
    pub(crate) globals: Globals,
    pub(crate) help: Option<Vec<String>>,
    pub(crate) http: Option<(String, Vec<String>)>,
}
pub(crate) fn query_schema(request: &Request) -> &'static str {
    match request {
        Request::VariantNormalize(_) => "biodata.variant-normalize/1",
        Request::SearchVariant(_) => "biodata.variant-search/1",
        Request::VariantRecurrence(_) => "biodata.variant-recurrence-query/1",
        Request::GetVariant(_) => "biodata.variant-get/1",
        Request::TrialDocument(_) => "biodata.trial-document/1",
        Request::GetTrial(_) => "biodata.trial-get/1",
        Request::SearchTrial(_) => "biodata.trial-search/1",
        Request::ArticlePdf(_) => "biodata.article-pdf/1",
        Request::ArticleEvidence(_) => "biodata.article-evidence/1",
        Request::ArticleRecommend(_) => "biodata.article-recommend/1",
        Request::ArticleSupplement(_) => "biodata.article-supplement/1",
        Request::ArticleText(_) => "biodata.article-text/2",
        Request::ArticleCitations(_) | Request::ArticleReferences(_) => "biodata.article-edges/2",
        Request::GetArticle(_) => "biodata.article-get/2",
        Request::SearchArticle(_) => "biodata.article-search/1",
        Request::GetGene(_) => "biodata.gene-get/1",
        Request::SearchGene(_) => "biodata.gene-search/1",
    }
}
pub(crate) const QUERIES: &[(&str, &str, &str)] = &[
    ("get", "gene", "biodata.gene-get/1"),
    ("search", "gene", "biodata.gene-search/1"),
    ("search", "article", "biodata.article-search/1"),
    ("get", "article", "biodata.article-get/2"),
    ("article", "supplement", "biodata.article-supplement/1"),
    ("article", "pdf", "biodata.article-pdf/1"),
    ("article", "text", "biodata.article-text/2"),
    ("article", "citations", "biodata.article-edges/2"),
    ("article", "references", "biodata.article-edges/2"),
    ("article", "citation-evidence", "biodata.article-evidence/1"),
    ("article", "recommendations", "biodata.article-recommend/1"),
    ("search", "trial", "biodata.trial-search/1"),
    ("get", "trial", "biodata.trial-get/1"),
    ("trial", "document", "biodata.trial-document/1"),
    ("get", "variant", "biodata.variant-get/1"),
    ("search", "variant", "biodata.variant-search/1"),
    (
        "variant",
        "recurrence",
        "biodata.variant-recurrence-query/1",
    ),
    ("variant", "normalize", "biodata.variant-normalize/1"),
