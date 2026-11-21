//! Feature schema-discovery: an advertised publisher has a usable command form.
use super::launch::Environment;
use biomcp_types::{Help, HelpCommand, Source};
use serde_json::Value;

type Check<T = ()> = Result<T, String>;

pub(super) fn trial() -> libtest_mimic::Trial {
    super::transcript::trial("schema-discovery/help-sources-validate-offline", || {
        check().map_err(Into::into)
    })
}

fn check() -> Check {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let mut failures = Vec::new();
    for (verb, entity, _) in biomcp_types::command_paths() {
        let (_, reply) = run(&env, &["help".into(), verb.into(), entity.into()])?;
        let help: Help = serde_json::from_value(reply["result"].clone())
            .map_err(|error| format!("help {verb} {entity}: {error}"))?;
        let command = help.commands.first().ok_or("missing command help")?;
        let (_, sources) = command
            .description
            .lines()
            .next()
            .and_then(|line| line.trim_end_matches('.').split_once(" Sources: "))
            .ok_or_else(|| format!("missing source line for {verb} {entity}"))?;
        for source in sources.split(", ") {
            if let Err(error) = validates(&env, command, source) {
                failures.push(format!("{verb} {entity} / {source}: {error}"));
            }
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}

fn validates(env: &Environment, command: &HelpCommand, source: &str) -> Check {
    let [verb, entity] = command.command.as_slice() else {
        return Err("expected two command words".into());
    };
    if let Some(flag) = command.arguments.iter().find(|arg| {
        matches!(arg.name.as_str(), "--source" | "--sources" | "--resolver")
            && arg.choices.iter().any(|choice| choice == source)
    }) {
        let sample = selected_form(verb, entity, source);
        let mut args = vec![verb.clone(), entity.clone()];
        args.extend(sample.iter().map(|word| (*word).to_owned()));
        args.extend([flag.name.clone(), source.into()]);
        return accepted(env, &args);
    }

    // Fixed and implicit publishers use a section, an ID form, or the ordinary example.
    let parsed = source
        .parse::<Source>()
        .map_err(|error| error.to_string())?;
    if let Some(section) = command.sections.iter().find(|section| {
        biomcp_types::command_section_sources(verb, entity, &section.word, &[])
            .is_some_and(|publishers| publishers.contains(&parsed))
    }) {
        let operands = match (verb.as_str(), entity.as_str()) {
            ("get", "gene") => vec!["BRAF", "--section", &section.word],
            ("get", "article") => vec!["30545842", "--section", &section.word],
            ("get", "protein") => vec!["P00533", &section.word],
            ("get", "variant") => vec!["rs113488022", &section.word],
            _ => return Err(format!("missing example for section {}", section.word)),
        };
        let mut args = command.command.clone();
        args.extend(operands.into_iter().map(str::to_owned));
        return accepted(env, &args);
    }
    if let Some(operands) = implicit_form(verb, entity, source) {
        let mut args = command.command.clone();
        args.extend(operands.iter().map(|word| (*word).to_owned()));
        return accepted(env, &args);
    }
    let example = command.examples.first().ok_or("missing help example")?;
    let (_, reply) = run(env, example)?;
    if reply["sources"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|entry| entry["source"] == source)
    {
        Ok(())
    } else {
        Err(format!(
            "no source route witnessed by {example:?}: {}",
            reply["error"]
        ))
    }
}

fn accepted(env: &Environment, args: &[String]) -> Check {
    let (code, reply) = run(env, args)?;
    if matches!(code, Some(0 | 1 | 4)) {
        Ok(())
    } else {
        Err(format!("{args:?} exited {code:?}: {}", reply["error"]))
    }
}

fn run(env: &Environment, args: &[String]) -> Check<(Option<i32>, Value)> {
    let args = std::iter::once("--offline".to_owned())
        .chain(args.iter().cloned())
        .collect::<Vec<_>>();
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|error| error.to_string())?;
    let reply =
        serde_json::from_slice(&output.stdout).map_err(|error| format!("{args:?}: {error}"))?;
    Ok((output.status.code(), reply))
}

fn selected_form(verb: &str, entity: &str, source: &str) -> &'static [&'static str] {
    match (verb, entity, source) {
        ("search", "biomarker", _) => &["PD-L1"],
        ("search", "organization", _) => &["Mayo"],
        ("search", "author", _) => &["Jane Smith"],
        ("search", "pathway", _) => &["MAPK"],
        ("search", "pgx", _) => &["--gene", "CYP2C19"],
        ("search", "phenotype", _) => &["seizure"],
        ("search", "diagnostic", _) => &[],
        ("search", "adverse-event", "vaers") => &["--vaccine", "COVID19"],
        ("search", "adverse-event", "maude") => &["--device", "pump"],
        ("search", "adverse-event", _) => &["--drug", "imatinib"],
        ("search", "disease", "hpo") => {
            &["--phenotype", "HP:0001250", "--hpo-release", "v2025-11-24"]
        }
        ("search", "disease", _) => &["melanoma"],
        ("search", "drug", _) => &["imatinib"],
        ("search", "article", _) => &["BRAF V600E"],
        ("search", "trial", _) => &["--condition", "melanoma"],
        ("search", "variant", _) => &["BRAF V600E"],
        ("get", "biomarker", _) => &["C96022"],
        ("get", "phenotype", "ols") => &["HP:0001250", "ancestors"],
        ("get", "phenotype", _) => &["HP:0001250"],
        ("get", "pgx", "clinpgx") => &["--gene", "CYP2C19", "annotations"],
        ("get", "pgx", _) => &["--gene", "CYP2C19"],
        ("get", "diagnostic", "fda-510k") => &["K171887"],
        ("get", "diagnostic", "fda-device-recall") => &["Z-0877-2024"],
        ("get", "diagnostic", "fda-pma") => &["P170019"],
        ("get", "diagnostic", "gtr") => &["GTR000000001.1"],
        ("get", "diagnostic", "who-ivd") => &["PQDx 0157-013-00"],
        ("get", "adverse-event", _) => &["12345678"],
        ("get", "disease", "orphadata") => &["ORPHA:558", "prevalence"],
        ("get", "disease", "seer") => &["melanoma"],
        ("get", "disease", _) => &["MONDO:0005105"],
        ("get", "drug", "fda-dsc") => &["ffffffff-ffff-ffff-ffff-ffffffffffff"],
        ("get", "drug", _) => &["imatinib"],
        ("get", "trial", _) => &["NCT02568267"],
        ("get", "variant", _) | ("variant", "population", _) => {
            &["NM_004333.6:c.1799T>A", "--build", "GRCh38"]
        }
        ("article", "citations" | "references", _) => &["30545842"],
        ("variant", "assertions", _) => &["clinvar:13961"],
        ("variant", "recurrence", _) => &["7-140753336-A-T", "--study", "msk_impact_2017"],
        ("variant", "normalize", _) => &["--hgvs", "NM_004333.6:c.1799T>A", "--build", "GRCh38"],
        _ => &[],
    }
}

fn implicit_form(verb: &str, entity: &str, source: &str) -> Option<&'static [&'static str]> {
    Some(match (verb, entity, source) {
        ("search", "gene", "clingen") => &["--cytoband", "17p13.3"],
        ("get", "author", "orcid") | ("author", "papers", "orcid") => {
            &["orcid:0000-0002-1825-0097"]
        }
        ("get", "author", "semantic-scholar") | ("author", "papers", "semantic-scholar") => {
            &["semantic-scholar:1741101"]
        }
        ("get", "gene", "mygene") => &["BRAF"],
        ("get", "protein", "hgnc") => &["EGFR"],
        ("get", "article", "europepmc") => &["PMC6549796"],
        ("get", "pathway", "wikipathways") | ("pathway", "genes", "wikipathways") => &["WP254"],
        ("article", "text", "europepmc" | "pmc-oa") => &["PMC6549796", "--body"],
        ("article", "supplement", "pmc-oa") => &["PMC6549796", "--file", "supplement.pdf"],
        ("article", "citation-evidence", "pmc-oa") => &["30545842"],
        ("variant", "population", "gnomad" | "myvariant") => &["7-140753336-A-T"],
        _ => return None,
    })
}
