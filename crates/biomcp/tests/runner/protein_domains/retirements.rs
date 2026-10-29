//! Feature protein-search: retired spellings give fixed guidance before any request.
use super::super::mcp;
use super::{Environment, Stub, Trial, invoke, json, setup};
use crate::runner::transcript::trial as transcript_trial;
const CLI_SECTION_USAGE_BYTES: usize = 166;
const MCP_SECTION_USAGE_BYTES: usize = 165;
pub(super) fn trials() -> Vec<Trial> {
    CASES
        .iter()
        .enumerate()
        .map(|(n, &(command, message, cli_ceiling, mcp_ceiling))| {
            transcript_trial(format!("protein-search/retired-spelling-{n}"), move || {
                check(command, message, cli_ceiling, mcp_ceiling).map_err(Into::into)
            })
        })
        .chain([transcript_trial(
            "protein-search/unsupported-section-and-shared-repeat-grammar",
            || boundaries().map_err(Into::into),
        )])
        .collect()
}
fn check(
    command: &str,
    message: &str,
    cli_ceiling: usize,
    mcp_ceiling: usize,
) -> Result<(), String> {
    let stub = Stub::for_sources(
        &["rest.uniprot.org", "www.ebi.ac.uk"],
        |_| "application/json",
        |_| Err("retired command reached source".into()),
    )?;
    let env = setup(&stub)?;
    let args: Vec<String> = command.split_whitespace().map(str::to_owned).collect();
    let cli = invoke(&env, args.clone(), 2)?;
    let mcp = mcp::call_with_globals(&env, &args, true, &[])?;

    if cli != mcp
        || cli.pointer("/error/code") != Some(&json!("usage.retired"))
        || cli.pointer("/error/message") != Some(&json!(message))
    {
        return Err(format!("retirement changed guidance: {cli}"));
    }
    super::card_failures::bounded(&cli, cli_ceiling, mcp_ceiling)?;
    if !stub.finish()?.is_empty() {
        return Err("retirement made a provider request".into());
    }
    Ok(())
}
// Complete synthetic migration envelopes measured in j15006, with fifteen percent headroom.
const CASES: &[(&str, &str, usize, usize)] = &[
    (
        "search protein EGFR --include-unreviewed",
        "--include-unreviewed is not yet supported because no unreviewed positive was captured",
        203,
        202,
    ),
    (
        "search protein -q EGFR",
        "use: search protein <TEXT>; the positional query stays unchanged",
        179,
        178,
    ),
    (
        "search protein --query=EGFR",
        "use: search protein <TEXT>; the positional query stays unchanged",
        179,
        178,
    ),
    (
        "search protein EGFR --all-species",
        "--all-species is dropped; protein access stays within human-health scope",
        188,
        187,
    ),
    (
        "search protein EGFR --disease cancer",
        "--disease and --existence are not yet supported because no harvested question needs these selectors",
        219,
        218,
    ),
    (
        "search protein EGFR --existence 1",
        "--disease and --existence are not yet supported because no harvested question needs these selectors",
        219,
        218,
    ),
    (
        "search protein EGFR --next-page cursor",
        "use: --pool and --offset from the returned continuation",
        168,
        167,
    ),
    (
        "get protein P00533 interactions",
        "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein products; \
        use: get protein <ACCESSION> complexes",
        272,
        271,
    ),
    (
        "get protein P00533 --section interactions",
        "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein products; \
        use: get protein <ACCESSION> complexes",
        272,
        271,
    ),
    (
        "protein interactions P00533",
        "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein products; \
        use: get protein <ACCESSION> complexes",
        272,
        271,
    ),
    (
        "protein associations P00533",
        "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein products; \
        use: get protein <ACCESSION> complexes",
        272,
        271,
    ),
];

fn boundaries() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    for args in [
        vec!["get", "protein", "P00533", "--section=unknown"],
        vec!["get", "protein", "P00533", "--section", "unknown"],
    ] {
        let mut words = vec!["--offline".into()];
        words.extend(args.iter().map(|s| (*s).to_owned()));
        let value = invoke(&env, words, 2)?;
        if value.pointer("/error/code") != Some(&json!("usage.arguments")) {
            return Err("unsupported section was accepted".into());
        }

        let mcp = mcp::call_with_globals(
            &env,
            &args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
            true,
            &["--offline".into()],
        )?;

        if value != mcp {
            return Err("unsupported section CLI/MCP differs".into());
        }
        super::card_failures::bounded(&value, CLI_SECTION_USAGE_BYTES, MCP_SECTION_USAGE_BYTES)?;
    }
    Ok(())
}
