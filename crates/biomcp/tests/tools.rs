//! Explicit parity, measurement, live-check and generation tools.
#![expect(
    clippy::disallowed_methods,
    reason = "The test entry point reads arguments and locates its own executable"
)]
mod runner;

use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    if env::args().nth(1).as_deref() == Some("--parity") {
        return runner::parity::main(env::args().skip(2).collect());
    }
    if env::args().nth(1).as_deref() == Some("--parity-inventory") {
        return runner::parity::inventory_main(env::args().skip(2).collect());
    }
    if let Some(code) = native_proof() {
        return code;
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
    ExitCode::FAILURE
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

