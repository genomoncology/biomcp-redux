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

## Progress

- Source: ClinicalTrials.gov returns each site's `geoPoint` and no distance or distance order, so BioMCP computes it. The recorded NCT00627419 reply (live, 2026-11-11) lists 17 sites; the first in source order, Arizona Cancer Center, has no `geoPoint`, so the case reuses that real recording rather than a new one.
- Change: `get trial ID locations` takes `--lat` and `--lon`, checked by the same helper `search trial` now uses (both or neither, numbers in range); a point without the locations section, or with `all`, is a usage error. With a point, every site row with coordinates gets `distance_mi`, the haversine straight-line miles (Earth radius 3,958.8 mi, rounded to 0.1), and the whole site list sorts nearest first before paging, sites without coordinates last in source order. Continuations of the locations section carry the point, so later pages keep the order. The reply echoes the point in `query.lat`/`query.lon`, and the `distance_mi` description says it is straight-line, not travel distance. Without a point, order, fields and paging are unchanged.
- Schema: the site row stays `Reply_TrialLocation` (a wrapper flattens the biodata row and adds optional `distance_mi`); the trial-get query adds optional `lat` and `lon`. `distance_mi` is a plain number: a checked `FiniteNumber` needed a dropped `.ok()` that the optional-value ratchet refuses, and checked degrees keep the haversine finite.
- Proof: `tests/cases/trial/sites/distance-nearest-first.yaml` expects, from 39.96,-83.00, Gabrail (Canton) 103.2, Pittsburgh 162.0, Indianapolis 168.0 first, Los Angeles 1973.6 fifteenth and Arizona last without a distance; the second page starts at Dana-Farber 642.2; no point keeps source order; `--lat` alone and a point without `locations` are refused. Expected miles were computed independently in Python. Base refuses `--lat` (the query denies unknown fields).
- Jobs: j42429 (regenerated schemas and docs; cases passed; `check-cheap` failed on the `.ok()` ratchet), j42435 (`check-cheap`, `trial/sites`, `mcp/parity` and `schema-release-check` passed).
