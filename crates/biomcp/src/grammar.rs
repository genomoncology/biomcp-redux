//! Schema-fed command parsing shared by CLI and MCP.
mod entity;
mod arguments;
mod errors;
mod metadata;
use arguments::command_for;
pub(crate) use errors::argument_error;
pub(crate) mod gwas;
mod local;
mod queries;
#[cfg(test)]
mod reference;
pub(crate) use local::housekeeping_commands;
pub(crate) use queries::QUERIES;
pub(crate) use queries::query_schema;

pub(crate) const BUNDLE_HELP: &[&str] = &[
    "wikipathways",
    "cbioportal-studies",
    "ema",
    "ddinter",
    "who-pq",
    "gtr",
    "who-ivd",
    "fda-cdx",
];

use biomcp_query::{Action, Bundle};
use biomcp_types::{
    ArticleEdges, ArticleGet, ArticleSearch, ArticleText, GeneGet, GeneSearch, Request,
    VariantAssertionsGet, VariantPopulationQuery, VariantRecurrenceQuery,
};
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
    pub(crate) housekeeping: Option<crate::housekeeping::Command>,
    pub(crate) action: Option<Action>,
    pub(crate) globals: Globals,
    pub(crate) help: Option<Vec<String>>,
    pub(crate) http: Option<(String, Vec<String>)>,
}
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
            .help("Choose JSON or Markdown output.")
            .global(true)
            .value_parser(["json", "md"]),
    );
    root = root.arg(
        Arg::new("disable-sources")
            .long("disable-sources")
            .help("Exclude source names or source terms groups.")
            .global(true)
            .num_args(1)
            .value_delimiter(',')
            .action(ArgAction::Append),
    );
    for name in ["config", "replay", "record", "case-cache", "observe"] {
        root = root.arg(
            Arg::new(name)
                .long(name)
                .hide(name != "config")
                .help("Read configuration from this file.")
                .global(true)
                .value_parser(clap::value_parser!(PathBuf)),
        );
    }
    for name in ["offline", "fresh"] {
        root = root.arg(
            Arg::new(name)
                .long(name)
                .help(if name == "offline" { "Read local data without source requests." } else { "Request fresh source data instead of cached replies." })
                .global(true)
                .action(ArgAction::SetTrue)
                .overrides_with(name),
        );
    }
    root
}
pub(crate) fn is_housekeeping(args: &[OsString]) -> bool {
    scan_command_words(args, true).next().is_some_and(|name| {
        housekeeping_commands()
            .iter()
            .any(|command| command.get_name() == name)
    })
}

// Root recognition must also work when global values or repetitions are invalid.
pub(crate) fn command_words(args: &[OsString]) -> impl Iterator<Item = &str> {
    scan_command_words(args, is_housekeeping(args))
}

fn scan_command_words(args: &[OsString], tolerate_unknown: bool) -> impl Iterator<Item = &str> {
    let globals = global_arguments();
    let mut words = args.iter().peekable();
    while let Some(word) = words.peek().and_then(|word| word.to_str()) {
        if word == "--" {
            words.next();
            break;
        }
        let Some(flag) = word.strip_prefix('-') else {
            break;
        };
        let flag = flag.strip_prefix('-').unwrap_or(flag);
        let (name, inline) = flag
            .split_once('=')
            .map_or((flag, false), |(name, _)| (name, true));
        let argument = globals
            .get_arguments()
            .find(|arg| arg.get_long() == Some(name));
        if argument.is_none() && !tolerate_unknown {
            break;
        }
        let takes_value = argument.is_some_and(|arg| arg.get_action().takes_values());
        words.next();
        if takes_value && !inline {
            words.next();
        }
    }
    words.filter_map(|arg| arg.to_str())
}

pub(crate) fn root() -> Result<Command, ()> {
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
    root = root.subcommand(Command::new("sync").arg(Arg::new("bundle").help("Local source bundle to synchronize; default hgnc.").value_parser([
        "wikipathways",
        "cbioportal-studies",
        "hgnc",
        "clingen",
        "ema",
        "ddinter",
        "who-pq",
        "gtr",
        "who-ivd",
        "fda-cdx",
    ])));
    root = root.subcommands(local::housekeeping_commands());
    root = root.subcommand(Command::new("health"));
    root = root.subcommand(
        Command::new("list")
            .subcommand_required(true)
            .subcommand(Command::new("author")),
    );
    root = root.subcommand(
        Command::new("store")
            .subcommand_required(true)
            .subcommand(Command::new("clean")),
    );
    root = root.subcommand(crate::keys::grammar());
    root = root.subcommand(local::help_command());
    Ok(metadata::describe(root.subcommand(local::serve_command())))
}

pub(crate) fn parse(args: &[OsString]) -> Result<Parsed, ()> {
    let root = root()?;
    let matches = root
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(args.iter().cloned()))
        .map_err(|_| ())?;
    let globals = globals(&matches)?;
    let (verb, matches) = matches.subcommand().ok_or(())?;
    if verb == "serve" {
        return Ok(local::serve(matches, globals));
    }
    if verb == "key" {
        return Ok(Parsed {
            key: Some(crate::keys::parse(matches)?),
            housekeeping: None,
            http: None,
            action: None,
            globals,
            help: None,
        });
    }
    if let Some(command) = local::housekeeping(verb, matches) {
        return Ok(Parsed {
            key: None,
            housekeeping: Some(command),
            action: None,
            globals,
            help: None,
            http: None,
        });
    }
    let help = help_path(verb, matches)?;
    let action = if verb == "store" {
        Action::StoreClean
    } else if verb == "health" {
        Action::Health
    } else if verb == "help" || verb == "list" {
        Action::KeyHelp(local::help_source(matches)?)
    } else if verb == "sync" {
        Action::Sync {
            bundle: match matches.get_one::<String>("bundle").map(String::as_str) {
                Some("wikipathways") => Bundle::WikiPathways,
                Some("cbioportal-studies") => Bundle::CbioportalStudies,
                Some("fda-cdx") => Bundle::FdaCdx,
                Some("gtr") => Bundle::Gtr,
                Some("who-ivd") => Bundle::WhoIvd,
                Some("ema") => Bundle::Ema,
                Some("who-pq") => Bundle::WhoPq,
                Some("ddinter") => Bundle::Ddinter,
                Some("clingen") => Bundle::ClinGen,
                _ => Bundle::Hgnc,
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
        housekeeping: None,
        http: None,
        action: Some(action),
        globals,
        help,
    })
}
fn help_path(verb: &str, matches: &clap::ArgMatches) -> Result<Option<Vec<String>>, ()> {
    Ok(if verb == "list" {
        Some(vec!["author".into()])
    } else if verb == "help" {
        match matches.subcommand() {
            Some(("key", _)) => None,
            Some((name @ ("version" | "mcp-config" | "skill" | "sources" | "config"), _)) => {
                Some(vec![name.into()])
            }
            Some(("author", matches)) if matches.subcommand().is_none() => {
                Some(vec!["author".into()])
            }
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
        if property_type(property) == Some("boolean") {
            if matches.get_flag(name) {
                result.insert(name.clone(), Value::Bool(true));
            }
            continue;
        }
        let Some(values) = matches.get_many::<String>(name) else {
            continue;
        };
        let values: Vec<_> = values.cloned().collect();
        let value = if property_type(property) == Some("array") {
            Value::Array(values.into_iter().map(Value::String).collect())
        } else {
            let value = values.into_iter().next().ok_or(())?;
            if property_type(property) == Some("integer") {
                Value::Number(value.parse::<u64>().map_err(|_| ())?.into())
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
    let value = request_arguments(verb, entity, matches)?;
    match (verb, entity) {
        ("search" | "get", "gwas") => gwas::request(verb, value),
        ("cell-line", "assay-count") => entity::request(verb, entity, value),
        ("protein", _) | ("author", "papers") => entity::request(verb, entity, value),
        ("pathway", "genes" | "events") => entity::request(verb, entity, value),
        (
            "search" | "get",
            "pgx" | "adverse-event" | "disease" | "drug" | "diagnostic" | "pathway" | "phenotype"
            | "protein" | "study" | "cell-line" | "author",
        ) => entity::request(verb, entity, value),
        ("variant", "assertions") => serde_json::from_value::<VariantAssertionsGet>(value)
            .map(Request::VariantAssertions)
            .map_err(|_| ()),
        ("variant", "population") => serde_json::from_value::<VariantPopulationQuery>(value)
            .map(Request::VariantPopulation)
            .map_err(|_| ()),
        ("variant", "normalize") => serde_json::from_value::<biomcp_types::VariantNormalize>(value)
            .map(Request::VariantNormalize)
            .map_err(|_| ()),
        ("search", "variant") => serde_json::from_value::<biomcp_types::VariantSearch>(value)
            .map(Request::SearchVariant)
            .map_err(|_| ()),
        ("variant", "recurrence") => serde_json::from_value::<VariantRecurrenceQuery>(value)
            .map(Request::VariantRecurrence)
            .map_err(|_| ()),
        ("get", "variant") => serde_json::from_value::<biomcp_types::VariantGet>(value)
            .map(Request::GetVariant)
            .map_err(|_| ()),
        ("get", "trial") => serde_json::from_value::<biomcp_types::TrialGet>(value)
            .map(Request::GetTrial)
            .map_err(|_| ()),
        ("search", "trial") => serde_json::from_value::<biomcp_types::TrialSearch>(value)
            .map(Request::SearchTrial)
            .map_err(|_| ()),
        ("trial", "document") => serde_json::from_value::<biomcp_types::TrialDocumentGet>(value)
            .map(Request::TrialDocument)
            .map_err(|_| ()),
        ("article", "pdf") => serde_json::from_value::<biomcp_types::ArticlePdf>(value)
            .map(Request::ArticlePdf)
            .map_err(|_| ()),
        ("article", "citation-evidence") => {
            serde_json::from_value::<biomcp_types::ArticleEvidence>(value)
                .map(Request::ArticleEvidence)
                .map_err(|_| ())
        }
        ("article", "recommendations") => {
            serde_json::from_value::<biomcp_types::ArticleRecommend>(value)
                .map(Request::ArticleRecommend)
                .map_err(|_| ())
        }
        ("article", "supplement") => {
            serde_json::from_value::<biomcp_types::ArticleSupplement>(value)
                .map(Request::ArticleSupplement)
                .map_err(|_| ())
        }
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

fn request_arguments(verb: &str, entity: &str, matches: &clap::ArgMatches) -> Result<Value, ()> {
    let id = QUERIES
        .iter()
        .find(|(v, e, _)| *v == verb && *e == entity)
        .map(|(_, _, id)| *id)
        .ok_or(())?;
    let schema: Value = serde_json::from_str(crate::schema::find(id).ok_or(())?).map_err(|_| ())?;
    let mut value = arguments(matches, &schema)?;
    if verb == "search"
        && matches!(entity, "cell-line" | "author")
        && let Some(query) = matches.get_one::<String>("query_option")
    {
        value
            .as_object_mut()
            .ok_or(())?
            .insert("query".into(), query.clone().into());
    }
    Ok(value)
}

fn property_type(property: &Value) -> Option<&str> {
    let kind = property.get("type")?;
    if let Some(kind) = kind.as_str() {
        return Some(kind);
    }
    let kinds = kind.as_array()?;
    let kind = kinds
        .iter()
        .filter_map(Value::as_str)
        .find(|s| *s != "null")?;
    kinds
        .iter()
        .all(|s| s.as_str().is_some_and(|s| s == kind || s == "null"))
        .then_some(kind)
}
