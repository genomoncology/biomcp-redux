# 1034 — replayed-name-lookups-finish-as-they-did-live

Status: OPEN.

Milestone: 7

## Outcome

A call recorded live and then replayed gives the same answer: name lookups that ran side by side live do not run out of time in replay. `search drug "dabigatran etexilate" --source fda-recall` (or the `get` form 1033 tried) replays as it ran live.

## Why

1033 (2026-11-17) dropped an FDA recall replay step: in replay the name lookup came back "name lookup unfinished" (`crates/biomcp-query/src/drug/search_names.rs:162`) and retried "Pradaxa", which was not recorded. Its review suspects replay charges each recorded request's delay one after another against one deadline (`crates/biomcp-fetch/src/request.rs:467-473`), so lookups that ran in parallel live add up past it. If so, any source that fans out name lookups is affected, and replay cases can't cover them.

## Proof

- Proof: 1033's dropped recall step, recorded live and replayed, matches the live answer; it fails on the base commit.

## Starts from

- Starts from: the replay delay accounting and the drug name fan-out.

## Keeps

- Keeps: live deadlines and pacing.

## Defers

- Defers: nothing.

## Changes

- Changes: replay timing, the recall case, its recording.

## Progress

- Reproduced with request timing traces (temporary commits, since reverted). j42224 and j42226 recorded `get drug "dabigatran etexilate" --source fda-drug-recall` live on main and replayed it. The lookups do not run side by side: the alias retries run one after another, live and in replay, so the suspected adding-up of parallel delays is not the cause.
- Cause: in replay, a command that joins its time from a source leg (`command.finish(leg.remaining())`) charged the gap between the command's budget and the source's own cap as spent time. The leg's end is capped at the source deadline (MyChem 15 s), and `CommandDeadline::finish` turns "time left" into "time spent" against the command's end, so a 19 s command over a 15 s leg charged about 4 s that no request took. j42226 shows it: MyChem's name read for the alias lookup finished at 2.08 s of replay time and the next request started at 6.01 s, with no request or pacing wait in between; live, the same step took 1 s. Live is unaffected because live time is the wall clock. Every later lookup then got a smaller share, so with slower recorded replies (j42203's recording, 1033) the last name lookups ran out and the call took a path that was never recorded.
- Change: `Leg::command_remaining` reads the time left on the deadline the leg was opened under, on the leg's clock, and the ten callers that joined a command from a leg's remaining time use it (drug MyChem search and get, name lookups, native name pages, ChEMBL/Open Targets native reads, openFDA label reads, trial documents). Live deadlines and pacing do not change.
- Case: 1033's dropped step is back in `tests/cases/drug/spaced-names.yaml`, replaying j42203's live recording (27 replies), asserting absent, the ten names its one recall query searched, and "one recall query", as live answered.
- j42228 (at d35498180): schemas and docs regenerate unchanged; `drug/` cases (90) and all `biomcp-fetch` tests pass. Base check in the same job, with main's `crates/` and this case: step 4 fails as in 1033, exit 5, `recording-missing`, "name lookup unfinished; typed only", then a replay miss on the "Pradaxa" retry. check-cheap failed one lint: holding a whole second `Deadline` on `Leg` pushed an enum holding a leg past clippy's size limit, so the leg now keeps only the opened deadline's end (f1eaae32f).
