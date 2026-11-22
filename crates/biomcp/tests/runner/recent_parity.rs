//! Compare the CLI and default MCP call against independently prepared replay fixtures.
use super::{assertions, case_data, launch, mcp, response_cases, run_sequence, schema, transcript};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::path::Path;

const WITNESSES: &[(&str, &str, usize)] = &[
    ("1056", "diagnostic/native-cdx.yaml", 6),
    ("1057", "drug/markets.yaml", 9),
    ("1058", "drug/label-printed-names.yaml", 1),
    ("1059", "drug/approvals-incomplete-top.yaml", 1),
    ("1060", "drug/label-names-from-text.yaml", 1),
    ("1061", "pathway/genes-fields.yaml", 2),
    ("1062", "disease/orphanet-id.yaml", 1),
    ("1063", "trial/printed-phase.yaml", 1),
    ("1063", "drug/label-application.yaml", 1),
    ("1064", "grammar/usage-names-the-cause.yaml", 1),
    ("1065", "diagnostic/native-cdx.yaml", 8),
    ("1066", "drug/label-history-rinvoq.yaml", 1),
    ("1067", "drug/fda-dsc-navigation.yaml", 1),
    ("1068", "diagnostic/fda-letter-partial.yaml", 1),
    ("1069", "diagnostic/fda-letter-map-read-again.yaml", 1),
    ("1071", "drug/approvals-fields.yaml", 1),
    ("1072", "article/search/pubmed-access-timeout.yaml", 1),
    ("1073", "gene/unknown-words.yaml", 3),
    ("1075", "diagnostic/fda-letter-damaged.yaml", 1),
    ("1076", "drug/label-shared-budget.yaml", 1),
    ("1078", "gene/sections/constraint-fields.yaml", 1),
    ("1079", "diagnostic/pdf-symbol-characters.yaml", 1),
    ("1081", "drug/approvals-default-page.yaml", 1),
    ("1082", "drug/label-history-lamictal.yaml", 1),
    ("1083", "drug/label-history-example.yaml", 2),
    ("1084", "diagnostic/native-cdx.yaml", 4),
    ("1085", "drug/label-history-refusals.yaml", 1),
    ("1086", "drug/label-mechanism.yaml", 1),
    ("1087", "drug/source-aliases-review-evidence.yaml", 1),
    ("1088", "drug/fda-safety-signals.yaml", 1),
    ("1090", "drug/label-find.yaml", 1),
    ("1091", "drug/label-subsections.yaml", 1),
    ("1093", "drug/fda-withdrawn-approvals.yaml", 1),
    ("1094", "drug/purple-book-age.yaml", 2),
    ("1095", "trial/acronym-and-official-title.yaml", 1),
    ("1096", "trial/sites/filters.yaml", 1),
    ("1097", "trial/matched-arms.yaml", 1),
    ("1098", "disease/gene-validity.yaml", 2),
    ("1099", "disease/hpo-release.yaml", 1),
    ("1100", "disease/orphadata-xrefs.yaml", 2),
    ("1101", "gene/sections/constraint-release.yaml", 3),
    ("1102", "pathway/member-count.yaml", 1),
    ("1103", "gene/multi-id.yaml", 1),
    ("1104", "diagnostic/fda-cdx-regimen-partner.yaml", 2),
    ("1105", "drug/approval-document-fields.yaml", 1),
    ("1106", "drug/label-fallback-application.yaml", 1),
    ("1107", "drug/label-holder-recovery.yaml", 1),
    ("1108", "drug/ranked-names.yaml", 1),
    ("1109", "trial/acronym-and-official-title.yaml", 4),
    ("1110", "trial/get/arm-projection-bytes.yaml", 1),
    ("1111", "drug/supplement-purpose-changes.yaml", 1),
    ("1112", "drug/get-refusals.yaml", 1),
    ("1113", "gene/search-source-refusals.yaml", 2),
    ("1114", "gene/sections/source-refusal-clingen.yaml", 1),
    ("1115", "disease/get-refusals.yaml", 1),
    ("1116", "phenotype/get-refusals.yaml", 1),
    ("1117", "pathway/source-refusal.yaml", 1),
    ("1118", "trial/skill-examples.yaml", 1),
    ("1118", "trial/skill-examples.yaml", 2),
    ("1118", "pgx/skill-example.yaml", 1),
    ("1120", "disease/omim-gene-entry.yaml", 1),
    ("1121", "drug/help.yaml", 1),
    ("1122", "drug/purple-book-age.yaml", 1),
    ("1125", "drug/help.yaml", 2),
    ("1126", "drug/label-section-limit.yaml", 1),
    ("1128", "gene/batch-mygene-disabled.yaml", 1),
    ("1129", "drug/indication.yaml", 6),
    ("1129", "drug/fda-withdrawn-approvals.yaml", 4),
    ("1129", "drug/fda-safety-signals.yaml", 5),
];

pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = WITNESSES
        .iter()
        .map(|&(ticket, file, step)| {
            let name = file.trim_end_matches(".yaml");
            transcript::trial(
                format!("mcp-parity/recent/{ticket}/{name}/{step}"),
                move || recorded(file, step).map_err(Into::into),
            )
        })
        .collect();
    trials.extend([
        transcript::trial("mcp-parity/recent/skill-guide", || {
            guide().map_err(Into::into)
        }),
        transcript::trial("mcp-parity/recent/live", || live().map_err(Into::into))
            .with_ignored_flag(true),
    ]);
    trials
}

fn recorded(file: &str, step: usize) -> Result<(), String> {
    let path = launch::fixtures().join("cases").join(file);
    let case = case_data::read_case(&path)?;
    let cli = replay(&case, step, false)?;
    let mcp = replay(&case, step, true)?;
    compare(&cli, &mcp)
}

fn replay(case: &case_data::Case, step: usize, mcp: bool) -> Result<Value, String> {
    let env = launch::Environment::new().map_err(|error| error.to_string())?;
    env.case_config(case).map_err(|error| error.to_string())?;
    let registry = schema::Registry::embedded()?;
    let recordings = response_cases::replay(&env, case.replay.as_deref(), case.cache_acquired)?;
    let mut previous = None;
    let mut previous_step = None;
    for (index, mut current) in case.steps.iter().take(step).cloned().enumerate() {
        transcript::case_step(index);
        current.offline |= recordings.is_none();
        if index + 1 == step {
            let args = current
                .run
                .take()
                .ok_or("parity witness needs a CLI command")?;
            if current.follow.is_some() || current.walk.is_some() || current.stdout.is_some() {
                return Err("parity witness must be one JSON command".into());
            }
            current.mcp = None;
            if mcp {
                current.mcp = Some(args);
                current.is_error = current.exit >= 2;
                current.max_bytes = None;
                current.max_tokens = None;
                current.starts_within.clear();
                current.key_order.clear();
            } else {
                current.run = Some(args);
            }
        }
        previous = Some(run_sequence(
            &current,
            &env,
            &assertions::prepare(&current)?,
            registry.as_ref(),
            recordings.as_deref(),
            previous.as_ref(),
            previous_step,
        )?);
        previous_step = case.steps.get(index);
    }
    if step > case.steps.len() {
        return Err("parity witness step is outside its case".into());
    }
    previous.ok_or_else(|| "parity witness step must be positive".into())
}

fn guide() -> Result<(), String> {
    let env = launch::Environment::new().map_err(|error| error.to_string())?;
    let cli = invoke(&env, &["skill", "render"])?;
    for mode in ["single", "typed"] {
        let (tool, arguments) = if mode == "single" {
            ("biomcp", json!({"args":["help","skill","render"]}))
        } else {
            ("help", json!({"command":"skill render"}))
        };
        let replies = mcp::exchange_with_globals(
            &env,
            &[
                mcp::request(json!(1), "tools/list", json!({})),
                mcp::request(
                    json!(2),
                    "tools/call",
                    json!({"name":tool,"arguments":arguments}),
                ),
            ],
            &["--offline".into(), "--tool-mode".into(), mode.into()],
        )?;
        let actual = replies
            .iter()
            .find(|reply| reply["id"] == 2)
            .and_then(|reply| {
                reply.pointer("/result/structuredContent/result/commands/0/description")
            })
            .and_then(Value::as_str);
        if actual != Some(cli.trim_end()) {
            return Err(format!(
                "{mode} help skill render did not return the CLI guide: {actual:?}"
            ));
        }
        if !replies
            .iter()
            .find(|reply| reply["id"] == 1)
            .and_then(|reply| reply.pointer("/result/tools"))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .any(|item| {
                item["name"] == tool
                    && item["description"]
                        .as_str()
                        .is_some_and(|text| text.contains("skill render"))
            })
        {
            return Err(format!(
                "{mode} tool catalog does not point to the agent guide"
            ));
        }
    }
    Ok(())
}

fn invoke(env: &launch::Environment, args: &[&str]) -> Result<String, String> {
    let output = env
        .command(
            Path::new(env!("CARGO_BIN_EXE_biomcp")),
            &args.iter().map(|word| (*word).into()).collect::<Vec<_>>(),
        )
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!("{args:?}: exit {}", output.status));
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn live() -> Result<(), String> {
    let cli = invoke(
        &launch::Environment::new().map_err(|error| error.to_string())?,
        &["get", "trial", "NCT03548987"],
    )?;
    let value = serde_json::from_str(&cli).map_err(|error| error.to_string())?;
    let env = launch::Environment::new().map_err(|error| error.to_string())?;
    let args = ["get", "trial", "NCT03548987"].map(str::to_owned);
    compare(&value, &mcp::call_with_globals(&env, &args, false, &[])?)
}

fn compare(cli: &Value, mcp: &Value) -> Result<(), String> {
    match assertions::difference(cli, mcp, "") {
        Some(diff) => Err(format!("CLI/MCP parity: unequal results at {diff}")),
        None => Ok(()),
    }
}
