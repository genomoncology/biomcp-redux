//! Deliberately synthetic source tables, not captured CDC content.
pub(super) fn table(vaccine: &str, group: &str) -> String {
    let total = match group {
        "D8.V10" => "7",
        "D8.V1" => "11",
        _ => "5",
    };
    format!(
        "<page><dataset code=\"D8\" vintage=\"synthetic-vintage\"><variable code=\"D8.V12\" label=\"State / \
            Territory\"><value code=\"00\" label=\"The United \
            States/Territories/Unknown\"/></variable><measure code=\"D8.M1\" label=\"Events \
            Reported\"/><measure code=\"D8.M2\" \
            label=\"Percent\"/></dataset><response><request><variable-selections><variable \
            code=\"D8.V14\"><value code=\"{vaccine}\" label=\"Synthetic vaccine\"/></variable><variable \
            code=\"D8.V12\"><value code=\"00\"/></variable></variable-selections><byvariables><variable \
            code=\"{group}\"/></byvariables></request><measure-selections><measure code=\"D8.M1\" \
            max=\"9\"/><measure code=\"D8.M2\" max=\"180\"/></measure-selections><data-table><r><c \
            l=\"Synthetic A\"/><c v=\"4\"/><c v=\"80.00%\"/></r><r><c l=\"Synthetic B\" \
            code=\"synthetic-code\"/><c v=\"5\"/><c v=\"100.00%\"/></r><r><c l=\"Synthetic suppressed\"/><c \
            v=\"Suppressed\" suppression=\"synthetic\"/><c v=\"Suppressed\"/></r><r><c c=\"1\"/><c \
            dt=\"9\"/><c \
            dt=\"180.00%\"/></r></data-table><footnotes><footnote><symbol>Note:</symbol><text>Synthetic source \
            footnote.</text></footnote></footnotes><caveats><caveat><![CDATA[Synthetic applied \
            <p>notice</p>.]]></caveat></caveats></response><message>These results are for {total} total \
            events.</message><message>Synthetic zero-row notice.</message></page>"
    )
}
