use super::Snapshot;
use crate::{Action, Outcome};
use biodata::{
    ArticleEvidence, ArticleEvidenceResult, QueryBody, Request, Section, Status, StepKind, Text,
};
use biomcp_fetch::RequestError;
/// Exact Unicode fragments leave room for both independent evidence sections and envelope.
pub(super) fn split(text: &str) -> Vec<(u32, String)> {
    let mut out = Vec::new();
    let mut buffer = String::new();
    let mut bytes = 0;
    let mut offset = 0;
    for (chars, ch) in text.chars().enumerate() {
        let cost = biomcp_adapters::jats::escaped_bytes(&ch.to_string());
        if bytes + cost > 3500 {
            out.push((offset, std::mem::take(&mut buffer)));
            bytes = 0;
            offset = chars as u32;
        }
        buffer.push(ch);
        bytes += cost;
    }
    if !buffer.is_empty() {
        out.push((offset, buffer))
    }
    out
}

pub(super) fn render(
    query: &ArticleEvidence,
    mut snapshot: Snapshot,
    id: Text,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let mut context_end = query.offset();
    let mut paragraph_end = query.paragraph_offset();
    let mut more = false;
    if let Section::Data { data, .. } = &mut snapshot.graph {
        let total = data.contexts.items.len();
        let start = query.offset() as usize;
        if start > total {
            out.body = QueryBody::TextPositionError;
            return Ok(());
        }
        let items = page(&data.contexts.items, start, 4500)?;
        context_end += items.len() as u32;
        more = (context_end as usize) < total;
        data.contexts.items = items.into();
    }
    if let Some(Section::Data { data, .. }) = &mut snapshot.fulltext {
        let total = data.paragraphs.items.len();
        let start = query.paragraph_offset() as usize;
        if start > total {
            out.body = QueryBody::TextPositionError;
            return Ok(());
        }
        let items = page(&data.paragraphs.items, start, 8500)?;
        paragraph_end += items.len() as u32;
        more |= (paragraph_end as usize) < total;
        data.paragraphs.items = items.into();
    }
    let failed = |section: &Section<_>| {
        matches!(
            section,
            Section::Unavailable { .. } | Section::Skipped { .. }
        )
    };
    let graph_failed = failed(&snapshot.graph);
    let text_failed = snapshot
        .fulltext
        .as_ref()
        .is_some_and(|s| matches!(s, Section::Unavailable { .. } | Section::Skipped { .. }));
    let graph_data = matches!(&snapshot.graph, Section::Data { .. });
    let text_data = matches!(&snapshot.fulltext, Some(Section::Data { .. }));
    out.status = if graph_failed || text_failed {
        if graph_data || text_data {
            Status::Partial
        } else {
            Status::Failed
        }
    } else if !snapshot.terminal
        || snapshot
            .sources
            .iter()
            .any(|s| matches!(s.state,biodata::SourceState::Stale|biodata::SourceState::Failed|biodata::SourceState::Skipped))
    {
        Status::Partial
    } else {
        Status::Complete
    };
    if more || !snapshot.terminal {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: Request::ArticleEvidence(query.page(id, context_end, paragraph_end)),
                fresh: false,
            },
        ));
    }
    sources(out,snapshot.sources);
    out.body = QueryBody::ArticleEvidence(ArticleEvidenceResult {
        semantic_scholar: snapshot.graph,
        fulltext: snapshot.fulltext,
    });
    Ok(())
}
fn page<T: Clone + serde::Serialize>(
    items: &[T],
    offset: usize,
    budget: usize,
) -> Result<Vec<T>, RequestError> {
    let mut out = Vec::new();
    let mut bytes = 0;
    for item in items.iter().skip(offset) {
        let size = serde_json::to_vec(item)
            .map_err(|_| biomcp_fetch::CacheError::Database)?
            .len()
            + 1;
        if !out.is_empty() && bytes + size > budget {
            break;
        }
        bytes += size;
        out.push(item.clone());
    }
    Ok(out)
}

fn sources(out:&mut Outcome,sources:Vec<biodata::SourceOutcome>) {
    for incoming in sources {
        if let Some(original)=out.sources.iter_mut().find(|s|s.source==incoming.source) {crate::status::retain_source_state(original,&incoming);}else{out.sources.push(incoming)}
    }
}
