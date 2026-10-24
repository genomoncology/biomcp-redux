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
];
pub(crate) fn global_arguments() -> Command {
    let mut root = Command::new("biomcp")
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .disable_version_flag(true)
        .arg_required_else_help(true)
        .subcommand_required(true);
    root = root.arg(
        Arg::new("format")
            .long("format")
            .global(true)
            .value_parser(["json", "md"]),
    );
    root = root.arg(
        Arg::new("disable-sources")
            .long("disable-sources")
            .global(true)
            .num_args(1)
            .value_delimiter(',')
            .action(ArgAction::Append),
    );
    for name in ["config", "replay", "record", "case-cache", "observe"] {
        root = root.arg(
            Arg::new(name)
                .long(name)
                .global(true)
                .value_parser(clap::value_parser!(PathBuf)),
        );
    }
    for name in ["offline", "fresh"] {
        root = root.arg(
            Arg::new(name)
                .long(name)
                .global(true)
                .action(ArgAction::SetTrue)
                .overrides_with(name),
        );
    }
    root
}
fn root() -> Result<Command, ()> {
    let mut root = global_arguments();
    for (index, &(verb, _, _)) in QUERIES.iter().enumerate() {
        if QUERIES
            .iter()
            .take(index)
            .any(|(prior, _, _)| *prior == verb)
        {
            continue;
        }
        let mut command = Command::new(verb).subcommand_required(true);
        for &(_, entity, id) in QUERIES.iter().filter(|(v, _, _)| *v == verb) {
            let schema: Value =
                serde_json::from_str(crate::schema::find(id).ok_or(())?).map_err(|_| ())?;
            command = command.subcommand(command_for(entity, &schema)?);
        }
        root = root.subcommand(command);
    }
    root = root
        .subcommand(Command::new("sync").arg(Arg::new("bundle").value_parser(["hgnc", "clingen"])));
    root = root.subcommand(Command::new("health"));
    root = root.subcommand(crate::keys::grammar());
    root = root.subcommand(help_command());
    Ok(root.subcommand(serve_command()))
}
fn help_command() -> Command {
    let mut help = Command::new("help")
        .subcommand(
            Command::new("network").arg(
                Arg::new("setting")
                    .required(true)
                    .value_parser(["ca_bundle", "proxy"]),
            ),
        )
        .subcommand(
            Command::new("key").arg(
                Arg::new("source")
                    .required(true)
                    .value_parser(crate::keys::names()),
            ),
        );
    let mut verbs = std::collections::BTreeMap::<&str, Command>::new();
    for &(verb, entity, _) in QUERIES {
        let command = verbs
            .remove(verb)
            .unwrap_or_else(|| Command::new(verb).subcommand_required(true));
        verbs.insert(verb, command.subcommand(Command::new(entity)));
    }
    for command in verbs.into_values() {
        help = help.subcommand(command);
    }
    help
}
fn serve_command() -> Command {
    Command::new("serve")
        .arg(
            Arg::new("http")
                .long("http")
                .num_args(0..=1)
                .default_missing_value("127.0.0.1:8080"),
        )
        .arg(
            Arg::new("allow-host")
                .long("allow-host")
                .action(ArgAction::Append)
                .requires("http"),
        )
}

pub(crate) fn parse(args: &[OsString]) -> Result<Parsed, ()> {
    let root = root()?;
    let matches = root
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args.iter().cloned()))
        .map_err(|_| ())?;
    let globals = globals(&matches)?;
    let (verb, matches) = matches.subcommand().ok_or(())?;
    if verb == "serve" {
        return Ok(Parsed {
            key: None,
            http: matches.get_one::<String>("http").map(|address| {
                (
                    address.clone(),
                    matches
                        .get_many::<String>("allow-host")
                        .into_iter()
                        .flatten()
                        .cloned()
                        .collect(),
                )
            }),
            action: None,
            globals,
            help: None,
        });
    }
    if verb == "key" {
        return Ok(Parsed {
            key: Some(crate::keys::parse(matches)?),
            http: None,
            action: None,
            globals,
            help: None,
        });
    }
    let help = help_path(verb, matches)?;
    let action = if verb == "health" {
        Action::Health
    } else if verb == "help" {
        let source = matches
            .subcommand()
            .filter(|(name, _)| *name == "key")
            .and_then(|(_, m)| m.get_one::<String>("source"))
            .map_or(Ok(biodata::Source::Oncokb), |s| {
                if s == "ncbi" {
                    Ok(biodata::Source::PubMed)
                } else {
                    s.parse()
                }
            })
            .map_err(|_| ())?;
        Action::KeyHelp(source)
    } else if verb == "sync" {
        Action::Sync {
            bundle: if matches
                .get_one::<String>("bundle")
                .is_some_and(|v| v == "clingen")
            {
                Bundle::ClinGen
            } else {
                Bundle::Hgnc
            },
        }
    } else {
        let (entity, matches) = matches.subcommand().ok_or(())?;
        let request = request(verb, entity, matches)?;
        Action::Run {
            request,
            fresh: globals.fresh,
        }
    };
    Ok(Parsed {
        key: None,
        http: None,
        action: Some(action),
        globals,
        help,
    })
}
fn help_path(verb: &str, matches: &clap::ArgMatches) -> Result<Option<Vec<String>>, ()> {
    Ok(if verb == "help" {
        match matches.subcommand() {
            Some(("key", _)) => None,
            Some(("network", matches)) => Some(vec![
                "network".into(),
                matches.get_one::<String>("setting").ok_or(())?.clone(),
            ]),
            Some((verb, matches)) => {
                Some(vec![verb.into(), matches.subcommand().ok_or(())?.0.into()])
            }
            None => Some(Vec::new()),
        }
    } else {
        None
    })
}
fn globals(matches: &clap::ArgMatches) -> Result<Globals, ()> {
    let globals = Globals {
        config: matches.get_one::<PathBuf>("config").cloned(),
        disable_sources: matches
            .get_many::<String>("disable-sources")
            .into_iter()
            .flatten()
            .cloned()
            .collect(),
        replay: matches.get_one::<PathBuf>("replay").cloned(),
        observe: matches.get_one::<PathBuf>("observe").cloned(),
        record: matches.get_one::<PathBuf>("record").cloned(),
        case_cache: matches.get_one::<PathBuf>("case-cache").cloned(),
        offline: matches.get_flag("offline"),
        fresh: matches.get_flag("fresh"),
        format: matches
            .get_one::<String>("format")
            .map(|v| if v == "md" { Format::Md } else { Format::Json }),
    };
    globals.check()?;
    Ok(globals)
}

fn properties(schema: &Value) -> Result<&Map<String, Value>, ()> {
    schema
        .get("properties")
        .and_then(Value::as_object)
        .ok_or(())
}
fn command_for(name: &'static str, schema: &Value) -> Result<Command, ()> {
    let mut command = Command::new(name)
        .disable_help_flag(true)
        .disable_help_subcommand(true);
    for (name, property) in properties(schema)? {
        if let Some(flags) = property.get("x-cli-flags").and_then(Value::as_object) {
            for flag in flags.keys() {
                command = command.arg(
                    Arg::new(flag.clone())
                        .long(flag.clone())
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(flags.keys().filter(|other| *other != flag).cloned()),
                );
            }
            continue;
        }
        let mut argument = Arg::new(name.clone());
        if let Some(position) = property.get("x-positional").and_then(Value::as_u64) {
            argument = argument.index(position as usize + 1);
        } else {
            argument = argument.long(name.replace('_', "-"));
        }
        if schema
            .get("required")
            .and_then(Value::as_array)
            .is_some_and(|required| required.contains(&Value::String(name.clone())))
        {
            argument = argument.required(true);
        }
        if property.get("type").and_then(Value::as_str) == Some("array") {
            argument = if property.get("x-comma-separated") == Some(&Value::Bool(true)) {
                argument
                    .num_args(1)
                    .value_delimiter(',')
                    .action(ArgAction::Append)
            } else if property.get("x-trailing-section").is_some() {
                argument.num_args(1).action(ArgAction::Append)
            } else {
                argument.num_args(1..).action(ArgAction::Append)
            };
        }
        command = command.arg(
            if property.get("type").and_then(Value::as_str) == Some("boolean") {
                argument.action(ArgAction::SetTrue)
            } else {
                argument.value_parser(clap::builder::StringValueParser::new())
            },
        );
    }
    Ok(command)
}
fn arguments(matches: &clap::ArgMatches, schema: &Value) -> Result<Value, ()> {
    let mut result = Map::new();
    for (name, property) in properties(schema)? {
        if let Some(flags) = property.get("x-cli-flags").and_then(Value::as_object) {
            for (flag, value) in flags {
                if matches.get_flag(flag) {
                    result.insert(name.clone(), value.clone());
                }
            }
            continue;
        }
        if property.get("type").and_then(Value::as_str) == Some("boolean") {
            if matches.get_flag(name) {
                result.insert(name.clone(), Value::Bool(true));
            }
            continue;
        }
        let Some(values) = matches.get_many::<String>(name) else {
            continue;
        };
        let values: Vec<_> = values.cloned().collect();
        let value = if property.get("type").and_then(Value::as_str) == Some("array") {
            Value::Array(values.into_iter().map(Value::String).collect())
        } else {
            let value = values.into_iter().next().ok_or(())?;
            if property.get("type").and_then(Value::as_str) == Some("integer") {
                Value::Number(value.parse::<u32>().map_err(|_| ())?.into())
            } else {
                Value::String(value)
            }
        };
        result.insert(name.clone(), value);
    }
    split_sections(&mut result, schema)?;
    Ok(Value::Object(result))
}

impl Globals {
    pub(crate) fn inherit(&mut self, process: &Self) -> Result<(), ()> {
        if self.config.is_some() {
            return Err(());
        }
        self.config = process.config.clone();
        self.disable_sources.extend(process.disable_sources.clone());
        self.disable_sources.sort();
        self.disable_sources.dedup();
        self.replay = process.replay.clone();
        self.observe = process.observe.clone();
        self.record = process.record.clone();
        self.case_cache = process.case_cache.clone();
        self.fresh |= process.fresh;
        self.format = self.format.or(process.format);
        self.offline |= process.offline;
        self.check()
    }
    fn check(&self) -> Result<(), ()> {
        biomcp_adapters::registry::disabled(&self.disable_sources).map_err(|_| ())?;
        if usize::from(self.replay.is_some())
            + usize::from(self.record.is_some())
            + usize::from(self.offline)
            > 1
            || self.case_cache.is_some() && self.replay.is_none() && !self.offline
        {
            Err(())
        } else {
            Ok(())
        }
    }
}

fn split_sections(result: &mut Map<String, Value>, schema: &Value) -> Result<(), ()> {
    for (name, property) in properties(schema)? {
        if property.get("x-trailing-section") != Some(&Value::Bool(true))
            || result.contains_key(name)
        {
            continue;
        }
        let item = property.get("items").ok_or(())?;
        let item = item
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|s| schema.pointer(s.trim_start_matches('#')))
            .unwrap_or(item);
        let words = item.get("enum").and_then(Value::as_array).ok_or(())?;
        let Some(input) = result.get_mut("input").and_then(Value::as_array_mut) else {
            return Err(());
        };
        let mut split = input.len();
        while split > 1 && input.get(split - 1).is_some_and(|v| words.contains(v)) {
            split -= 1;
        }
        let section = input.split_off(split);
        if !section.is_empty() {
            result.insert(name.clone(), Value::Array(section));
        }
    }
    Ok(())
}

fn request(verb: &str, entity: &str, matches: &clap::ArgMatches) -> Result<Request, ()> {
    let id = QUERIES
        .iter()
        .find(|(v, e, _)| *v == verb && *e == entity)
        .map(|(_, _, id)| *id)
        .ok_or(())?;
    let schema: Value = serde_json::from_str(crate::schema::find(id).ok_or(())?).map_err(|_| ())?;
    let value = arguments(matches, &schema)?;
    match (verb, entity) {
        ("get", "trial") => serde_json::from_value::<biodata::TrialGet>(value)
            .map(Request::GetTrial)
            .map_err(|_| ()),
        ("search", "trial") => serde_json::from_value::<biodata::TrialSearch>(value)
            .map(Request::SearchTrial)
            .map_err(|_| ()),
        ("trial", "document") => serde_json::from_value::<biodata::TrialDocumentGet>(value)
            .map(Request::TrialDocument)
            .map_err(|_| ()),
        ("article", "pdf") => serde_json::from_value::<biodata::ArticlePdf>(value)
            .map(Request::ArticlePdf)
            .map_err(|_| ()),
        ("article", "citation-evidence") => {
            serde_json::from_value::<biodata::ArticleEvidence>(value)
                .map(Request::ArticleEvidence)
                .map_err(|_| ())
        }
        ("article", "recommendations") => {
            serde_json::from_value::<biodata::ArticleRecommend>(value)
                .map(Request::ArticleRecommend)
                .map_err(|_| ())
        }
        ("article", "supplement") => serde_json::from_value::<biodata::ArticleSupplement>(value)
            .map(Request::ArticleSupplement)
            .map_err(|_| ()),
        ("article", "text") => serde_json::from_value::<ArticleText>(value)
            .map(Request::ArticleText)
            .map_err(|_| ()),
        ("article", "citations") => serde_json::from_value::<ArticleEdges>(value)
            .map(Request::ArticleCitations)
            .map_err(|_| ()),
        ("article", "references") => serde_json::from_value::<ArticleEdges>(value)
            .map(Request::ArticleReferences)
            .map_err(|_| ()),
        ("get", "article") => serde_json::from_value::<ArticleGet>(value)
            .map(Request::GetArticle)
            .map_err(|_| ()),
        ("search", "article") => serde_json::from_value::<ArticleSearch>(value)
            .map(Request::SearchArticle)
            .map_err(|_| ()),
        ("get", "gene") => serde_json::from_value::<GeneGet>(value)
            .map(Request::GetGene)
            .map_err(|_| ()),
        ("search", "gene") => serde_json::from_value::<GeneSearch>(value)
            .map(Request::SearchGene)
            .map_err(|_| ()),
        _ => Err(()),
    }
}
