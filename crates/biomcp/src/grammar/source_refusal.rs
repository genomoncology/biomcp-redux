//! Explain rejected source selectors and exclusions.
use biodata::{PathwayId, Text};
use biomcp_query::sources as registry;
use biomcp_types::{ErrorMessage, Request, Rewrite, Source, UsageArguments, UsageError};
use clap::Command;
use std::ffi::OsString;

pub(super) fn check_exclusions(names: &[String]) -> Result<(), super::ParseInvalid> {
    registry::disabled(names).map(|_| ()).map_err(|_| {
        let accepted = registry::switchable()
            .into_iter()
            .map(|source| source.to_string())
            .chain(
                registry::disable_groups()
                    .into_iter()
                    .map(|group| group.group.as_str().to_owned()),
            )
            .collect::<Vec<_>>();
        let value = names
            .iter()
            .find(|name| !accepted.contains(name))
            .filter(|name| {
                registry::components().any(|component| {
                    name.as_str() == component.name
                        || name.split_once(':').is_some_and(|(source, field)| {
                            field == component.name && source == <&str>::from(component.source)
                        })
                })
            })
            .and_then(|name| Text::new(name).ok());
        super::ParseInvalid {
            usage: Some(UsageError::SourceExclusion {
                value,
                valid_values: accepted
                    .iter()
                    .filter_map(|name| Text::new(name).ok())
                    .collect(),
            }),
            ..super::ParseInvalid::default()
        }
    })
}

pub(crate) fn for_command(
    root: &Command,
    path: &[&str],
    spelling: Option<&str>,
    args: &[OsString],
) -> Option<super::ParseInvalid> {
    if path == ["get", "pathway"] && spelling?.split('=').next() == Some("--source") {
        return pathway(root, args);
    }
    super::context::gene_source_refusal(root, path, spelling, args)
}

fn pathway(root: &Command, args: &[OsString]) -> Option<super::ParseInvalid> {
    let index = args
        .iter()
        .take_while(|word| *word != "--")
        .position(|word| {
            word.to_str().and_then(|word| word.split('=').next()) == Some("--source")
        })?;
    let (source, remaining) = super::positional::source_value(args, index)?;
    if !source.eq_ignore_ascii_case(Source::Reactome.into()) {
        return None;
    }
    let matches = root
        .clone()
        .try_get_matches_from(std::iter::once(OsString::from("biomcp")).chain(remaining))
        .ok()?;
    let matches = matches
        .subcommand_matches("get")?
        .subcommand_matches("pathway")?;
    let value = super::request_arguments("get", "pathway", matches).ok()?;
    let request = super::rewrite::request(Rewrite::Direct, "get", "pathway", value).ok()?;
    let Request::GetPathway(query) = &request else {
        return None;
    };
    if !matches!(query.id(), PathwayId::Reactome(_)) {
        return None;
    }
    Some(super::ParseInvalid {
        usage: Some(UsageError::Arguments(UsageArguments {
            argument: Some("--source".into()),
            message: Some(ErrorMessage::PathwaySourceImplicit),
            valid_values: None,
            next: vec![super::recovery::run(&request)?],
        })),
        ..super::ParseInvalid::default()
    })
}
