//! YAML cases run the binary Cargo built, one independently named test per file.
#![expect(
    clippy::disallowed_methods,
    reason = "The test entry point reads arguments and locates its own executable"
)]
mod runner;

use libtest_mimic::{Arguments, Trial};
use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    if env::args().nth(1).as_deref() == Some("--parity") {
        return runner::parity::main(env::args().skip(2).collect());
    }
    if let Some(code) = native_proof() {
        return code;
    }
    if env::args().nth(1).as_deref() == Some("--installed-http-proof") {
        if env::args().len() != 2 {
            return ExitCode::FAILURE;
        }
        return libtest_mimic::run(
            &Arguments::from_iter(env::args().take(1)),
            runner::installed_http_trials(),
        )
        .exit_code();
    }
    if env::args().nth(1).as_deref() == Some("--live-check") {
        let mode = env::args().nth(2);
        let list = mode.as_deref() == Some("--list");
        let details = mode.as_deref() == Some("--details");
        if mode.is_some() && !list && !details
            || env::args().len() > if mode.is_some() { 3 } else { 2 }
        {
            return ExitCode::FAILURE;
        }
        return match runner::live::run(list, details) {
            Ok(report) => {
                use std::io::Write;
                let _ = writeln!(std::io::stdout(), "{report}");
                ExitCode::SUCCESS
            }
            Err(error) => {
                use std::io::Write;
                let _ = writeln!(std::io::stderr(), "live-check: {error}");
                ExitCode::FAILURE
            }
        };
    }
    if env::args().nth(1).as_deref() == Some("--runner-environment-probe") {
        return runner::proof::environment_probe();
    }
    if env::args().nth(1).as_deref() == Some("--installed-process-proof") {
        return match runner::installed_process_proof() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                use std::io::Write;
                let _ = writeln!(std::io::stderr(), "installed process proof: {error}");
                ExitCode::FAILURE
            }
        };
    }
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

fn native_proof() -> Option<ExitCode> {
    let measure: Option<fn() -> Result<(), String>> = match env::args().nth(1).as_deref() {
        Some("--cell-line-section-release-measure") => {
            Some(runner::cell_line::measure_section_release)
        }
        Some("--cell-line-help-measure") => Some(runner::cell_line::measure_help),
        Some("--cell-line-sections-measure") => Some(runner::cell_line::measure_sections),
        Some("--cell-line-measure") => Some(runner::cell_line::measure),
        Some("--cell-line-measure-remaining") => Some(runner::cell_line::measure_remaining),
        Some("--cell-line-canonical-measure") => Some(runner::cell_line::measure_canonical),
        _ => None,
    };
    if let Some(measure) = measure {
        return Some(match measure() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                use std::io::Write;
                let _ = writeln!(std::io::stderr(), "{error}");
                ExitCode::FAILURE
            }
        });
    }
    let mode = env::args().nth(1)?;
    let run = match mode.as_str() {
        "--cell-line-xrefs" => runner::cell_line::generate_xrefs,
        "--cell-line-examples" => runner::cell_line::generate_examples,
        "--faers-search-synthetic-examples" => runner::faers_search_examples,
        "--seer-native-proof" => runner::seer_native,
        "--trial-condition-native-proof" => runner::trial_condition_native,
        "--monarch-native-proof" => runner::monarch_native,
        "--phenotype-native-proof" => runner::phenotype_native,
        "--protein-native-proof" => runner::protein_native,

        "--mydisease-native-proof" => runner::mydisease_native,
        "--disease-native-proof" => runner::disease_native,
        _ => return None,
    };
    Some(
        match env::args()
            .nth(2)
            .map(PathBuf::from)
            .ok_or("missing evidence directory".to_owned())
            .and_then(|p| run(&p))
        {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                use std::io::Write;
                let _ = writeln!(std::io::stderr(), "{error}");
                ExitCode::FAILURE
            }
        },
    )
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
