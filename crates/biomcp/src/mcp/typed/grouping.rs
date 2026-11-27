//! One clinical catalog policy shared by discovery and executable follow-ups.
use super::catalog::front_map::ENTITY_TOOLS;
use serde_json::{Value, json};
use std::{collections::BTreeSet, io};

/// The typed `fields` list, for search, subcommand and section rows alike (1071).
const FIELDS: &str = "Row columns or a.b.";

pub(in crate::mcp) struct Catalog {
    pub(super) tools: Vec<Value>,
    commands: Vec<Value>,
    globals: crate::grammar::Globals,
    read_only: bool,
    excluded: BTreeSet<biomcp_types::Source>,
    /// The entity tool being called; a next call names it when it carries the command.
    pub(super) current: Option<String>,
}

impl Catalog {
    pub(super) fn permits(&self, parsed: &mut crate::grammar::Parsed) -> bool {
        parsed.globals.inherit(&self.globals).is_ok()
            && crate::command::mcp_policy::refusal(parsed, self.read_only).is_none()
            && parsed.action.as_ref().is_none_or(|action| match action {
                biomcp_query::Action::Run { request, .. } => matches!(
                    super::catalog::request_allowed(request, &self.excluded),
                    Ok(true)
                ),
                _ => true,
            })
    }

    /// The advertised actions of an entity tool, in the tool map's order.
    pub(super) fn actions(&self, tool: &str) -> Vec<String> {
        let offered = self
            .tools
            .iter()
            .find(|advertised| advertised["name"] == tool)
            .and_then(|advertised| advertised.pointer("/inputSchema/properties"))
            .and_then(Value::as_object);
        ENTITY_TOOLS
            .iter()
            .filter(|(name, ..)| *name == tool)
            .flat_map(|(_, _, placed, ..)| placed.iter())
            .filter(|(action, ..)| offered.is_some_and(|fields| fields.contains_key(*action)))
            .map(|(action, ..)| (*action).into())
            .collect()
    }
}

/// One advertised registry command, before it is placed on its entity tools.
pub(super) struct Command {
    pub(super) path: (&'static str, &'static str, &'static str),
    pub(super) description: String,
    pub(super) input: Value,
    pub(super) read_only: bool,
    pub(super) sources: BTreeSet<String>,
    /// The caller schema the follow-up check reads, keyed by the command name.
    pub(super) canonical: Value,
}

pub(super) fn catalog(
    context: crate::command::Context<'_>,
    read_only: bool,
    discovery: bool,
    commands: Vec<Command>,
    excluded: BTreeSet<biomcp_types::Source>,
) -> io::Result<Catalog> {
    let output_schema = discovery && crate::config::mcp_typed_schema_options(context.settings).0;
    let enum_limit = crate::config::mcp_typed_schema_options(context.settings).1;
    let mut tools = ENTITY_TOOLS
        .iter()
        .filter_map(|tool| {
            entity(tool, &commands, output_schema, &excluded, enum_limit).transpose()
        })
        .collect::<io::Result<Vec<_>>>()?;
    if super::saved::offered(context.globals, read_only) {
        tools.push(super::saved::tool(output_schema)?);
    }
    tools.push(super::help::tool(output_schema)?);
    // MCP tool schemas default to 2020-12. Omit the repeated default in typed discovery.
    for tool in &mut tools {
        for key in ["inputSchema", "outputSchema"] {
            if let Some(schema) = tool.get_mut(key).and_then(Value::as_object_mut)
                && schema.get("$schema").and_then(Value::as_str)
                    == Some(super::super::protocol::SCHEMA_DIALECT)
            {
                schema.shift_remove("$schema");
            }
        }
    }
    tools.extend(super::super::protocol::tools(context.root, read_only)?);
    Ok(Catalog {
        tools,
        commands: commands
            .into_iter()
            .map(|command| command.canonical)
            .collect(),
        globals: context.globals.clone(),
        read_only,
        excluded,
        current: None,
    })
}

pub(super) fn check_budget(settings: &crate::config::Settings, tools: &[Value]) -> io::Result<()> {
    let (tool_limit, byte_limit) = crate::config::mcp_typed_budget(settings);
    let bytes = serde_json::to_vec(tools)?.len();
    if tools.len() > tool_limit || bytes > byte_limit {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Typed catalog requires {} tools and {bytes} definition bytes; \
                mcp_typed_tool_limit={tool_limit}, mcp_typed_definition_bytes={byte_limit}. \
                Clinical tools and complete descriptions are retained; configure sufficient budgets.",
                tools.len(),
            ),
        ));
    }
    Ok(())
}

/// An entity tool: `action` picks one command, whose own schema is the object named after it.
fn entity(
    tool: &super::catalog::front_map::EntityTool,
    commands: &[Command],
    output_schema: bool,
    excluded: &BTreeSet<biomcp_types::Source>,
    enum_limit: usize,
) -> io::Result<Option<Value>> {
    let &(name, _, placed, _, _) = tool;
    let actions = placed
        .iter()
        .filter_map(|&(action, command, note)| {
            commands
                .iter()
                .find(|found| super::name(found.path.0, found.path.1) == command)
                .map(|found| (action, note, found))
        })
        .collect::<Vec<_>>();
    if actions.is_empty() {
        return Ok(None);
    }
    let mut properties = serde_json::Map::new();
    // The actions are named, not enumerated, and other fields are not forbidden: a client
    // that checks arguments then passes a guess such as drug `approvals` to the server,
    // which answers with the call that works.
    let names = actions
        .iter()
        .map(|(action, _, _)| *action)
        .collect::<Vec<_>>();
    properties.insert(
        "action".into(),
        json!({"type":"string","description":format!(
            "One of: {}. Fill its object.",
            names.join(", ")
        )}),
    );
    for (action, _, command) in &actions {
        let mut input = command.input.clone();
        super::arguments::advertise_action(&mut input, command.path, enum_limit);
        if let Some(fields) = input.as_object_mut() {
            fields.shift_remove("$schema");
            fields.insert(
                "description".into(),
                json!(super::description::action(action, &command.description)),
            );
        }
        properties.insert((*action).into(), input);
    }
    // Any list action keeps only the named row fields; help lists a search's names.
    if actions
        .iter()
        .any(|(_, _, command)| command.path.0 == "search")
    {
        properties.insert(
            "fields".into(),
            json!({"type":["array","null"],"items":{"type":"string"},"description":FIELDS}),
        );
    }
    let description = entity_description(tool, &actions, excluded);
    let mut tool = json!({"name":name,
        "description":description,
        "inputSchema":{"$schema":super::super::protocol::SCHEMA_DIALECT,"type":"object",
            "properties":properties,"required":["action"]},
        "annotations":{"readOnlyHint":actions.iter().all(|(_, _, command)| command.read_only),
            "destructiveHint":false,"openWorldHint":true}});
    if output_schema && let Some(fields) = tool.as_object_mut() {
        let paths = actions
            .iter()
            .map(|(_, _, command)| command.path)
            .collect::<Vec<_>>();
        fields.insert(
            "outputSchema".into(),
            super::super::output_shape::for_commands(&paths)?,
        );
    }
    let title = super::super::protocol::display_name(name);
    Ok(Some(super::super::protocol::named_tool(&title, tool)))
}

fn entity_description(
    tool: &super::catalog::front_map::EntityTool,
    actions: &[(&str, &str, &Command)],
    excluded: &BTreeSet<biomcp_types::Source>,
) -> String {
    let sources = actions
        .iter()
        .flat_map(|(_, _, command)| command.sources.iter().cloned())
        .collect::<BTreeSet<_>>();
    let notes = actions
        .iter()
        .map(|(action, note, _)| (*action, *note))
        .collect::<Vec<_>>();
    let action_sources = |word| {
        actions
            .iter()
            .find(|(action, _, _)| *action == word)
            .map(|(_, _, command)| &command.sources)
    };
    let source_scope = if matches!(tool.0, "drug" | "disease" | "phenotype") {
        action_sources("search").zip(action_sources("get"))
    } else {
        None
    };
    super::description::tool(tool, &sources, source_scope, &notes, excluded)
}

/// Write a canonical `{name: command, arguments}` call in its advertised entity form.
pub(super) fn call(canonical: Value, catalog: &Catalog) -> Option<Value> {
    let tools = &catalog.tools;
    let name = canonical.get("name")?.as_str()?;
    let command = catalog.commands.iter().find(|tool| tool["name"] == name);
    if command
        .is_some_and(|tool| !super::followups::allowed(&canonical, std::slice::from_ref(tool)))
    {
        return None;
    }
    if tools.iter().any(|tool| tool["name"] == name) {
        return super::followups::allowed(&canonical, tools).then_some(canonical);
    }
    command?;
    let arguments = canonical.get("arguments")?;
    let arguments = &advertised(name, arguments);
    let mut placements = ENTITY_TOOLS
        .iter()
        .flat_map(|(tool, _, actions, _, _)| {
            actions
                .iter()
                .filter(|(_, placed, _)| *placed == name)
                .map(move |(action, _, _)| (*tool, *action))
        })
        .collect::<Vec<_>>();
    // Prefer the tool already in use, so a disease reply's GWAS step stays on disease.
    placements.sort_by_key(|(tool, _)| catalog.current.as_deref() != Some(*tool));
    placements.into_iter().find_map(|(tool, action)| {
        let call = json!({"name":tool,"arguments":{"action":action,action:arguments}});
        super::followups::allowed(&call, tools).then_some(call)
    })
}

/// A registry command's arguments under their advertised names.
pub(super) fn advertised(command: &str, arguments: &Value) -> Value {
    let Some((verb, schema)) = crate::grammar::command_paths()
        .into_iter()
        .find(|(verb, entity, _)| super::name(verb, entity) == command)
        .and_then(|(verb, _, id)| Some((verb, super::schema(id).ok()?)))
    else {
        return arguments.clone();
    };
    let mut arguments =
        super::forms::advertised_arguments(arguments, &super::forms::forms(verb, &schema));
    if crate::grammar::paging::both(&schema)
        && let Some(fields) = arguments.as_object_mut()
    {
        for (candidate, list) in crate::grammar::paging::CANDIDATES {
            if let Some(value) = fields.remove(list) {
                fields.insert(candidate.into(), value);
            }
        }
    }
    arguments
}
