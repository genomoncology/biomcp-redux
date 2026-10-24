//! The BioMCP command entry point.
mod command;
mod config;
mod exit;
mod grammar;
mod mcp;
mod output;
mod paths;
mod retired;
mod schema;
mod spell;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args = config::arguments();
    if let Ok(grammar::Parsed {
        action: None,
        globals,
    }) = grammar::parse(&args)
    {
        let fetch = config::fetch(&globals);
        return exit::finish(mcp::serve(globals, fetch), exit::Status::Success);
    }
    match command::dispatch(&args) {
        Ok(outcome) => exit::finish(output::command(&outcome), outcome.status),
        Err(error) => exit::finish(Err(error), exit::Status::Success),
    }
}
