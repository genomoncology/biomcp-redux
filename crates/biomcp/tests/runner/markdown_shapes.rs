//! People can read every command shape and copy its next commands.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("markdown-view/command-shapes", || {
        shapes().map_err(Into::into)
    })]
}
struct Shape {
    args: &'static [&'static str],
    replies: &'static str,
    required: &'static [&'static str],
    max_bytes: usize,
}
// Measured Markdown plus 15%; the protein ceiling also respects the matching
// 0.9 JSON baseline. Reader, supplement-row and usage shapes have no comparable old output.
const SHAPES: &[Shape] = &[
    Shape {
        args: &["get", "gene", "BRAF"],
        replies: "gene/main",
        required: &[
            "# BRAF",
            "**status:**",
            "| Build | Chromosome | End | Start | Strand |",
            "biomcp get gene HGNC:1097",
        ],
        max_bytes: 2382,
    },
    Shape {
        args: &["get", "gene", "CDKN2A", "protein"],
        replies: "gene/protein",
        required: &[
            "# CDKN2A",
            "Protein",
            "**state:** data",
            "| Accession | Function | Length | Name | Subcellular locations |",
            "P42771",
            "Q8N726",
        ],
        max_bytes: 5325,
    },
    Shape {
        args: &["search", "gene", "KRAS"],
        replies: "gene/main",
        required: &[
            "| Symbol | Name | Hgnc id | Location | Status |",
            "KRAS",
            "biomcp get gene HGNC:6407",
        ],
        max_bytes: 592,
    },
    Shape {
        args: &["search", "article", "AKT1 E17K"],
        replies: "article/search/live",
        required: &[
            "| Pmid | Title | Journal | Date |",
            "biomcp search article 'AKT1 E17K'",
            "biomcp get article 33377972",
        ],
        max_bytes: 3835,
    },
    Shape {
        args: &["get", "article", "33123537"],
        replies: "article/get/synthetic-derived",
        required: &[
            "# Effect of AKT1 (p. E17K) Hotspot Mutation on Malignant Tumorigenesis and Prognosis.",
            "**status:**",
            "biomcp",
        ],
        max_bytes: 2421,
    },
    Shape {
        args: &["article", "text", "PMC5606870", "--part", "1"],
        replies: "article/fulltext/synthetic",
        required: &["## Introduction", "*text*", "**state:** data"],
        max_bytes: 650,
    },
    Shape {
        args: &[
            "article",
            "supplement",
            "28944238",
            "MGG3-5-553-s002.xlsx",
            "--sheet",
            "ST2",
            "--rows",
            "84-120",
        ],
        replies: "article/supplements/live",
        required: &[
            "Rows",
            "| Sheet | Row | Header | Cells |",
            "AXIN2",
            "p.Pro245Ser",
            "biomcp article supplement 28944238 MGG3-5-553-s002.xlsx",
        ],
        max_bytes: 4934,
    },
    Shape {
        args: &["get", "gene"],
        replies: "gene/main",
        required: &["# Error", "**status:** usage", "usage.arguments"],
        max_bytes: 151,
    },
];
fn shapes() -> Result<(), String> {
    for shape in SHAPES {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = super::launch::fixtures()
            .join("replies")
            .join(shape.replies);
        let globals = env.globals(Some(&replay), false);
        let mut command = globals.clone();
        command.extend(shape.args.iter().map(|arg| (*arg).to_owned()));
        let json_output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &command)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value =
            serde_json::from_slice(&json_output.stdout).map_err(|e| e.to_string())?;
        env.config(&json!({"format":"md"}))
            .map_err(|e| e.to_string())?;
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &command)
            .output()
            .map_err(|e| e.to_string())?;
        let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        for required in shape.required {
            if !text.contains(required) {
                return Err(format!("{command:?} lacks {required}: {text}"));
            }
        }
        if text.len() > shape.max_bytes {
            return Err(format!(
                "{:?}: {} Markdown bytes exceed {}",
                shape.args,
                text.len(),
                shape.max_bytes
            ));
        }
        if output.status != json_output.status || !output.stderr.is_empty() {
            return Err("Markdown changed command status or emitted diagnostics".into());
        }
        let mut tool_args: Vec<String> = shape.args.iter().map(|arg| (*arg).to_owned()).collect();
        tool_args.extend(["--format".into(), "md".into()]);
        let tool = super::mcp::call_with_globals(
            &env,
            &tool_args,
            !json_output.status.success(),
            &globals,
        )?;
        if tool != value {
            return Err("MCP changed JSON under Markdown configuration".into());
        }
        command.extend(["--format".into(), "json".into()]);
        let explicit = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &command)
            .output()
            .map_err(|e| e.to_string())?;
        if explicit.stdout != json_output.stdout {
            return Err("explicit JSON changed under Markdown configuration".into());
        }
    }
    Ok(())
}
