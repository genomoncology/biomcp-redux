//! YAML cases run the binary Cargo built, one independently named test per file.
#![expect(
    clippy::disallowed_methods,
    reason = "The test entry point reads arguments and locates its own executable"
)]
mod runner;

use libtest_mimic::{Arguments, Trial};
use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    match trials() {
        Ok((arguments, trials)) => {
            // libtest-mimic 0.8.2 exposes its own exact/skip/ignored selector.
            // Selected ignored tests remain selected; --list has no result counts.
            if let Some(filter) = &arguments.filter
                && trials.iter().all(|trial| arguments.is_filtered_out(trial))
            {
                use std::io::Write;
                let _ = writeln!(
                    std::io::stderr(),
                    "cases: filter {filter:?} selected no tests"
                );
                return ExitCode::FAILURE;
            }
            libtest_mimic::run(&arguments, trials).exit_code()
        }
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
    let directory = directory.unwrap_or_else(|| runner::launch::fixtures().join("cases"));
    let mut trials = runner::discover(&directory)?;
    trials.extend(runner::proof::trials());
    trials.extend(runner::protocol_trials());
    runner::validate_features(&trials, directory == runner::launch::fixtures().join("cases"))?;
    Ok((Arguments::from_iter(cargo_args), trials))
}
