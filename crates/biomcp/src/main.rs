//! The BioMCP command entry point.
mod command;
mod config;
mod exit;
mod grammar;
mod help;
mod housekeeping;
mod keys;
mod mcp;
mod output;
mod paths;
mod retired;
mod save;
mod schema;
mod spell;
mod view;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args = config::arguments();
    if let Ok(parsed) = grammar::parse(&args)
        && let Some(command) = parsed.housekeeping
    {
        return exit::finish(
            command
                .render(&parsed.globals)
                .and_then(|bytes| output::bytes(&bytes)),
            exit::Status::Success,
        );
    }
    if let Ok(grammar::Parsed {
        action: None,
        housekeeping: None,
        key: None,
        globals,
        http,
        ..
    }) = grammar::parse(&args)
    {
        if let Some((address, hosts)) = http {
            return match mcp::http::serve(&address, hosts, globals) {
                Ok(()) => ExitCode::SUCCESS,
                Err(mcp::http::StartError::Usage) => ExitCode::from(2),
                Err(mcp::http::StartError::Local) => ExitCode::from(5),
            };
        }
        let fetch = config::fetch(&globals);
        return exit::finish(mcp::serve(globals, fetch), exit::Status::Success);
    }
    match command::dispatch(&args) {
        Ok(outcome) => exit::finish(output::command(&outcome, &args), outcome.status),
        Err(error) => exit::finish(Err(error), exit::Status::Success),
    }
}
