# 1094 — query-the-purple-book-roster-and-interchangeability

Status: COMPLETE.
Landed: 34ee27c673eeffef3eee57e8fad71e6e2e3bb93c

Milestone: 7

## Outcome

Query a dated FDA Purple Book release by reference application, strength and device, with the biosimilar roster, presentation-level licensure type and interchangeability dates the release supplies.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp search drug adalimumab --source purple-book` returned `value not accepted: give error.option a value valid_values accepts`. The workaround drew on separate drug labels and approval letters, and the requested download redirected to an error during the run.

New source: Purple Book downloadable data, https://purplebooksearch.fda.gov/downloads, U.S. Food and Drug Administration

## Proof

- Proof: Replay a saved monthly release with multiple presentations for one product and check that each keeps FDA’s own designation. Show an unavailable release as unavailable and preserve the last good release date.

## Starts from

- Starts from: Ticket 1067’s stored-list design, Drugs@FDA application identifiers and FDA’s dated download index.

## Keeps

- Keeps: A Drugs@FDA product flag stays attributed to Drugs@FDA.

## Defers

- Defers: Inferring designation or dates from manufacturer statements or missing letters.

## Changes

- Changes: Drug regulatory search, a Purple Book release reader, help and cases.
