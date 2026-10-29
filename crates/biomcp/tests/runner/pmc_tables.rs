use super::response_cases::checked_case;
use serde_json::{Value, json};
type Check = Result<(), Box<dyn std::error::Error>>;

fn run(
    front: &str,
    body: &str,
    change: impl Fn(&mut Value),
    args: &[&str],
    expect: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let xml = format!(
        "<article><front><article-meta>{front}</article-meta></front><body>{body}</body></article>"
    );
    checked_case(
        "article/fulltext/live-introduction",
        args,
        |source, record| {
            if source != "pmc-oa" {
                return;
            }
            let url = record
                .get("url")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let metadata = url.ends_with(".json");
            let text = url.contains(".xml?");
            if let Some(value) = record.pointer_mut("/exchanges/0/body/value") {
                if metadata {
                    change(value);
                } else if text {
                    *value = xml.clone().into();
                }
            }
        },
        expect,
        0,
    )
}

pub(super) fn check() -> Check {
    licenses()?;
    identities()?;
    markup()
}

fn licenses() -> Check {
    for (source, expected) in [
        ("CC BY", "cc_by"),
        ("cc-by-nc", "cc_by_nc"),
        ("CC BY SA", "cc_by_sa"),
        ("CC BY NC SA", "cc_by_nc_sa"),
        ("CC BY ND", "cc_by_nd"),
        ("CC BY NC ND", "cc_by_nc_nd"),
        ("CC0", "cc0"),
        ("PUBLIC DOMAIN", "public_domain"),
        ("TDM", "tdm"),
        ("source-specific", "other"),
    ] {
        run(
            "",
            "<p>Invented source text.</p>",
            |record| record["license_code"] = source.into(),
            &["article", "text", "PMC5606870", "--part", "1"],
            json!({"/result/content/data/license": expected}),
        )?;
    }
    Ok(())
}

fn identities() -> Check {
    for (field, value) in [
        ("pmcid", json!("PMC1")),
        ("pmid", json!(1)),
        ("pmid", json!(0)),
        ("license_code", json!(" ")),
    ] {
        run(
            "",
            "<p>Invented source.</p>",
            |record| record[field] = value.clone(),
            &["get", "article", "28944238", "fulltext"],
            json!({"/result/items/0/sections/fulltext/reason": "bad-reply"}),
        )?;
    }
    for front in [
        "<article-id pub-id-type=\"pmc\">1</article-id>",
        "<article-id pub-id-type=\"pmc\">5606870</article-id><article-id pub-id-type=\"pmcid\">PMC1</article-id>",
        "<article-id pub-id-type=\"pmc\">not-an-id</article-id>",
    ] {
        run(
            front,
            "<p>Invented source.</p>",
            |_| {},
            &["get", "article", "28944238", "fulltext"],
            json!({"/result/items/0/sections/fulltext/reason": "bad-reply"}),
        )?;
    }
    Ok(())
}

fn markup() -> Check {
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
        run(
            "",
            xml,
            |_| {},
            &["article", "text", "PMC5606870", "--part", "1"],
            json!({"/result/content/data/markdown": expected}),
        )?;
    }
    run(
        "",
        "<p>&undefined;</p>",
        |_| {},
        &["get", "article", "28944238", "fulltext"],
        json!({"/result/items/0/sections/fulltext/reason": "bad-reply"}),
    )?;
    let body = "<sec><title>Results</title><p><xref ref-type=\"table\" rid=\"T2 T1\">2, 1</xref></p><table-wrap \
        id=\"T1\"><label>Table 1</label></table-wrap><table-wrap id=\"T2\"><label>Table \
        2</label></table-wrap></sec>";
    run(
        "",
        body,
        |_| {},
        &["get", "article", "28944238", "fulltext"],
        json!({"/result/items/0/sections/fulltext/data/parts/*/title": ["Results", "Table 2", "Table 1"]}),
    )?;
    let body = "<sec><title>Results</title><sec><title>First</title><p>A &amp; B &#945; <![CDATA[<sample>]]> <xref \
        ref-type=\"table\" rid=\"T1\">1</xref>.</p></sec><sec><title>Second</title><p>Again <xref \
        ref-type=\"table-wrap\" rid=\"T1\">1</xref>.</p></sec><table-wrap id=\"T1\"><label>Table \
        1</label><caption><p>Invented caption.</p></caption><table><tr><th \
        colspan=\"2\">Header</th></tr><tr><td>Left</td><td/></tr></table></table-wrap></sec>";
    run(
        "",
        body,
        |_| {},
        &["article", "text", "PMC5606870", "--part", "2"],
        json!({"/result/content/data/markdown": "### First\n\nA & B α &lt;sample&gt; 1."}),
    )?;
    run(
        "",
        body,
        |_| {},
        &["article", "text", "PMC5606870", "--part", "3"],
        json!({"/result/content/data/heading_path": ["Results", "First"],
            "/result/content/data/markdown": "**Table 1.** Invented caption.\n| Header | Header |\n| --- | --- |\n| Left |  |"}),
    )?;
    let answer = run(
        "",
        body,
        |_| {},
        &["get", "article", "28944238", "fulltext"],
        json!({"/status": "complete"}),
    )?;
    let parts = answer
        .pointer("/result/items/0/sections/fulltext/data/parts")
        .and_then(Value::as_array)
        .ok_or("missing outline parts")?;
    if parts
        .iter()
        .filter(|part| part.get("kind").and_then(Value::as_str) == Some("table"))
        .count()
        != 1
    {
        return Err("repeated citations duplicated a table".into());
    }
    Ok(())
}
