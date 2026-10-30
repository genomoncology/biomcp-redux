# 0456 — Cut the cases to one best call per required item

Status: OPEN.

Milestone: 5

Depends on: 0483

## Outcome

`tests/cases/` holds about 400 command-line calls, one best call for each item on 0483's list, and 0483's pending list is empty. Duplicate cases are gone, the gaps have calls, and the routine suite runs in about a minute without coverage instrumentation.

- Why: Ian's ruling in `decisions/2026-10-29-one-small-outside-in-suite.md`. Of 680 case files with 1,756 steps, about 251 steps are needed. About 110 new calls fill the gaps: 49 refusals, about 47 option and section calls, 10 commands, 2 MCP tools and 1 global option. About 40 more add per-source failure checks for trials, variants, drug labels and articles.
- Starts from: 0483's per-item counts, `tests/cases/`, `tests/replies/`, and the runner trials that already launch the binary for a gap item, which can be ported to YAML.
- Rules: for each item, keep the call that checks answer values, uses real recorded replies, is live-checkable and tells a clinical story, in that order. Merge cases for one command into one file with several steps where that reads better. Synthetic replies are only for failures that cannot be triggered on purpose. MCP runs once per MCP tool and where MCP output differs, not as a repeat of every step. Delete unused recordings with their cases. Expected values are literals, never copied from product tables. Coverage drops as the duplicate cases go. After the cut, the PM sets the coverage floor from the new figure. A step whose successful output is plain text (`version`, `--version`, skill render) may assert exact stdout text or compare with an existing file, through the current runner, with no new runner or assertion language (m16104). The existing housekeeping runner trial that runs `version --verbose` and asserts its commit, target and path fields counts toward required coverage through the existing collector, registered once, because its output varies by build (m16107).
- Proof: 0483's check passes with an empty pending list. The case count and runtime are reported from a plain run. For a sample of ten deleted cases, a planted break of the deleted case's behavior fails a kept case. The reply diff is empty.
- Changes: `tests/cases/`, `tests/replies/`, 0483's pending list and `coverage.floor`.
- Defers: runner trials (0484), the nightly selection (0485).
