//! Feature protein-search: retired spellings give fixed guidance before any request.
use super::*;
use super::super::mcp;
const CLI_USAGE_BYTES: usize = 640;
const MCP_USAGE_BYTES: usize = 639;
pub(super) fn trials() -> Vec<Trial> {
    CASES.iter().enumerate().map(|(n, &(command, message))| {
        Trial::test(format!("protein-search/retired-spelling-{n}"), move || {
            check(command, message).map_err(Into::into)
        })
    }).collect()
}
fn check(command: &str, message: &str) -> Result<(), String> {
    let stub = Stub::for_sources(&["rest.uniprot.org", "www.ebi.ac.uk"], |_| "application/json", |_| Err("retired command reached source".into()))?;
    let env = setup(&stub)?;
    let args: Vec<String> = command.split_whitespace().map(str::to_owned).collect();
    let cli = invoke(&env, args.clone(), 2)?;
    let mcp = mcp::call_with_globals(&env, &args, true, &[])?;
    if cli != mcp || cli.pointer("/error/code") != Some(&json!("usage.retired")) || cli.pointer("/error/message") != Some(&json!(message)) {
        return Err(format!("retirement changed guidance: {cli}"));
    }
    super::card_failures::bounded(&cli, CLI_USAGE_BYTES, MCP_USAGE_BYTES)?;
    if !stub.finish()?.is_empty() { return Err("retirement made a provider request".into()); }
    Ok(())
}
const CASES: &[(&str, &str)] = &[
    ("search protein EGFR --include-unreviewed", "--include-unreviewed is not yet supported because no unreviewed positive was captured"),
    ("search protein -q EGFR", "use: search protein <TEXT>; the positional query stays unchanged"),
    ("search protein --query=EGFR", "use: search protein <TEXT>; the positional query stays unchanged"),
    ("search protein EGFR --all-species", "--all-species is dropped; protein access stays within human-health scope"),
    ("search protein EGFR --disease cancer", "--disease and --existence are not yet supported because no harvested question needs these selectors"),
    ("search protein EGFR --existence 1", "--disease and --existence are not yet supported because no harvested question needs these selectors"),
    ("search protein EGFR --next-page cursor", "use: --pool and --offset from the returned continuation"),
    ("get protein P00533 interactions", "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein products; use: get protein <ACCESSION> complexes"),
    ("get protein P00533 --section interactions", "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein products; use: get protein <ACCESSION> complexes"),
    ("protein interactions P00533", "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein products; use: get protein <ACCESSION> complexes"),
    ("protein associations P00533", "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein products; use: get protein <ACCESSION> complexes"),
];
