//! The BioMCP command entry point.
mod command;
mod config;
mod exit;
mod mcp;
mod output;
mod schema;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args = config::arguments();
    if args.as_slice() == ["serve"] {
        return exit::finish(mcp::serve(), exit::Status::Success);
    }
    match command::dispatch(&args) {
        Ok(outcome) => exit::finish(output::command(&outcome), outcome.status),
        Err(error) => exit::finish(Err(error), exit::Status::Success),
    }
}
