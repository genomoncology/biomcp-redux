//! Schema-fed command parsing shared by CLI and MCP.
use biodata::{ArticleEdges, ArticleGet, ArticleSearch, GeneGet, GeneSearch, Request};
use biomcp_query::{Action, Bundle};
use clap::{Arg, ArgAction, Command};
use serde_json::{Map, Value};
use std::{ffi::OsString, path::PathBuf};

#[derive(Clone, Debug, Default)]
pub(crate) struct Globals {
    pub(crate) replay: Option<PathBuf>,
    pub(crate) record: Option<PathBuf>,
    pub(crate) case_cache: Option<PathBuf>,
    pub(crate) offline: bool,
    pub(crate) fresh: bool,
}
#[derive(Debug)]
pub(crate) struct Parsed {
    pub(crate) action: Option<Action>,
    pub(crate) globals: Globals,
    pub(crate) help: Option<Vec<String>>,
}
pub(crate) fn query_schema(request: &Request) -> &'static str {
    match request {
        Request::ArticleCitations(_) | Request::ArticleReferences(_) => "biodata.article-edges/1",
        Request::GetArticle(_) => "biodata.article-get/1",
        Request::SearchArticle(_) => "biodata.article-search/1",
        Request::GetGene(_) => "biodata.gene-get/1",
        Request::SearchGene(_) => "biodata.gene-search/1",
    }
}
pub(crate) const QUERIES: &[(&str, &str, &str)] = &[
    ("get", "gene", "biodata.gene-get/1"),
    ("search", "gene", "biodata.gene-search/1"),
    ("search", "article", "biodata.article-search/1"),
    ("get", "article", "biodata.article-get/1"),
    ("article", "citations", "biodata.article-edges/1"),
    ("article", "references", "biodata.article-edges/1"),
];
fn root() -> Result<Command, ()> {
    let mut root = Command::new("biomcp")
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .disable_version_flag(true)
        .arg_required_else_help(true)
        .subcommand_required(true);
    for name in ["replay", "record", "case-cache"] {
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
    for verb in ["get", "search", "article"] {
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
    root = root.subcommand(
        Command::new("help")
            .subcommand(
                Command::new("key").arg(Arg::new("source").required(true).value_parser(["oncokb"])),
            )
            .subcommand(
                Command::new("get")
                    .subcommand_required(true)
                    .subcommand(Command::new("gene"))
                    .subcommand(Command::new("article")),
            )
            .subcommand(
                Command::new("search")
                    .subcommand_required(true)
                    .subcommand(Command::new("gene"))
                    .subcommand(Command::new("article")),
            ),
    );
    Ok(root.subcommand(Command::new("serve")))
}
pub(crate) fn parse(args: &[OsString]) -> Result<Parsed, ()> {
    let root = root()?;
    let matches = root
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args.iter().cloned()))
        .map_err(|_| ())?;
    let globals = Globals {
        replay: matches.get_one::<PathBuf>("replay").cloned(),
        record: matches.get_one::<PathBuf>("record").cloned(),
        case_cache: matches.get_one::<PathBuf>("case-cache").cloned(),
        offline: matches.get_flag("offline"),
        fresh: matches.get_flag("fresh"),
    };
    globals.check()?;
    let (verb, matches) = matches.subcommand().ok_or(())?;
    if verb == "serve" {
        return Ok(Parsed {
            action: None,
            globals,
            help: None,
        });
    }
    let help = if verb == "help" {
        match matches.subcommand() {
            Some(("key", _)) => None,
            Some((verb, matches)) => {
                Some(vec![verb.into(), matches.subcommand().ok_or(())?.0.into()])
            }
            None => Some(Vec::new()),
        }
    } else {
        None
    };
    let action = if verb == "help" {
        Action::KeyHelp
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
        let request=request(verb,entity,matches)?;
        Action::Run {
            request,
            fresh: globals.fresh,
        }
    };
    Ok(Parsed {
        action: Some(action),
        globals,
        help,
    })
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
        command = command.arg(argument.value_parser(clap::builder::StringValueParser::new()));
    }
    Ok(command)
}
fn arguments(matches: &clap::ArgMatches, schema: &Value) -> Result<Value, ()> {
    let mut result = Map::new();
    for (name, property) in properties(schema)? {
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
        self.replay = process.replay.clone();
        self.record = process.record.clone();
        self.case_cache = process.case_cache.clone();
        self.fresh |= process.fresh;
        self.offline |= process.offline;
        self.check()
    }
    fn check(&self) -> Result<(), ()> {
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

fn request(verb:&str,entity:&str,matches:&clap::ArgMatches)->Result<Request,()> {
    let id=QUERIES.iter().find(|(v,e,_)|*v==verb && *e==entity).map(|(_,_,id)|*id).ok_or(())?;
    let schema:Value=serde_json::from_str(crate::schema::find(id).ok_or(())?).map_err(|_|())?;
    let value=arguments(matches,&schema)?;
    match (verb,entity) {
        ("article","citations")=>serde_json::from_value::<ArticleEdges>(value).map(Request::ArticleCitations).map_err(|_|()),
        ("article","references")=>serde_json::from_value::<ArticleEdges>(value).map(Request::ArticleReferences).map_err(|_|()),
        ("get","article")=>serde_json::from_value::<ArticleGet>(value).map(Request::GetArticle).map_err(|_|()),
        ("search","article")=>serde_json::from_value::<ArticleSearch>(value).map(Request::SearchArticle).map_err(|_|()),
        ("get","gene")=>serde_json::from_value::<GeneGet>(value).map(Request::GetGene).map_err(|_|()),
        ("search","gene")=>serde_json::from_value::<GeneSearch>(value).map(Request::SearchGene).map_err(|_|()),
        _=>Err(()),
    }
}
