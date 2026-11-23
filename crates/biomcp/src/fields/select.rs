//! Check chosen fields against a call's rows, narrow its finished reply, and say so in help.
use super::{
    narrow,
    rows::{Call, List, Row},
    sections,
};
use biomcp_query::Action;
use biomcp_types::{ErrorMessage, Request, Source, Step, StepKind, UsageArguments, UsageError};
use serde_json::Value;
use std::{collections::BTreeSet, io};

pub(super) fn schema(id: &str) -> Option<Value> {
    serde_json::from_str(crate::schema::find(id)?).ok()
}

/// The row lists of every reply this call's command can give.
fn call_lists(call: &Call) -> Vec<List> {
    let Some((_, _, id)) = crate::grammar::command_paths()
        .into_iter()
        .find(|path| (path.0, path.1) == (call.verb.as_str(), call.entity.as_str()))
    else {
        return Vec::new();
    };
    crate::schema::replies(id, &call.verb, &call.entity)
        .iter()
        .filter_map(|reply| schema(reply))
        .flat_map(|schema| sections::lists(&schema, call))
        .collect()
}

fn union(lists: Vec<List>) -> Row {
    let mut row = Row::default();
    for list in lists {
        row.extend(list.row);
    }
    row
}

/// The rows of one reply schema's `result.items`, for a search or subcommand.
fn reply_row(id: &str) -> Option<Row> {
    let call = Call::default();
    let lists = sections::lists(&schema(id)?, &call);
    (!lists.is_empty()).then(|| union(lists))
}

/// Every row name any reply of this search can carry.
fn search_row(verb: &str, entity: &str) -> Row {
    union(call_lists(&Call::of(verb, entity, &[])))
}

fn help_step(path: (&str, &str)) -> Step {
    Step {
        kind: StepKind::Configure,
        args: vec!["help".into(), path.0.into(), path.1.into()].into(),
        options: Default::default(),
    }
}

/// The chosen names this call's rows know, in the caller's order.
fn known(request: &Request, fields: &[String]) -> Vec<String> {
    let row = selected(request)
        .and_then(|(_, id)| reply_row(id))
        .unwrap_or_else(|| union(call_lists(&Call::new(request))));
    fields
        .iter()
        .filter(|name| row.knows(name))
        .cloned()
        .collect()
}

/// Keep the selected fields that the corrected command can return.
pub(crate) fn carry_to_correction(request: &Request, fields: &[String], step: &mut Step) {
    if fields.is_empty() {
        return;
    }
    let fields = known(request, fields);
    if !fields.is_empty() {
        let mut args = std::mem::take(&mut step.args).into_vec();
        args.insert(0, format!("--fields={}", fields.join(",")).into());
        step.args = args.into();
    }
}

/// Refuse chosen fields on a call without rows, and names its rows do not carry.
pub(crate) fn check(parsed: &crate::grammar::Parsed) -> Result<(), UsageError> {
    let fields = &parsed.globals.fields;
    if fields.is_empty() || parsed.help.is_some() {
        return Ok(());
    }
    let (request, path) = match &parsed.action {
        Some(Action::Run { request, .. }) => (request, request.path()),
        _ => return Err(UsageError::argument("--fields".into())),
    };
    let row = union(call_lists(&Call::new(request)));
    if row.names.is_empty() {
        return Err(UsageError::Arguments(UsageArguments {
            argument: Some("--fields".into()),
            message: Some(
                super::section_rows::refusal(request).unwrap_or(ErrorMessage::FieldsRowsOnly),
            ),
            next: vec![help_step(path)],
            ..Default::default()
        }));
    }
    if fields.iter().all(|name| row.knows(name)) {
        return source_check(parsed, request, path);
    }
    // A selected source's own row, so the suggested call keeps only names it carries.
    let row = selected(request)
        .and_then(|(_, id)| reply_row(id))
        .unwrap_or(row);
    Err(unknown(parsed, &row, None, path))
}

/// The refusal of chosen names a row lacks: its field names, the same call with each name it
/// can place in its field, and the names it cannot place.
fn unknown(
    parsed: &crate::grammar::Parsed,
    row: &Row,
    source: Option<Source>,
    path: (&str, &str),
) -> UsageError {
    let (mut next, unplaced, unsaid) = kept_step(parsed, row);
    let label_text = !unsaid
        && unplaced.iter().any(|name| name.as_str() == "text")
        && matches!(
            &parsed.action,
            Some(Action::Run { request: Request::GetDrug(query), .. })
                if query.sections() == [biomcp_types::DrugSection::Label]
                    && query.source() == biomcp_types::DrugSource::OpenfdaLabel
                    && query.label_section().is_some()
                    && query.label_find().is_empty()
        );
    if next.is_empty() {
        if label_text
            && let Some(action) = &parsed.action
            && let Ok(step) = crate::spell::step(StepKind::Configure, action)
        {
            next.push(step);
        } else {
            next.push(help_step(path));
        }
    }
    UsageError::FieldsUnknown {
        source,
        valid_values: texts(&row.names).unwrap_or_default().into(),
        unplaced,
        message: if unsaid {
            ErrorMessage::FieldsUnsaid
        } else if label_text {
            ErrorMessage::FieldsLabelText
        } else {
            ErrorMessage::FieldsUnknown
        },
        next,
    }
}

fn texts(names: &BTreeSet<String>) -> Option<Vec<biodata::Text>> {
    names
        .iter()
        .map(|name| biodata::Text::new(name).ok())
        .collect()
}

/// openFDA's Drugs@FDA names, and the Drugs@FDA download's, for values the approvals row
/// holds under its own names; a document's values are inside its application_docs list.
const SOURCE_NAMES: &[(&str, &str)] = &[
    ("submission_type", "event.kind"),
    ("submission_status", "event.status"),
    ("submission_status_date", "event.dates"),
    ("action_date", "event.dates"),
    ("submission_class", "submission_class_code_description"),
    ("doc_type", "application_docs"),
    ("doc_date", "application_docs"),
    ("doc_url", "application_docs"),
    ("doc_id", "application_docs"),
];

/// The row field a chosen name stands for: itself, a source's name for it, the one field
/// that contains it or that it contains, or a dotted path ending in it.
fn placed<'a>(row: &'a Row, name: &'a String) -> Option<&'a str> {
    if row.knows(name) {
        return Some(name);
    }
    let source = SOURCE_NAMES
        .iter()
        .find(|(source, field)| source == name && row.knows(field));
    source.map(|(_, field)| *field).or_else(|| {
        nearest(row, name)
            .or_else(|| row.inner(name))
            .map(String::as_str)
    })
}

/// A field name as a caller writes one; only such a name is said back.
fn spelled(name: &str) -> Option<biodata::Text> {
    let plain = name.len() <= 64
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.');
    plain.then(|| biodata::Text::new(name).ok()).flatten()
}

/// The same call with each chosen field the row lacks moved to the field that holds it, the
/// names it cannot place, and whether it left out one of them it cannot repeat (1081).
fn kept_step(
    parsed: &crate::grammar::Parsed,
    row: &Row,
) -> (Vec<Step>, Box<[biodata::Text]>, bool) {
    let mut chosen: Vec<&str> = Vec::new();
    let mut unplaced = Vec::new();
    let mut unsaid = false;
    for name in &parsed.globals.fields {
        match placed(row, name) {
            Some(field) if !chosen.contains(&field) => chosen.push(field),
            Some(_) => {}
            None => match spelled(name) {
                Some(name) => unplaced.push(name),
                None => unsaid = true,
            },
        }
    }
    let step = parsed
        .action
        .as_ref()
        .filter(|_| !chosen.is_empty())
        .and_then(|action| crate::spell::step(StepKind::Configure, action).ok())
        .map(|mut step| {
            let mut args = step.args.into_vec();
            args.insert(0, format!("--fields={}", chosen.join(",")).into());
            step.args = args.into();
            step
        });
    (step.into_iter().collect(), unplaced.into(), unsaid)
}

/// The source a search query selects, with the reply schema of its rows, when the
/// query alone fixes it.
fn selected(request: &Request) -> Option<(Source, &'static str)> {
    match request {
        Request::SearchDrug(query) => {
            use biomcp_types::{
                DrugFdaPage, DrugPage, DrugPurpleBookPage, DrugPurpleBookProducts, DrugRecallPage,
                DrugSafetyCommunicationPage, DrugSafetySignalPage, DrugSource, DrugWhoPage,
                DrugWithdrawalPage, ResponseBody,
            };
            let source = query.source();
            let schema = match source {
                DrugSource::FdaWithdrawnApprovals => DrugWithdrawalPage::SCHEMA,
                DrugSource::FdaSafetySignals => DrugSafetySignalPage::SCHEMA,
                DrugSource::FdaDsc => DrugSafetyCommunicationPage::SCHEMA,
                DrugSource::PurpleBook if query.purple_book_presentations() => {
                    DrugPurpleBookPage::SCHEMA
                }
                DrugSource::PurpleBook => DrugPurpleBookProducts::SCHEMA,
                DrugSource::FdaDrugRecall => DrugRecallPage::SCHEMA,
                DrugSource::Drugsfda => DrugFdaPage::SCHEMA,
                DrugSource::WhoPq => DrugWhoPage::SCHEMA,
                _ => DrugPage::SCHEMA,
            };
            Some((source.source(), schema))
        }
        Request::SearchTrial(query) => Some((
            query.source().source(),
            biomcp_query::trial::reply_schema(query.source()),
        )),
        _ => None,
    }
}

/// Rows of a search by source selector, when its sources return different rows.
fn source_rows(verb: &str, entity: &str) -> Vec<(&'static str, Row)> {
    if (verb, entity) == ("search", "adverse-event") {
        // One row schema serves every source, but each source fills only its own fields.
        let union = search_row(verb, entity);
        return biomcp_types::AdverseEventHeader::ROW_FIELDS
            .iter()
            .map(|(source, names)| {
                let keep =
                    |name: &&String| names.contains(&name.split('.').next().unwrap_or_default());
                let row = Row {
                    names: union.names.iter().filter(keep).cloned().collect(),
                    paths: union.paths.iter().filter(keep).cloned().collect(),
                    kept: union.kept.iter().filter(keep).cloned().collect(),
                };
                (source.as_str(), row)
            })
            .collect();
    }
    if (verb, entity) != ("search", "trial") {
        return Vec::new();
    }
    biomcp_query::trial::SOURCES
        .iter()
        .filter_map(|source| {
            reply_row(biomcp_query::trial::reply_schema(*source)).map(|row| (source.as_str(), row))
        })
        .collect()
}

/// The one field of the source's row that contains the asked name or that it contains.
fn nearest<'a>(row: &'a Row, name: &str) -> Option<&'a String> {
    let mut near = row
        .names
        .iter()
        .filter(|field| field.contains(name) || name.contains(field.as_str()));
    near.next().filter(|_| near.next().is_none())
}

/// Refuse chosen fields that the selected source's rows cannot carry, naming that
/// source, its fields, and the same call with each such field moved to its nearest.
fn source_check(
    parsed: &crate::grammar::Parsed,
    request: &Request,
    path: (&str, &str),
) -> Result<(), UsageError> {
    let fields = &parsed.globals.fields;
    let Some((source, row)) =
        selected(request).and_then(|(source, id)| Some((source, reply_row(id)?)))
    else {
        return Ok(());
    };
    if fields.iter().all(|name| row.knows(name)) {
        return Ok(());
    }
    Err(unknown(parsed, &row, Some(source), path))
}

/// The names a call chose and the rows they narrow, kept for its finished reply.
#[derive(Clone, Debug)]
pub(crate) struct Chosen {
    pub(crate) names: Vec<String>,
    call: Call,
}

impl Chosen {
    pub(crate) fn new(request: &Request, names: &[String]) -> Option<Self> {
        (!names.is_empty()).then(|| Self {
            names: names.to_vec(),
            call: Call::new(request),
        })
    }
}

/// Narrow each row list of a finished reply that offers a chosen name.
pub(crate) fn project(value: &mut Value, chosen: &Chosen) -> io::Result<()> {
    let Some(schema) = value.get("schema").and_then(Value::as_str).and_then(schema) else {
        return Ok(());
    };
    for list in sections::lists(&schema, &chosen.call) {
        narrow::list(value, &list, &chosen.names);
    }
    crate::incomplete::projected_purpose(value);
    Ok(())
}

const CHOOSE: &str = "Choose some with --fields a,b or a dotted path a.b (typed: fields list)";

/// The one help line on choosing row fields: a search or subcommand names its row fields,
/// and a get whose sections have rows says how to narrow them.
pub(crate) fn help_line(verb: &str, entity: &str, sections: &[String]) -> Option<String> {
    if verb == "get" {
        let call = Call::of(verb, entity, sections);
        return (!call_lists(&call).is_empty()).then(|| {
            if entity == "article" {
                "--fields keeps card fields (answer.card.title) or columns in named section rows."
            } else if entity == "disease" {
                "--fields a,b keeps columns. Use a.b for nested fields."
            } else {
                "--fields a,b keeps only those columns of a named section's rows; a.b names a value inside one."
            }
            .to_owned()
        });
    }
    let sources = source_rows(verb, entity);
    if !sources.is_empty() {
        let rows = sources
            .iter()
            .map(|(source, row)| format!("{source}: {}", listed(row)))
            .collect::<Vec<_>>()
            .join("; ");
        return Some(format!("Fields by --source: {rows}. {CHOOSE}."));
    }
    let row = search_row(verb, entity);
    if row.names.is_empty() {
        return None;
    }
    let required = row.required().cloned().collect::<BTreeSet<_>>();
    if (verb, entity) == ("search", "drug") {
        let optional = row.names.difference(&required).cloned().collect();
        return Some(format!(
            "Optional fields: {}. Always kept: {}. --fields a,b or a.b (MCP: fields).",
            joined(&optional),
            joined(&required)
        ));
    }
    if (verb, entity) == ("protein", "sequence") {
        return Some("--fields crc64,md5 or --fields audit.sequence_version.".into());
    }
    let kept = if required.is_empty() {
        String::new()
    } else {
        format!("; always kept: {}", joined(&required))
    };
    Some(format!("Fields: {}. {CHOOSE}{kept}.", joined(&row.names)))
}

fn joined(names: &BTreeSet<String>) -> String {
    names.iter().cloned().collect::<Vec<_>>().join(", ")
}

/// A row's names, then those it always keeps.
fn listed(row: &Row) -> String {
    let names = joined(&row.names);
    let required = row.required().cloned().collect::<BTreeSet<_>>();
    if required.is_empty() {
        return names;
    }
    format!("{names} (always kept: {})", joined(&required))
}

/// The chosen names a paging, sort, restart or fresh step keeps: those its own call's rows
/// know, so a section's page keeps its own names and a step whose call takes none, such as
/// a text section or a command without rows, carries no `--fields` it would refuse.
pub(crate) fn carried(kind: StepKind, action: &Action, fields: &[String]) -> Vec<String> {
    match action {
        Action::Run { request, .. }
            if matches!(
                kind,
                StepKind::Page
                    | StepKind::Section
                    | StepKind::Sort
                    | StepKind::Restart
                    | StepKind::Fresh
            ) =>
        {
            known(request, fields)
        }
        Action::PubmedSearch(query) if kind == StepKind::Restart => {
            known(&Request::SearchArticle(query.clone()), fields)
        }
        _ => Vec::new(),
    }
}
