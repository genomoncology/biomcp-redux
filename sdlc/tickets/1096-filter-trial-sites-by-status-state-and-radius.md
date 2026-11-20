# 1096 — filter-trial-sites-by-status-state-and-radius

Status: OPEN.

Milestone: 7

## Outcome

Filter the sites of a registered trial by site status, state and radius before paging, keep nearest-first order for a supplied point, and report the count of sites with missing coordinates.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get trial NCT04613596 locations --state Ohio --status RECRUITING --fields=facility,city,state,status,contacts,distance_mi` returned `"option":"--state"` and `unknown option: this command has no error.option`. The measured workaround read four pages totaling 155,728 bytes.

## Proof

- Proof: A saved trial includes recruiting and closed sites in two states, one beyond the radius and one without coordinates. Check combined filters, the pre-page count and the missing-coordinate count.

## Starts from

- Starts from: `crates/biomcp-query/src/trial/get/page/sites.rs` and ticket 1042’s distance behavior.

## Keeps

- Keeps: Unfiltered site reads, registry contacts and straight-line distance units remain available.

## Defers

- Defers: Geocoding sites with missing coordinates.

## Changes

- Changes: Trial location options, `crates/biomcp-query/src/trial/get/page/sites.rs`, help and cases.
