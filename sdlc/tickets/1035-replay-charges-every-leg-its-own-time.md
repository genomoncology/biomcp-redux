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
- Stopped here to ask which shape to build before editing.
