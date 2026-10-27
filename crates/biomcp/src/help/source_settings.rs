//! Explain source policy from the same registry used by health.
use biomcp_adapters::registry;
use biomcp_types::{Help, HelpCommand};

pub(super) fn help(topic: &str) -> Help {
    let mut description = String::from(
        "Every source and carried field is enabled by default. Publisher terms remain authoritative. \
        Commercial users must set {\"disable_sources\":[\"noncommercial\",\"restricted\"]}. \
        Use config disable_sources or --disable-sources with source names or groups; exclusions combine. \
        restricted means each publisher's terms need checking. Group consequences follow. ",
    );
    for group in registry::disable_groups() {
        let sources = group
            .sources
            .iter()
            .map(|&source| format!("{} ({source})", registry::display_name(source)))
            .collect::<Vec<_>>()
            .join(", ");
        let components = group
            .components
            .iter()
            .map(|c| format!("{}:{}", c.source, c.name))
            .collect::<Vec<_>>()
            .join(", ");
        description.push_str(&format!(
            "{} turns off sources: {sources}. Carried fields: {components}. ",
            group.group.as_str()
        ));
    }
    description.push_str("Carried-field exclusions leave the aggregator's other fields available. PMC Open Access (pmc-oa) supplies PMC reading.");
    Help {
        commands: vec![HelpCommand {
            reply_schemas: crate::schema::local_replies(&["help".into()]),
            command: vec!["help".into(), topic.into()],
            description,
            arguments: Vec::new(),
            sections: Vec::new(),
            section_rule: None,
            examples: Vec::new(),
        }],
    }
}
