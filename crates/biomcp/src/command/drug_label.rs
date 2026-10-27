//! Keep complete fragment units inside the label envelope allocation.
use biomcp_types::{Envelope, QueryBody, Request, Section, StepKind};
use std::io;
const LABEL_BYTES: usize = 8192;
pub(super) fn bound(
    request: &Request,
    out: &mut biomcp_query::Outcome,
    fresh: bool,
    globals: &crate::grammar::Globals,
) -> io::Result<()> {
    let Request::GetDrug(query) = request else {
        return Ok(());
    };
    loop {
        let QueryBody::DrugLabel(Some(body)) = &out.body else {
            return Ok(());
        };
        if !matches!(body.label, Section::Data { .. }) {
            return Ok(());
        }
        let mut envelope = Envelope::new(out.status, query.clone(), Some(body.clone()));
        envelope.sources = out
            .sources
            .iter()
            .cloned()
            .map(biomcp_adapters::registry::restore_outcome)
            .collect();
        envelope.next = super::steps::next( out, globals)?;
        envelope.problems = out.problems.clone();
        envelope.page = out.page.clone();
        if serde_json::to_vec(&envelope)?.len() < LABEL_BYTES {
            return Ok(());
        }
        let QueryBody::DrugLabel(Some(body)) = &mut out.body else {
            return Ok(());
        };
        let Section::Data { data, .. } = &mut body.label else {
            return Ok(());
        };
        let mut fragments = std::mem::take(&mut data.fragments).into_vec();
        if fragments.len() <= 1 {
            return Err(io::Error::other(
                "label identity and one fragment exceed the output allocation",
            ));
        }
        fragments.pop();
        let characters = fragments
            .iter()
            .map(|f| f.characters as usize)
            .sum::<usize>();
        data.text = data
            .text
            .chars()
            .take(characters)
            .collect::<String>()
            .into();
        data.fragments = fragments.into();
        let returned = data.fragments.len() as u32;
        if let Some(page) = &mut out.page {
            page.returned = returned;
        }
        out.next.retain(|(kind, _)| *kind != StepKind::Page);
        let offset = data
            .offset
            .checked_add(returned)
            .ok_or_else(|| io::Error::other("label offset overflow"))?;
        let request = query
            .at_section_offset(offset)
            .map_err(|_| io::Error::other("invalid label continuation"))?;
        out.next.push((
            StepKind::Page,
            biomcp_query::Action::Run {
                request: Request::GetDrug(request),
                fresh,
            },
        ));
    }
}
