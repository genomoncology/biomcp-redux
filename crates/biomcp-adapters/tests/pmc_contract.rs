//! Confined native request identities with synthetic response-only boundary variants.
use biodata::{License, Reason};
use biomcp_adapters::pmc_oa;
use biomcp_fetch::{Fetch, FetchConfig, Mode, RequestError};
use serde_json::Value;
use std::{fs, path::Path, time::Duration};
type Error = Box<dyn std::error::Error>;
#[expect(
    clippy::disallowed_methods,
    reason = "Public adapter proof edits response exchanges in an isolated replay directory"
)]
fn fixture(change: impl FnOnce(&mut Value), xml: &str) -> Result<tempfile::TempDir, Error> {
    let directory = tempfile::tempdir()?;
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/replies/article/fulltext/live-introduction/pmc-oa");
    let target = directory.path().join("pmc-oa");
    fs::create_dir(&target)?;
    let mut change = Some(change);
    for file in fs::read_dir(source)? {
        let file = file?;
        let mut record: Value = serde_json::from_slice(&fs::read(file.path())?)?;
        let url = record
            .get("url")
            .and_then(Value::as_str)
            .ok_or("missing native URL")?
            .to_owned();
        let body = record
            .pointer_mut("/exchanges/0/body/value")
            .ok_or("missing exchange")?;
        if url.ends_with(".json") {
            change.take().ok_or("duplicate metadata recording")?(body);
        } else if url.contains(".xml?") {
            *body = Value::String(xml.to_owned());
        }
        fs::write(target.join(file.file_name()), serde_json::to_vec(&record)?)?;
    }
    Ok(directory)
}
fn read(
    change: impl FnOnce(&mut Value),
    xml: &str,
) -> Result<Result<Option<pmc_oa::Document>, RequestError>, Error> {
    let directory = fixture(change, xml)?;
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: directory.path().into(),
        },
        ..FetchConfig::default()
    })?;
    let leg = fetch.leg(&pmc_oa::SPEC, fetch.deadline(Duration::from_secs(20)));
    Ok(pmc_oa::read(
        &leg,
        "PMC5606870".parse()?,
        Some("28944238".parse()?),
    ))
}
fn article(front: &str, body: &str) -> String {
    format!(
        "<article><front><article-meta>{front}</article-meta></front><body>{body}</body></article>"
    )
}
#[test]
fn metadata_license_owns_the_closed_vocabulary() -> Result<(), Error> {
    let xml = article("", "<p>Invented source text.</p>");
    for (source, expected) in [
        ("CC BY", License::CcBy),
        ("cc-by-nc", License::CcByNc),
        ("CC BY SA", License::CcBySa),
        ("CC BY NC SA", License::CcByNcSa),
        ("CC BY ND", License::CcByNd),
        ("CC BY NC ND", License::CcByNcNd),
        ("CC0", License::Cc0),
        ("PUBLIC DOMAIN", License::PublicDomain),
        ("TDM", License::Tdm),
        ("source-specific", License::Other),
    ] {
        let document = read(
            |record| {
                record["license_code"] = source.into();
            },
            &xml,
        )??
        .ok_or("missing document")?;
        assert_eq!(document.license, expected);
    }
    Ok(())
}
#[test]
fn returned_identities_and_empty_metadata_fail_closed() -> Result<(), Error> {
    let xml = article("", "<p>Invented source.</p>");
    for (field, value) in [
        ("pmcid", Value::from("PMC1")),
        ("pmid", Value::from(1)),
        ("pmid", Value::from(0)),
        ("license_code", Value::from(" ")),
    ] {
        bad(read(
            |record| {
                record[field] = value;
            },
            &xml,
        )?)?;
    }
    for ids in [
        "<article-id pub-id-type=\"pmc\">1</article-id>",
        "<article-id pub-id-type=\"pmc\">5606870</article-id><article-id pub-id-type=\"pmcid\">PMC1</article-id>",
        "<article-id pub-id-type=\"pmc\">not-an-id</article-id>",
    ] {
        bad(read(|_| {}, &article(ids, "<p>Invented source.</p>"))?)?;
    }
    Ok(())
}
fn bad(result: Result<Option<pmc_oa::Document>, RequestError>) -> Result<(), Error> {
    match result {
        Err(RequestError::Source(error)) if error.reason == Reason::BadReply => Ok(()),
        _ => Err("invalid source response did not produce bad_reply".into()),
    }
}
#[test]
fn table_spans_are_bounded_before_expansion() -> Result<(), Error> {
    for span in ["0", "257", "99999999999999999999999999999999", "no"] {
        let body = format!(
            "<table-wrap id=\"T1\"><table><tr><td colspan=\"{span}\">invented</td></tr></table></table-wrap>"
        );
        bad(read(|_| {}, &article("", &body))?)?;
    }
    bad(read(
        |_| {},
        &article("", "<table-wrap id=\"T1\"/><table-wrap id=\"T1\"/>"),
    )?)?;
    Ok(())
}
#[test]
fn source_markup_and_first_citations_survive_without_duplicate_tables() -> Result<(), Error> {
    let body = "<sec><title>Results</title><sec><title>First</title><p>A &amp; B &#945; <![CDATA[<sample>]]> <xref ref-type=\"table\" rid=\"T1\">1</xref>.</p></sec><sec><title>Second</title><p>Again <xref ref-type=\"table-wrap\" rid=\"T1\">1</xref>.</p></sec><table-wrap id=\"T1\"><label>Table 1</label><caption><p>Invented caption.</p></caption><table><tr><th colspan=\"2\">Header</th></tr><tr><td>Left</td><td/></tr></table></table-wrap></sec>";
    let document = read(
        |_| {},
        &article("<article-id pub-id-type=\"pmc\">5606870</article-id>", body),
    )??
    .ok_or("missing document")?;
    let tables: Vec<_> = document
        .body
        .parts
        .iter()
        .filter(|part| part.meta.kind == biodata::TextPartKind::Table)
        .collect();
    assert_eq!(tables.len(), 1);
    let table = tables.first().ok_or("missing table")?;
    assert_eq!(
        table
            .heading_path
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
        ["Results", "First"]
    );
    assert!(
        table
            .rows
            .iter()
            .any(|row| row.text == "| Header | Header |")
    );
    assert!(table.rows.iter().any(|row| row.text == "| Left |  |"));
    assert!(
        document
            .body
            .rows
            .iter()
            .any(|row| row.plain.contains("A & B α <sample>"))
    );
    assert!(
        document
            .body
            .rows
            .iter()
            .any(|row| row.markdown.contains("&lt;sample&gt;"))
    );
    Ok(())
}

#[test]
fn inline_markup_preserves_non_citation_superscripts() -> Result<(), Error> {
    for (xml, expected) in [
        (
            "<p>A<sup>2<xref ref-type=\"bibr\" rid=\"B1\">11</xref></sup>.</p>",
            "A<sup>2[11]</sup>.",
        ),
        (
            "<p>A<sup>&#945;<xref ref-type=\"bibr\" rid=\"B1\">11</xref></sup>.</p>",
            "A<sup>α[11]</sup>.",
        ),
        (
            "<p>A<sup><xref ref-type=\"bibr\" rid=\"B1\">11</xref>,<xref ref-type=\"bibr\" rid=\"B2\">12</xref></sup>.</p>",
            "A[11,12].",
        ),
    ] {
        assert_eq!(biomcp_adapters::jats::markdown(xml).map_err(|_| "invalid source markup")?.as_str(), expected);
    }
    assert!(biomcp_adapters::jats::markdown("<p>&undefined;</p>").is_err());
    Ok(())
}
