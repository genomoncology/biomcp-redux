//! Schema-fed command parsing shared by CLI and MCP.
use biodata::{GeneGet, GeneSearch, Request};
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
}
pub(crate) fn query_schema(request: &Request) -> &'static str {
    match request {
        Request::GetGene(_) => "biodata.gene-get/1",
        Request::SearchGene(_) => "biodata.gene-search/1",
    }
}
fn root() -> Result<Command, ()> {
    let mut root = Command::new("biomcp")
        .disable_help_flag(true)
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
    for (verb, id) in [
        ("get", "biodata.gene-get/1"),
        ("search", "biodata.gene-search/1"),
    ] {
        let schema: Value =
            serde_json::from_str(crate::schema::find(id).ok_or(())?).map_err(|_| ())?;
        root = root.subcommand(
            Command::new(verb)
                .subcommand_required(true)
                .subcommand(command_for("gene", &schema)?),
        );
    }
    root = root.subcommand(Command::new("sync").arg(Arg::new("bundle").value_parser(["hgnc"])));
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
        });
    }
    let action = if verb == "sync" {
        Action::Sync {
            bundle: Bundle::Hgnc,
        }
    } else {
        let (_, matches) = matches.subcommand().ok_or(())?;
        let id = if verb == "get" {
            "biodata.gene-get/1"
        } else {
            "biodata.gene-search/1"
        };
        let schema: Value =
            serde_json::from_str(crate::schema::find(id).ok_or(())?).map_err(|_| ())?;
        let value = arguments(matches, &schema)?;
        let request = if verb == "get" {
            Request::GetGene(serde_json::from_value::<GeneGet>(value).map_err(|_| ())?)
        } else {
            Request::SearchGene(serde_json::from_value::<GeneSearch>(value).map_err(|_| ())?)
        };
        Action::Run {
            request,
            fresh: globals.fresh,
        }
    };
    Ok(Parsed {
        action: Some(action),
        globals,
    })
}
fn properties(schema: &Value) -> Result<&Map<String, Value>, ()> {
    schema
        .get("properties")
        .and_then(Value::as_object)
        .ok_or(())
}
fn command_for(name: &'static str, schema: &Value) -> Result<Command, ()> {
    let mut command = Command::new(name).disable_help_flag(true);
    for (name, property) in properties(schema)? {
        let mut argument = Arg::new(name.clone());
        if let Some(position) = property.get("x-positional").and_then(Value::as_u64) {
            argument = argument.index(position as usize + 1);
        } else {
            argument = argument.long(name.clone());
        }
        if schema
            .get("required")
            .and_then(Value::as_array)
            .is_some_and(|required| required.contains(&Value::String(name.clone())))
        {
            argument = argument.required(true);
        }
        if property.get("type").and_then(Value::as_str) == Some("array") {
            argument = argument.num_args(1..).action(ArgAction::Append);
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
