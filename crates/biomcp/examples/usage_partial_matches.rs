//! Temporary scaffold for the parser's partial-match ownership boundary.
use clap::{Arg, Command};

fn main() {
    for mode in ["root", "root-and-leaf", "every-parent"] {
        let leaf = Command::new("pgx")
            .arg(Arg::new("gene").long("gene").required(true))
            .arg(Arg::new("source").long("source").required(true));
        let mut root = Command::new("biomcp").subcommand(Command::new("search").subcommand(leaf));
        root.build();
        root = root.ignore_errors(true);
        if mode != "root" {
            root = root.mut_subcommand("search", |parent| {
                let parent = parent.mut_subcommand("pgx", |leaf| leaf.ignore_errors(true));
                if mode == "every-parent" {
                    parent.ignore_errors(true)
                } else {
                    parent
                }
            });
        }
        let matches = root
            .try_get_matches_from(["biomcp", "search", "pgx", "--gene", "CYP2D6"])
            .expect("partial parsing returns a root");
        let gene = matches
            .subcommand_matches("search")
            .and_then(|matches| matches.subcommand_matches("pgx"))
            .and_then(|matches| matches.get_one::<String>("gene"));
        println!("{mode}: retained gene={gene:?}");
        assert_eq!(
            gene.map(String::as_str),
            (mode == "every-parent").then_some("CYP2D6")
        );
    }
}
