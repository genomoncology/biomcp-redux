# 0904 — replies-offer-runnable-related-queries

Status: OPEN.

Milestone: 7

## Outcome

An agent learns what it can ask next from the reply in hand, not from help. Every card and first page carries its related queries as runnable commands. The options that apply to that reply come as fill-in templates, such as `get drug BLA125514 approvals --decision-from <YYYY-MM-DD>`. A refusal's next step is the corrected command, not a help page. CLI and MCP read the same JSON, so both get it.

## Why

Ian, 2026-11-13, on reg-affairs round 2: "do we need to look at adding HATEOS-like 'related queries' on the JSON that informs both the CLI and MCP users."

Evidence from that run (biomed-agent-bench `personas/reg-affairs/runs/2026-11-13-r2/`, `calls.md` in `0041-persona-reg-affairs-r2` on the M5):
- 10 of its 11 help reads came before its first real query (shell calls 2-11). Four of them were spent learning one section's options (`help get drug approvals` and variants).
- It sent `--bogus x` three times (calls 22, 29, 33) to get the list of valid options from the refusal. It was using errors as a menu.
- Replies already carry `next` steps (section, page, pivot, disambiguate, configure). But 27 of the run's next steps were `configure` pointing to `help get drug` or `help search drug`, not to a command the agent could run.
- The pharmacovigilance persona's round 2 (bench `personas/pv-scientist/runs/2026-11-13-r2/`, log.md:48-52) found no DDInter row for any pair in its triplet. The reply said no row does not mean safe, but offered no next step. The label's own drug-interactions section, which BioMCP already reads (`get drug X label --label-section drug_interactions`), is the related query the persona asked for.

## Proof

- Proof: recorded cases for a drug card, a drug approvals page, a diagnostic search page and an option refusal show:
  - the card lists each section with a runnable command and that section's options as templates;
  - the approvals page offers its date window, document and page forms as templates;
  - the refusal's first next step is the corrected command;
  - no `next` entry on these replies points to help when a runnable form exists.
- Each reply stays within its measured size limit (re-measured, with the growth stated).
- Measure after landing: help reads plus probing refusals per run, in the next persona round 2s (rare-disease, pv-scientist), against reg-affairs round 2's 11 help reads and 3 probes.

## Starts from

- Starts from:
  - the `next` step builder (`crates/biomcp-query/src/next.rs`, including its five-step cap at line 207, which 0881 ran into);
  - the per-command option lists that help and refusals already use;
  - the MCP structured content.

## Keeps

- Keeps:
  - existing `next` kinds;
  - the connect-time text caps (0836);
  - lean replies (0849): put templates on cards, first pages and refusals, not on every row.

## Defers

- Defers: suggesting other entities' queries (cross-entity pivots beyond the ones that exist). A suggestion that is accurate but narrow can steer an agent wrong (paper W6, rare-disease SCN1A), so add those only with evidence.

## Changes

- Changes:
  - `crates/biomcp-query/src/next.rs` and the reply builders that call it;
  - `crates/biomcp-types/`, adding an optional template form to a next step;
  - `tests/cases/`;
  - `docs/`;
  - the skill's paragraph on reading `next`.

## Progress

### Design, 2026-11-13

Build on the existing `next` steps. No new step kind and no kind removed.

- **Templates are an optional `options` map on a step.** `args` stays a runnable command. `options` names the options that apply to that command, each with a fill-in placeholder; the agent adds one or more of them to `args`. A step without `options` is unchanged, so the reply schemas only gain an optional field.
  - CLI JSON: `{"kind":"section","args":["get","drug","BLA125514","approvals","--source","drugsfda"],"options":{"--decision-from":"<YYYY-MM-DD>","--decision-to":"<YYYY-MM-DD>","--document-id":"<TEXT>","--product-number":"<TEXT>","--section-limit":"<1-200>","--section-offset":"<N>"}}`
  - MCP typed: the same step already becomes `{"kind":"section","name":"get_drug","arguments":{...}}`; it gains `"options":{"decision_from":"<YYYY-MM-DD>",...}` keyed by the tool's own argument names, and only names that tool advertises (so hosted exclusions hold).
  - Placeholders come from the query schema: a date is `<YYYY-MM-DD>`, a bounded number `<MIN-MAX>`, an open number `<N>`, a short closed list `<a|b|c>`, other text `<TEXT>`. Flags without a value (three in the catalog) are not offered.
- **Which options apply is decided by the checked constructors**, the same way refusal corrections are checked (`grammar/recovery.rs`): for each value option the command takes and the call has not set, try the call with a sample value; offer the option only if the request still checks. So `--partner` is not offered on Drugs@FDA and `--section-limit` is not offered on an identity-only card. No per-source lists.
- **Where templates go** (lean replies, 0849): on `section` steps, so a card lists each section with its runnable command and that section's options; on one `configure` step for the reply's own call, added last on a first page (no offset, or offset 0) when its options are not empty; and on the corrected command of a refusal. Never on rows, continuation pages or other kinds.
- **Next kinds that change**:
  - `configure` steps that point at help are dropped from a refusal when a corrected runnable command exists (`--fields`, source-filter and checked-value refusals already build one; help stays only when nothing runnable can be formed).
  - An unknown option (`--bogus x`, `--prequalified-from` on WHO IVD) now gets a corrected command: the same call without that option (and the word after it, when that word is not an option), checked by the parser; it carries the command's options as templates. Help stays only if that call does not check.
  - First pages gain the `configure` self-template step described above, after the existing steps, outside the five-step follow-up cap, which keeps applying to follow-ups.
- **DDInter pair with no row**: when `--partner` names a DDInter drug and the pair has no row, add a `section` step for each drug's label drug-interactions section, `get drug <name> label --source openfda-label --label-section drug_interactions`, for the drug and its partner. Same entity, another source BioMCP already reads, so no new cross-entity pivot.
- Size: one measuring job for all growth (0905), caps re-set at measured plus 15% per the measured-caps decision, growth stated here.
