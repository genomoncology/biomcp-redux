//! The BioMCP command entry point.
mod command;
mod config;
mod exit;
mod grammar;
mod mcp;
mod output;
mod paths;
mod schema;
mod spell;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args = config::arguments();
    if let Some(position) = args.iter().position(|arg| arg == "serve") {
        let mut globals = args.clone();
        globals.remove(position);
        let mut probe = vec!["get".into(), "gene".into(), "BRAF".into()];
        probe.extend_from_slice(&globals);
        if grammar::parse(&probe).is_ok() {
            return exit::finish(mcp::serve(globals), exit::Status::Success);
        }
    }
    match command::dispatch(&args) {
        Ok(outcome) => exit::finish(output::command(&outcome), outcome.status),
        Err(error) => exit::finish(Err(error), exit::Status::Success),
    }
}
