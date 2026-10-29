//! Error signatures, exit codes and shared value diagnostics have one owner.
use super::{Check, source::tokens};
use std::collections::BTreeSet;

pub(super) fn signatures(text: &str) -> Check {
    let words = tokens(text);
    for (start, word) in words.iter().enumerate() {
        let tail = &words[start..];
        if word == "ExitCode"
            && tail.get(1..5).is_some_and(|v| v == [":", ":", "from", "("])
            && tail
                .get(5)
                .is_some_and(|v| v.chars().next().is_some_and(|c| c.is_ascii_digit()))
        {
            return Err("exit literals require Status::code()".into());
        }
        if word == "From" && tail.get(1..5).is_some_and(|v| v == ["<", "(", ")", ">"]) {
            return Err("unit error conversions are forbidden".into());
        }
        if word == "Result" && tail.get(1).is_some_and(|v| v == "<") {
            let mut depth = 0usize;
            for (i, word) in tail.iter().enumerate().skip(1) {
                match word.as_str() {
                    "<" => depth += 1,
                    ">" => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    "," if depth == 1
                        && tail.get(i + 1..i + 4).is_some_and(|v| v == ["(", ")", ">"]) =>
                    {
                        return Err("unit Result errors are forbidden".into());
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

pub(super) fn display_codes(declaration: &str, text: &str) -> Check {
    let declaration = tokens(declaration);
    let mut codes = BTreeSet::new();
    for code in declaration
        .windows(3)
        .filter(|v| v[0] == "=" && v[1] == ">" && v[2].starts_with('"'))
    {
        if !codes.insert(code[2].as_str()) {
            return Err(format!("duplicate biodata error code: {}", code[2]).into());
        }
    }
    let words = tokens(text);
    for (start, word) in words.iter().enumerate() {
        if word != "impl" {
            continue;
        }
        let Some(open) = words[start..].iter().position(|v| v == "{") else {
            continue;
        };
        if !words[start..start + open].iter().any(|v| v == "Display") {
            continue;
        }
        let mut depth = 0;
        for token in &words[start + open..] {
            match token.as_str() {
                "{" => depth += 1,
                "}" => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                token
                    if token.starts_with('"')
                        && token.contains(['.', '_'])
                        && token.chars().any(|c| c.is_ascii_lowercase())
                        && token
                            .trim_matches('"')
                            .chars()
                            .all(|c| c.is_ascii_lowercase() || c == '.' || c == '_') =>
                {
                    if !codes.contains(token) {
                        return Err(format!("undeclared biodata display code: {token}").into());
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
