# 1042 — trial-sites-give-distance-nearest-first

Status: OPEN.

Milestone: 7

## Outcome

Given a point, a trial's site rows give each site's distance from it and come nearest first, so "which sites are closest to the patient" is one call. `get trial NCT… locations` takes `--lat` and `--lon` (the same spelling and checks as `search trial`), adds a distance in miles to every site row that has coordinates, orders the whole site list nearest first before paging, and puts sites without coordinates last. The reply says which point it measured from and that the distance is a straight line, not travel distance.

## Why

The patient-navigator persona sorted sites by hand in three runs: 2026-11-10 (biomed-agent-bench `personas/rollup.md`, gap backlog), 2026-11-12 (`personas/patient-navigator/runs/2026-11-12/log.md:167-178`, listed under "Waiting" in `records/2026-11-12-persona-review-findings.md`) and 2026-11-16 (`personas/patient-navigator/runs/2026-11-16/analysis.md:17`: "site rows carry coordinates and no distance, so the agent sorted by hand again"). 0083 deferred it because the registry does not return a distance, and 0995 deferred it until another run confirmed it. The radius search (`search trial --lat --lon --distance`) already finds the trials, but the agent still writes its own haversine sort to rank their sites.

## Proof

- Proof: a replayed case runs `get trial NCT… locations --lat 39.96 --lon -83.00` on a recorded multi-site trial and expects each row's distance and nearest-first order, with a site lacking coordinates last. It fails on the base commit, which refuses `--lat`.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/detail/sites.rs` (rows already carry `geoPoint` as `TrialCoordinates`), the locations section in `crates/biomcp-types/src/query/trial_get.rs`, and the `lat`/`lon` checks in `crates/biomcp-types/src/query/trial.rs`.

## Keeps

- Keeps: the locations section's order, fields and paging when no point is given; the radius search; NCI's refusal of the locations section.

## Defers

- Defers: travel time and distance on search rows. A search row could name its nearest site later if a persona run asks for it.

## Changes

- Changes: the trial get query and locations section, the site row schema (one optional field), a case and its recording, and one release-notes line.
