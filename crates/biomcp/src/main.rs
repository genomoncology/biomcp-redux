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
            return match command.render(&parsed.globals) {
                Ok(bytes) => exit::finish(output::bytes(&bytes), exit::Status::Success),
                Err(error) => exit::failure(error),
            };
        }
        if parsed.action.is_none() && parsed.key.is_none() {
            let globals = parsed.globals;
            if let Some((address, hosts)) = parsed.http {
                return match mcp::http::serve(&address, hosts, globals) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(mcp::http::StartError::Configuration(error)) => {
                        match command::local::startup(error) {
                            Ok(outcome) => {
                                exit::finish(output::command(&outcome, &args), outcome.status)
                            }
                            Err(error) => exit::failure(error),
                        }
                    }
                    Err(mcp::http::StartError::AllowHost) => exit::finish(
                        command::encode(
                            biomcp_types::ErrorEnvelope::from(biomcp_types::UsageError::argument(
                                "--allow-host".into(),
                            )),
                            exit::Status::Usage,
                        )
                        .and_then(|outcome| output::bytes(&outcome.cli_bytes)),
                        exit::Status::Usage,
                    ),
                    Err(mcp::http::StartError::Usage) => ExitCode::from(exit::Status::Usage.code()),
                    Err(mcp::http::StartError::Local) => ExitCode::from(exit::Status::Local.code()),
                };
            }
            let fetch = config::fetch(&globals);
            return exit::finish(mcp::serve(globals, fetch), exit::Status::Success);
        }
    }
    match command::dispatch(&args) {
        Ok(outcome) => exit::finish(output::command(&outcome, &args), outcome.status),
        Err(error) => exit::failure(error),
    }
}
