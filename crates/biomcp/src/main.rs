//! The BioMCP command entry point.
mod config;
mod output;
mod schema;

use std::process::ExitCode;

fn main() -> ExitCode {
    let name = config::schema_name();
    let Some(text) = name.as_deref().and_then(|name| name.to_str()).and_then(schema::find) else {
        output::diagnostic(b"biomcp: expected schema <known artifact id>\n");
        return ExitCode::from(2);
    };
    match output::schema(text) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => {
            output::diagnostic(b"biomcp: output failed\n");
            ExitCode::from(5)
        }
    }
}
