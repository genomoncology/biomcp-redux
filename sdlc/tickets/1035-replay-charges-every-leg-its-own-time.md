# 1035 — replay-charges-every-leg-its-own-time

Status: OPEN.

Milestone: 7

## Outcome

Every command that runs a source leg charges that leg's replay time to the command's replay clock, so a replayed case takes the same path live would and replay cases catch a change that makes a call too slow for its live limit.

## Why

1034's review (2026-11-17) found about 100 sites ending with `command.finish(deadline.remaining())` where the leg's request time is never added to the command deadline, so replay runs faster than live. Example: recall search (`crates/biomcp-query/src/drug/recall.rs:81`) leaves about 1.5-2 s of alias-walk recall queries uncharged. A replay miss still fails loudly, so no wrong answer ships, but replay can pass a call that would time out live.

## Proof

- Proof: a replayed case whose live run nears its limit (for example the recall alias walk) shows replay's elapsed time matching live's within pacing, and fails on the base commit; a check or test lists no remaining site that finishes a command without its leg's time.

## Starts from

- Starts from: 1034's `Leg::command_remaining` and the `command.finish` sites.

## Keeps

- Keeps: live deadlines and pacing.

## Defers

- Defers: nothing.

## Changes

- Changes: the finish sites, one case, and a doc note on `Leg::sibling`/`Leg::dependency` that `command_remaining` measures against the leg's opened end.

## Progress

- Survey at d42922dfe (no build yet): 103 `command.finish(deadline.remaining())` sites in 91 functions. The command deadline is frozen at its replay origin, so `deadline.remaining()` is the whole budget and each of these charges zero replay time.
- 47 sites in 42 functions open exactly one leg in the same function and hand `deadline` to nothing else (for example drug recall search and get, WHO EML, CIViC, EMA, Drugs@FDA, diagnostic, disease OLS/SEER/Monarch, phenotype, GWAS search, pgx). Each is a two-line edit: read `leg.command_remaining()` before `leg.finish()` and pass it to `command.finish`.
- 56 sites in 49 functions are not: the legs open in helpers that take `deadline` (article, author, adverse event, trial, variant, protein relations, pathway search, NCI search and others), several legs share one function (protein get reads HGNC then UniProt; both fork from the same origin, so taking the later end still counts them as running side by side, not one after another as live does), or the command already advances stage by stage with `after_stage` and misses only its last stage (variant get, article get, adverse-event name widening). These need each helper to return its leg's remaining time, or a change in `biomcp-fetch` so a finished leg reports its end to the command.
- Stopped to ask which shape to build. Coordinator (2026-11-17): option 2 with a shared command replay clock in the fetch layer, so legs opened one after another add up and legs opened side by side share a start; live timing unchanged; a check that no site can bypass the clock.
- Change (`crates/biomcp-fetch/src/deadline.rs`, `lib.rs`): each replay command gets its own clock. A leg opened under the command starts at the clock's current time, catches up to it before each step, and moves it forward with every request and pacing wait. So HGNC then UniProt add up even though protein get opens both legs first. Legs on worker threads (batch workers, `fan_out` jobs) hold their time apart until the command's thread reads the clock again, so every worker starts from the same instant whatever order the threads finish in, and the command reaches the latest one. The command's own deadline reads the clock, so the 103 `command.finish(deadline.remaining())` sites now charge their legs and needed no edit; `finish` can only add time, and the command also charges its clock to shared time when it is dropped, so a site that returns early or skips `finish` still charges. A dependency starts at its calling leg's time on a clock of its own and is charged back to that leg, as before. Live has no clock, so live deadlines and pacing do not change. The check is the fetch test `command_legs_charge_their_replay_time_one_after_another`: two legs opened first and read one after another take about 6 s of 3 s replies, two workers side by side take about 3 s, and the command's deadline sees both.
- Case: `tests/cases/protein/sequential-sources.yaml` replays synthetic copies of protein/synthetic's HGNC set and UniProt P00533 entry with delays of 13 s and 2.5 s (UniProt's stall retry starts at 3 s, so its reply stays under that). Together they pass UniProt's 15-second limit, so `get protein EGFR` fails with UniProt deadline-exceeded, as live would.
- j42253 (at a2f957e0d): all 1,031 existing replay cases pass unchanged with the shared clock, so no case's timing shifted and nothing was re-recorded. The new case's first delays (8 s and 9 s) hit UniProt's stall retry, and check-cheap found one unneeded lint expectation and `biomcp-fetch/src/lib.rs` over its 600-line limit; all were fixed in 665b96201.
- j42256 (at 665b96201): check-cheap, all `biomcp-fetch` tests, `protein/` cases (17) and `drug/spaced-names` pass. Base check: on main's `crates/` the protein case fails (UniProt reads complete, since its 2.5 s starts at the command's start).
- Recall alias walk (j42256, with 1034's request trace applied to both trees, `get drug "dabigatran etexilate" --source fda-drug-recall` replaying j42203's recording): with the fix, the last request ends at 13.4 s of replay time; live, the recorded fetch times run from second 1794895875 to 1794895889, about 14 s at one-second resolution. On main the request timeline is the same, because MyChem's 1-per-second pacing re-syncs each step, but the command clock lags, and each alias lookup starts with the previous recall query's time (about 150-380 ms) still uncharged, for example `BIBR-1048` with 15,765 ms left on main and 15,613 ms with the fix. So the walk cannot fail on main; the protein case is the one that does.
- Gate: j42257 `make --keep-going landing-gate docs-check schema-release-check` passed at 829b0e574 (branched from origin/main d42922dfe).
