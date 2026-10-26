//! Credential reads are observable only at the settings boundary, before outgoing headers exist.
use super::*;
use std::sync::mpsc;

fn execute(words: &[&str], allowed: Option<SecretName>) -> usize {
    let (send, receive) = mpsc::channel();
    let secrets = secrets_using(None, move |name| {
        assert_eq!(Some(name), allowed.map(SecretName::environment), "forbidden key read");
        send.send(()).unwrap();
        None
    });
    assert!(!format!("{secrets:?}").contains("API_KEY"));
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Offline,
        secrets,
        ..FetchConfig::default()
    }).unwrap();
    let args = words.iter().map(OsString::from).collect::<Vec<_>>();
    let parsed = crate::grammar::parse(&args).unwrap();
    let fetch = super::restrict(fetch, &parsed.globals).unwrap();
    crate::command::execute(&fetch, parsed.action.unwrap(), &parsed.globals, false).unwrap();
    receive.try_iter().count()
}

#[test]
fn unrelated_and_disabled_commands_never_read_keys() {
    for command in [
        vec!["get", "gene", "HGNC:3236"],
        vec!["search", "trial", "--condition", "melanoma"],
        vec!["get", "variant", "clinvar:13961", "clinvar", "--disable-sources", "clinvar"],
        vec!["search", "article", "EGFR", "--source", "semantic-scholar", "--disable-sources", "semantic-scholar"],
        vec!["get", "variant", "7:g.140453136A>T", "oncokb", "--build", "GRCh37", "--disable-sources", "oncokb"],
        vec!["search", "trial", "--source", "nci", "--keyword", "melanoma", "--disable-sources", "nci"],
    ] {
        assert_eq!(execute(&command, None), 0);
    }
}

#[test]
fn called_sources_read_only_their_own_environment_key_once() {
    for (name, command) in [
        (SecretName::NcbiApiKey, vec!["get", "variant", "clinvar:13961", "clinvar"]),
        (SecretName::SemanticScholarApiKey, vec!["search", "article", "EGFR", "--source", "semantic-scholar"]),
        (SecretName::OncokbApiKey, vec!["get", "variant", "7:g.140453136A>T", "oncokb", "--build", "GRCh37"]),
        (SecretName::NciApiKey, vec!["search", "trial", "--source", "nci", "--keyword", "melanoma"]),
    ] {
        assert_eq!(execute(&command, Some(name)), 1, "{name:?}");
    }
}
