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
- Bot's run reader over four runs (2026-11-14, bench `personas/_eval/help-reads-2026-11-14.md`): when a refusal's next step was a runnable command, the agent ran it with no help read (reg-affairs r2 `--sort newest`; rare-disease r2 `variant normalize --source`, then `--build`). When the next step pointed at help, the agent read help. But one runnable next step was itself refused: rare-disease r2 session lines 66-74, `get disease MONDO:0013267 genes --section-limit 5`, whose refusal pointed only at `help get disease`. So the Proof must also show each offered command runs. Mid-run help reads are about 15-20% of help cost; the front-loaded reads are 0922 and 0923.

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

### Built, 2026-11-15

- `Step` has an optional `options` map (`crates/biomcp-types/src/response/step.rs`); reply schemas gain only that optional field.
- `crates/biomcp/src/grammar/options.rs` finds a request's options by probing the checked constructors with a sample value per unset option (a date, the minimum number, each enum value, a few text spellings). It skips `--source`, positional and section words, aliases, flags without a value, names that only continue a reply (`civic_cursor`, `bundle_revision`, `pool`, `snapshot`, `name_snapshot`, `next_page`, `page`, `state`) and `--save`, which writes a file.
- `command/steps.rs`: each `section` step carries its options. A paged first reply (a `page` block, no offset, no continuation) that is not an identity card gains a last `configure` step, its own call with its options, outside the follow-up cap, but only when it can add more than paging (`--limit`, `--offset`, `--section-limit`, `--section-offset`), which the `page` step already offers. An identity card's options live on its section steps. A first run that offered the step on every reply touched 58 cases and trials, mostly replies whose only options were paging or continuation names, so it was narrowed to this.
- Markdown shows a step's options under its command as "Options to add".
- Refusals: a correction now replaces the help step (`grammar/recovery.rs::next`, the `--fields` refusal). An unknown option with no other fix gets the call without it (and without the word after it, taken as its value; `grammar/errors.rs::dropped`). A refusal raised before any source is read with one argument and named valid values (such as `get disease MONDO:0013267 genes --section-limit 5`, which needs `--source monarch`) gets the first valid value that checks as its corrected call (`grammar/recovery.rs::prepared`). Every corrected call carries its options.
- MCP: typed next calls carry `options` under the action's own argument names, only names the tool advertises (`mcp/typed/followups.rs`), and the output schema allows it.
- DDInter: a named pair with no row offers each drug's label `drug_interactions` section as `section` steps.
- Skill: the `next` line says to add a step's `options` with placeholders filled (same byte count).

### Measured, 2026-11-15

- Cases: `tests/cases/drug/runnable-related.yaml` (Drugs@FDA card, approvals page, `--bogus` refusal, each over CLI and typed MCP, following the offered section and corrected calls and running filled-in options) and `tests/cases/diagnostic/runnable-related.yaml` (fda-pma search page, CLI and typed MCP, a filled-in date window runs). `drug/ddinter-pair` asserts the two label steps on an unlisted pair.
- Sizes: j41316 measured every case step on main and on the branch with ceilings logged instead of enforced; j41331 re-measured the final rules. 189 of 603 measured case steps grew: median +226 bytes, most trial searches +570 to +880 (a trial search takes about twenty filters), refusals that lost their help step shrank by 5. 61 case ceilings and the Markdown pathway ceiling were raised to measured plus 15% (sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md), as were the measured runner budgets for research article and trial search (1732 to 2192, 8192 to 9913), VAERS summaries (four phases, about +340), study file and navigation pages (1416/1415) and phenotype retained sections (871). No global cap moved.
- Changed expectations: refusals whose correction now replaces help (`--fields`, source filters, missing `--source`, unknown option, HGVS name, phenotype admission, typed reply forms); first pages that now end with their own call (paging-end, wider-zero, four-term-short).


### Gate, 2026-11-15

- `make --keep-going landing-gate docs-check schema-release-check` passed in j41341 on 1efcd64f7, which was already up to date with origin/main.

### Review fixes, 2026-11-15

An independent review rejected 3d44a1569. Changed:

- A refused value is no longer replaced by a guess. A correction replaces the help step only when it is certain: the call itself, a declared default, or the only listed value that passes when the call already names something (an ID, a word, a section or a filter; counts and switches don't count). When it picks one of several passing values, or fills in a bare command, it goes first and help stays beside it, as on main (`grammar/recovery.rs::corrected` and `next`, `prepared`; `grammar/options.rs::names_something`). Bare `search drug` gets `--source fda-drug-recall` and help again, as main did; `get disease MONDO:0013267 genes --section-limit 5` gets only `--source monarch` (both in `tests/cases/grammar/usage-missing-option.yaml`). The bare `search drug` guess comes from main's missing-value correction, not from this ticket; 0904 only stopped it from replacing help.
- Dropping an unknown option no longer drops the caller's search word. When the word after the unknown option also reads as part of the call, `grammar/errors.rs::dropped` offers nothing and help stays (`search drug --bogus imatinib --source fda-drug-recall` in `usage-unknown-option.yaml`).
- Search pages no longer end with their own call and every filter. Only a section's own first page (a get that names a section, such as `get drug BLA761445 approvals`) offers its own call with options (`command/steps.rs::adjust`). Section steps on cards and refusals' corrected calls keep their options. `diagnostic/runnable-related.yaml` now asserts that the search page stays lean.
- Merged origin/main with 0970. A typed refusal's fix now replaces the help step, carries the options its call takes, and is never only the call without the unknown key: a refusal whose only correction is dropping an unknown option marks itself (`ParseInvalid::dropped`) and `grammar::candidate` offers nothing for it, so such keys keep only help as 0970 says. `mcp/typed-unknown-key-fixes.yaml` expects the HGVS-name fix alone and the options on the `diagnostic` fix.
- Not done: running a DDInter label step from a case (optional in the review). The label steps' drug names are synthetic, so running one would need a recorded openFDA reply for a made-up name.

### Re-measured, 2026-11-15

- j41353 measured every case step on the branch and on origin/main with ceilings logged instead of enforced; j41361 re-measured the branch with line-atomic logging after the first log interleaved some lines.
- Search first pages are back to main's sizes: gene search 462 and 650 bytes (main 462, 650; the rejected build was +39%), diagnostic recall 864/838/779/866 (unchanged), trial search 6106-6387 (unchanged; was +570-880).
- What still grows is cards and section pages: 85 of 613 measured steps grew, median +160 bytes. Drug name cards +280-300 (each section step's options), approvals pages +240-270, trial cards +160, protein and pathway cards +170-230, VAERS summaries +96-138, the DDInter unlisted pair +414 (two label steps). Two refusals that lost their help step shrank by 5.
- Ceilings: every case ceiling the rejected build raised is back to main's when the reply fits it, and otherwise measured plus 15% (sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md); about 50 went back to main's value. Still raised: cell-line examples (2823, 2580), pathway examples (887, 871) and Markdown (893), pgx examples (1000), protein gene-crossrefs and displayed features (1298, 1351), study examples (1470, 1471), trial documents (3131), trial cards NCT04303780 (2351, 3028) and NCT02987543 sites (3181). Runner budgets: research article and trial search back to 1732 and 8192; VAERS seriousness and age back to main; VAERS default summary 2346/2233 (was raised to 2501/2388) and summary with reports 2177/2064; study files and navigation pages 1416/1415; phenotype retained sections 871. The new runnable-related cases' ceilings are measured plus 15% (diagnostic 3185, drug card 1026, approvals 3326).
- After the review fixes, `make --keep-going landing-gate docs-check schema-release-check` passed in j41386 on 05e071b68, merged with origin/main (including 0970, 0906, 0972 and 0920).

### Second review fixes, 2026-11-15

- A filter or section the selected source refuses is no longer dropped as a certain fix. If exactly one of the accepting sources passes and the call names something, the call keeps the filter with that source and replaces help (`grammar/recovery.rs::source_swap`): `get phenotype HP:0001250 hierarchy --source hpo` offers `get phenotype HP:0001250 hierarchy` (Monarch, the default), `search adverse-event pump --source maude --reaction Headache` offers the FAERS call with `--reaction Headache`, and the typed trial refusal keeps both sections with `nci`. Otherwise dropping the filter is a guess with help beside it (`search diagnostic --source gtr --gene BRAF --marker malaria`), and a call with nothing left to run gets help alone (`search pathway --source reactome,wikipathways --top-level`).
- The call offered without an unknown option must also pass the check made before any source is read; if it does not, it takes the one correction that check names, or help stays. `get disease MONDO:0013267 genes --section-limit 5 --bogus x` offers the call with `--source monarch` (`usage-unknown-option.yaml`).
- Completed calls from positional and section fixes carry the same certain-or-guess flag, so a guess keeps help (`positional::completed_steps`).
- Typed case: `{"id":"MSTN","bogus":5}` on gene get keeps only help (`mcp/typed-unknown-key-fixes.yaml`).
- Checked in j41389 (format, regeneration, `check-cheap`, probes) and j41390 (grammar, phenotype, mcp and mcp-parity cases).

### Third review fix, 2026-11-15

- Swapping in the one accepting source is certain only when the caller named no source, so the refused source was the default. When the caller named a source (`--source`, or `source` in a typed call), the swapped call is a guess: it comes first and help stays beside it (`grammar/recovery.rs::source_swap`). `search adverse-event pump --source maude --reaction Headache`, `search drug artemether --source who-pq --initiated-from 2026-01-01`, `search pathway --source wikipathways --gene BRAF`, the three `get phenotype HP:0001250 ... --source hpo` refusals and the typed trial refusal (ctgov to nci) now offer the swapped call plus help. The swapped call must also pass the check made before any source is read.
- Measured in j41399: the phenotype refusals with help beside the swapped call are 398 bytes (main 331, with help only); their ceiling is 461, measured plus 15% for the longest section word.

