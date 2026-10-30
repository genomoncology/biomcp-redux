//! Feature article sources: native recordings retain records after one bad metadata value.
use super::{native_fields::check as native, response_cases::checked_case};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;
mod cache;
mod identity;

#[expect(
    clippy::disallowed_methods,
    reason = "Select an explicitly staged native capture and previous installed artifact"
)]
pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = [
        (
            "article-preprints/adapter-values-europepmc",
            preprint as fn() -> Check,
        ),
        ("article-named-search/adapter-values-litsense", litsense),
        ("article-sections/adapter-values-pubtator", pubtator),
        ("article-fulltext/adapter-values-pmc-oa", pmc),
        ("article-fulltext/adapter-values-jats", jats),
        (
            "article-fulltext/adapter-values-jats-figure-label",
            figure_label,
        ),
        (
            "article-fulltext/adapter-values-jats-section-title",
            section_title,
        ),
        (
            "article-evidence/adapter-values-semantic-context-native-strings",
            context_strings,
        ),
        ("article-citations/adapter-values-icite", icite),
        (
            "article-sections/adapter-values-independent-identities",
            identity::check,
        ),
    ]
    .into_iter()
    .map(|(name, run)| transcript_trial(name, move || run().map_err(|e| e.to_string().into())))
    .collect();
    if let Some(capture) = std::env::var_os("BIOMCP_ORCID_NATIVE_CAPTURE") {
        let identity_capture = capture.to_string_lossy().into_owned();
        trials.push(transcript_trial(
            "author-orcid/adapter-values-independent-identity",
            move || {
                identity::append_bad(
                    &identity_capture,
                    &["author", "papers", "orcid:0000-0002-7433-2740"],
                    "orcid",
                    "/exchanges/0/body/value/group/0/work-summary",
                    "/put-code",
                    "put_code",
                )
                .map_err(|e| e.to_string().into())
            },
        ));
        let capture = capture.to_string_lossy().into_owned();
        trials.push(transcript_trial(
            "author-orcid/adapter-values-native-source",
            move || {
                native(
                    &capture,
                    &["author", "papers", "orcid:0000-0002-7433-2740"],
                    "orcid",
                    "/exchanges/0/body/value/group/0/work-summary/0/source",
                    "source",
                    json!({"/result/groups/0/assertions/0/put_code":9370420}),
                    "/result/groups/0/assertions/0/source",
                )
                .map_err(|e| e.to_string().into())
            },
        ));
    }
    if let Some(previous) = std::env::var_os("BIOMCP_PREVIOUS_EXECUTABLE") {
        trials.push(transcript_trial(
            "author-papers/adapter-values-previous-snapshots",
            move || {
                cache::check(&std::path::PathBuf::from(&previous)).map_err(|e| e.to_string().into())
            },
        ));
    }
    trials
}

fn preprint() -> Check {
    native(
        "article/preprint/live",
        &["get", "article", "PPR1064174"],
        "europepmc",
        "/exchanges/0/body/value/result/firstPublicationDate",
        "date",
        json!({"/result/items/0/answer/card/europepmc_id":"PPR1064174"}),
        "/result/items/0/answer/card/date",
    )
}
fn litsense() -> Check {
    native(
        "article/litsense/licensed-subset",
        &[
            "search",
            "article",
            "xqzvw zzyzxq kinase",
            "--source",
            "litsense",
        ],
        "litsense",
        "/exchanges/0/body/value/0/section",
        "section",
        json!({"/result/items/0/pmid":"34718584"}),
        "/result/items/0/sentence/section",
    )
}
fn pubtator() -> Check {
    native(
        "article/annotations/live",
        &["get", "article", "33123537", "annotations"],
        "pubtator",
        "/exchanges/0/body/value/PubTator3/0/passages/0/annotations/0/text",
        "text",
        json!({"/result/items/0/answer/card/pmid":"33123537", "/result/items/0/sections/annotations/data/items/*/name": {"includes":"AKT1"}}),
        "/result/items/0/sections/annotations/data/items/0/text",
    )
}
fn pmc() -> Check {
    native(
        "article/fulltext/live-introduction",
        &["get", "article", "28944238", "fulltext"],
        "pmc-oa",
        "/exchanges/0/body/value/is_manuscript",
        "manuscript",
        json!({"/result/items/0/sections/fulltext/data/parts/0/title":"Introduction"}),
        "/result/items/0/sections/fulltext/data/manuscript",
    )
}
fn jats() -> Check {
    let capture = "article/fulltext/live-all";
    let args = ["get", "article", "33123537", "fulltext"];
    let expect = json!({"/result/items/0/sections/fulltext/data/parts/0/title":"Introduction"});
    checked_case(capture, &args, |_, _| {}, expect.clone(), 0)?;
    let mut edits = 0;
    let value = checked_case(
        capture,
        &args,
        |source, recording| {
            if source != "pmc-oa" {
                return;
            }
            if let Some(body) = recording.pointer_mut("/exchanges/0/body/value")
                && let Some(xml) = body.as_str()
                && let Some(references) = xml.find("<ref-list")
                && let Some(offset) = xml[references..].find("<pub-id pub-id-type=\"doi\">")
            {
                let from = references + offset + "<pub-id pub-id-type=\"doi\">".len();
                if let Some(end) = xml[from..].find("</pub-id>") {
                    let mut changed = xml.to_owned();
                    changed.replace_range(from..from + end, "bad-doi");
                    *body = changed.into();
                    edits += 1;
                }
            }
        },
        expect,
        0,
    )?;
    if edits != 1
        || value
            .get("problems")
            .and_then(Value::as_array)
            .is_none_or(|ps| ps.iter().filter(|p| p["field"] == "references.doi").count() != 1)
    {
        return Err(format!("JATS reference edit lost document or problem: {value}").into());
    }
    Ok(())
}
fn icite() -> Check {
    native(
        "article/annotations/live",
        &["get", "article", "33123537", "citations"],
        "icite",
        "/exchanges/0/body/value/data/0/rcr",
        "rcr",
        json!({"/result/items/0/answer/card/pmid":"33123537"}),
        "/result/items/0/sections/citations/data/rcr",
    )
}

fn context_strings() -> Check {
    let value = checked_case(
        "article/edges/live",
        &[
            "article",
            "references",
            "33123537",
            "--source",
            "semantic-scholar",
        ],
        |source, recording| {
            if source == "semantic-scholar"
                && let Some(contexts) =
                    recording.pointer_mut("/exchanges/0/body/value/data/0/contexts")
            {
                *contexts = json!(["Native context", "x".repeat(biodata::Text::MAX_CHARS + 1)]);
            }
        },
        json!({"/result/items/0/contexts/items/0":"Native context", "/result/items/0/contexts/total":2}),
        0,
    )?;
    if value
        .get("problems")
        .and_then(Value::as_array)
        .is_some_and(|ps| ps.iter().any(|p| p["field"] == "contexts"))
    {
        return Err("Unselected native context acquired a text admission problem".into());
    }
    Ok(())
}

fn figure_label() -> Check {
    checked_case(
        "article/fulltext/live-all",
        &["get", "article", "33123537", "fulltext"],
        |_, _| {},
        json!({"/result/items/0/sections/fulltext/data/figures/0/label":"Figure 1"}),
        0,
    )?;
    let mut edits = 0;
    let value = checked_case(
        "article/fulltext/live-all",
        &["get", "article", "33123537", "fulltext"],
        |source, recording| {
            if source != "pmc-oa" {
                return;
            }
            if let Some(body) = recording.pointer_mut("/exchanges/0/body/value")
                && let Some(xml) = body.as_str()
                && let Some(figure) = xml.find("<fig ")
                && let Some(label) = xml[figure..].find("<label>")
            {
                let start = figure + label + "<label>".len();
                if let Some(end) = xml[start..].find("</label>") {
                    let mut changed = xml.to_owned();
                    changed.replace_range(start..start + end, " ");
                    *body = changed.into();
                    edits += 1;
                }
            }
        },
        json!({"/result/items/0/sections/fulltext/data/parts/0/title":"Introduction"}),
        0,
    )?;
    if edits != 1
        || value
            .pointer("/result/items/0/sections/fulltext/data/figures/0/caption")
            .is_none()
        || value
            .pointer("/result/items/0/sections/fulltext/data/figures/0/label")
            .is_some()
        || value
            .get("problems")
            .and_then(Value::as_array)
            .is_none_or(|ps| ps.iter().filter(|p| p["field"] == "figures.label").count() != 1)
    {
        return Err(format!("Figure label edit lost caption, document or problem: {value}").into());
    }
    Ok(())
}

fn section_title() -> Check {
    let mut edits = 0;
    let value = checked_case(
        "article/fulltext/live-introduction",
        &["get", "article", "28944238", "fulltext"],
        |source, recording| {
            if source != "pmc-oa" {
                return;
            }
            if let Some(body) = recording.pointer_mut("/exchanges/0/body/value")
                && let Some(xml) = body.as_str()
                && let Some(start) = xml.find("<title>Introduction</title>")
            {
                let mut changed = xml.to_owned();
                changed.replace_range(
                    start..start + "<title>Introduction</title>".len(),
                    "<title> </title>",
                );
                *body = changed.into();
                edits += 1;
            }
        },
        json!({"/result/items/0/sections/fulltext/data/parts/0/number":1,"/result/items/0/sections/fulltext/data/parts/0/kind":"section"}),
        0,
    )?;
    if edits != 1
        || value
            .pointer("/result/items/0/sections/fulltext/data/parts/0/title")
            .is_some()
        || value
            .get("problems")
            .and_then(Value::as_array)
            .is_none_or(|ps| ps.iter().filter(|p| p["field"] == "parts.title").count() != 1)
    {
        return Err(format!("Section title edit lost part or problem: {value}").into());
    }
    Ok(())
}
