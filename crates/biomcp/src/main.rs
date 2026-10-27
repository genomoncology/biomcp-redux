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
mod skill;
mod spell;
mod view;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args = config::arguments();
    if let Ok(parsed) = grammar::parse(&args) {
        if let Some(command) = parsed.housekeeping {
            return exit::finish(
                command
                    .render(&parsed.globals)
                    .and_then(|bytes| output::bytes(&bytes)),
                exit::Status::Success,
            );
        }
        if parsed.action.is_none() && parsed.key.is_none() {
            let globals = parsed.globals;
            if let Some((address, hosts)) = parsed.http {
                return match mcp::http::serve(&address, hosts, globals) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(mcp::http::StartError::AllowHost) => exit::finish(
                        command::encode(
                            biomcp_types::ErrorEnvelope::argument("--allow-host".into()),
                            exit::Status::Usage,
                        )
                        .and_then(|outcome| output::bytes(&outcome.cli_bytes)),
                        exit::Status::Usage,
                    ),
                    Err(mcp::http::StartError::Usage) => ExitCode::from(2),
                    Err(mcp::http::StartError::Local) => ExitCode::from(5),
                };
            }
            let fetch = config::fetch(&globals);
            return exit::finish(mcp::serve(globals, fetch), exit::Status::Success);
        }
    }
    match command::dispatch(&args) {
        Ok(outcome) => exit::finish(output::command(&outcome, &args), outcome.status),
        Err(error) => exit::finish(Err(error), exit::Status::Success),
    }
}
