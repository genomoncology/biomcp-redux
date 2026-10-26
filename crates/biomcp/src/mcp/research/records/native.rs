//! Source-owned links and labels for native records.
use super::{hit, next_args, paths, text};
use biomcp_adapters::registry::records::{RecordKind, declaration, record_url};
use biomcp_types::{ResearchHit, Source};
use serde_json::Value;

pub(super) fn kind(entity: &str) -> Option<RecordKind> {
    Some(match entity {
        "pathway" => RecordKind::Pathway,
        "study" => RecordKind::Study,
        "gwas" => RecordKind::Association,
        "adverse-event" => RecordKind::AdverseEvent,
        "trial" => RecordKind::Trial,
        "diagnostic" => RecordKind::Diagnostic,
        "drug" => RecordKind::Drug,
        _ => return None,
    })
}
fn source(row: &Value, envelope: &Value) -> Option<Source> {
    text(row, &["/source", "/attribution/source", "/term/source"])
        .or_else(|| text(envelope, &["/query/source"]))
        .or_else(|| {
            let sources = envelope.get("sources")?.as_array()?;
            (sources.len() == 1)
                .then(|| sources.first())
                .flatten()
                .and_then(|s| text(s, &["/source"]))
        })?
        .parse()
        .ok()
}
fn identity(row: &Value) -> Option<String> {
    text(row, paths::IDS).map(str::to_owned).or_else(|| {
        paths::IDS
            .iter()
            .find_map(|path| row.pointer(path)?.as_u64().map(|id| id.to_string()))
    })
}
pub(super) fn project(
    entity: &str,
    row: &Value,
    envelope: &Value,
    args: Vec<String>,
    card: Option<&Value>,
) -> Option<ResearchHit> {
    let id = identity(row)?;
    let declared = source(row, envelope).zip(kind(entity));
    let template = declared.and_then(|(source, kind)| declaration(source, kind));
    let url = text(row, paths::URLS)
        .or_else(|| card.and_then(|v| text(v, &["/report_url", "/source_url"])))
        .map(str::to_owned)
        .or_else(|| {
            let (source, kind) = declared?;
            record_url(source, kind, &id)
        })?;
    let fallback;
    let title = if let Some(title) = text(row, paths::TITLES) {
        title
    } else {
        fallback = format!("{} {id}", template?.display_name);
        &fallback
    };
    hit(args, title, &url)
}
pub(super) fn search(entity: &str, row: &Value, envelope: &Value) -> Option<ResearchHit> {
    project(
        entity,
        row,
        envelope,
        next_args(entity, &identity(row)?, envelope)?,
        None,
    )
}
pub(super) fn fetch(
    entity: &str,
    result: &Value,
    first: Option<&Value>,
    envelope: &Value,
) -> Option<ResearchHit> {
    let row = first
        .and_then(|v| v.get("header"))
        .or_else(|| result.get("header"))
        .unwrap_or(result);
    project(entity, row, envelope, vec![entity.into()], Some(result))
}
