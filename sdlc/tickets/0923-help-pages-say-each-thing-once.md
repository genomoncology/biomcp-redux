# 0923 — help-pages-say-each-thing-once

Status: OPEN.

Milestone: 7

## Outcome

A command's help page states each sentence once. A line that applies to every option, such as which sources the command reads or that listed choices accept any letter case, appears once at the top, not after every option. `help search adverse-event` loses its repeats with no loss of information.

## Why

The bench's help-read analysis of 2026-11-14 (bench `personas/_eval/help-reads-2026-11-14.md`): command help pages were about 35% of help cost, read whole before each command's first use. In `help search adverse-event` (9k characters, about 2k tokens; pv-scientist round 2), "Command sources: faers, maude, vaers; source restrictions in the field description apply." appears 17 times, "Listed choices accept any letter case…" 6 times and "; help search adverse-event" 6 times: 24% of the page.

## Proof

- Proof: a help case under `tests/cases/` for `help search adverse-event` that fails if a sentence repeats; the help size caps (0905) re-measured downwards.

## Starts from

- Starts from: where help assembles option descriptions (find the source of "Command sources:" and "Listed choices accept any letter case" in the crates); 0905's measured caps.

## Keeps

- Keeps: every option, value list and source restriction the page states today; the CLI and MCP help reading the same text.

## Defers

- Defers: a short form (flag, values, one line each) as the default page, until this lands and the next persona round shows what help costs then.

## Changes

- Changes: the help rendering code, help cases, size caps and generated docs.

## Progress

- 2026-11-14: Help now states command-wide lines once, on the command, in the shared help assembly (`crates/biomcp/src/help/argument_details.rs`), so every command's page benefits. "Command sources: …; source restrictions in the field description apply." after every option became one line, "Options apply to every listed source unless their description restricts them." (or the local-option line for commands without sources). "Listed choices accept any letter case…" after every option with choices became one line on the command. A field's compact pointer to the page being read ("…; help search adverse-event") is dropped, with the short line it duplicated. Per-option facts stay with their option as one sentence that names it: "Value form: string; example: --reaction 'x'; only with --source vaers." A flag pair that sets one field (article `--include-retracted`) points to the first flag instead of repeating its text, and large value lists name their option. Entity and overview pages drop the option-wide lines along with the options; section pages keep the case line.
- Proof: `tests/cases/adverse-event/help.yaml` checks both adverse-event help pages with a new `sentences_once` case assertion (no sentence of two or more words appears twice), with a negative runner case for the operator. A library test (`grammar::reference::every_command_help_page_says_each_generated_sentence_once`) checks every command page; it allows only repeats the query schema's own text already makes (authored pairs such as diagnostic's two date-range descriptions), which remain on gwas, diagnostic, drug and variant pages.
- Measured (offline JSON / Markdown bytes, main 9330e44bc → this branch): search adverse-event 8,961/7,532 → 6,880/5,418; get adverse-event 3,517/2,662 → 3,021/2,157; search trial 12,946/10,774 → 9,632/7,412; search drug 9,110/7,666 → 7,776/6,308; get drug 9,119/7,835 → 6,866/5,555; search article 9,379/7,866 → 6,763/5,223; search diagnostic 7,474/6,053 → 6,749/5,309; get variant 8,656/7,197 → 5,661/4,169. Help caps were re-measured with `scripts/refresh_help_cases.py` (measured + 15%, 0905): 59 caps fell, none rose. Query schemas and release/schema-baseline.json are unchanged; the typed MCP tool schemas don't read this text.
