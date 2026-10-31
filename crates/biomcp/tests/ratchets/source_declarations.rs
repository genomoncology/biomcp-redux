//! Bind adapter policies to their source declarations and exhaustive dispatcher.
use super::{Check, source::tokens};
use std::{collections::BTreeMap, fs, path::Path};

struct Policy {
    path: String,
    source: Option<String>,
}
struct Declaration {
    path: String,
    name: String,
    spec: Vec<String>,
    auxiliary: Vec<String>,
}

pub(super) fn check(root: &Path) -> Check {
    let adapters = root.join("crates/biomcp-adapters/src");
    if !adapters.exists() {
        return Ok(());
    }
    let mut policies = BTreeMap::new();
    let mut declarations = Vec::new();
    for path in super::rust_files(&adapters)? {
        let relative = path
            .strip_prefix(&adapters)?
            .to_string_lossy()
            .replace('\\', "/");
        let input = tokens(&fs::read_to_string(&path)?);
        collect(&relative, &input, &mut policies, &mut declarations)?;
    }
    let dispatcher = fs::read_to_string(adapters.join("decl.rs")).unwrap_or_default();
    let dispatcher = tokens(&dispatcher).join("");
    let mut registered = BTreeMap::new();
    for declaration in declarations {
        let target = format!(
            "&crate::{}::{}",
            module(&declaration.path),
            declaration.name
        );
        let source = dispatcher
            .split(',')
            .find_map(|arm| {
                let (before, after) = arm.split_once("=>")?;
                (after == target).then(|| before.rsplit("Source::").next().unwrap_or("").to_owned())
            })
            .ok_or_else(|| {
                format!(
                    "{}: source declaration must appear in decl(source): {target}",
                    declaration.path
                )
            })?;
        let primary = references(&declaration.spec, &declaration.path);
        if declaration
            .spec
            .iter()
            .any(|token| token == "component_spec")
        {
            if source_name(&declaration.spec).as_deref() != Some(source.as_str()) {
                return Err(format!(
                    "{}: source declaration references the wrong source",
                    declaration.path
                )
                .into());
            }
        } else {
            register(
                &declaration.path,
                &source,
                primary,
                &policies,
                &mut registered,
            )?;
        }
        register(
            &declaration.path,
            &source,
            references(&declaration.auxiliary, &declaration.path),
            &policies,
            &mut registered,
        )?;
    }
    for (reference, policy) in policies {
        if !registered.contains_key(&reference) {
            return Err(format!(
                "{}: implemented source must appear in its SourceDecl: expected {reference}",
                policy.path
            )
            .into());
        }
    }
    Ok(())
}

fn collect(
    path: &str,
    input: &[String],
    policies: &mut BTreeMap<String, Policy>,
    declarations: &mut Vec<Declaration>,
) -> Check {
    for (i, token) in input.iter().enumerate() {
        if input.get(i + 1).is_none_or(|token| token != "{")
            || input
                .get(i.wrapping_sub(1))
                .is_some_and(|token| token == "struct")
        {
            continue;
        }
        let body = body(input.get(i + 2..).unwrap_or_default());
        if token == "Terms" {
            terms(path, body)?;
        } else if token == "SourceSpec" {
            if field(body, "terms").is_none() {
                return Err(format!("{path}: source declaration requires publisher terms").into());
            }
            // A generic constructor carries policy arguments and declares no source facts.
            if source_name(body).is_none()
                && body.starts_with(&["source".into(), ",".into(), "terms".into(), ",".into()])
                && field(body, "terms").is_some_and(|value| value == ["terms"])
            {
                continue;
            }
            let name = named(input.get(..i).unwrap_or_default(), "static")
                .ok_or_else(|| format!("{path}: source declarations require named statics"))?;
            policies.insert(
                format!("&crate::{}::{name}", module(path)),
                Policy {
                    path: path.into(),
                    source: source_name(field(body, "source").unwrap_or_default()),
                },
            );
        } else if token == "SourceDecl" {
            let name = named(input.get(..i).unwrap_or_default(), "static")
                .ok_or_else(|| format!("{path}: source declarations require named statics"))?;
            declarations.push(Declaration {
                path: path.into(),
                name,
                spec: field(body, "spec").unwrap_or_default().to_vec(),
                auxiliary: field(body, "auxiliary").unwrap_or_default().to_vec(),
            });
        }
    }
    Ok(())
}

fn register(
    path: &str,
    source: &str,
    references: Vec<String>,
    policies: &BTreeMap<String, Policy>,
    registered: &mut BTreeMap<String, String>,
) -> Check {
    for reference in references {
        let policy = policies.get(&reference).ok_or_else(|| {
            format!("{path}: source declaration references an unknown policy: {reference}")
        })?;
        if policy
            .source
            .as_deref()
            .is_some_and(|declared| declared != source)
        {
            return Err(format!(
                "{path}: source declaration references the wrong source: {reference}"
            )
            .into());
        }
        if registered.insert(reference.clone(), path.into()).is_some() {
            return Err(format!(
                "{path}: source policy has more than one declaration owner: {reference}"
            )
            .into());
        }
    }
    Ok(())
}

fn terms(path: &str, input: &[String]) -> Check {
    for name in ["name", "url", "usage"] {
        let value = field(input, name).unwrap_or_default();
        if value.is_empty() || name != "usage" && value == ["\"\""] {
            return Err(format!("{path}: source declaration requires publisher terms").into());
        }
    }
    Ok(())
}

fn named(input: &[String], keyword: &str) -> Option<String> {
    let i = input.iter().rposition(|token| token == keyword)?;
    let suffix = input.get(i..)?;
    (!suffix
        .iter()
        .any(|token| matches!(token.as_str(), ";" | "{" | "}")))
    .then(|| suffix.get(1).cloned())
    .flatten()
}

fn module(path: &str) -> String {
    path.trim_end_matches(".rs")
        .trim_end_matches("/mod")
        .replace('/', "::")
}

fn source_name(input: &[String]) -> Option<String> {
    input.windows(4).find_map(|part| {
        (part.first().is_some_and(|token| token == "Source")
            && part.get(1).is_some_and(|token| token == ":")
            && part.get(2).is_some_and(|token| token == ":"))
        .then(|| part.get(3).cloned())
        .flatten()
    })
}

fn references(input: &[String], path: &str) -> Vec<String> {
    let parent = module(path)
        .rsplit_once("::")
        .map_or(String::new(), |(parent, _)| parent.to_owned());
    let mut references = Vec::new();
    for (i, token) in input.iter().enumerate() {
        if token != "&"
            || input
                .get(i + 1)
                .is_none_or(|token| !matches!(token.as_str(), "crate" | "super"))
        {
            continue;
        }
        let value: String = input
            .get(i..)
            .unwrap_or_default()
            .iter()
            .take_while(|token| {
                token.as_str() == "&"
                    || token.as_str() == ":"
                    || token.chars().all(|c| c.is_alphanumeric() || c == '_')
            })
            .cloned()
            .collect();
        references.push(value.replacen("&super::", &format!("&crate::{parent}::"), 1));
    }
    references
}

fn body(input: &[String]) -> &[String] {
    let mut depth = 1;
    let end = input
        .iter()
        .position(|token| {
            if token == "{" {
                depth += 1;
            }
            if token == "}" {
                depth -= 1;
            }
            depth == 0
        })
        .unwrap_or(input.len());
    input.get(..end).unwrap_or_default()
}

fn field<'a>(input: &'a [String], name: &str) -> Option<&'a [String]> {
    let mut depth = 0;
    let mut start = 0;
    for (i, token) in input.iter().enumerate() {
        if matches!(token.as_str(), "{" | "[" | "(") {
            depth += 1;
        }
        if matches!(token.as_str(), "}" | "]" | ")") {
            depth -= 1;
        }
        if token == "," && depth == 0 {
            if input.get(start).is_some_and(|token| token == name) {
                let value = if input.get(start + 1).is_some_and(|token| token == ":") {
                    start + 2
                } else {
                    start
                };
                return input.get(value..i);
            }
            start = i + 1;
        }
    }
    if input.get(start).is_some_and(|token| token == name) {
        let value = if input.get(start + 1).is_some_and(|token| token == ":") {
            start + 2
        } else {
            start
        };
        input.get(value..)
    } else {
        None
    }
}
