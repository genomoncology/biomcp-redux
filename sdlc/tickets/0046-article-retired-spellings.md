# 0046 — Retire the 0.9 article spellings that 1.0 renamed, moved or dropped

Status: OPEN.

Milestone: 1

## Outcome

Every 0.9 article spelling that article.md's keep, change or drop table renames, moves or drops, outside Ian's open question C, returns a `usage.retired` error envelope with exit 2 and a fixed message naming the 1.0 spelling or saying it was dropped, through the CLI and MCP.

Depends on: 0038

- Starts from: article.md section 1 (0.9 commands and options) and section 5 (keep, change or drop each 0.9 feature) and case 52; decision `2026-10-24-gene-slice-rulings.md` (retired spellings name the new command); 0033's `retired.rs` table, its matching rules and `ErrorMessage`; 0038's article section grammar.
- Changes: `crates/biomcp/src/retired.rs`, `crates/biomcp/src/grammar.rs`, `crates/biodata/src/response/error.rs`, `crates/biodata/schemas/error.json`, `tests/cases/article/retired.yaml`.
- PM decision: 2026-10-24. Fixed messages that never echo input, following 0033's matching (command structure, recognized options and lowercase trailing section words after the first input; never argument values). `search article -k`, `-q` and `--query` → `use: search article <text>`. `--date-from` and `--year-min` → `use: --since`. `--date-to` and `--year-max` → `use: --until`. `--weight-citations` → `use: --sort citations`. `--open-access`, `--no-preprints`, `--exclude-retracted`, `--include-retracted`, `--ranking-mode`, `--weight-semantic`, `--weight-lexical`, `--weight-position`, `--max-per-source`, `--session`, `--debug-plan` and `--full` → `dropped in 1.0`. `-g`, `-d` and `-a` → `use: --gene`, `use: --disease` and `use: --author`, because 1.0 consumes no short flag yet (architecture 3.7); article.md marks them kept, so the PM confirms this or files the `x-short` work. `get article` trailing words `assets` → `use: supplements`, `asset` → `use: article supplement <id> <file>`, `indexing` → `dropped in 1.0`; options `--outline` → `use: fulltext`, `--lines` → `use: article text <id> --part <n>`, `--out`, `--output`, `--asset-view`, `--asset-limit` and `--asset-offset` → `dropped in 1.0`. Commands `article supplement-search` → `use: article supplement <id> <file> --find <text>`; `article entities` → `use: get article <id> annotations`; `article batch`, `batch article` and `article authors` → `use: get article <id>...`; `variant articles` → `use: search article --variant <variant>`; `disease articles` → `use: search article --disease <disease>`; `drug articles` → `use: search article --drug <drug>`; `pathway articles` → `moved to the pathway entity`. 0033 already retires `gene articles`.
- Keeps: Every earlier case. Legitimate inputs that equal a retired word keep resolving, as 0033 proves for genes.
- Proof: `tests/cases/article/retired.yaml` runs one step per spelling above, each asserting exit 2, code `usage.retired`, the exact message and that no input text appears, under 400 bytes; case 52 (`variant articles "AKT1 E17K"`) and two others also run through MCP with `isError` true. `get article 33123537 --section annotations` and an input behind `--` that equals a retired word prove the matching boundary. Then make landing-gate runs through pm job after current main.
- Defers: `--pdf`, `tldr`, `article citation-evidence`, `article recommendations` and citation contexts wait for Ian's ruling on question C; until then they fall to the ordinary `usage.arguments` error. The accounting record goes to 0047.
