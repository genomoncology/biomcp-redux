//! Feature source-controls: source-owned citation policy is unique and safe.
use biomcp_adapters::registry::{SOURCES, records};
use biomcp_types::Source;

pub(super) fn run() -> Result<(), String> {
    for (index, row) in records::DECLARATIONS.iter().enumerate() {
        if !SOURCES.iter().any(|spec| spec.source == row.source)
            || row.display_name.is_empty()
            || records::DECLARATIONS
                .iter()
                .skip(index + 1)
                .any(|other| row.source == other.source && row.kind == other.kind)
        {
            return Err(format!("ambiguous or unregistered record policy: {row:?}"));
        }
        if let Some(template) = row.template
            && (!template.starts_with("https://") || template.matches("{id}").count() != 1)
        {
            return Err(format!("invalid canonical record template: {row:?}"));
        }
        for id in ["", ".", "..", "bad\nrecord"] {
            if records::record_url(row.source, row.kind, id).is_some() {
                return Err("invalid native identity became a citation".into());
            }
        }
    }
    let actual = records::record_url(Source::Cbioportal, records::RecordKind::Study, "a&b/#é");
    if actual.as_deref() != Some("https://www.cbioportal.org/study/summary?id=a%26b%2F%23%C3%A9") {
        return Err(format!(
            "opaque identity changed citation URL structure: {actual:?}"
        ));
    }
    if records::record_url(
        Source::Reactome,
        records::RecordKind::Pathway,
        "R-HSA-109581",
    )
    .as_deref()
        != Some("https://reactome.org/content/detail/R-HSA-109581")
        || records::record_url(Source::WhoPq, records::RecordKind::Drug, "BT-ON001").is_some()
        || records::declaration(Source::Reactome, records::RecordKind::Study).is_some()
    {
        return Err("citation policy crossed a native ID namespace or invented a page".into());
    }
    Ok(())
}
