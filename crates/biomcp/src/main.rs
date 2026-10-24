//! The BioMCP command entry point.
mod config;
mod exit;
mod output;
mod schema;

use std::process::ExitCode;

fn main() -> ExitCode {
    let name = config::schema_name();
    match name
        .as_deref()
        .and_then(|name| name.to_str())
        .and_then(schema::find)
    {
        Some(text) => exit::finish(output::schema(text), exit::Status::Success),
        None => exit::finish(output::usage(), exit::Status::Usage),
    }
}
