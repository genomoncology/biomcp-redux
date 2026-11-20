# Status words match what happened: proposal for 1059

Base: origin/main 66a5e8c67. Evidence: the full replay suite on that commit with reply transcripts (pm job j43110, 1,068 cases passed) and live runs of the 1.0.0 build be96d8d40, whose status, article-lookup, Drugs@FDA and envelope code is identical to main.

## What the rules say now

`crates/biomcp-query/src/status.rs` and `sdlc/planning/architecture.md` section 6: `complete` means every requested source answered; `partial` means some requested data failed, is stale or is synchronizing; `absent` means every identity was confirmed absent; `failed` means no requested identity was answered. A search is classified from the providers it asked (`search_status`). A get is classified from identity plus requested sections (`item_status`).

## Article search: the rules allow it

"Requested" means the providers the caller named. Europe PMC's open-access lookup is a step the command adds by itself, so it is outside that set. `article/access.rs` `failed()` adds the failed Europe PMC row after `page.rs` `finish()` has classified the page. Ticket 1046 chose this ("the page status is unchanged"), and `tests/cases/article/search/pubmed-access-timeout.yaml` asserts `complete` beside `europepmc failed deadline-exceeded`. On main that case passes.

A second route reaches the same result. `status::search_for` classifies only the selected provider and treats the name-lookup reads as provenance (`drug/aliases.rs:467`, and also `trial/nci.rs`, `adverse_event/names.rs` and the LitVar candidates path). Across the whole suite, 85 replies say `complete` with a failed source row. 82 of them are article pages, in 17 cases and 32 runner trials: Europe PMC's lookup, 81 replay-miss and 1 deadline. The other 3 are `search drug` replies in 2 cases, with MyChem and ChEMBL name reads failed `bad-reply`. No `complete` reply has a stale or syncing row. Live today, 3 of 4 default article searches had the lookup time out at its 3-second budget.

## Drugs@FDA approvals: the command follows the rules

An efficacy supplement whose letter states no purpose marks a requested field unavailable, so the item is `partial` (`drug/drugsfda/purpose.rs:46`). Today a caller sees `sources` all `ok` and no `problems`. Only `result.approvals.data.records[2].supplement_purpose.state: unavailable` says what is missing, after about 3 KB of records. The suite holds 21 such `partial` replies with nothing at the top, in 6 reply kinds: drug approvals (10), article fulltext cards (4), Drugs@FDA sponsor search (3), PMA summary with no text (2), capped drug name get (1) and variant card ClinVar (1). Ticket 1068's PDF fix will reduce how often this happens, but it does not remove the need to explain it.

## Recommended rule

1. `complete`: every row in `sources` answered, or was skipped by the caller's or the policy's choice, and the reply marks nothing as missing. Any failed, stale or syncing row makes an otherwise answered reply `partial`. This holds whether the row is a named provider, a name lookup or a per-row lookup. `absent` and `failed` are unchanged.
2. Every `partial` reply carries `incomplete` right after `status`, and no other reply does. It is a list of `{at, source, reason?}`. `at` is a JSON pointer to where the detail already lives. Nothing moves, so the per-source and per-record detail stays as it is.
3. Enforce both rules in one place, where the envelope is built (`crates/biomcp/src/command.rs:284`). That code turns `complete` into `partial` when any source row failed, and derives entries from those rows and from result nodes marked `state: unavailable`. The few partial causes that leave no such marker add their entry where they set partial: PDF with no text, short sponsor read and capped name get. Today recall, companion, clearance and GWAS each patch stale status by hand; one owner replaces those patches.

Examples:

- Article: `"status":"partial","incomplete":[{"at":"/sources/1","source":"europepmc","reason":"deadline-exceeded"}]`. This adds 85 bytes; the timeout page goes from 1,929 to about 2,014 bytes, under its 2,219 ceiling.
- Drug: `"incomplete":[{"at":"/result/approvals/data/records/2/supplement_purpose","source":"drugsfda","reason":"not-stated"}]`. This adds about 115 bytes to a 4,745-byte reply, or 2.4 percent. `not-stated` is a new reason code for "letter read, no purpose statement found". Without it the entry carries no reason, and the PM may prefer that.

## Replies that change

- `complete` becomes `partial` in `search article` when the access lookup fails (every source set; research search and fetch through it) and in `search drug` when a name-lookup source fails. Exit codes stay the same (0 for both).
- `incomplete` appears on every `partial` reply of every command. Nothing else changes.

## Schema impact

One optional field on the shared `Envelope`. That is additive, so the compatibility check needs no version bump. About 316 generated schema files gain the one property. Add one release-notes line, and the `not-stated` reason code if accepted.

## Cases

- `article/search/pubmed-access-timeout`: expect `partial` and `/incomplete/0/source` `europepmc` with `deadline-exceeded`. This fails on base.
- New `drug/approvals-incomplete-top`: `get drug NDA216340 approvals --source drugsfda`, recorded live, expects `partial` and `/incomplete/0/at` pointing at record 2's `supplement_purpose`. This fails on base.
- `drug/source-aliases-review-evidence` (CLI and MCP): expect `partial` with MyChem and ChEMBL entries.
- A runner check over every replayed reply: no `complete` reply has a failed, stale or syncing row or an `incomplete` entry, and every `partial` reply has at least one entry. This guards the whole class, not two commands.
- 15 other article cases and the runner trials that now see a replay-miss Europe PMC row get a recorded lookup reply or `--no-access-lookup`, so each keeps its own claim.
- Docs: the architecture section 6 paragraph and rule row, and the `Status` doc comments.

## Decision for the PM

Rule 1 reverses 1046's choice. With Europe PMC's lookup taking 11 to 19 seconds against a 3-second budget (1046's measurements; it timed out 3 times in 4 tries today), most default article searches would read `partial` until that budget or endpoint is fixed. Recommendation: adopt rule 1 as the ticket's Outcome says, because the single `incomplete` line tells an agent that only open-access flags are missing. File the lookup budget as its own ticket. The alternative, keeping `complete` and only listing the lookup in `incomplete`, contradicts the Outcome.
