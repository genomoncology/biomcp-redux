//! Complete discovery keeps both typed and literal query reply forms.
use biomcp_types::{CellLineFilteredPage, CellLinePage, ResponseBody};

#[test]
fn filtered_cell_line_search_discovery_keeps_both_reply_forms() {
    let root = crate::grammar::root().expect("command grammar");
    let help = crate::help::reference(&root).expect("complete help");
    let command = help
        .commands
        .iter()
        .find(|command| command.command == ["search", "cell-line"])
        .expect("cell-line search command");
    for id in [CellLinePage::SCHEMA, CellLineFilteredPage::SCHEMA] {
        assert!(
            command.reply_schemas.iter().any(|reply| reply == id),
            "complete help omits {id} for cell-line search"
        );
    }
}
