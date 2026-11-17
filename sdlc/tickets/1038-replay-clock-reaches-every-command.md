# 1038 — replay-clock-reaches-every-command

Status: OPEN.

Milestone: 7

## Outcome

Every command that reads a source runs on 1035's replay clock, legs read one after another inside a worker add up, and `Leg::dependency` keeps the looked-up source's own time limit live.

## Why

1035's review (2026-11-17) accepted it with these gaps:
- Commands started from the plain deadline get no command clock: gene get (`crates/biomcp-query/src/gene/get.rs:20`), drug aliases, resolve and markets, gene search, health. Nothing checks that new code uses the command deadline.
- Two legs used one after another inside one worker each start at the command's time (`crates/biomcp-fetch/src/deadline.rs:79-83`, `124-132`), e.g. a variant section `fan_out` job.
- `Leg::dependency` (`crates/biomcp-fetch/src/lib.rs:512-515`) no longer applies the source's own limit; a user-set PubMed or PMC OA limit under 3 s is not enforced there, live.
- Drop folds worker time only on the command's own thread (`deadline.rs:124-131`, `352-362`); fold unconditionally.

## Proof

- Proof: a replayed gene get case shows HGNC then section time adding, failing on the base commit; a check fails on any new command that bypasses the command deadline; a test shows `Leg::dependency` honours a configured source limit below its cap.

## Starts from

- Starts from: 1035's clock in `crates/biomcp-fetch/src/deadline.rs`.

## Keeps

- Keeps: live deadlines and pacing.

## Defers

- Defers: nested commands sharing an outer command's clock.

## Changes

- Changes: the listed commands' deadline start, worker leg chaining, `Leg::dependency`, one check and cases.

## Progress

- `Leg::dependency` (live fix, `crates/biomcp-fetch/src/lib.rs`): the dependency's end is again the earliest of the calling leg's end, `cap`, and the dependency source's own limit (the user's `source_deadlines` setting, else its declared one). 1035 had dropped the last. Live this matches the end before 1035 exactly; nothing else live changes. Test: `dependency_honours_a_configured_source_limit_below_its_cap` (a 2 s PubMed reply fails a 1 s user limit inside a 10 s cap, and passes a 5 s one, its time charged to the caller).
- Worker legs (`crates/biomcp-fetch/src/deadline.rs`): each worker thread keeps its own time on the command clock, so legs one after another inside a worker add up, and workers side by side still start together. A command dropped on any thread folds every worker's time. Test: `command_legs_charge_their_replay_time_one_after_another` gains two workers that each read two 3 s replies in turn (about 6 s, not 3 s), with the command dropped off its own thread.
- Commands: gene get and health now start on `command_deadline` (one clock; live end unchanged). Gene search opens one command per read (HGNC, then ClinGen), each with its own budget as live, so replay charges each in turn. The drug orchestrators (aliases search/get/with_aliases/spelling, resolve get/discover/chembl, markets get, search-names resolve) open no leg: they run nested commands, which already charge shared replay time one after another, and read the time charged. Moving them onto a command clock would freeze their budget, since nested commands do not join an outer command's clock (deferred). They keep the plain deadline with a stated exemption.
- Check: `crates/biomcp-query/clippy.toml` disallows `Fetch::deadline`, so a new command in biomcp-query that bypasses the command deadline fails clippy unless it states why.
- Case: `tests/cases/gene/sections/sequential-sources.yaml` replays gene/protein's CDKN2A recordings with HGNC at 18 s and UniProt at 2.5 s (`tests/replies/gene/sequential`): together they pass the 20 s command budget, so the protein section's UniProt read fails with deadline-exceeded.
