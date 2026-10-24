//! YAML cases run the binary Cargo built, one independently named test per file.
#![expect(
    clippy::disallowed_methods,
    reason = "The test entry point reads arguments and locates its own executable"
)]
mod runner;

use libtest_mimic::{Arguments, Trial};
use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    if env::args().nth(1).as_deref() == Some("--live-check") {
        return match runner::live::run() {
            Ok(report) => { use std::io::Write; let _ = writeln!(std::io::stdout(), "{report}"); ExitCode::SUCCESS },
            Err(error) => { use std::io::Write; let _ = writeln!(std::io::stderr(), "live-check: {error}"); ExitCode::FAILURE },
        };
    }
    if env::args().nth(1).as_deref() == Some("--runner-environment-probe") {
        return runner::proof::environment_probe();
    }
    match trials() {
        Ok((arguments, trials)) => libtest_mimic::run(&arguments, trials).exit_code(),
        Err(error) => {
            use std::io::Write;
            let _ = writeln!(std::io::stderr(), "cases: {error}");
            ExitCode::FAILURE
        }
    }
}

fn trials() -> Result<(Arguments, Vec<Trial>), String> {
    let mut args = env::args();
    let mut cargo_args = vec![args.next().unwrap_or_else(|| "cases".into())];
    let mut directory = None;
    while let Some(argument) = args.next() {
        if argument == "--cases" {
            if directory.is_some() {
                return Err("--cases may appear only once".into());
            }
            directory = Some(PathBuf::from(
                args.next().ok_or("--cases requires a directory")?,
            ));
        } else {
            cargo_args.push(argument);
        }
    }
    let default = directory.is_none();
    let directory = directory
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/cases"));
    let mut trials = runner::discover(&directory, default)?;
    if default {
        trials.extend(runner::proof::trials());
        trials.extend(runner::protocol_trials());
    }
    Ok((Arguments::from_iter(cargo_args), trials))
}
